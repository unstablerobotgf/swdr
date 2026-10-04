#[doc = "Register `IWDG_RLR` reader"]
pub type R = crate::R<IwdgRlrSpec>;
#[doc = "Register `IWDG_RLR` writer"]
pub type W = crate::W<IwdgRlrSpec>;
#[doc = "Field `RL` reader - Watchdog counter reload value. Set and reset by software. These bits are write access protected. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG_KR register. The watchdog counter counts down from this value. The timeout period is a function of this value and the clock prescaler. The RVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
pub type RlR = crate::FieldReader<u16>;
#[doc = "Field `RL` writer - Watchdog counter reload value. Set and reset by software. These bits are write access protected. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG_KR register. The watchdog counter counts down from this value. The timeout period is a function of this value and the clock prescaler. The RVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
pub type RlW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value. Set and reset by software. These bits are write access protected. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG_KR register. The watchdog counter counts down from this value. The timeout period is a function of this value and the clock prescaler. The RVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value. Set and reset by software. These bits are write access protected. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG_KR register. The watchdog counter counts down from this value. The timeout period is a function of this value and the clock prescaler. The RVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
    #[inline(always)]
    pub fn rl(&mut self) -> RlW<'_, IwdgRlrSpec> {
        RlW::new(self, 0)
    }
}
#[doc = "IWDG_RLR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgRlrSpec;
impl crate::RegisterSpec for IwdgRlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_rlr::R`](R) reader structure"]
impl crate::Readable for IwdgRlrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_rlr::W`](W) writer structure"]
impl crate::Writable for IwdgRlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_RLR to value 0x0fff"]
impl crate::Resettable for IwdgRlrSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
