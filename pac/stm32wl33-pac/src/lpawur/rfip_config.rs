#[doc = "Register `RFIP_CONFIG` reader"]
pub type R = crate::R<RfipConfigSpec>;
#[doc = "Register `RFIP_CONFIG` writer"]
pub type W = crate::W<RfipConfigSpec>;
#[doc = "Field `LPAWUR_ENABLE` reader - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)"]
pub type LpawurEnableR = crate::BitReader;
#[doc = "Field `LPAWUR_ENABLE` writer - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)"]
pub type LpawurEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_LEVEL` reader - - 00: the bit Sync has been detected"]
pub type WakeupLevelR = crate::FieldReader;
#[doc = "Field `WAKEUP_LEVEL` writer - - 00: the bit Sync has been detected"]
pub type WakeupLevelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)"]
    #[inline(always)]
    pub fn lpawur_enable(&self) -> LpawurEnableR {
        LpawurEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - - 00: the bit Sync has been detected"]
    #[inline(always)]
    pub fn wakeup_level(&self) -> WakeupLevelR {
        WakeupLevelR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)"]
    #[inline(always)]
    pub fn lpawur_enable(&mut self) -> LpawurEnableW<'_, RfipConfigSpec> {
        LpawurEnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - - 00: the bit Sync has been detected"]
    #[inline(always)]
    pub fn wakeup_level(&mut self) -> WakeupLevelW<'_, RfipConfigSpec> {
        WakeupLevelW::new(self, 1)
    }
}
#[doc = "RFIP_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfip_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfip_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfipConfigSpec;
impl crate::RegisterSpec for RfipConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfip_config::R`](R) reader structure"]
impl crate::Readable for RfipConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`rfip_config::W`](W) writer structure"]
impl crate::Writable for RfipConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFIP_CONFIG to value 0x06"]
impl crate::Resettable for RfipConfigSpec {
    const RESET_VALUE: u32 = 0x06;
}
