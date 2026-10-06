//! Frame transport to an SPI master (STM32MP257F-DK SPI6): SPI1 slave on the Nucleo Arduino header,
//! D13 PB11 SCK, D12 PB8 MISO, D11 PB9 MOSI, D10 PB10 NSS (all AF3), DRDY out on D8 PA5.
//! SPI3 is not used: its PB4/PB5 drive LD2/LD3 on this board.
//!
//! One transfer = one frame, full duplex: the master clocks FRAME bytes while DRDY is high and
//! may put a C5 command in the first 6 MOSI bytes. The TX FIFO cannot be flushed, so SPI1 is
//! reset between frames; a short or aborted transfer then costs one frame, not the alignment.

use core::sync::atomic::{AtomicBool, Ordering};
use stm32wl33_pac as pac;

pub const PAYLOAD: usize = 1024;
pub const HDR: usize = 16;
pub const FRAME: usize = HDR + PAYLOAD;
/// DMAMUX requests (RM0511 Table 30): SPI1_RX = 4, SPI1_TX = 5. Channel 1 belongs to the VCP.
const DMAREQ_SPI1_RX: u8 = 4;
const DMAREQ_SPI1_TX: u8 = 5;

/// Frame kinds.
pub const KIND_TEST: u8 = 0;

/// Header: A5 5B kind:u8 flags:u8 seq:u16 len:u16 aux0:u32 aux1:u32, then `len` payload bytes.
/// Test frames: flags = last command op, aux0 = commands seen, aux1 = aborted transfers.
#[repr(C, align(4))]
pub struct Frame(pub [u8; FRAME]);

impl Frame {
    pub const fn new() -> Self {
        Self([0; FRAME])
    }

    pub fn payload(&mut self) -> &mut [u8] {
        &mut self.0[HDR..]
    }

    pub fn seal(&mut self, kind: u8, flags: u8, seq: u16, aux0: u32, aux1: u32) {
        let len = PAYLOAD as u16;
        let h = &mut self.0[..HDR];
        h[..4].copy_from_slice(&[0xA5, 0x5B, kind, flags]);
        h[4..6].copy_from_slice(&seq.to_le_bytes());
        h[6..8].copy_from_slice(&len.to_le_bytes());
        h[8..12].copy_from_slice(&aux0.to_le_bytes());
        h[12..16].copy_from_slice(&aux1.to_le_bytes());
    }
}

/// Set by the DMA IRQ when RX DMA has taken the last MOSI byte of the frame.
static DONE: AtomicBool = AtomicBool::new(false);

/// DMA IRQ: drop DRDY the moment the frame completes, so the master never sees a stale high.
pub fn on_dma_irq() {
    let p = unsafe { pac::Peripherals::steal() };
    if p.dma.dma_isr().read().bits() & 1 << 9 != 0 {
        p.dma.dma_ifcr().write(|w| unsafe { w.bits(0xF << 8) });
        p.gpioa.bsrr().write(|w| unsafe { w.bits(1 << (5 + 16)) });
        DONE.store(true, Ordering::Release);
    }
}

pub struct Link {
    p: pac::Peripherals,
}

impl Link {
    pub fn new() -> Self {
        let p = unsafe { pac::Peripherals::steal() };
        p.rcc.ahbenr().modify(|_, w| w.dmaen().set_bit().gpioaen().set_bit().gpioben().set_bit());
        p.rcc.apb1enr().modify(|_, w| w.spi1en().set_bit());

        // PB8..PB11 -> AF3; MISO (PB8) at high speed for 32 MHz SCK.
        p.gpiob.moder().modify(|r, w| unsafe { w.bits((r.bits() & !(0xFF << 16)) | 0xAA << 16) });
        p.gpiob.afrh().modify(|r, w| unsafe { w.bits((r.bits() & !0xFFFF) | 0x3333) });
        p.gpiob.ospeedr().modify(|r, w| unsafe { w.bits(r.bits() | 0b11 << 16) });
        // PA5 DRDY: push-pull output, low.
        p.gpioa.bsrr().write(|w| unsafe { w.bits(1 << (5 + 16)) });
        p.gpioa.moder().modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << 10)) | 0b01 << 10) });

        p.dmamux.c1cr().write(|w| unsafe { w.dmareq_id().bits(DMAREQ_SPI1_TX) });
        p.dmamux.c2cr().write(|w| unsafe { w.dmareq_id().bits(DMAREQ_SPI1_RX) });
        let dr = p.spi.spi_sspdr().as_ptr() as u32;
        p.dma.dma_cpar2().write(|w| unsafe { w.bits(dr) });
        p.dma.dma_cpar3().write(|w| unsafe { w.bits(dr) });
        unsafe { cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA) };
        Self { p }
    }

    pub fn drdy(&self, on: bool) {
        self.p.gpioa.bsrr().write(|w| unsafe { w.bits(if on { 1 << 5 } else { 1 << (5 + 16) }) });
    }

    /// Reset SPI1 (empties both FIFOs), load `tx` for the next transfer and capture MOSI into `rx`.
    /// Order per RM0511 27.5.8: RXDMAEN, DMA channels, TXDMAEN, then SPE.
    pub fn arm(&mut self, tx: &Frame, rx: &mut [u8; FRAME]) {
        let p = &self.p;
        let (d, s) = (&p.dma, &p.spi);
        d.dma_ccr2().write(|w| unsafe { w.bits(0) });
        d.dma_ccr3().write(|w| unsafe { w.bits(0) });
        p.rcc.apb1rstr().modify(|_, w| w.spi1rst().set_bit());
        p.rcc.apb1rstr().modify(|_, w| w.spi1rst().clear_bit());
        d.dma_ifcr().write(|w| unsafe { w.bits(0xF << 4 | 0xF << 8) }); // ch2, ch3 flags

        // Slave, mode 0, MSB first, hardware NSS; 8-bit frames, RXNE at 8 bits.
        s.spi_sspcr1().write(|w| unsafe { w.bits(0) });
        s.spi_sspcr2().write(|w| unsafe { w.bits(0b0111 << 8 | 1 << 12 | 1 << 0) }); // DS=8, FRXTH, RXDMAEN

        d.dma_cmar3().write(|w| unsafe { w.bits(rx.as_mut_ptr() as u32) });
        d.dma_cndtr3().write(|w| unsafe { w.bits(FRAME as u32) });
        DONE.store(false, Ordering::Release);
        d.dma_ccr3().write(|w| unsafe { w.bits(1 << 7 | 1 << 1 | 1) }); // MINC, TCIE, EN; 8-bit both sides
        d.dma_cmar2().write(|w| unsafe { w.bits(tx.0.as_ptr() as u32) });
        d.dma_cndtr2().write(|w| unsafe { w.bits(FRAME as u32) });
        d.dma_ccr2().write(|w| unsafe { w.bits(1 << 7 | 1 << 4 | 1) }); // MINC, DIR mem->periph, EN

        s.spi_sspcr2().modify(|r, w| unsafe { w.bits(r.bits() | 1 << 1) }); // TXDMAEN
        s.spi_sspcr1().modify(|r, w| unsafe { w.bits(r.bits() | 1 << 6) }); // SPE
    }

    /// The master has clocked a whole frame (every MOSI byte landed).
    pub fn done(&self) -> bool {
        DONE.load(Ordering::Acquire)
    }

    /// MOSI bytes received so far in the current transfer.
    pub fn received(&self) -> usize {
        FRAME - self.p.dma.dma_cndtr3().read().bits() as usize
    }

    /// NSS is high: no transfer in progress.
    pub fn idle(&self) -> bool {
        self.p.gpiob.idr().read().bits() & 1 << 10 != 0
    }

    /// RX overrun seen (DMA too slow for SCK).
    pub fn overrun(&self) -> bool {
        self.p.spi.spi_sspsr().read().bits() & 1 << 6 != 0
    }
}

pub const KIND_TAP: u8 = 1;
/// Narrowed I/Q, 512 complex u8 offset-binary pairs; flags = rate_exp | shift << 4.
pub const KIND_IQ: u8 = 2;
const QDEPTH: usize = 3;

/// Frames queued for the master (kind TAP or IQ), aux0 = frames evicted
/// (queue full), aux1 = 10 ms ticks at capture. Polled from the stream loop, so a finished
/// transfer is noticed within one SysTick (10 ms); the tap fills a frame every 65 ms.
pub struct Tap {
    q: [Frame; QDEPTH],
    rx: [u8; FRAME],
    head: usize,
    len: usize,
    armed: bool,
    stalled: bool,
    seq: u16,
    pub dropped: u32,
    pub aborts: u32,
}

impl Tap {
    pub const fn new() -> Self {
        Self { q: [const { Frame::new() }; QDEPTH], rx: [0; FRAME], head: 0, len: 0, armed: false, stalled: false, seq: 0, dropped: 0, aborts: 0 }
    }

    /// Full queue: evict the oldest frame not being transferred, so the master always gets the
    /// freshest data (no stale backlog after an idle master). The gap shows in seq and aux0.
    pub fn push(&mut self, kind: u8, flags: u8, data: &[u8], ticks: u32) {
        if self.len == QDEPTH {
            self.dropped += 1;
            if self.armed {
                for k in 1..QDEPTH - 1 {
                    let next = self.q[(self.head + k + 1) % QDEPTH].0;
                    self.q[(self.head + k) % QDEPTH].0 = next;
                }
            } else {
                self.head = (self.head + 1) % QDEPTH;
            }
            self.len -= 1;
        }
        let f = &mut self.q[(self.head + self.len) % QDEPTH];
        f.payload().copy_from_slice(&data[..PAYLOAD]);
        f.seal(kind, flags, self.seq, self.dropped, ticks);
        self.seq = self.seq.wrapping_add(1);
        self.len += 1;
    }

    /// Advance the transfer state; returns a command the master sent with the last frame.
    pub fn poll(&mut self, link: &mut Link) -> Option<(u8, u32)> {
        let mut cmd = None;
        if self.armed {
            if link.done() {
                let mut parser = crate::uart::CmdParser::new();
                cmd = self.rx[..6].iter().find_map(|&b| parser.feed(b));
                self.head = (self.head + 1) % QDEPTH;
                self.len -= 1;
                self.armed = false;
            } else if link.received() > 0 && link.idle() {
                // Aborted short transfer: seen on two polls (>= 10 ms apart), so not a mid-frame gap.
                if self.stalled {
                    self.aborts += 1;
                    self.armed = false;
                }
                self.stalled = !self.stalled;
            } else if link.received() == 0 && self.len == QDEPTH {
                // Master idle and queue full: retire the armed frame too, or it goes stale.
                link.drdy(false);
                self.head = (self.head + 1) % QDEPTH;
                self.len -= 1;
                self.dropped += 1;
                self.armed = false;
            } else {
                self.stalled = false;
            }
            if self.armed {
                return cmd;
            }
            link.drdy(false);
        }
        if self.len > 0 {
            link.arm(&self.q[self.head], &mut self.rx);
            link.drdy(true);
            self.armed = true;
            self.stalled = false;
        }
        cmd
    }
}
