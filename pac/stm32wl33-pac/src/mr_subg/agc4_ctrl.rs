#[doc = "Register `AGC4_CTRL` reader"]
pub type R = crate::R<Agc4CtrlSpec>;
#[doc = "Register `AGC4_CTRL` writer"]
pub type W = crate::W<Agc4CtrlSpec>;
#[doc = "Field `AGC_FREEZE_THR` reader - Signal threshold for the autofreeze feature."]
pub type AgcFreezeThrR = crate::FieldReader;
#[doc = "Field `AGC_FREEZE_THR` writer - Signal threshold for the autofreeze feature."]
pub type AgcFreezeThrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Signal threshold for the autofreeze feature."]
    #[inline(always)]
    pub fn agc_freeze_thr(&self) -> AgcFreezeThrR {
        AgcFreezeThrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal threshold for the autofreeze feature."]
    #[inline(always)]
    pub fn agc_freeze_thr(&mut self) -> AgcFreezeThrW<'_, Agc4CtrlSpec> {
        AgcFreezeThrW::new(self, 0)
    }
}
#[doc = "AGC4_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`agc4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agc4CtrlSpec;
impl crate::RegisterSpec for Agc4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agc4_ctrl::R`](R) reader structure"]
impl crate::Readable for Agc4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`agc4_ctrl::W`](W) writer structure"]
impl crate::Writable for Agc4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGC4_CTRL to value 0x02"]
impl crate::Resettable for Agc4CtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
