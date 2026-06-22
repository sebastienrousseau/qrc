//! Local AI art-QR demo.
//!
//! Pipeline: `qrc` renders a high-ECC **control image**, then a local
//! Stable-Diffusion pass (via [candle]) with a true **ControlNet** paints
//! artwork while the control image constrains the structure so the result still
//! scans. Weights are pulled from the Hugging Face Hub at run time (multi-GB; a
//! GPU is strongly recommended).
//!
//! ```sh
//! cargo run --release -- \
//!   --data "https://example.com" \
//!   --prompt "a serene koi pond, japanese ink wash painting" \
//!   --output art.png
//! ```
//!
//! [candle]: https://github.com/huggingface/candle

mod controlnet;

use anyhow::{Context, Result};
use candle_core::{DType, Device, IndexOp, Module, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::stable_diffusion::unet_2d::{
    BlockConfig, UNet2DConditionModelConfig,
};
use candle_transformers::models::stable_diffusion::{self, StableDiffusionConfig};
use clap::Parser;
use std::path::PathBuf;
use tokenizers::Tokenizer;

use controlnet::ControlNet;
use qrc::encode::{Ecc, QrOptions};
use qrc::render::control::ControlOptions;
use qrc::QRCode;

/// Local AI art-QR generator (qrc control image + candle Stable Diffusion + ControlNet).
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

    /// ControlNet conditioning scale: higher constrains the art more tightly to
    /// the QR structure (more scannable, less free); ~1.1–2.0 is typical.
    #[arg(long, default_value_t = 1.5)]
    conditioning_scale: f64,

    /// Classifier-free guidance scale.
    #[arg(long, default_value_t = 7.5)]
    guidance_scale: f64,

    /// Square size of the control image / generated art in pixels.
    #[arg(long, default_value_t = 768)]
    size: usize,

    /// Hugging Face repo of the SD1.5 QR ControlNet.
    #[arg(long, default_value = "monster-labs/control_v1p_sd15_qrcode_monster")]
    controlnet_repo: String,

    /// ControlNet weights filename within the repo.
    #[arg(long, default_value = "diffusion_pytorch_model.safetensors")]
    controlnet_file: String,

    /// RNG seed.
    #[arg(long, default_value_t = 42)]
    seed: u64,

    /// Force CPU even if a GPU is available (very slow).
    #[arg(long)]
    cpu: bool,
}

/// Stable-Diffusion v1.5 weight locations on the Hugging Face Hub.
mod hub {
    pub const SD15_REPO: &str = "stable-diffusion-v1-5/stable-diffusion-v1-5";
    pub const CLIP_TOKENIZER_REPO: &str = "openai/clip-vit-base-patch32";
    pub const VAE: &str = "vae/diffusion_pytorch_model.safetensors";
    pub const UNET: &str = "unet/diffusion_pytorch_model.safetensors";
    pub const CLIP: &str = "text_encoder/model.safetensors";
}

/// The SD1.5 UNet config (also used to lay out the matching ControlNet).
fn v15_unet_config() -> UNet2DConditionModelConfig {
    let bc = |out_channels, use_cross_attn, attention_head_dim| BlockConfig {
        out_channels,
        use_cross_attn,
        attention_head_dim,
    };
    UNet2DConditionModelConfig {
        blocks: vec![
            bc(320, Some(1), 8),
            bc(640, Some(1), 8),
            bc(1280, Some(1), 8),
            bc(1280, None, 8),
        ],
        center_input_sample: false,
        cross_attention_dim: 768,
        downsample_padding: 1,
        flip_sin_to_cos: true,
        freq_shift: 0.,
        layers_per_block: 2,
        mid_block_scale_factor: 1.,
        norm_eps: 1e-5,
        norm_num_groups: 32,
        sliced_attention_size: None,
        use_linear_projection: false,
    }
}

/// Downloads (and caches) a file from a Hugging Face repo, returning its path.
fn hf_file(repo: &str, filename: &str) -> Result<PathBuf> {
    let api = hf_hub::api::sync::Api::new()?;
    Ok(api.model(repo.to_string()).get(filename)?)
}

/// Loads the qrc control image as a `[0, 1]` tensor of shape `(2, 3, size, size)`
/// (duplicated for classifier-free guidance), as ControlNet expects.
fn control_cond_tensor(args: &Args, device: &Device) -> Result<Tensor> {
    let qr = QRCode::from_string(args.data.clone());
    let rgba = qr
        .to_control_image(
            &QrOptions::new().ecc(Ecc::High),
            &ControlOptions::with_size(args.size as u32),
        )
        .map_err(|e| anyhow::anyhow!("control image: {e}"))?;
    let rgb = image::DynamicImage::ImageRgba8(rgba)
        .resize_exact(
            args.size as u32,
            args.size as u32,
            image::imageops::FilterType::Nearest,
        )
        .to_rgb8();
    let (w, h) = rgb.dimensions();
    let t = Tensor::from_vec(rgb.into_raw(), (h as usize, w as usize, 3), device)?
        .permute((2, 0, 1))?
        .to_dtype(DType::F32)?;
    let t = (t / 255.0)?.unsqueeze(0)?;
    Ok(Tensor::cat(&[&t, &t], 0)?)
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
        tokens.resize(config.clip.max_position_embeddings, pad_id);
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
    let text_embeddings = text_embeddings(args, &config, &device)?;

    // VAE (decode) + UNet + ControlNet.
    let vae = config.build_vae(hf_file(hub::SD15_REPO, hub::VAE)?, &device, DType::F32)?;
    let unet = config.build_unet(
        hf_file(hub::SD15_REPO, hub::UNET)?,
        &device,
        4,
        false,
        DType::F32,
    )?;
    let cn_weights = hf_file(&args.controlnet_repo, &args.controlnet_file)?;
    let cn_vs = unsafe { VarBuilder::from_mmaped_safetensors(&[cn_weights], DType::F32, &device)? };
    let controlnet = ControlNet::new(cn_vs, 4, false, &v15_unet_config())?;

    let control_cond = control_cond_tensor(args, &device)?;

    // Start from pure noise (text2img), conditioned by ControlNet every step.
    let mut scheduler = config.build_scheduler(args.steps)?;
    let latent = args.size / 8;
    device.set_seed(args.seed)?;
    let mut latents = (Tensor::randn(0f32, 1f32, (1, 4, latent, latent), &device)?
        * scheduler.init_noise_sigma())?;

    for &timestep in scheduler.timesteps().to_vec().iter() {
        let input = Tensor::cat(&[&latents, &latents], 0)?;
        let input = scheduler.scale_model_input(input, timestep)?;
        let t = timestep as f64;

        let (down_residuals, mid_residual) = controlnet.forward(
            &input,
            t,
            &text_embeddings,
            &control_cond,
            args.conditioning_scale,
        )?;
        let noise_pred = unet.forward_with_additional_residuals(
            &input,
            t,
            &text_embeddings,
            Some(down_residuals.as_slice()),
            Some(&mid_residual),
        )?;

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
    println!("Tip: verify it scans; raise --conditioning-scale if it doesn't.");
    Ok(())
}

fn main() -> Result<()> {
    run(&Args::parse())
}
