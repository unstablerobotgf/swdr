#[doc = "Register `IWDG_WINR` reader"]
pub type R = crate::R<IwdgWinrSpec>;
#[doc = "Register `IWDG_WINR` writer"]
pub type W = crate::W<IwdgWinrSpec>;
#[doc = "Field `WIN` reader - Watchdog counter window value. Set and reset by software. These bits are write access protected. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
pub type WinR = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Watchdog counter window value. Set and reset by software. These bits are write access protected. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value. Set and reset by software. These bits are write access protected. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value. Set and reset by software. These bits are write access protected. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value."]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<'_, IwdgWinrSpec> {
        WinW::new(self, 0)
    }
}
#[doc = "IWDG_WINR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgWinrSpec;
impl crate::RegisterSpec for IwdgWinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_winr::R`](R) reader structure"]
impl crate::Readable for IwdgWinrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_winr::W`](W) writer structure"]
impl crate::Writable for IwdgWinrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_WINR to value 0x0fff"]
impl crate::Resettable for IwdgWinrSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
