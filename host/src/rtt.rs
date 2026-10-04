//! Minimal SEGGER RTT client that tolerates AP faults.
//! AP faults seen earlier were a JP2-missing brown-out (0% with JP2 fitted); retries are cheap insurance.
//! probe-rs's own RTT reader gives up on the first fault.

use anyhow::{bail, Result};
use probe_rs::{Core, MemoryInterface};

const CHUNK: usize = 4096;
const RETRIES: usize = 32;
const ID: &[u8; 10] = b"SEGGER RTT";

fn retry<T>(mut f: impl FnMut() -> Result<T, probe_rs::Error>) -> Result<T> {
    let mut last = None;
    for _ in 0..RETRIES {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap().into())
}

pub fn read(core: &mut Core, addr: u64, buf: &mut [u8]) -> Result<()> {
    for (i, c) in buf.chunks_mut(CHUNK).enumerate() {
        retry(|| core.read(addr + (i * CHUNK) as u64, c))?;
    }
    Ok(())
}

fn word(core: &mut Core, addr: u64) -> Result<u32> {
    retry(|| core.read_word_32(addr))
}

fn set_word(core: &mut Core, addr: u64, v: u32) -> Result<()> {
    retry(|| core.write_word_32(addr, v))
}

/// One RTT ring; `desc` is the address of its 24-byte descriptor.
pub struct Ring {
    desc: u64,
    buf: u64,
    size: u32,
}

impl Ring {
    const WR: u64 = 12;
    const RD: u64 = 16;

    /// Up channel (target -> host): drain up to `out.len()` bytes.
    pub fn read(&self, core: &mut Core, out: &mut [u8]) -> Result<usize> {
        let w = word(core, self.desc + Self::WR)?;
        let mut r = word(core, self.desc + Self::RD)?;
        let mut n = 0;
        while r != w && n < out.len() {
            let end = if w > r { w } else { self.size };
            let len = ((end - r) as usize).min(out.len() - n);
            read(core, self.buf + r as u64, &mut out[n..n + len])?;
            n += len;
            r = (r + len as u32) % self.size;
        }
        if n > 0 {
            set_word(core, self.desc + Self::RD, r)?;
        }
        Ok(n)
    }

    /// Down channel (host -> target): all-or-nothing write of a small command.
    pub fn write(&self, core: &mut Core, data: &[u8]) -> Result<bool> {
        let w = word(core, self.desc + Self::WR)?;
        let r = word(core, self.desc + Self::RD)?;
        let free = (r + self.size - w - 1) % self.size;
        if (free as usize) < data.len() {
            return Ok(false);
        }
        for (i, b) in data.iter().enumerate() {
            let a = self.buf + ((w as usize + i) % self.size as usize) as u64;
            retry(|| core.write_word_8(a, *b))?;
        }
        set_word(core, self.desc + Self::WR, (w + data.len() as u32) % self.size)?;
        Ok(true)
    }
}

pub struct Rtt {
    pub up: Vec<Ring>,
    pub down: Vec<Ring>,
}

impl Rtt {
    /// Scan `range` for the control block, then parse the channel descriptors.
    pub fn attach(core: &mut Core, range: std::ops::Range<u64>) -> Result<Self> {
        let mut ram = vec![0u8; (range.end - range.start) as usize];
        read(core, range.start, &mut ram)?;
        let Some(off) = ram.windows(ID.len()).position(|w| w == ID) else { bail!("RTT control block not found") };
        let cb = range.start + off as u64;
        let max_up = word(core, cb + 16)? as u64;
        let max_down = word(core, cb + 20)? as u64;
        if max_up > 16 || max_down > 16 {
            bail!("RTT control block at {cb:#x} looks corrupt");
        }
        let mut ring = |d: u64| -> Result<Ring> {
            Ok(Ring { desc: d, buf: word(core, d + 4)? as u64, size: word(core, d + 8)? })
        };
        let up = (0..max_up).map(|i| ring(cb + 24 + 24 * i)).collect::<Result<_>>()?;
        let down = (0..max_down).map(|i| ring(cb + 24 + 24 * (max_up + i))).collect::<Result<_>>()?;
        Ok(Self { up, down })
    }
}
