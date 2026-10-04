#[doc = "Register `IWUF` reader"]
pub type R = crate::R<IwufSpec>;
#[doc = "Register `IWUF` writer"]
pub type W = crate::W<IwufSpec>;
#[doc = "Field `IWUF0` reader - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf0R = crate::BitReader;
#[doc = "Field `IWUF0` writer - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUF1` reader - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred"]
pub type Iwuf1R = crate::BitReader;
#[doc = "Field `IWUF1` writer - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred"]
pub type Iwuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUF2` reader - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf2R = crate::BitReader;
#[doc = "Field `IWUF2` writer - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUF3` reader - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf3R = crate::BitReader;
#[doc = "Field `IWUF3` writer - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUF4` reader - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf4R = crate::BitReader;
#[doc = "Field `IWUF4` writer - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit."]
pub type Iwuf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMRSUBGF` reader - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt"]
pub type WmrsubgfR = crate::BitReader;
#[doc = "Field `WMRSUBGF` writer - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt"]
pub type WmrsubgfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMRSUBGHCPUF` reader - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt"]
pub type WmrsubghcpufR = crate::BitReader;
#[doc = "Field `WMRSUBGHCPUF` writer - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt"]
pub type WmrsubghcpufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLPAWURF` reader - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt"]
pub type WlpawurfR = crate::BitReader;
#[doc = "Field `WLPAWURF` writer - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt"]
pub type WlpawurfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf0(&self) -> Iwuf0R {
        Iwuf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred"]
    #[inline(always)]
    pub fn iwuf1(&self) -> Iwuf1R {
        Iwuf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf2(&self) -> Iwuf2R {
        Iwuf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf3(&self) -> Iwuf3R {
        Iwuf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf4(&self) -> Iwuf4R {
        Iwuf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt"]
    #[inline(always)]
    pub fn wmrsubgf(&self) -> WmrsubgfR {
        WmrsubgfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt"]
    #[inline(always)]
    pub fn wmrsubghcpuf(&self) -> WmrsubghcpufR {
        WmrsubghcpufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt"]
    #[inline(always)]
    pub fn wlpawurf(&self) -> WlpawurfR {
        WlpawurfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf0(&mut self) -> Iwuf0W<'_, IwufSpec> {
        Iwuf0W::new(self, 0)
    }
    #[doc = "Bit 1 - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred"]
    #[inline(always)]
    pub fn iwuf1(&mut self) -> Iwuf1W<'_, IwufSpec> {
        Iwuf1W::new(self, 1)
    }
    #[doc = "Bit 2 - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf2(&mut self) -> Iwuf2W<'_, IwufSpec> {
        Iwuf2W::new(self, 2)
    }
    #[doc = "Bit 3 - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf3(&mut self) -> Iwuf3W<'_, IwufSpec> {
        Iwuf3W::new(self, 3)
    }
    #[doc = "Bit 4 - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit."]
    #[inline(always)]
    pub fn iwuf4(&mut self) -> Iwuf4W<'_, IwufSpec> {
        Iwuf4W::new(self, 4)
    }
    #[doc = "Bit 8 - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt"]
    #[inline(always)]
    pub fn wmrsubgf(&mut self) -> WmrsubgfW<'_, IwufSpec> {
        WmrsubgfW::new(self, 8)
    }
    #[doc = "Bit 9 - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt"]
    #[inline(always)]
    pub fn wmrsubghcpuf(&mut self) -> WmrsubghcpufW<'_, IwufSpec> {
        WmrsubghcpufW::new(self, 9)
    }
    #[doc = "Bit 10 - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt"]
    #[inline(always)]
    pub fn wlpawurf(&mut self) -> WlpawurfW<'_, IwufSpec> {
        WlpawurfW::new(self, 10)
    }
}
#[doc = "IWUF register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwufSpec;
impl crate::RegisterSpec for IwufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwuf::R`](R) reader structure"]
impl crate::Readable for IwufSpec {}
#[doc = "`write(|w| ..)` method takes [`iwuf::W`](W) writer structure"]
impl crate::Writable for IwufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWUF to value 0"]
impl crate::Resettable for IwufSpec {}
