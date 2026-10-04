#[doc = "Register `AGC0_CTRL` reader"]
pub type R = crate::R<Agc0CtrlSpec>;
#[doc = "Register `AGC0_CTRL` writer"]
pub type W = crate::W<Agc0CtrlSpec>;
#[doc = "Field `AGC_HOLD_TIME` reader - AGC hold time."]
pub type AgcHoldTimeR = crate::FieldReader;
#[doc = "Field `AGC_HOLD_TIME` writer - AGC hold time."]
pub type AgcHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AGC_START_ONHOLD` reader - Start the AGC with a hold phase."]
pub type AgcStartOnholdR = crate::BitReader;
#[doc = "Field `AGC_START_ONHOLD` writer - Start the AGC with a hold phase."]
pub type AgcStartOnholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_EN` reader - Enable the AGC"]
pub type AgcEnR = crate::BitReader;
#[doc = "Field `AGC_EN` writer - Enable the AGC"]
pub type AgcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - AGC hold time."]
    #[inline(always)]
    pub fn agc_hold_time(&self) -> AgcHoldTimeR {
        AgcHoldTimeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Start the AGC with a hold phase."]
    #[inline(always)]
    pub fn agc_start_onhold(&self) -> AgcStartOnholdR {
        AgcStartOnholdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the AGC"]
    #[inline(always)]
    pub fn agc_en(&self) -> AgcEnR {
        AgcEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - AGC hold time."]
    #[inline(always)]
    pub fn agc_hold_time(&mut self) -> AgcHoldTimeW<'_, Agc0CtrlSpec> {
        AgcHoldTimeW::new(self, 0)
    }
    #[doc = "Bit 6 - Start the AGC with a hold phase."]
    #[inline(always)]
    pub fn agc_start_onhold(&mut self) -> AgcStartOnholdW<'_, Agc0CtrlSpec> {
        AgcStartOnholdW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable the AGC"]
    #[inline(always)]
    pub fn agc_en(&mut self) -> AgcEnW<'_, Agc0CtrlSpec> {
        AgcEnW::new(self, 7)
    }
}
#[doc = "AGC0_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agc0CtrlSpec;
impl crate::RegisterSpec for Agc0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc0_ctrl::R`](R) reader structure"]
impl crate::Readable for Agc0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`agc0_ctrl::W`](W) writer structure"]
impl crate::Writable for Agc0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC0_CTRL to value 0x99"]
impl crate::Resettable for Agc0CtrlSpec {
    const RESET_VALUE: u32 = 0x99;
}
