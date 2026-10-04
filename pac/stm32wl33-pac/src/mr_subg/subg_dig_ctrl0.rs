#[doc = "Register `SUBG_DIG_CTRL0` reader"]
pub type R = crate::R<SubgDigCtrl0Spec>;
#[doc = "Register `SUBG_DIG_CTRL0` writer"]
pub type W = crate::W<SubgDigCtrl0Spec>;
#[doc = "Field `FORCE_GPIO_OUTPUT` reader - Option for the direct GPIO signal output"]
pub type ForceGpioOutputR = crate::BitReader;
#[doc = "Field `FORCE_GPIO_OUTPUT` writer - Option for the direct GPIO signal output"]
pub type ForceGpioOutputW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Option for the direct GPIO signal output"]
    #[inline(always)]
    pub fn force_gpio_output(&self) -> ForceGpioOutputR {
        ForceGpioOutputR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option for the direct GPIO signal output"]
    #[inline(always)]
    pub fn force_gpio_output(&mut self) -> ForceGpioOutputW<'_, SubgDigCtrl0Spec> {
        ForceGpioOutputW::new(self, 0)
    }
}
#[doc = "SUBG_DIG_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`subg_dig_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subg_dig_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubgDigCtrl0Spec;
impl crate::RegisterSpec for SubgDigCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subg_dig_ctrl0::R`](R) reader structure"]
impl crate::Readable for SubgDigCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`subg_dig_ctrl0::W`](W) writer structure"]
impl crate::Writable for SubgDigCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUBG_DIG_CTRL0 to value 0"]
impl crate::Resettable for SubgDigCtrl0Spec {}
