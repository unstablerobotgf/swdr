#[doc = "Register `CSCMDR` reader"]
pub type R = crate::R<CscmdrSpec>;
#[doc = "Register `CSCMDR` writer"]
pub type W = crate::W<CscmdrSpec>;
#[doc = "Field `REQUEST` reader - Request for system clock switching Cleared by hardware when system clock frequency switch is done 0: To cancel an ongiong request - still possible until IRQ assertion 1: To update the system clock frequency"]
pub type RequestR = crate::BitReader;
#[doc = "Field `REQUEST` writer - Request for system clock switching Cleared by hardware when system clock frequency switch is done 0: To cancel an ongiong request - still possible until IRQ assertion 1: To update the system clock frequency"]
pub type RequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSYSDIV_REQ` reader - system clock frequency selection request 000: div1 (HSI 64M / HSE) (48M) 001: div2 (HSI 32M / HSE (24M*) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN"]
pub type ClksysdivReqR = crate::FieldReader;
#[doc = "Field `CLKSYSDIV_REQ` writer - system clock frequency selection request 000: div1 (HSI 64M / HSE) (48M) 001: div2 (HSI 32M / HSE (24M*) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN"]
pub type ClksysdivReqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STATUS` reader - Status of clock switch sequence 00: IDLE no switch requested 01: ONGOING clock frequency switch is ongoing 10: DONE clock frequency switch done 11: Reserved"]
pub type StatusR = crate::FieldReader;
#[doc = "Field `EOFSEQ_IE` reader - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch. 0: End of sequence interrupt disabled 1: End of sequence interrupt enabled"]
pub type EofseqIeR = crate::BitReader;
#[doc = "Field `EOFSEQ_IE` writer - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch. 0: End of sequence interrupt disabled 1: End of sequence interrupt enabled"]
pub type EofseqIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOFSEQ_IRQ` reader - End of Sequence flag Set by hardware when clock system swtich is ended 0: No end of sequence event occured 1: End of sequece event occured"]
pub type EofseqIrqR = crate::BitReader;
#[doc = "Field `EOFSEQ_IRQ` writer - End of Sequence flag Set by hardware when clock system swtich is ended 0: No end of sequence event occured 1: End of sequece event occured"]
pub type EofseqIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Request for system clock switching Cleared by hardware when system clock frequency switch is done 0: To cancel an ongiong request - still possible until IRQ assertion 1: To update the system clock frequency"]
    #[inline(always)]
    pub fn request(&self) -> RequestR {
        RequestR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - system clock frequency selection request 000: div1 (HSI 64M / HSE) (48M) 001: div2 (HSI 32M / HSE (24M*) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN"]
    #[inline(always)]
    pub fn clksysdiv_req(&self) -> ClksysdivReqR {
        ClksysdivReqR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Status of clock switch sequence 00: IDLE no switch requested 01: ONGOING clock frequency switch is ongoing 10: DONE clock frequency switch done 11: Reserved"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch. 0: End of sequence interrupt disabled 1: End of sequence interrupt enabled"]
    #[inline(always)]
    pub fn eofseq_ie(&self) -> EofseqIeR {
        EofseqIeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Sequence flag Set by hardware when clock system swtich is ended 0: No end of sequence event occured 1: End of sequece event occured"]
    #[inline(always)]
    pub fn eofseq_irq(&self) -> EofseqIrqR {
        EofseqIrqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request for system clock switching Cleared by hardware when system clock frequency switch is done 0: To cancel an ongiong request - still possible until IRQ assertion 1: To update the system clock frequency"]
    #[inline(always)]
    pub fn request(&mut self) -> RequestW<'_, CscmdrSpec> {
        RequestW::new(self, 0)
    }
    #[doc = "Bits 1:3 - system clock frequency selection request 000: div1 (HSI 64M / HSE) (48M) 001: div2 (HSI 32M / HSE (24M*) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN"]
    #[inline(always)]
    pub fn clksysdiv_req(&mut self) -> ClksysdivReqW<'_, CscmdrSpec> {
        ClksysdivReqW::new(self, 1)
    }
    #[doc = "Bit 6 - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch. 0: End of sequence interrupt disabled 1: End of sequence interrupt enabled"]
    #[inline(always)]
    pub fn eofseq_ie(&mut self) -> EofseqIeW<'_, CscmdrSpec> {
        EofseqIeW::new(self, 6)
    }
    #[doc = "Bit 7 - End of Sequence flag Set by hardware when clock system swtich is ended 0: No end of sequence event occured 1: End of sequece event occured"]
    #[inline(always)]
    pub fn eofseq_irq(&mut self) -> EofseqIrqW<'_, CscmdrSpec> {
        EofseqIrqW::new(self, 7)
    }
}
#[doc = "CSCMDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cscmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscmdrSpec;
impl crate::RegisterSpec for CscmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cscmdr::R`](R) reader structure"]
impl crate::Readable for CscmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`cscmdr::W`](W) writer structure"]
impl crate::Writable for CscmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCMDR to value 0x80"]
impl crate::Resettable for CscmdrSpec {
    const RESET_VALUE: u32 = 0x80;
}
