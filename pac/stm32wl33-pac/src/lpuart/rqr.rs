#[doc = "Register `RQR` reader"]
pub type R = crate::R<RqrSpec>;
#[doc = "Register `RQR` writer"]
pub type W = crate::W<RqrSpec>;
#[doc = "Field `SBKRQ` writer - SBKRQ: Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available."]
pub type SbkrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` writer - MMRQ: Mute mode request Writing 1 to this bit puts the USART in mute mode and resets the RWU flag."]
pub type MmrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` writer - RXFRQ: Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This allows to discard the received data without reading them, and avoid an overrun condition."]
pub type RxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` writer - TXFRQ: Transmit data flush request When FIFO mode is disabled, Writing 1 to this bit sets the TXE flag. This allows to discard the transmit data. This bit must be used only in Smartcard mode, when data has not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and forced by hardware to 0 When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO . This will set the flag TXFE (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes."]
pub type TxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - SBKRQ: Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available."]
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SbkrqW<'_, RqrSpec> {
        SbkrqW::new(self, 1)
    }
    #[doc = "Bit 2 - MMRQ: Mute mode request Writing 1 to this bit puts the USART in mute mode and resets the RWU flag."]
    #[inline(always)]
    pub fn mmrq(&mut self) -> MmrqW<'_, RqrSpec> {
        MmrqW::new(self, 2)
    }
    #[doc = "Bit 3 - RXFRQ: Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This allows to discard the received data without reading them, and avoid an overrun condition."]
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RxfrqW<'_, RqrSpec> {
        RxfrqW::new(self, 3)
    }
    #[doc = "Bit 4 - TXFRQ: Transmit data flush request When FIFO mode is disabled, Writing 1 to this bit sets the TXE flag. This allows to discard the transmit data. This bit must be used only in Smartcard mode, when data has not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and forced by hardware to 0 When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO . This will set the flag TXFE (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes."]
    #[inline(always)]
    pub fn txfrq(&mut self) -> TxfrqW<'_, RqrSpec> {
        TxfrqW::new(self, 4)
    }
}
#[doc = "RQR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RqrSpec;
impl crate::RegisterSpec for RqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rqr::R`](R) reader structure"]
impl crate::Readable for RqrSpec {}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RqrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RqrSpec {}
