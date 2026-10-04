#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `MRSUBGRST` reader - Radio MRSUBG reset. Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type MrsubgrstR = crate::BitReader;
#[doc = "Field `MRSUBGRST` writer - Radio MRSUBG reset. Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type MrsubgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPAWURRST` reader - Bubble reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type LpawurrstR = crate::BitReader;
#[doc = "Field `LPAWURRST` writer - Bubble reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
pub type LpawurrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Radio MRSUBG reset. Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn mrsubgrst(&self) -> MrsubgrstR {
        MrsubgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Bubble reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn lpawurrst(&self) -> LpawurrstR {
        LpawurrstR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Radio MRSUBG reset. Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn mrsubgrst(&mut self) -> MrsubgrstW<'_, Apb2rstrSpec> {
        MrsubgrstW::new(self, 0)
    }
    #[doc = "Bit 3 - Bubble reset Set and reset by software. 0: IP is not under reset. 1: IP is under reset."]
    #[inline(always)]
    pub fn lpawurrst(&mut self) -> LpawurrstW<'_, Apb2rstrSpec> {
        LpawurrstW::new(self, 3)
    }
}
#[doc = "APB2RSTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {}
