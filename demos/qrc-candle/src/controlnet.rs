//! A ControlNet model for candle Stable Diffusion.
//!
//! candle ships the UNet hook (`forward_with_additional_residuals`) but not a
//! ControlNet model to produce the residuals. This module is a port of the
//! diffusers `ControlNetModel` (SD1.5 layout): it shares the UNet's encoder
//! (down blocks + mid block) plus a conditioning embedding for the control
//! hint and the zero-convolution output projections.
//!
//! Weight names follow the diffusers checkpoint layout, so a standard SD1.5
//! ControlNet safetensors (e.g. a QR-code ControlNet) loads directly.

use anyhow::Result;
use candle_core::{Module, Tensor};
use candle_nn::{conv2d, Conv2d, Conv2dConfig, VarBuilder};
use candle_transformers::models::stable_diffusion::embeddings::{TimestepEmbedding, Timesteps};
use candle_transformers::models::stable_diffusion::unet_2d::UNet2DConditionModelConfig;
use candle_transformers::models::stable_diffusion::unet_2d_blocks::{
    CrossAttnDownBlock2D, CrossAttnDownBlock2DConfig, DownBlock2D, DownBlock2DConfig,
    UNetMidBlock2DCrossAttn, UNetMidBlock2DCrossAttnConfig,
};

/// A down block, mirroring the UNet's cross-attention vs. plain split.
enum DownBlock {
    Cross(CrossAttnDownBlock2D),
    Basic(DownBlock2D),
}

/// Processes the control hint image into the UNet's first feature space.
/// (diffusers `ControlNetConditioningEmbedding`.)
struct CondEmbedding {
    conv_in: Conv2d,
    blocks: Vec<Conv2d>,
    conv_out: Conv2d,
}

impl CondEmbedding {
    fn new(vs: VarBuilder, conditioning_channels: usize, out_channels: usize) -> Result<Self> {
        let pad = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };
        let stride2 = Conv2dConfig {
            padding: 1,
            stride: 2,
            ..Default::default()
        };
        let channels = [16usize, 32, 96, 256];
        let conv_in = conv2d(conditioning_channels, channels[0], 3, pad, vs.pp("conv_in"))?;
        let vb = vs.pp("blocks");
        let mut blocks = Vec::new();
        for i in 0..channels.len() - 1 {
            let (ci, co) = (channels[i], channels[i + 1]);
            blocks.push(conv2d(ci, ci, 3, pad, vb.pp(blocks.len().to_string()))?);
            blocks.push(conv2d(ci, co, 3, stride2, vb.pp(blocks.len().to_string()))?);
        }
        let conv_out = conv2d(
            *channels.last().unwrap(),
            out_channels,
            3,
            pad,
            vs.pp("conv_out"),
        )?;
        Ok(CondEmbedding {
            conv_in,
            blocks,
            conv_out,
        })
    }

    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let mut xs = candle_nn::ops::silu(&self.conv_in.forward(xs)?)?;
        for block in &self.blocks {
            xs = candle_nn::ops::silu(&block.forward(&xs)?)?;
        }
        Ok(self.conv_out.forward(&xs)?)
    }
}

/// A ControlNet that conditions a Stable-Diffusion UNet on a control image.
pub struct ControlNet {
    conv_in: Conv2d,
    time_proj: Timesteps,
    time_embedding: TimestepEmbedding,
    cond_embedding: CondEmbedding,
    down_blocks: Vec<DownBlock>,
    mid_block: UNetMidBlock2DCrossAttn,
    controlnet_down_blocks: Vec<Conv2d>,
    controlnet_mid_block: Conv2d,
}

impl ControlNet {
    /// Builds a ControlNet from a diffusers-layout `VarBuilder`. `in_channels`
    /// is the latent channel count (4) and `config` the matching UNet config.
    pub fn new(
        vs: VarBuilder,
        in_channels: usize,
        use_flash_attn: bool,
        config: &UNet2DConditionModelConfig,
    ) -> Result<Self> {
        let n_blocks = config.blocks.len();
        let b_channels = config.blocks[0].out_channels;
        let bl_channels = config.blocks.last().unwrap().out_channels;
        let bl_attn = config.blocks.last().unwrap().attention_head_dim;
        let time_embed_dim = b_channels * 4;
        let conv_cfg = Conv2dConfig {
            padding: 1,
            ..Default::default()
        };

        let conv_in = conv2d(in_channels, b_channels, 3, conv_cfg, vs.pp("conv_in"))?;
        let time_proj = Timesteps::new(b_channels, config.flip_sin_to_cos, config.freq_shift);
        let time_embedding =
            TimestepEmbedding::new(vs.pp("time_embedding"), b_channels, time_embed_dim)?;
        let cond_embedding = CondEmbedding::new(vs.pp("controlnet_cond_embedding"), 3, b_channels)?;

        // Down blocks — identical construction (and weight paths) to the UNet.
        let vs_db = vs.pp("down_blocks");
        let mut down_blocks = Vec::with_capacity(n_blocks);
        for i in 0..n_blocks {
            let cfg = config.blocks[i];
            let sliced = match config.sliced_attention_size {
                Some(0) => Some(cfg.attention_head_dim / 2),
                other => other,
            };
            let in_c = if i > 0 {
                config.blocks[i - 1].out_channels
            } else {
                b_channels
            };
            let db_cfg = DownBlock2DConfig {
                num_layers: config.layers_per_block,
                resnet_eps: config.norm_eps,
                resnet_groups: config.norm_num_groups,
                add_downsample: i < n_blocks - 1,
                downsample_padding: config.downsample_padding,
                ..Default::default()
            };
            let vb = vs_db.pp(i.to_string());
            let block = if let Some(layers) = cfg.use_cross_attn {
                DownBlock::Cross(CrossAttnDownBlock2D::new(
                    vb,
                    in_c,
                    cfg.out_channels,
                    Some(time_embed_dim),
                    use_flash_attn,
                    CrossAttnDownBlock2DConfig {
                        downblock: db_cfg,
                        attn_num_head_channels: cfg.attention_head_dim,
                        cross_attention_dim: config.cross_attention_dim,
                        sliced_attention_size: sliced,
                        use_linear_projection: config.use_linear_projection,
                        transformer_layers_per_block: layers,
                    },
                )?)
            } else {
                DownBlock::Basic(DownBlock2D::new(
                    vb,
                    in_c,
                    cfg.out_channels,
                    Some(time_embed_dim),
                    db_cfg,
                )?)
            };
            down_blocks.push(block);
        }

        let mid_block = UNetMidBlock2DCrossAttn::new(
            vs.pp("mid_block"),
            bl_channels,
            Some(time_embed_dim),
            use_flash_attn,
            UNetMidBlock2DCrossAttnConfig {
                resnet_eps: config.norm_eps,
                output_scale_factor: config.mid_block_scale_factor,
                cross_attn_dim: config.cross_attention_dim,
                attn_num_head_channels: bl_attn,
                resnet_groups: Some(config.norm_num_groups),
                use_linear_projection: config.use_linear_projection,
                transformer_layers_per_block: config
                    .blocks
                    .last()
                    .and_then(|b| b.use_cross_attn)
                    .unwrap_or(1),
                ..Default::default()
            },
        )?;

        // Zero-conv output projections — one per down residual plus the mid.
        let zero_cfg = Conv2dConfig::default();
        let mut res_channels = vec![b_channels];
        for i in 0..n_blocks {
            let oc = config.blocks[i].out_channels;
            for _ in 0..config.layers_per_block {
                res_channels.push(oc);
            }
            if i < n_blocks - 1 {
                res_channels.push(oc);
            }
        }
        let vb_cdb = vs.pp("controlnet_down_blocks");
        let controlnet_down_blocks = res_channels
            .iter()
            .enumerate()
            .map(|(i, &c)| conv2d(c, c, 1, zero_cfg, vb_cdb.pp(i.to_string())))
            .collect::<candle_core::Result<Vec<_>>>()?;
        let controlnet_mid_block = conv2d(
            bl_channels,
            bl_channels,
            1,
            zero_cfg,
            vs.pp("controlnet_mid_block"),
        )?;

        Ok(ControlNet {
            conv_in,
            time_proj,
            time_embedding,
            cond_embedding,
            down_blocks,
            mid_block,
            controlnet_down_blocks,
            controlnet_mid_block,
        })
    }

    /// Runs the ControlNet, returning `(down_block_residuals, mid_residual)`
    /// scaled by `conditioning_scale`, ready for
    /// `UNet2DConditionModel::forward_with_additional_residuals`.
    pub fn forward(
        &self,
        sample: &Tensor,
        timestep: f64,
        encoder_hidden_states: &Tensor,
        controlnet_cond: &Tensor,
        conditioning_scale: f64,
    ) -> Result<(Vec<Tensor>, Tensor)> {
        let (bsize, _c, _h, _w) = sample.dims4()?;
        let device = sample.device();
        let emb = (Tensor::ones(bsize, sample.dtype(), device)? * timestep)?;
        let emb = self.time_proj.forward(&emb)?;
        let emb = self.time_embedding.forward(&emb)?;

        // The hint is added to the latent feature map before the encoder.
        let xs = self.conv_in.forward(sample)?;
        let mut xs = (xs + self.cond_embedding.forward(controlnet_cond)?)?;

        let mut down_res = vec![xs.clone()];
        for block in &self.down_blocks {
            let (x, res) = match block {
                DownBlock::Cross(b) => b.forward(&xs, Some(&emb), Some(encoder_hidden_states))?,
                DownBlock::Basic(b) => b.forward(&xs, Some(&emb))?,
            };
            down_res.extend(res);
            xs = x;
        }
        let mid = self
            .mid_block
            .forward(&xs, Some(&emb), Some(encoder_hidden_states))?;

        // Project through the zero-convs and scale.
        let mut down_residuals = Vec::with_capacity(self.controlnet_down_blocks.len());
        for (res, conv) in down_res.iter().zip(&self.controlnet_down_blocks) {
            down_residuals.push((conv.forward(res)? * conditioning_scale)?);
        }
        let mid_residual = (self.controlnet_mid_block.forward(&mid)? * conditioning_scale)?;
        Ok((down_residuals, mid_residual))
    }
}
