#[doc = "Register `DBGR` reader"]
pub type R = crate::R<DbgrSpec>;
#[doc = "Register `DBGR` writer"]
pub type W = crate::W<DbgrSpec>;
#[doc = "Field `DBGHSIOFF` reader - used for debug or test 0: No effect (default) 1: HSI forced off."]
pub type DbghsioffR = crate::BitReader;
#[doc = "Field `DBGHSIOFF` writer - used for debug or test 0: No effect (default) 1: HSI forced off."]
pub type DbghsioffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGBYPHSI` reader - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE."]
pub type DbgbyphsiR = crate::BitReader;
#[doc = "Field `DBGBYPHSI` writer - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE."]
pub type DbgbyphsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGXOEXT` reader - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used."]
pub type DbgxoextR = crate::BitReader;
#[doc = "Field `DBGXOEXT` writer - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used."]
pub type DbgxoextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEXO48MREADY` reader - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1"]
pub type Forcexo48mreadyR = crate::BitReader;
#[doc = "Field `FORCEXO48MREADY` writer - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1"]
pub type Forcexo48mreadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 19 - used for debug or test 0: No effect (default) 1: HSI forced off."]
    #[inline(always)]
    pub fn dbghsioff(&self) -> DbghsioffR {
        DbghsioffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE."]
    #[inline(always)]
    pub fn dbgbyphsi(&self) -> DbgbyphsiR {
        DbgbyphsiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used."]
    #[inline(always)]
    pub fn dbgxoext(&self) -> DbgxoextR {
        DbgxoextR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1"]
    #[inline(always)]
    pub fn forcexo48mready(&self) -> Forcexo48mreadyR {
        Forcexo48mreadyR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - used for debug or test 0: No effect (default) 1: HSI forced off."]
    #[inline(always)]
    pub fn dbghsioff(&mut self) -> DbghsioffW<'_, DbgrSpec> {
        DbghsioffW::new(self, 19)
    }
    #[doc = "Bit 20 - used for debug mode with HSI bypassed by HSE 0: No effect (default) 1: HSI bypassed HSE."]
    #[inline(always)]
    pub fn dbgbyphsi(&mut self) -> DbgbyphsiW<'_, DbgrSpec> {
        DbgbyphsiW::new(self, 20)
    }
    #[doc = "Bit 21 - used for debug mode with HSE bypassed by FXTAL_IN clock and ZIV12 output used. 0: No effect (default) 1: HSE bypassed by FXTAL_IN clock and ZIV12 output used."]
    #[inline(always)]
    pub fn dbgxoext(&mut self) -> DbgxoextW<'_, DbgrSpec> {
        DbgxoextW::new(self, 21)
    }
    #[doc = "Bit 22 - FORCEXO48MREADY Force XO48M Ready input signal This bit is for debug and force the XO48M ready input, in order to bypass XO48M comparators. 0: No effect (default) 1: Force XOREADY=1"]
    #[inline(always)]
    pub fn forcexo48mready(&mut self) -> Forcexo48mreadyW<'_, DbgrSpec> {
        Forcexo48mreadyW::new(self, 22)
    }
}
#[doc = "DBGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgrSpec;
impl crate::RegisterSpec for DbgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgr::R`](R) reader structure"]
impl crate::Readable for DbgrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgr::W`](W) writer structure"]
impl crate::Writable for DbgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGR to value 0"]
impl crate::Resettable for DbgrSpec {}
