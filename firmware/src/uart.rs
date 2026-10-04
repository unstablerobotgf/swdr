//! I/Q transport over the STLINK-V3EC VCP: USART1 (PA1 TX / PA15 RX, AF1), 2 Mbaud, DMA1 ch1.
//! SWD block reads zero-fill while MRSUBG runs, so bulk data must not go through the debugger.

use stm32wl33_pac as pac;

pub const PAYLOAD: usize = 1024;
const HDR: usize = 8;
/// DMAMUX request 13 = USART_TX (RM0511 Table 30).
const DMAREQ_USART_TX: u8 = 13;

/// Frame: A5 5A seq:u16 len:u16 rate_exp:u8 shift:u8, then `len` bytes of u8 I/Q.
#[repr(C, align(4))]
pub struct Frame([u8; HDR + PAYLOAD]);

impl Frame {
    pub const fn new() -> Self {
        Self([0; HDR + PAYLOAD])
    }

    pub fn payload(&mut self) -> &mut [u8] {
        &mut self.0[HDR..]
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn seal(&mut self, seq: u16, rate_exp: u8, shift: u8) {
        let len = PAYLOAD as u16;
        self.0[..HDR].copy_from_slice(&[0xA5, 0x5A, seq as u8, (seq >> 8) as u8, len as u8, (len >> 8) as u8, rate_exp, shift]);
    }
}

/// Host commands on any transport: C5 op arg:u32 LE. Ops: 1 freq Hz, 2 rate exponent, 3 shift.
pub struct CmdParser {
    buf: [u8; 6],
    n: usize,
}

impl CmdParser {
    pub const fn new() -> Self {
        Self { buf: [0; 6], n: 0 }
    }

    pub fn feed(&mut self, b: u8) -> Option<(u8, u32)> {
        if self.n == 0 && b != 0xC5 {
            return None;
        }
        self.buf[self.n] = b;
        self.n += 1;
        if self.n < 6 {
            return None;
        }
        self.n = 0;
        let c = self.buf;
        Some((c[1], u32::from_le_bytes([c[2], c[3], c[4], c[5]])))
    }
}

pub struct Vcp {
    p: pac::Peripherals,
    cmd: CmdParser,
}

impl Vcp {
    pub fn new() -> Self {
        let p = unsafe { pac::Peripherals::steal() };
        p.rcc.apb1enr().modify(|_, w| w.usarten().set_bit());
        p.rcc.ahbenr().modify(|_, w| w.dmaen().set_bit().gpioaen().set_bit());

        // PA1, PA15 -> AF1. PA2/PA3 are SWD; leave them alone.
        p.gpioa.moder().modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << 2 | 0b11 << 30)) | 0b10 << 2 | 0b10 << 30) });
        p.gpioa.afrl().modify(|r, w| unsafe { w.bits((r.bits() & !(0xF << 4)) | 1 << 4) });
        p.gpioa.afrh().modify(|r, w| unsafe { w.bits((r.bits() & !(0xF << 28)) | 1 << 28) });

        // 16 MHz kernel clock, OVER8: BRR = 0x10 is exactly 2 Mbaud (HAL UART_DIV_SAMPLING8).
        let u = &p.usart;
        u.cr1().write(|w| unsafe { w.bits(0) });
        u.brr().write(|w| unsafe { w.bits(0x10) });
        u.cr3().write(|w| unsafe { w.bits(0) });
        u.cr1().write(|w| w.over8().set_bit().fifoen().set_bit().te().set_bit().re().set_bit().ue().set_bit());

        Self { p, cmd: CmdParser::new() }
    }

    /// Switch from blocking markers to DMA streaming.
    pub fn start_dma(&mut self) {
        let p = &self.p;
        let u = &p.usart;
        u.cr3().write(|w| w.dmat().set_bit());
        p.dmamux.c0cr().write(|w| unsafe { w.dmareq_id().bits(DMAREQ_USART_TX) });
        p.dma.dma_cpar1().write(|w| unsafe { w.bits(u.tdr().as_ptr() as u32) });
    }

    /// Blocking write for boot-stage markers; only used before DMA streaming starts.
    pub fn mark(&self, msg: &str) {
        let u = &self.p.usart;
        for b in msg.bytes().chain([13u8, 10]) {
            while u.isr().read().txe_txfnf().bit_is_clear() {}
            u.tdr().write(|w| unsafe { w.bits(b as u32) });
        }
        while u.isr().read().tc().bit_is_clear() {}
    }

    pub fn busy(&self) -> bool {
        let d = &self.p.dma;
        d.dma_ccr1().read().en().bit_is_set() && d.dma_isr().read().tcif1().bit_is_clear()
    }

    /// Start DMA of a sealed frame. Caller must not touch `f` until `busy()` is false.
    pub fn send(&mut self, f: &Frame) {
        let d = &self.p.dma;
        d.dma_ccr1().write(|w| unsafe { w.bits(0) });
        d.dma_ifcr().write(|w| w.cgif1().set_bit());
        d.dma_cmar1().write(|w| unsafe { w.bits(f.0.as_ptr() as u32) });
        d.dma_cndtr1().write(|w| unsafe { w.bits(f.0.len() as u32) });
        d.dma_ccr1().write(|w| w.dir().set_bit().minc().set_bit().en().set_bit());
    }

    pub fn poll_cmd(&mut self) -> Option<(u8, u32)> {
        let u = &self.p.usart;
        if u.isr().read().ore().bit_is_set() {
            u.icr().write(|w| unsafe { w.bits(1 << 3) }); // ORECF
        }
        while u.isr().read().rxne_rxfne().bit_is_set() {
            if let Some(c) = self.cmd.feed(u.rdr().read().bits() as u8) {
                return Some(c);
            }
        }
        None
    }
}
