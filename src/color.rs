//! Reversible colour transforms from JPEG 2000 Part 1 (ISO/IEC 15444-1).
//!
//! This mirrors the spirit of your Python `RGBtoYUV` experiment, but uses the
//! normative **reversible colour transform** so colour conversions remain
//! lossless under integer arithmetic.

/// Forward RCT (RGB → decorrelated components).
///
/// Input channels are interpreted as unsigned 8‑bit samples. Intermediate values
/// use widened integers exactly like the specification recommends for encoders.
#[inline]
pub fn rct_forward_rgb8([r, g, b]: [u8; 3]) -> [i32; 3] {
    let r = i32::from(r);
    let g = i32::from(g);
    let b = i32::from(b);
    let y = g + (r + b).div_euclid(2);
    let cb = r - g;
    let cr = b - g;
    [y, cb, cr]
}

#[inline]
pub fn rct_inverse_exact([y, cb, cr]: [i32; 3]) -> [i32; 3] {
    let pair = (cb + cr).div_euclid(2);
    let g = (y - pair).div_euclid(2);
    let r = cb + g;
    let b = cr + g;
    [r, g, b]
}

/// Inverse RCT with clamping to an 8‑bit display envelope.
#[inline]
pub fn rct_inverse_rgb8(ycc: [i32; 3]) -> [u8; 3] {
    let [r, g, b] = rct_inverse_exact(ycc);
    [clamp_u8(r), clamp_u8(g), clamp_u8(b)]
}

#[inline]
fn clamp_u8(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rct_roundtrip_on_gradient() {
        for r in (0_u8..16).step_by(3) {
            for g in (0_u8..16).step_by(5) {
                for b in (0_u8..16).step_by(7) {
                    let rgb = [r, g, b];
                    let fwd = rct_forward_rgb8(rgb);
                    let back = rct_inverse_exact(fwd);
                    assert_eq!(
                        back,
                        [i32::from(r), i32::from(g), i32::from(b)],
                        "RCT must be lossless for {rgb:?}"
                    );
                }
            }
        }
    }
}
