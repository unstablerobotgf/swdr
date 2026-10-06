//! RS(63,35) over GF(64) (x^6+x+1, roots a^1..a^28), errors and erasures; port of tools/rs63.py.
//! Symbol 0 is the highest-order coefficient (first transmitted).

const N: usize = 63;
const NPAR: usize = 28;

struct Gf {
    exp: [u8; 128],
    log: [u8; 64],
}

const GF: Gf = {
    let (mut exp, mut log, mut x, mut i) = ([0u8; 128], [0u8; 64], 1u8, 0);
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
    Gf { exp, log }
};

fn mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 { 0 } else { GF.exp[GF.log[a as usize] as usize + GF.log[b as usize] as usize] }
}

fn inv(a: u8) -> u8 {
    GF.exp[63 - GF.log[a as usize] as usize]
}

fn alpha(p: usize) -> u8 {
    GF.exp[p % 63]
}

/// p[0] is the lowest-degree coefficient.
fn peval(p: &[u8], x: u8) -> u8 {
    p.iter().rev().fold(0, |y, &c| mul(y, x) ^ c)
}

fn pmul(p: &[u8], q: &[u8]) -> Vec<u8> {
    let mut r = vec![0u8; p.len() + q.len() - 1];
    for (i, &a) in p.iter().enumerate() {
        for (j, &b) in q.iter().enumerate() {
            r[i + j] ^= mul(a, b);
        }
    }
    r
}

fn syndromes(cw: &[u8; N]) -> [u8; NPAR] {
    let mut s = [0u8; NPAR];
    for (j, sj) in s.iter_mut().enumerate() {
        let x = alpha(j + 1);
        *sj = cw.iter().fold(0, |y, &c| mul(y, x) ^ c); // Horner over highest-first symbols
    }
    s
}

/// Corrects `cw` in place; returns the number of symbols changed, or None if uncorrectable.
pub fn decode(cw: &mut [u8; N], erasures: &[usize]) -> Option<usize> {
    let s = syndromes(cw);
    if s.iter().all(|&v| v == 0) {
        return Some(0);
    }
    let mut gamma = vec![1u8];
    for &p in erasures {
        gamma = pmul(&gamma, &[1, alpha(N - 1 - p)]);
    }
    // Modified syndromes drop the erasures; plain Berlekamp-Massey on T_e..T_27 finds the errors.
    let t = pmul(&gamma, &s);
    let seq = &t[erasures.len()..NPAR];
    let (mut c, mut bpoly, mut l, mut m, mut b) = (vec![1u8], vec![1u8], 0usize, 1usize, 1u8);
    for n in 0..seq.len() {
        let mut d = seq[n];
        for i in 1..=l.min(c.len() - 1) {
            if n >= i {
                d ^= mul(c[i], seq[n - i]);
            }
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = mul(d, inv(b));
        let mut new = c.clone();
        new.resize(new.len().max(m + bpoly.len()), 0);
        for (i, &v) in bpoly.iter().enumerate() {
            new[m + i] ^= mul(coef, v);
        }
        if 2 * l <= n {
            (bpoly, l, b, m) = (c, n + 1 - l, d, 1);
        } else {
            m += 1;
        }
        c = new;
    }
    if 2 * l + erasures.len() > NPAR {
        return None;
    }
    let mut psi = pmul(&c, &gamma);
    while psi.len() > 1 && *psi.last().unwrap() == 0 {
        psi.pop();
    }
    let pos: Vec<usize> = (0..N).filter(|&p| peval(&psi, inv(alpha(N - 1 - p))) == 0).collect();
    if pos.len() != psi.len() - 1 {
        return None;
    }
    let om = &pmul(&s, &psi)[..NPAR];
    let dpsi: Vec<u8> = (1..psi.len()).map(|i| if i % 2 == 1 { psi[i] } else { 0 }).collect();
    let mut changed = 0;
    for &p in &pos {
        let xi_inv = inv(alpha(N - 1 - p));
        let den = peval(&dpsi, xi_inv);
        if den == 0 {
            return None;
        }
        let y = mul(peval(om, xi_inv), inv(den)); // fcr = 1
        changed += (y != 0) as usize;
        cw[p] ^= y;
    }
    if syndromes(cw).iter().any(|&v| v != 0) {
        return None;
    }
    Some(changed)
}
