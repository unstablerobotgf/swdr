#[doc = "Register `GPIO_SWA_CTRL` reader"]
pub type R = crate::R<GpioSwaCtrlSpec>;
#[doc = "Register `GPIO_SWA_CTRL` writer"]
pub type W = crate::W<GpioSwaCtrlSpec>;
#[doc = "Field `ATB1_nPVD` reader - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected"]
pub type Atb1NPvdR = crate::BitReader;
#[doc = "Field `ATB1_nPVD` writer - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected"]
pub type Atb1NPvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected"]
    #[inline(always)]
    pub fn atb1_n_pvd(&self) -> Atb1NPvdR {
        Atb1NPvdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected"]
    #[inline(always)]
    pub fn atb1_n_pvd(&mut self) -> Atb1NPvdW<'_, GpioSwaCtrlSpec> {
        Atb1NPvdW::new(self, 0)
    }
}
#[doc = "GPIO_SWA_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_swa_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_swa_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSwaCtrlSpec;
impl crate::RegisterSpec for GpioSwaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_swa_ctrl::R`](R) reader structure"]
impl crate::Readable for GpioSwaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_swa_ctrl::W`](W) writer structure"]
impl crate::Writable for GpioSwaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_SWA_CTRL to value 0"]
impl crate::Resettable for GpioSwaCtrlSpec {}
