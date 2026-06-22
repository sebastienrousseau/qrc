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
  --strength 0.6 \
  --output art-qr.png
```

On first run the Stable-Diffusion v1.5 weights (~4 GB) are downloaded from the
Hugging Face Hub and cached. A CUDA GPU is strongly recommended; `--cpu` works
but is very slow.

Key flags: `--steps`, `--strength` (lower → more scannable, higher → more
artistic), `--guidance-scale`, `--size`, `--seed`. See `--help`.

**Always test that the result scans before printing it**, and lower
`--strength` if it doesn't.

## A note on ControlNet

For simplicity and to work with stock `candle-transformers`, this demo uses
**img2img** — the control image seeds the denoising process. That biases the
output toward the QR structure but does not enforce it as strongly as true
**ControlNet** conditioning (which injects the control features into every UNet
block and is what dedicated QR-art models use). Adding a ControlNet UNet hook is
the natural next step for higher scan reliability; the cloud `api` feature
already targets ControlNet models on the provider side.

## License

MIT OR Apache-2.0, matching `qrc`.
