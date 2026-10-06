//! Optional AMBE+2 (3600x2450) synthesis through mbelib, behind the `vocoder` feature.
//! AMBE+2 is a DVSI codec under patent; this module only links when asked for.

#[repr(C)]
struct MbeParms {
    w0: f32,
    l: i32,
    k: i32,
    vl: [i32; 57],
    ml: [f32; 57],
    log2ml: [f32; 57],
    phil: [f32; 57],
    psil: [f32; 57],
    gamma: f32,
    un: i32,
    repeat: i32,
}

#[link(name = "mbe")]
extern "C" {
    fn mbe_initMbeParms(cur: *mut MbeParms, prev: *mut MbeParms, prev_enh: *mut MbeParms);
    fn mbe_processAmbe3600x2450Frame(
        aout: *mut i16,
        errs: *mut i32,
        errs2: *mut i32,
        err_str: *mut u8,
        ambe_fr: *mut [u8; 24],
        ambe_d: *mut u8,
        cur: *mut MbeParms,
        prev: *mut MbeParms,
        prev_enh: *mut MbeParms,
        uvquality: i32,
    );
}

pub struct Vocoder {
    p: Box<[MbeParms; 3]>,
}

impl Vocoder {
    pub fn new() -> Self {
        // SAFETY: plain-old-data floats and ints; mbe_initMbeParms fills them in.
        let mut p: Box<[MbeParms; 3]> = Box::new(unsafe { std::mem::zeroed() });
        let [a, b, c] = &mut *p;
        unsafe { mbe_initMbeParms(a, b, c) };
        Self { p }
    }

    /// One 20 ms frame (c0..c3, LSB = bit 0 as in ambe_fr) -> 160 samples at 8 kHz.
    /// Returns the samples and mbelib's own (errs, errs2) counts.
    pub fn frame(&mut self, c: [u32; 4]) -> ([i16; 160], i32, i32) {
        let mut fr = [[0u8; 24]; 4];
        for (w, n) in [(0, 24), (1, 23), (2, 11), (3, 14)] {
            for b in 0..n {
                fr[w][b] = ((c[w] >> b) & 1) as u8;
            }
        }
        let (mut out, mut e1, mut e2, mut s, mut d) = ([0i16; 160], 0, 0, [0u8; 64], [0u8; 49]);
        let [a, b, cc] = &mut *self.p;
        unsafe { mbe_processAmbe3600x2450Frame(out.as_mut_ptr(), &mut e1, &mut e2, s.as_mut_ptr(), fr.as_mut_ptr(), d.as_mut_ptr(), a, b, cc, 3) };
        (out, e1, e2)
    }
}

/// Minimal 8 kHz mono 16-bit WAV writer (header patched on finish).
pub struct Wav {
    f: std::fs::File,
    n: u32,
}

impl Wav {
    pub fn create(path: &str) -> std::io::Result<Self> {
        use std::io::Write;
        let mut f = std::fs::File::create(path)?;
        f.write_all(&[0u8; 44])?;
        Ok(Self { f, n: 0 })
    }

    pub fn write(&mut self, s: &[i16]) -> std::io::Result<()> {
        use std::io::Write;
        let b: Vec<u8> = s.iter().flat_map(|v| v.to_le_bytes()).collect();
        self.n += b.len() as u32;
        self.f.write_all(&b)
    }

    pub fn finish(mut self) -> std::io::Result<()> {
        use std::io::{Seek, SeekFrom, Write};
        let mut h = Vec::with_capacity(44);
        h.extend(b"RIFF");
        h.extend((36 + self.n).to_le_bytes());
        h.extend(b"WAVEfmt ");
        h.extend(16u32.to_le_bytes());
        h.extend(1u16.to_le_bytes()); // PCM
        h.extend(1u16.to_le_bytes()); // mono
        h.extend(8000u32.to_le_bytes());
        h.extend(16000u32.to_le_bytes());
        h.extend(2u16.to_le_bytes());
        h.extend(16u16.to_le_bytes());
        h.extend(b"data");
        h.extend(self.n.to_le_bytes());
        self.f.seek(SeekFrom::Start(0))?;
        self.f.write_all(&h)
    }
}
