# jpeg2000-rs

Rust implementation of the **mathematical core** behind JPEG 2000 Part‑1—built as a modern continuation of the wavelet experiments from your 2016 Python project (`~/dev/jpeg2000`), which used Haar analysis + YCbCr visualisation.

This crate currently ships:

- reversible **Le Gall 5 / 3** lifting (the integer wavelet used for lossless JP2),
- the normative **reversible colour transform** for RGB inputs,
- a faithful **MQ arithmetic coder** baseline (tables aligned with OpenJPEG’s reference data).

Tier‑1 (EBCOT) bit‑plane passes, Tier‑2 packet headers, and ISO‑BMFF JP2 boxes are the next milestones toward emitting decodable codestreams. Contributions welcome.

## Quick usage

```toml
[dependencies]
jpeg2000_rs = "0.1"
```

```rust
use jpeg2000_rs::{color, dwt};

let rgba = [128_u8, 64, 192];
let decorrelated = color::rct_forward_rgb8(rgba);
assert_eq!(color::rct_inverse_exact(decorrelated), [128, 64, 192]);

let mut plane: Vec<i32> = (0..256).map(|v| v % 32).collect();
let transformed = dwt::forward_multi(plane.clone(), 16, 16, 2);
let restored = dwt::inverse_multi(transformed, 16, 16, 2);
assert_eq!(restored, plane);
```

## Benchmark fixtures

Sample PNGs for upcoming benches and external compressor comparisons live under [`benchmark/`](benchmark/README.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate shall be dual licensed as above, without any additional terms or conditions.
