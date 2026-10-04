#[doc = "Register `AFC2_CONFIG` reader"]
pub type R = crate::R<Afc2ConfigSpec>;
#[doc = "Register `AFC2_CONFIG` writer"]
pub type W = crate::W<Afc2ConfigSpec>;
#[doc = "Field `AFC_PD_LEAKAGE` reader - AFC Peak Detection leakage."]
pub type AfcPdLeakageR = crate::FieldReader;
#[doc = "Field `AFC_PD_LEAKAGE` writer - AFC Peak Detection leakage."]
pub type AfcPdLeakageW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AFC_MODE` reader - Select AFC mode:"]
pub type AfcModeR = crate::BitReader;
#[doc = "Field `AFC_MODE` writer - Select AFC mode:"]
pub type AfcModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFC_EN` reader - Enable AFC."]
pub type AfcEnR = crate::BitReader;
#[doc = "Field `AFC_EN` writer - Enable AFC."]
pub type AfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFC_FREEZE_ON_SYNC` reader - Freeze AFC correction upon SYNC word detection"]
pub type AfcFreezeOnSyncR = crate::BitReader;
#[doc = "Field `AFC_FREEZE_ON_SYNC` writer - Freeze AFC correction upon SYNC word detection"]
pub type AfcFreezeOnSyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - AFC Peak Detection leakage."]
    #[inline(always)]
    pub fn afc_pd_leakage(&self) -> AfcPdLeakageR {
        AfcPdLeakageR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Select AFC mode:"]
    #[inline(always)]
    pub fn afc_mode(&self) -> AfcModeR {
        AfcModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AFC."]
    #[inline(always)]
    pub fn afc_en(&self) -> AfcEnR {
        AfcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Freeze AFC correction upon SYNC word detection"]
    #[inline(always)]
    pub fn afc_freeze_on_sync(&self) -> AfcFreezeOnSyncR {
        AfcFreezeOnSyncR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - AFC Peak Detection leakage."]
    #[inline(always)]
    pub fn afc_pd_leakage(&mut self) -> AfcPdLeakageW<'_, Afc2ConfigSpec> {
        AfcPdLeakageW::new(self, 0)
    }
    #[doc = "Bit 5 - Select AFC mode:"]
    #[inline(always)]
    pub fn afc_mode(&mut self) -> AfcModeW<'_, Afc2ConfigSpec> {
        AfcModeW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable AFC."]
    #[inline(always)]
    pub fn afc_en(&mut self) -> AfcEnW<'_, Afc2ConfigSpec> {
        AfcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Freeze AFC correction upon SYNC word detection"]
    #[inline(always)]
    pub fn afc_freeze_on_sync(&mut self) -> AfcFreezeOnSyncW<'_, Afc2ConfigSpec> {
        AfcFreezeOnSyncW::new(self, 7)
    }
}
#[doc = "AFC2_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc2_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc2_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afc2ConfigSpec;
impl crate::RegisterSpec for Afc2ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afc2_config::R`](R) reader structure"]
impl crate::Readable for Afc2ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`afc2_config::W`](W) writer structure"]
impl crate::Writable for Afc2ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFC2_CONFIG to value 0xc8"]
impl crate::Resettable for Afc2ConfigSpec {
    const RESET_VALUE: u32 = 0xc8;
}
