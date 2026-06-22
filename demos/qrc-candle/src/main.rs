//! Local AI art-QR demo.
//!
//! Pipeline: `qrc` renders a high-ECC **control image**, then a local
//! Stable-Diffusion **img2img** pass (via [candle]) paints artwork while the
//! control image biases the result toward the QR structure. Weights are pulled
//! from the Hugging Face Hub at run time (multi-GB; a GPU is strongly
//! recommended).
//!
//! This is intentionally an img2img pipeline (control image as the denoising
//! seed) because it works with stock `candle-transformers`. True ControlNet
//! conditioning — injecting the control features into every UNet block — would
//! scan more reliably; see the note in `README.md`.
//!
//! ```sh
//! cargo run --release -- \
//!   --data "https://example.com" \
//!   --prompt "a serene koi pond, japanese ink wash painting" \
//!   --output art.png
//! ```
//!
//! [candle]: https://github.com/huggingface/candle

use anyhow::{Context, Result};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_transformers::models::stable_diffusion::{self, StableDiffusionConfig};
use clap::Parser;
use std::path::PathBuf;
use tokenizers::Tokenizer;

use qrc::encode::{Ecc, QrOptions};
use qrc::render::control::ControlOptions;
use qrc::QRCode;

/// Local AI art-QR generator (qrc control image + candle Stable Diffusion).
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Data to encode in the QR code (URL, text, …).
    #[arg(long)]
    data: String,

    /// Text prompt describing the desired artwork.
    #[arg(long)]
    prompt: String,

    /// Negative prompt (things to avoid).
    #[arg(long, default_value = "ugly, blurry, low quality, deformed, watermark")]
    negative_prompt: String,

    /// Where to write the generated PNG.
    #[arg(long, default_value = "art-qr.png")]
    output: PathBuf,

    /// Number of denoising steps.
    #[arg(long, default_value_t = 30)]
    steps: usize,

    /// img2img strength in `0.0..=1.0`: how far from the control image to roam.
    /// Lower keeps the QR more intact (more scannable); higher is more artistic.
    #[arg(long, default_value_t = 0.6)]
    strength: f64,

    /// Classifier-free guidance scale.
    #[arg(long, default_value_t = 7.5)]
    guidance_scale: f64,

    /// Square size of the control image / generated art in pixels.
    #[arg(long, default_value_t = 768)]
    size: usize,

    /// RNG seed.
    #[arg(long, default_value_t = 42)]
    seed: u64,

    /// Force CPU even if a GPU is available (very slow).
    #[arg(long)]
    cpu: bool,
}

/// The Stable-Diffusion v1.5 weights/repo locations on the Hugging Face Hub.
mod hub {
    pub const SD15_REPO: &str = "stable-diffusion-v1-5/stable-diffusion-v1-5";
    pub const CLIP_TOKENIZER_REPO: &str = "openai/clip-vit-base-patch32";
    pub const VAE: &str = "vae/diffusion_pytorch_model.safetensors";
    pub const UNET: &str = "unet/diffusion_pytorch_model.safetensors";
    pub const CLIP: &str = "text_encoder/model.safetensors";
}

/// Downloads (and caches) a file from a Hugging Face repo, returning its path.
fn hf_file(repo: &str, filename: &str) -> Result<PathBuf> {
    let api = hf_hub::api::sync::Api::new()?;
    Ok(api.model(repo.to_string()).get(filename)?)
}

/// Renders the qrc control image and loads it as a normalised `[-1, 1]` tensor
/// of shape `(1, 3, size, size)` for the VAE encoder.
fn control_image_tensor(args: &Args, device: &Device) -> Result<Tensor> {
    let qr = QRCode::from_string(args.data.clone());
    let rgba = qr
        .to_control_image(
            &QrOptions::new().ecc(Ecc::High),
            &ControlOptions::with_size(args.size as u32),
        )
        .map_err(|e| anyhow::anyhow!("control image: {e}"))?;
    // qrc may grow the canvas to fit whole modules; force the exact size.
    let rgb = image::DynamicImage::ImageRgba8(rgba)
        .resize_exact(
            args.size as u32,
            args.size as u32,
            image::imageops::FilterType::Nearest,
        )
        .to_rgb8();
    let (w, h) = rgb.dimensions();
    let data = rgb.into_raw();
    let t = Tensor::from_vec(data, (h as usize, w as usize, 3), device)?
        .permute((2, 0, 1))?
        .to_dtype(DType::F32)?;
    // [0,255] -> [-1,1], then add the batch dimension.
    let t = ((t / 255.0)? * 2.0)?;
    let t = t.broadcast_sub(&Tensor::new(1f32, device)?)?;
    Ok(t.unsqueeze(0)?)
}

/// Builds the conditional+unconditional text embeddings for guidance.
fn text_embeddings(args: &Args, config: &StableDiffusionConfig, device: &Device) -> Result<Tensor> {
    let tokenizer_path = hf_file(hub::CLIP_TOKENIZER_REPO, "tokenizer.json")?;
    let tokenizer =
        Tokenizer::from_file(tokenizer_path).map_err(|e| anyhow::anyhow!("tokenizer: {e}"))?;
    let vocab = tokenizer.get_vocab(true);
    let pad_id = match &config.clip.pad_with {
        Some(pad) => *vocab.get(pad.as_str()).context("pad token not in vocab")?,
        None => *vocab
            .get("<|endoftext|>")
            .context("eot token not in vocab")?,
    };

    let encode = |text: &str| -> Result<Vec<u32>> {
        let mut tokens = tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::anyhow!("encode: {e}"))?
            .get_ids()
            .to_vec();
        let max = config.clip.max_position_embeddings;
        tokens.resize(max, pad_id);
        Ok(tokens)
    };

    let clip_weights = hf_file(hub::SD15_REPO, hub::CLIP)?;
    let text_model =
        stable_diffusion::build_clip_transformer(&config.clip, clip_weights, device, DType::F32)?;

    let cond = Tensor::new(encode(&args.prompt)?, device)?.unsqueeze(0)?;
    let uncond = Tensor::new(encode(&args.negative_prompt)?, device)?.unsqueeze(0)?;
    let cond = text_model.forward(&cond)?;
    let uncond = text_model.forward(&uncond)?;
    // [uncond; cond] for classifier-free guidance.
    Ok(Tensor::cat(&[uncond, cond], 0)?)
}

fn run(args: &Args) -> Result<()> {
    let device = if args.cpu {
        Device::Cpu
    } else {
        Device::cuda_if_available(0).unwrap_or(Device::Cpu)
    };
    println!("device: {device:?}");

    let config = StableDiffusionConfig::v1_5(None, Some(args.size), Some(args.size));

    // Text conditioning.
    let text_embeddings = text_embeddings(args, &config, &device)?;

    // VAE + UNet.
    let vae = config.build_vae(hf_file(hub::SD15_REPO, hub::VAE)?, &device, DType::F32)?;
    let unet = config.build_unet(
        hf_file(hub::SD15_REPO, hub::UNET)?,
        &device,
        4,
        false,
        DType::F32,
    )?;

    // Encode the control image into latents (img2img seed).
    let control = control_image_tensor(args, &device)?;
    let init_dist = vae.encode(&control)?;
    let init_latents = (init_dist.sample()? * 0.18215)?;

    // Scheduler — start partway through the schedule per `strength`.
    let mut scheduler = config.build_scheduler(args.steps)?;
    let timesteps = scheduler.timesteps().to_vec();
    let t_start =
        args.steps - ((args.steps as f64) * args.strength).min(args.steps as f64) as usize;
    let start_t = timesteps[t_start.min(timesteps.len() - 1)];

    device.set_seed(args.seed)?;
    let noise = init_latents.randn_like(0.0, 1.0)?;
    let mut latents = scheduler.add_noise(&init_latents, noise, start_t)?;

    for &timestep in timesteps.iter().skip(t_start) {
        let input = Tensor::cat(&[&latents, &latents], 0)?;
        let input = scheduler.scale_model_input(input, timestep)?;
        let noise_pred = unet.forward(&input, timestep as f64, &text_embeddings)?;

        let chunks = noise_pred.chunk(2, 0)?;
        let (uncond, cond) = (&chunks[0], &chunks[1]);
        let guided = (uncond + ((cond - uncond)? * args.guidance_scale)?)?;
        latents = scheduler.step(&guided, timestep, &latents)?;
    }

    // Decode latents -> image and save.
    let image = vae.decode(&(&latents / 0.18215)?)?;
    let image = ((image / 2.0)? + 0.5)?.clamp(0.0, 1.0)?;
    let image = (image * 255.0)?.to_dtype(DType::U8)?.i(0)?;
    let (channels, height, width) = image.dims3()?;
    anyhow::ensure!(channels == 3, "expected 3 channels");
    let pixels = image.permute((1, 2, 0))?.flatten_all()?.to_vec1::<u8>()?;
    let buffer = image::RgbImage::from_raw(width as u32, height as u32, pixels)
        .context("failed to build output image")?;
    buffer.save(&args.output)?;
    println!("wrote {}", args.output.display());
    println!("Tip: verify it scans before printing; lower --strength if it doesn't.");
    Ok(())
}

fn main() -> Result<()> {
    run(&Args::parse())
}
