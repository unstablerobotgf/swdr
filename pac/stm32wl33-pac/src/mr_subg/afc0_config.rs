#[doc = "Register `AFC0_CONFIG` reader"]
pub type R = crate::R<Afc0ConfigSpec>;
#[doc = "Register `AFC0_CONFIG` writer"]
pub type W = crate::W<Afc0ConfigSpec>;
#[doc = "Field `AFC_SLOW_GAIN_LOG2` reader - AFC loop gain in slow mode (2's log)"]
pub type AfcSlowGainLog2R = crate::FieldReader;
#[doc = "Field `AFC_SLOW_GAIN_LOG2` writer - AFC loop gain in slow mode (2's log)"]
pub type AfcSlowGainLog2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFC_FAST_GAIN_LOG2` reader - AFC loop gain in fast mode (2's log)"]
pub type AfcFastGainLog2R = crate::FieldReader;
#[doc = "Field `AFC_FAST_GAIN_LOG2` writer - AFC loop gain in fast mode (2's log)"]
pub type AfcFastGainLog2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AFC loop gain in slow mode (2's log)"]
    #[inline(always)]
    pub fn afc_slow_gain_log2(&self) -> AfcSlowGainLog2R {
        AfcSlowGainLog2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFC loop gain in fast mode (2's log)"]
    #[inline(always)]
    pub fn afc_fast_gain_log2(&self) -> AfcFastGainLog2R {
        AfcFastGainLog2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFC loop gain in slow mode (2's log)"]
    #[inline(always)]
    pub fn afc_slow_gain_log2(&mut self) -> AfcSlowGainLog2W<'_, Afc0ConfigSpec> {
        AfcSlowGainLog2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AFC loop gain in fast mode (2's log)"]
    #[inline(always)]
    pub fn afc_fast_gain_log2(&mut self) -> AfcFastGainLog2W<'_, Afc0ConfigSpec> {
        AfcFastGainLog2W::new(self, 4)
    }
}
#[doc = "AFC0_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc0_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc0_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afc0ConfigSpec;
impl crate::RegisterSpec for Afc0ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afc0_config::R`](R) reader structure"]
impl crate::Readable for Afc0ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`afc0_config::W`](W) writer structure"]
impl crate::Writable for Afc0ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFC0_CONFIG to value 0x25"]
impl crate::Resettable for Afc0ConfigSpec {
    const RESET_VALUE: u32 = 0x25;
}
