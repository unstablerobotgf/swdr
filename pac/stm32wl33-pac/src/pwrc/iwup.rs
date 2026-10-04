#[doc = "Register `IWUP` reader"]
pub type R = crate::R<IwupSpec>;
#[doc = "Register `IWUP` writer"]
pub type W = crate::W<IwupSpec>;
#[doc = "Field `IWUP0` reader - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup0R = crate::BitReader;
#[doc = "Field `IWUP0` writer - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUP1` reader - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup1R = crate::BitReader;
#[doc = "Field `IWUP1` writer - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUP2` reader - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup2R = crate::BitReader;
#[doc = "Field `IWUP2` writer - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUP3` reader - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup3R = crate::BitReader;
#[doc = "Field `IWUP3` writer - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWUP4` reader - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup4R = crate::BitReader;
#[doc = "Field `IWUP4` writer - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type Iwup4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMRSUBGHP` reader - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type WmrsubghpR = crate::BitReader;
#[doc = "Field `WMRSUBGHP` writer - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type WmrsubghpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMRSUBGHCPUP` reader - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type WmrsubghcpupR = crate::BitReader;
#[doc = "Field `WMRSUBGHCPUP` writer - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type WmrsubghcpupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLPAWURP` reader - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type WlpawurpR = crate::BitReader;
#[doc = "Field `WLPAWURP` writer - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
pub type WlpawurpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup0(&self) -> Iwup0R {
        Iwup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup1(&self) -> Iwup1R {
        Iwup1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup2(&self) -> Iwup2R {
        Iwup2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup3(&self) -> Iwup3R {
        Iwup3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup4(&self) -> Iwup4R {
        Iwup4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn wmrsubghp(&self) -> WmrsubghpR {
        WmrsubghpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn wmrsubghcpup(&self) -> WmrsubghcpupR {
        WmrsubghcpupR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn wlpawurp(&self) -> WlpawurpR {
        WlpawurpR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IWUP0: Wakeup polarity for internal wakeup line 0 event (LPUART). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup0(&mut self) -> Iwup0W<'_, IwupSpec> {
        Iwup0W::new(self, 0)
    }
    #[doc = "Bit 1 - IWUP1: Wakeup polarity for internal wakeup line 1 event (RTC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup1(&mut self) -> Iwup1W<'_, IwupSpec> {
        Iwup1W::new(self, 1)
    }
    #[doc = "Bit 2 - IWUP2: Wakeup polarity for internal wakeup line 2 event (LCD). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup2(&mut self) -> Iwup2W<'_, IwupSpec> {
        Iwup2W::new(self, 2)
    }
    #[doc = "Bit 3 - IWUP3: Wakeup polarity for internal wakeup line 3 event (COMP). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup3(&mut self) -> Iwup3W<'_, IwupSpec> {
        Iwup3W::new(self, 3)
    }
    #[doc = "Bit 4 - IWUP4: Wakeup polarity for internal wakeup line 4 event (LCSC). - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn iwup4(&mut self) -> Iwup4W<'_, IwupSpec> {
        Iwup4W::new(self, 4)
    }
    #[doc = "Bit 8 - WMRSUBGHP: Wakeup polarity for internal wakeup MRSUBG event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn wmrsubghp(&mut self) -> WmrsubghpW<'_, IwupSpec> {
        WmrsubghpW::new(self, 8)
    }
    #[doc = "Bit 9 - WMRSUBGHCPUP: Wakeup polarity for internal wakeup MRSUBG Host CPU event - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn wmrsubghcpup(&mut self) -> WmrsubghcpupW<'_, IwupSpec> {
        WmrsubghcpupW::new(self, 9)
    }
    #[doc = "Bit 10 - WLPAWURP: Wakeup polarity for wakeup LPAWUR event. - 0: Detection of wakeup event on rising edge (default). - 1: Detection of wakeup event on falling edge."]
    #[inline(always)]
    pub fn wlpawurp(&mut self) -> WlpawurpW<'_, IwupSpec> {
        WlpawurpW::new(self, 10)
    }
}
#[doc = "IWUP register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwupSpec;
impl crate::RegisterSpec for IwupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwup::R`](R) reader structure"]
impl crate::Readable for IwupSpec {}
#[doc = "`write(|w| ..)` method takes [`iwup::W`](W) writer structure"]
impl crate::Writable for IwupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWUP to value 0"]
impl crate::Resettable for IwupSpec {}
