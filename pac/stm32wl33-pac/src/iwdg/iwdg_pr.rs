#[doc = "Register `IWDG_PR` reader"]
pub type R = crate::R<IwdgPrSpec>;
#[doc = "Register `IWDG_PR` writer"]
pub type W = crate::W<IwdgPrSpec>;
#[doc = "Field `PR` reader - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256"]
pub type PrR = crate::FieldReader;
#[doc = "Field `PR` writer - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256"]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider. Set and reset by software. These bits are write access protected. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of IWDG_SR must be reset in order to be able to change the prescaler divider. 000: divider/4 001: divider/8 010: divider/16 011: divider/32 100: divider/64 101: divider/128 110: divider/256 111: divider/256"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, IwdgPrSpec> {
        PrW::new(self, 0)
    }
}
#[doc = "IWDG_PR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgPrSpec;
impl crate::RegisterSpec for IwdgPrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_pr::R`](R) reader structure"]
impl crate::Readable for IwdgPrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_pr::W`](W) writer structure"]
impl crate::Writable for IwdgPrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_PR to value 0"]
impl crate::Resettable for IwdgPrSpec {}
