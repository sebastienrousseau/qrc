# qrc-candle — local AI art-QR demo

A proof-of-concept CLI that generates an **AI art QR code entirely on your
machine**: [`qrc`](../..) renders a high-error-correction *control image*, and a
local [candle](https://github.com/huggingface/candle) Stable-Diffusion pass
paints artwork guided by it.

This crate is **standalone** — it declares its own `[workspace]`, so it is fully
detached from the `qrc` package's build, tests, CI and coverage. None of its
heavy ML dependencies touch the core library.

## How it relates to the other options

`qrc` gives you three ways to make a code that doesn't look like a plain QR:

| Option | Where | Network | Look |
| ------ | ----- | ------- | ---- |
| Offline blend (`to_art_image`) | core, `raster` | none | image woven into the code |
| Cloud generation (`to_ai_art`) | core, `api` | cloud API | full AI art |
| **Local generation (this crate)** | demo | none (after weight download) | full AI art |
| Control-image export (`to_control_image`) | core, `raster` | none | feeds any pipeline |

This demo is the **local** counterpart to the `api` feature: same idea, but the
diffusion model runs on your hardware instead of a hosted service.

## Run it

```sh
cd demos/qrc-candle
cargo run --release -- \
  --data "https://example.com" \
  --prompt "a serene koi pond, japanese ink wash painting, soft light" \
  --conditioning-scale 1.5 \
  --output art-qr.png
```

On first run the Stable-Diffusion v1.5 weights (~4 GB) plus the QR ControlNet
(~1.4 GB) are downloaded from the Hugging Face Hub and cached. A CUDA GPU is
strongly recommended; `--cpu` works but is very slow.

Key flags: `--steps`, `--conditioning-scale` (higher → more scannable, lower →
more artistic), `--guidance-scale`, `--size`, `--seed`, and
`--controlnet-repo`/`--controlnet-file` to swap the ControlNet model. See
`--help`.

**Always test that the result scans before printing it**, and raise
`--conditioning-scale` if it doesn't.

## ControlNet

This demo implements a **true ControlNet** (`src/controlnet.rs`): a port of the
diffusers `ControlNetModel` (SD1.5 layout) on top of candle. candle ships the
UNet hook (`forward_with_additional_residuals`) but not a ControlNet model, so
this module supplies one — the conditioning embedding for the control hint, an
encoder that mirrors the UNet's down/mid blocks (sharing their weight layout),
and the zero-convolution output projections. Its residuals are injected into
every UNet block each denoising step, which constrains the structure far more
reliably than an img2img seed.

It loads any standard SD1.5 ControlNet safetensors; the default targets a
QR-code ControlNet (`monster-labs/control_v1p_sd15_qrcode_monster`).

> Compile-verified against candle 0.8. Running it needs the model weights and
> (realistically) a GPU, which is why end-to-end output isn't shown here.

## License

MIT OR Apache-2.0, matching `qrc`.
