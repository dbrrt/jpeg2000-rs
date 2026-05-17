//! JPEG 2000 MQ arithmetic coder (Annex J / ISO/IEC 15444-1 Annex C).
//!
//! The probability state machine matches the tables published with OpenJPEG
//! (BSD‑licensed reference implementation). We ship the numeric constants only.

/// Raw MQ probability states (`qeval`, `mps`, `nmps`, `nlps`) for all 94 rows of the
/// expanded transition table used by OpenJPEG.
pub const MQ_TABLE: [(u16, u8, u8, u8); 94] = [
    (0x5601, 0, 2, 3),
    (0x5601, 1, 3, 2),
    (0x3401, 0, 4, 12),
    (0x3401, 1, 5, 13),
    (0x1801, 0, 6, 18),
    (0x1801, 1, 7, 19),
    (0x0ac1, 0, 8, 24),
    (0x0ac1, 1, 9, 25),
    (0x0521, 0, 10, 58),
    (0x0521, 1, 11, 59),
    (0x0221, 0, 76, 66),
    (0x0221, 1, 77, 67),
    (0x5601, 0, 14, 13),
    (0x5601, 1, 15, 12),
    (0x5401, 0, 16, 28),
    (0x5401, 1, 17, 29),
    (0x4801, 0, 18, 28),
    (0x4801, 1, 19, 29),
    (0x3801, 0, 20, 28),
    (0x3801, 1, 21, 29),
    (0x3001, 0, 22, 34),
    (0x3001, 1, 23, 35),
    (0x2401, 0, 24, 36),
    (0x2401, 1, 25, 37),
    (0x1c01, 0, 26, 40),
    (0x1c01, 1, 27, 41),
    (0x1601, 0, 58, 42),
    (0x1601, 1, 59, 43),
    (0x5601, 0, 30, 29),
    (0x5601, 1, 31, 28),
    (0x5401, 0, 32, 28),
    (0x5401, 1, 33, 29),
    (0x5101, 0, 34, 30),
    (0x5101, 1, 35, 31),
    (0x4801, 0, 36, 32),
    (0x4801, 1, 37, 33),
    (0x3801, 0, 38, 34),
    (0x3801, 1, 39, 35),
    (0x3401, 0, 40, 36),
    (0x3401, 1, 41, 37),
    (0x3001, 0, 42, 38),
    (0x3001, 1, 43, 39),
    (0x2801, 0, 44, 38),
    (0x2801, 1, 45, 39),
    (0x2401, 0, 46, 40),
    (0x2401, 1, 47, 41),
    (0x2201, 0, 48, 42),
    (0x2201, 1, 49, 43),
    (0x1c01, 0, 50, 44),
    (0x1c01, 1, 51, 45),
    (0x1801, 0, 52, 46),
    (0x1801, 1, 53, 47),
    (0x1601, 0, 54, 48),
    (0x1601, 1, 55, 49),
    (0x1401, 0, 56, 50),
    (0x1401, 1, 57, 51),
    (0x1201, 0, 58, 52),
    (0x1201, 1, 59, 53),
    (0x1101, 0, 60, 54),
    (0x1101, 1, 61, 55),
    (0x0ac1, 0, 62, 56),
    (0x0ac1, 1, 63, 57),
    (0x09c1, 0, 64, 58),
    (0x09c1, 1, 65, 59),
    (0x08a1, 0, 66, 60),
    (0x08a1, 1, 67, 61),
    (0x0521, 0, 68, 62),
    (0x0521, 1, 69, 63),
    (0x0441, 0, 70, 64),
    (0x0441, 1, 71, 65),
    (0x02a1, 0, 72, 66),
    (0x02a1, 1, 73, 67),
    (0x0221, 0, 74, 68),
    (0x0221, 1, 75, 69),
    (0x0141, 0, 76, 70),
    (0x0141, 1, 77, 71),
    (0x0111, 0, 78, 72),
    (0x0111, 1, 79, 73),
    (0x0085, 0, 80, 74),
    (0x0085, 1, 81, 75),
    (0x0049, 0, 82, 76),
    (0x0049, 1, 83, 77),
    (0x0025, 0, 84, 78),
    (0x0025, 1, 85, 79),
    (0x0015, 0, 86, 80),
    (0x0015, 1, 87, 81),
    (0x0009, 0, 88, 82),
    (0x0009, 1, 89, 83),
    (0x0005, 0, 90, 84),
    (0x0005, 1, 91, 85),
    (0x0001, 0, 90, 86),
    (0x0001, 1, 91, 87),
    (0x5601, 0, 92, 92),
    (0x5601, 1, 93, 93),
];

#[inline]
fn qeval(idx: usize) -> u32 {
    MQ_TABLE[idx].0 as u32
}

#[inline]
fn mps_bit(idx: usize) -> u32 {
    MQ_TABLE[idx].1 as u32
}

#[inline]
fn nmps(idx: usize) -> usize {
    MQ_TABLE[idx].2 as usize
}

#[inline]
fn nlps(idx: usize) -> usize {
    MQ_TABLE[idx].3 as usize
}

/// Encoder compatible with the JPEG 2000 MQ coder software conventions.
#[derive(Debug, Clone)]
pub struct MqEncoder {
    contexts: [usize; 19],
    cur_ctx_label: usize,
    cur_idx: usize,
    a: u32,
    c: u32,
    ct: u32,
    bytes: Vec<u8>,
    bp: usize,
}

impl Default for MqEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl MqEncoder {
    pub fn new() -> Self {
        let bytes = vec![0_u8];
        Self {
            contexts: [0_usize; 19],
            cur_ctx_label: 0,
            cur_idx: 0,
            a: 0x8000,
            c: 0,
            ct: 12,
            bp: 0,
            bytes,
        }
    }

    #[inline]
    pub fn reset_contexts(&mut self) {
        self.contexts.fill(0);
    }

    #[inline]
    pub fn set_context_index(&mut self, label: usize, state_index: usize) {
        debug_assert!(label < self.contexts.len());
        debug_assert!(state_index < MQ_TABLE.len());
        self.contexts[label] = state_index;
    }

    #[inline]
    pub fn select_context(&mut self, label: usize) {
        debug_assert!(label < self.contexts.len());
        self.cur_ctx_label = label;
        self.cur_idx = self.contexts[label];
    }

    fn renorm(&mut self, mut a: u32, mut c: u32, mut ct: u32) -> (u32, u32, u32) {
        loop {
            a <<= 1;
            c <<= 1;
            ct = ct.wrapping_sub(1);
            if ct == 0 {
                self.byte_out(&mut c, &mut ct);
            }
            if (a & 0x8000) != 0 {
                break;
            }
        }
        (a, c, ct)
    }

    fn code_mps(&mut self, mut a: u32, mut c: u32, ct: u32, idx: usize) -> (u32, u32, u32, usize) {
        let qe = qeval(idx);
        a = a.wrapping_sub(qe);
        if (a & 0x8000) == 0 {
            if a < qe {
                a = qe;
            } else {
                c = c.wrapping_add(qe);
            }
            let idx = nmps(idx);
            let (a2, c2, ct2) = self.renorm(a, c, ct);
            (a2, c2, ct2, idx)
        } else {
            c = c.wrapping_add(qe);
            (a, c, ct, idx)
        }
    }

    fn code_lps(&mut self, mut a: u32, mut c: u32, ct: u32, idx: usize) -> (u32, u32, u32, usize) {
        let qe = qeval(idx);
        a = a.wrapping_sub(qe);
        if a < qe {
            c = c.wrapping_add(qe);
            let idx = nmps(idx);
            let (a2, c2, ct2) = self.renorm(a, c, ct);
            (a2, c2, ct2, idx)
        } else {
            a = qe;
            let idx = nlps(idx);
            let (a2, c2, ct2) = self.renorm(a, c, ct);
            (a2, c2, ct2, idx)
        }
    }

    /// Encode a binary decision (`d ∈ {0, 1}`) for [`MqEncoder::select_context`].
    pub fn encode(&mut self, d: u32) {
        let mut idx = self.cur_idx;
        let mut a = self.a;
        let mut c = self.c;
        let mut ct = self.ct;

        if mps_bit(idx) == d {
            let (a2, c2, ct2, idx2) = self.code_mps(a, c, ct, idx);
            a = a2;
            c = c2;
            ct = ct2;
            idx = idx2;
        } else {
            let (a2, c2, ct2, idx2) = self.code_lps(a, c, ct, idx);
            a = a2;
            c = c2;
            ct = ct2;
            idx = idx2;
        }

        self.a = a;
        self.c = c;
        self.ct = ct;
        self.cur_idx = idx;
        self.contexts[self.cur_ctx_label] = idx;
    }

    fn byte_out(&mut self, c: &mut u32, ct: &mut u32) {
        debug_assert!(self.bp < self.bytes.len());
        if self.bytes[self.bp] == 0xff {
            self.bp += 1;
            if self.bp == self.bytes.len() {
                self.bytes.push((*c >> 20) as u8);
            } else {
                self.bytes[self.bp] = (*c >> 20) as u8;
            }
            *c &= 0x000f_ffff;
            *ct = 7;
        } else if (*c & 0x0800_0000) == 0 {
            self.bp += 1;
            if self.bp == self.bytes.len() {
                self.bytes.push((*c >> 19) as u8);
            } else {
                self.bytes[self.bp] = (*c >> 19) as u8;
            }
            *c &= 0x0007_ffff;
            *ct = 8;
        } else {
            self.bytes[self.bp] = self.bytes[self.bp].wrapping_add(1);
            if self.bytes[self.bp] == 0xff {
                *c &= 0x07ff_ffff;
                self.bp += 1;
                if self.bp == self.bytes.len() {
                    self.bytes.push((*c >> 20) as u8);
                } else {
                    self.bytes[self.bp] = (*c >> 20) as u8;
                }
                *c &= 0x000f_ffff;
                *ct = 7;
            } else {
                self.bp += 1;
                if self.bp == self.bytes.len() {
                    self.bytes.push((*c >> 19) as u8);
                } else {
                    self.bytes[self.bp] = (*c >> 19) as u8;
                }
                *c &= 0x0007_ffff;
                *ct = 8;
            }
        }
    }

    fn set_bits_for_flush(&mut self) {
        let tempc = self.c.wrapping_add(self.a);
        self.c |= 0xffff;
        if self.c >= tempc {
            self.c = self.c.wrapping_sub(0x8000);
        }
    }

    /// Finish entropy coding and return the serialized byte stream.
    pub fn finish(mut self) -> Vec<u8> {
        self.set_bits_for_flush();
        let mut c = self.c;
        let mut ct = self.ct;
        c <<= ct;
        self.byte_out(&mut c, &mut ct);
        c <<= ct;
        self.byte_out(&mut c, &mut ct);

        if self.bytes[self.bp] != 0xff {
            self.bp += 1;
        }

        let mut out = self.bytes;
        out.truncate(self.bp.max(1));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mq_encoder_smoke_length() {
        let mut mq = MqEncoder::new();
        mq.select_context(0);
        for bit in [0_u32, 1, 1, 0, 0, 1] {
            mq.encode(bit);
        }
        let data = mq.finish();
        assert!(!data.is_empty());
    }
}
