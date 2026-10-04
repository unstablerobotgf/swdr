#[doc = "Register `IQC_CONFIG` reader"]
pub type R = crate::R<IqcConfigSpec>;
#[doc = "Register `IQC_CONFIG` writer"]
pub type W = crate::W<IqcConfigSpec>;
#[doc = "Field `IQC_CORRECT_IN` reader - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled)."]
pub type IqcCorrectInR = crate::FieldReader<u32>;
#[doc = "Field `IQC_CORRECT_IN` writer - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled)."]
pub type IqcCorrectInW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `LOAD_IQC_INIT` writer - Action bit to load the IQC_CORRECT_IN\\[23:0\\] bit field in the recirculation register when this bit is written to 1."]
pub type LoadIqcInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REUSE_CORRECTION` reader - Reuse last correction value"]
pub type ReuseCorrectionR = crate::BitReader;
#[doc = "Field `REUSE_CORRECTION` writer - Reuse last correction value"]
pub type ReuseCorrectionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IQC_ENABLE` reader - Enable IQC"]
pub type IqcEnableR = crate::BitReader;
#[doc = "Field `IQC_ENABLE` writer - Enable IQC"]
pub type IqcEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled)."]
    #[inline(always)]
    pub fn iqc_correct_in(&self) -> IqcCorrectInR {
        IqcCorrectInR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Reuse last correction value"]
    #[inline(always)]
    pub fn reuse_correction(&self) -> ReuseCorrectionR {
        ReuseCorrectionR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable IQC"]
    #[inline(always)]
    pub fn iqc_enable(&self) -> IqcEnableR {
        IqcEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled)."]
    #[inline(always)]
    pub fn iqc_correct_in(&mut self) -> IqcCorrectInW<'_, IqcConfigSpec> {
        IqcCorrectInW::new(self, 0)
    }
    #[doc = "Bit 29 - Action bit to load the IQC_CORRECT_IN\\[23:0\\] bit field in the recirculation register when this bit is written to 1."]
    #[inline(always)]
    pub fn load_iqc_init(&mut self) -> LoadIqcInitW<'_, IqcConfigSpec> {
        LoadIqcInitW::new(self, 29)
    }
    #[doc = "Bit 30 - Reuse last correction value"]
    #[inline(always)]
    pub fn reuse_correction(&mut self) -> ReuseCorrectionW<'_, IqcConfigSpec> {
        ReuseCorrectionW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable IQC"]
    #[inline(always)]
    pub fn iqc_enable(&mut self) -> IqcEnableW<'_, IqcConfigSpec> {
        IqcEnableW::new(self, 31)
    }
}
#[doc = "IQC_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`iqc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IqcConfigSpec;
impl crate::RegisterSpec for IqcConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iqc_config::R`](R) reader structure"]
impl crate::Readable for IqcConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`iqc_config::W`](W) writer structure"]
impl crate::Writable for IqcConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IQC_CONFIG to value 0xc000_0000"]
impl crate::Resettable for IqcConfigSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
