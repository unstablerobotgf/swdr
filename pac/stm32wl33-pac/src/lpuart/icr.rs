#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `PECF` writer - PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
pub type PecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register"]
pub type FecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` writer - NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register."]
pub type NecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
pub type OrecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
pub type IdlecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register"]
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register"]
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register"]
pub type CmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - WUCF: Wakeup from Stop mode clear flag Writing 1 to this bit clears the WUF flag in the LPUART_ISR register."]
pub type WucfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn pecf(&mut self) -> PecfW<'_, IcrSpec> {
        PecfW::new(self, 0)
    }
    #[doc = "Bit 1 - FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register"]
    #[inline(always)]
    pub fn fecf(&mut self) -> FecfW<'_, IcrSpec> {
        FecfW::new(self, 1)
    }
    #[doc = "Bit 2 - NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register."]
    #[inline(always)]
    pub fn necf(&mut self) -> NecfW<'_, IcrSpec> {
        NecfW::new(self, 2)
    }
    #[doc = "Bit 3 - ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn orecf(&mut self) -> OrecfW<'_, IcrSpec> {
        OrecfW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn idlecf(&mut self) -> IdlecfW<'_, IcrSpec> {
        IdlecfW::new(self, 4)
    }
    #[doc = "Bit 6 - TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register"]
    #[inline(always)]
    pub fn tccf(&mut self) -> TccfW<'_, IcrSpec> {
        TccfW::new(self, 6)
    }
    #[doc = "Bit 9 - CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register"]
    #[inline(always)]
    pub fn ctscf(&mut self) -> CtscfW<'_, IcrSpec> {
        CtscfW::new(self, 9)
    }
    #[doc = "Bit 17 - CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register"]
    #[inline(always)]
    pub fn cmcf(&mut self) -> CmcfW<'_, IcrSpec> {
        CmcfW::new(self, 17)
    }
    #[doc = "Bit 20 - WUCF: Wakeup from Stop mode clear flag Writing 1 to this bit clears the WUF flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn wucf(&mut self) -> WucfW<'_, IcrSpec> {
        WucfW::new(self, 20)
    }
}
#[doc = "ICR register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
