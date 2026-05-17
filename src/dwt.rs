//! Le Gall 5 / 3 reversible discrete wavelet transform (JPEG 2000 Part‑1).
//!
//! Compared with the Haar transform used in the Python prototype (`pywt.dwt2(...,
//! 'haar')`), this implements the **normative integer lifting scheme** actually used
//! for lossless JPEG 2000.

/// Predict step lifting coefficient index should reference mirrored neighbours.
#[inline]
fn mirror(mut idx: i32, len: i32) -> usize {
    if len <= 1 {
        return 0;
    }
    let max = len - 1;
    while idx < 0 || idx > max {
        idx = if idx < 0 { -idx - 1 } else { 2 * max - idx };
    }
    idx as usize
}

#[inline]
fn sym_get(buf: &[i32], idx: i32) -> i32 {
    buf[mirror(idx, buf.len() as i32)]
}

/// One‑dimensional forward 5 / 3 analysis lifting pair operating **in place**.
pub fn fdwt_1d(buf: &mut [i32]) {
    let n = buf.len();
    assert!(
        n % 2 == 0 && n >= 2,
        "1‑D transform expects an even length of at least two samples"
    );

    let half = n / 2;
    let mut even: Vec<i32> = (0..half).map(|j| buf[2 * j]).collect();
    let mut odd: Vec<i32> = (0..half).map(|j| buf[2 * j + 1]).collect();

    for j in 0..half {
        let j = j as i32;
        let left = sym_get(&even, j);
        let right = sym_get(&even, j + 1);
        odd[j as usize] -= (left + right).div_euclid(2);
    }

    for j in 0..half {
        let j = j as i32;
        let up = sym_get(&odd, j - 1);
        let cur = sym_get(&odd, j);
        even[j as usize] += (up + cur + 2).div_euclid(4);
    }

    for j in 0..half {
        buf[2 * j] = even[j];
        buf[2 * j + 1] = odd[j];
    }
}

/// Inverse of [`fdwt_1d`].
pub fn idwt_1d(buf: &mut [i32]) {
    let n = buf.len();
    assert!(
        n % 2 == 0 && n >= 2,
        "1‑D transform expects an even length of at least two samples"
    );

    let half = n / 2;
    let mut even: Vec<i32> = (0..half).map(|j| buf[2 * j]).collect();
    let mut odd: Vec<i32> = (0..half).map(|j| buf[2 * j + 1]).collect();

    for j in 0..half {
        let j = j as i32;
        let up = sym_get(&odd, j - 1);
        let cur = sym_get(&odd, j);
        even[j as usize] -= (up + cur + 2).div_euclid(4);
    }

    for j in 0..half {
        let j = j as i32;
        let left = sym_get(&even, j);
        let right = sym_get(&even, j + 1);
        odd[j as usize] += (left + right).div_euclid(2);
    }

    for j in 0..half {
        buf[2 * j] = even[j];
        buf[2 * j + 1] = odd[j];
    }
}

fn horizontal_pass(buf: &mut [i32], width: usize, height: usize) {
    debug_assert_eq!(buf.len(), width * height);
    let mut row = vec![0_i32; width];
    for y in 0..height {
        let off = y * width;
        row.copy_from_slice(&buf[off..off + width]);
        fdwt_1d(&mut row);
        for x in 0..width / 2 {
            buf[off + x] = row[2 * x];
            buf[off + x + width / 2] = row[2 * x + 1];
        }
    }
}

fn vertical_pass_band(buf: &mut [i32], full_width: usize, height: usize, x0: usize, band: usize) {
    debug_assert_eq!(buf.len(), full_width * height);
    let mut col = vec![0_i32; height];
    for xi in 0..band {
        let x = x0 + xi;
        for y in 0..height {
            col[y] = buf[y * full_width + x];
        }
        fdwt_1d(&mut col);
        for y in 0..height / 2 {
            buf[y * full_width + x] = col[2 * y];
            buf[(y + height / 2) * full_width + x] = col[2 * y + 1];
        }
    }
}

/// Perform one dyadic decomposition level on a dense `width` × `height` tile.
pub fn decompose_level(buf: &mut [i32], width: usize, height: usize) {
    assert!(
        width >= 2 && height >= 2,
        "cannot split further when width or height fall below two samples"
    );
    horizontal_pass(buf, width, height);
    vertical_pass_band(buf, width, height, 0, width / 2);
    vertical_pass_band(buf, width, height, width / 2, width / 2);
}

fn vertical_inverse_band(
    buf: &mut [i32],
    full_width: usize,
    height: usize,
    x0: usize,
    band: usize,
) {
    debug_assert_eq!(buf.len(), full_width * height);
    let mut col = vec![0_i32; height];
    for xi in 0..band {
        let x = x0 + xi;
        for y in 0..height / 2 {
            col[2 * y] = buf[y * full_width + x];
            col[2 * y + 1] = buf[(y + height / 2) * full_width + x];
        }
        idwt_1d(&mut col);
        for y in 0..height {
            buf[y * full_width + x] = col[y];
        }
    }
}

fn horizontal_inverse(buf: &mut [i32], width: usize, height: usize) {
    debug_assert_eq!(buf.len(), width * height);
    let mut row = vec![0_i32; width];
    for y in 0..height {
        let off = y * width;
        for x in 0..width / 2 {
            row[2 * x] = buf[off + x];
            row[2 * x + 1] = buf[off + x + width / 2];
        }
        idwt_1d(&mut row);
        buf[off..off + width].copy_from_slice(&row);
    }
}

/// Reconstruct one dyadic decomposition level on a dense tile.
pub fn reconstruct_level(buf: &mut [i32], width: usize, height: usize) {
    assert!(
        width >= 2 && height >= 2,
        "cannot merge further when width or height fall below two samples"
    );
    vertical_inverse_band(buf, width, height, 0, width / 2);
    vertical_inverse_band(buf, width, height, width / 2, width / 2);
    horizontal_inverse(buf, width, height);
}

#[inline]
fn copy_rect(src: &[i32], stride: usize, rect_w: usize, rect_h: usize, dst: &mut [i32]) {
    assert_eq!(dst.len(), rect_w * rect_h);
    for y in 0..rect_h {
        dst[y * rect_w..(y + 1) * rect_w].copy_from_slice(&src[y * stride..y * stride + rect_w]);
    }
}

#[inline]
fn paste_rect(dst: &mut [i32], stride: usize, rect_w: usize, rect_h: usize, src: &[i32]) {
    assert_eq!(src.len(), rect_w * rect_h);
    for y in 0..rect_h {
        dst[y * stride..y * stride + rect_w].copy_from_slice(&src[y * rect_w..(y + 1) * rect_w]);
    }
}

fn level_schedule(mut width: usize, mut height: usize, levels: usize) -> Vec<(usize, usize)> {
    let mut schedule = Vec::new();
    for _ in 0..levels {
        if width < 2 || height < 2 {
            break;
        }
        schedule.push((width, height));
        width /= 2;
        height /= 2;
    }
    schedule
}

/// Multi‑level forward transform that updates only the nominal LL sub‑band between
/// resolutions—matching the behaviour defined in JPEG 2000.
pub fn forward_multi(buf: Vec<i32>, stride: usize, height: usize, levels: usize) -> Vec<i32> {
    assert_eq!(buf.len(), stride * height);
    let schedule = level_schedule(stride, height, levels);
    let mut buf = buf;
    for (tile_w, tile_h) in schedule {
        let mut tile = vec![0_i32; tile_w * tile_h];
        copy_rect(&buf, stride, tile_w, tile_h, &mut tile);
        decompose_level(&mut tile, tile_w, tile_h);
        paste_rect(&mut buf, stride, tile_w, tile_h, &tile);
    }
    buf
}

/// Inverse of [`forward_multi`].
pub fn inverse_multi(buf: Vec<i32>, stride: usize, height: usize, levels: usize) -> Vec<i32> {
    assert_eq!(buf.len(), stride * height);
    let schedule = level_schedule(stride, height, levels);
    let mut buf = buf;
    for (tile_w, tile_h) in schedule.into_iter().rev() {
        let mut tile = vec![0_i32; tile_w * tile_h];
        copy_rect(&buf, stride, tile_w, tile_h, &mut tile);
        reconstruct_level(&mut tile, tile_w, tile_h);
        paste_rect(&mut buf, stride, tile_w, tile_h, &tile);
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn merge_interleaved(low: &[i32], high: &[i32], out: &mut [i32]) {
        assert_eq!(low.len(), high.len());
        assert_eq!(out.len(), low.len() * 2);
        for i in 0..low.len() {
            out[2 * i] = low[i];
            out[2 * i + 1] = high[i];
        }
    }

    fn split_interleaved(buf: &[i32]) -> (Vec<i32>, Vec<i32>) {
        let mut low = Vec::with_capacity(buf.len() / 2);
        let mut high = Vec::with_capacity(buf.len() / 2);
        for chunk_pair in buf.chunks_exact(2) {
            low.push(chunk_pair[0]);
            high.push(chunk_pair[1]);
        }
        (low, high)
    }

    #[test]
    fn one_d_roundtrip_random_small() {
        let mut rng = rand::thread_rng();
        for len in [2_usize, 4, 16, 128] {
            let mut data: Vec<i32> = (0..len).map(|_| rng.gen_range(-1024..1024)).collect();
            let original = data.clone();
            fdwt_1d(&mut data);
            let (low, high) = split_interleaved(&data);
            let mut merged = vec![0_i32; len];
            merge_interleaved(&low, &high, &mut merged);
            idwt_1d(&mut merged);
            assert_eq!(merged, original);
        }
    }

    #[test]
    fn two_d_roundtrip_powers_of_two() {
        let mut rng = rand::thread_rng();
        for size in [8_usize, 16, 32] {
            let len = size * size;
            let flat: Vec<i32> = (0..len).map(|_| rng.gen_range(-512..512)).collect();
            let original = flat.clone();
            let transformed = forward_multi(flat, size, size, 3);
            let reconstructed = inverse_multi(transformed, size, size, 3);
            assert_eq!(reconstructed, original);
        }
    }
}
