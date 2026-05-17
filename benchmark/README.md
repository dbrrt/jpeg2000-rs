# Benchmark assets

PNG fixtures under `images/` for profiling wavelet transforms, future Tier‑1 coding, or comparisons with OpenJPEG.

| File | Role |
|------|------|
| `gray_gradient_64.png` | Tiny gray ramp — smoke tests / micro‑benches |
| `gray_gradient_512.png` | Smooth luminance gradient — friendly to lossless wavelets |
| `rgb_gradient_512.png` | RGB ramp — RCT + multi‑component paths |
| `checker_256.png` | High‑frequency checker — stresses HF subbands |
| `noise_384x256.png` | Gray noise (`-seed 42`) — pseudo‑natural entropy |
| `rose_512.png` | Photo‑like built‑in ImageMagick image — mixed spectra |

## Regenerating (ImageMagick 7)

From the repo root:

```bash
mkdir -p benchmark/images
magick -size 512x512 gradient:'#000000-#ffffff' -colorspace Gray -depth 8 \
  benchmark/images/gray_gradient_512.png
magick -size 512x512 gradient:'#ff0000-#0000ff' -depth 8 \
  benchmark/images/rgb_gradient_512.png
magick -size 256x256 pattern:checkerboard -depth 8 \
  benchmark/images/checker_256.png
magick -size 384x256 xc:'gray(50%)' +noise Random -seed 42 -depth 8 \
  benchmark/images/noise_384x256.png
magick rose: -resize 512x512! benchmark/images/rose_512.png
magick -size 64x64 gradient:'#000-#fff' -colorspace Gray -depth 8 \
  benchmark/images/gray_gradient_64.png
```

If `rose:` is unavailable, substitute `logo:` or another built‑in.
