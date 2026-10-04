//! BCH(63,16,23) decoder for the P25 NID (corrects up to 11 bit errors).
//! Generator octal 6331141367235453, GF(2^6) with x^6 + x + 1; bit 62 is the first transmitted bit,
//! info (NAC, DUID) is bits 62..47. Validated against tools/bch.py and 532 live NIDs.

const T: usize = 11;

const fn gf_tables() -> ([u8; 126], [u8; 64]) {
    let (mut exp, mut log) = ([0u8; 126], [0u8; 64]);
    let mut x: u8 = 1;
    let mut i = 0;
    while i < 63 {
        exp[i] = x;
        exp[i + 63] = x;
        log[x as usize] = i as u8;
        x <<= 1;
        if x & 0x40 != 0 {
            x ^= 0x43;
        }
        i += 1;
    }
    (exp, log)
}

const TABLES: ([u8; 126], [u8; 64]) = gf_tables();
const EXP: [u8; 126] = TABLES.0;
const LOG: [u8; 64] = TABLES.1;

fn mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        0
    } else {
        EXP[LOG[a as usize] as usize + LOG[b as usize] as usize]
    }
}

fn inv(a: u8) -> u8 {
    EXP[(63 - LOG[a as usize] as usize) % 63]
}

/// Decode a 63-bit NID codeword (bit 62 first). Returns (16 info bits, bits corrected).
pub fn decode(mut word: u64) -> Option<(u16, u8)> {
    let mut syn = [0u8; 2 * T];
    for (k, s) in syn.iter_mut().enumerate() {
        let power = k + 1;
        for i in 0..63 {
            if word >> i & 1 != 0 {
                *s ^= EXP[(i * power) % 63];
            }
        }
    }
    if syn.iter().all(|&s| s == 0) {
        return Some(((word >> 47) as u16, 0));
    }
    // Berlekamp-Massey.
    let (mut c, mut b) = ([0u8; 2 * T + 1], [0u8; 2 * T + 1]);
    c[0] = 1;
    b[0] = 1;
    let (mut l, mut m, mut bb) = (0usize, 1usize, 1u8);
    for n in 0..2 * T {
        let mut d = syn[n];
        for i in 1..=l {
            d ^= mul(c[i], syn[n - i]);
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = mul(d, inv(bb));
        let t = c;
        for i in m..=2 * T {
            c[i] ^= mul(coef, b[i - m]);
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    if l > T {
        return None;
    }
    // Chien search: error at bit i where C(alpha^-i) == 0.
    let mut found = 0;
    for i in 0..63 {
        let mut s = 0u8;
        for (j, &cj) in c.iter().enumerate().take(l + 1) {
            if cj != 0 {
                s ^= EXP[(LOG[cj as usize] as usize + j * (63 - i)) % 63];
            }
        }
        if s == 0 {
            word ^= 1 << i;
            found += 1;
        }
    }
    (found == l).then_some(((word >> 47) as u16, l as u8))
}
