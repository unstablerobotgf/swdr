#[doc = "Register `IWDG_KR` reader"]
pub type R = crate::R<IwdgKrSpec>;
#[doc = "Register `IWDG_KR` writer"]
pub type W = crate::W<IwdgKrSpec>;
#[doc = "Field `KEY` writer - Key value. Software can only write these bits. Reading returns the reset value. These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enables access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers. Writing the key value CCCCh starts the watchdog"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Key value. Software can only write these bits. Reading returns the reset value. These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enables access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers. Writing the key value CCCCh starts the watchdog"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, IwdgKrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "IWDG_KR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_kr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_kr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgKrSpec;
impl crate::RegisterSpec for IwdgKrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_kr::R`](R) reader structure"]
impl crate::Readable for IwdgKrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_kr::W`](W) writer structure"]
impl crate::Writable for IwdgKrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_KR to value 0"]
impl crate::Resettable for IwdgKrSpec {}
