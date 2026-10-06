//! SPI master side of the swdr frame link: spidev for frames, GPIO v2 uAPI for DRDY.
//! Frame format and slave behaviour: firmware/src/spi.rs.

use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

pub const FRAME: usize = 1040;
pub const HDR: usize = 16;
pub const KIND_TEST: u8 = 0;
pub const KIND_TAP: u8 = 1;

const SPI_IOC_MESSAGE_1: libc::c_ulong = 0x4020_6B00;
const GPIO_V2_GET_LINE_IOCTL: libc::c_ulong = 0xC250_B407;
const GPIO_V2_LINE_GET_VALUES_IOCTL: libc::c_ulong = 0xC010_B40E;
const GPIO_V2_LINE_FLAG_INPUT: u64 = 1 << 2;
const GPIO_V2_LINE_FLAG_EDGE_RISING: u64 = 1 << 4;

#[repr(C)]
struct SpiIocTransfer {
    tx_buf: u64,
    rx_buf: u64,
    len: u32,
    speed_hz: u32,
    delay_usecs: u16,
    bits_per_word: u8,
    cs_change: u8,
    tx_nbits: u8,
    rx_nbits: u8,
    word_delay_usecs: u8,
    pad: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub kind: u8,
    pub flags: u8,
    pub seq: u16,
    pub len: u16,
    pub aux0: u32,
    pub aux1: u32,
}

impl Header {
    pub fn parse(f: &[u8; FRAME]) -> Option<Self> {
        let u32le = |i: usize| u32::from_le_bytes([f[i], f[i + 1], f[i + 2], f[i + 3]]);
        let h = Header {
            kind: f[2],
            flags: f[3],
            seq: u16::from_le_bytes([f[4], f[5]]),
            len: u16::from_le_bytes([f[6], f[7]]),
            aux0: u32le(8),
            aux1: u32le(12),
        };
        (f[0] == 0xA5 && f[1] == 0x5B && h.len as usize == FRAME - HDR).then_some(h)
    }
}

pub struct Link {
    spi: File,
    drdy: OwnedFd,
    hz: u32,
    tx: Box<[u8; FRAME]>,
    rx: Box<[u8; FRAME]>,
    cmd: Option<(u8, u32)>,
}

fn ioctl<T>(fd: i32, req: libc::c_ulong, arg: *mut T) -> io::Result<()> {
    if unsafe { libc::ioctl(fd, req as _, arg) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

impl Link {
    pub fn open(spidev: &str, gpiochip: &str, line: u32, hz: u32) -> io::Result<Self> {
        let spi = OpenOptions::new().read(true).write(true).open(spidev)?;
        let chip = OpenOptions::new().read(true).write(true).open(gpiochip)?;
        // struct gpio_v2_line_request (592 B): offsets[64], consumer[32], config, num_lines, ..., fd @ 588.
        let mut req = [0u8; 592];
        req[..4].copy_from_slice(&line.to_le_bytes());
        req[256..265].copy_from_slice(b"swdr-drdy");
        req[288..296].copy_from_slice(&(GPIO_V2_LINE_FLAG_INPUT | GPIO_V2_LINE_FLAG_EDGE_RISING).to_le_bytes());
        req[560..564].copy_from_slice(&1u32.to_le_bytes());
        req[564..568].copy_from_slice(&64u32.to_le_bytes());
        ioctl(chip.as_raw_fd(), GPIO_V2_GET_LINE_IOCTL, req.as_mut_ptr())?;
        let fd = i32::from_le_bytes([req[588], req[589], req[590], req[591]]);
        let drdy = unsafe { OwnedFd::from_raw_fd(fd) };
        Ok(Self { spi, drdy, hz, tx: Box::new([0; FRAME]), rx: Box::new([0; FRAME]), cmd: None })
    }

    pub fn drdy(&self) -> io::Result<bool> {
        let mut v = [0u64, 1u64]; // bits, mask
        ioctl(self.drdy.as_raw_fd(), GPIO_V2_LINE_GET_VALUES_IOCTL, v.as_mut_ptr())?;
        Ok(v[0] & 1 != 0)
    }

    /// Wait for DRDY high; edges only wake the poll, the level decides. False on timeout.
    pub fn wait_ready(&self, timeout_ms: i32) -> io::Result<bool> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms as u64);
        while !self.drdy()? {
            let left = deadline.saturating_duration_since(std::time::Instant::now()).as_millis() as i32;
            if left == 0 {
                return Ok(false);
            }
            let mut pfd = libc::pollfd { fd: self.drdy.as_raw_fd(), events: libc::POLLIN, revents: 0 };
            if unsafe { libc::poll(&mut pfd, 1, left) } > 0 {
                let mut ev = [0u8; 48 * 16];
                unsafe { libc::read(self.drdy.as_raw_fd(), ev.as_mut_ptr().cast(), ev.len()) };
            }
        }
        Ok(true)
    }

    /// Queue a C5 command for the next transfer's first MOSI bytes.
    pub fn command(&mut self, op: u8, arg: u32) {
        self.cmd = Some((op, arg));
    }

    pub fn transfer(&mut self) -> io::Result<&[u8; FRAME]> {
        self.tx.fill(0);
        if let Some((op, arg)) = self.cmd.take() {
            self.tx[..2].copy_from_slice(&[0xC5, op]);
            self.tx[2..6].copy_from_slice(&arg.to_le_bytes());
        }
        let mut t = SpiIocTransfer {
            tx_buf: self.tx.as_ptr() as u64,
            rx_buf: self.rx.as_mut_ptr() as u64,
            len: FRAME as u32,
            speed_hz: self.hz,
            delay_usecs: 0,
            bits_per_word: 8,
            cs_change: 0,
            tx_nbits: 0,
            rx_nbits: 0,
            word_delay_usecs: 0,
            pad: 0,
        };
        ioctl(self.spi.as_raw_fd(), SPI_IOC_MESSAGE_1, &mut t)?;
        Ok(&self.rx)
    }
}
