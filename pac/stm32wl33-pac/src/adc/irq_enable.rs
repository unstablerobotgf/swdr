#[doc = "Register `IRQ_ENABLE` reader"]
pub type R = crate::R<IrqEnableSpec>;
#[doc = "Register `IRQ_ENABLE` writer"]
pub type W = crate::W<IrqEnableSpec>;
#[doc = "Field `EOC_IRQ` reader - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type EocIrqR = crate::BitReader;
#[doc = "Field `EOC_IRQ` writer - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type EocIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EODS_IRQ` reader - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type EodsIrqR = crate::BitReader;
#[doc = "Field `EODS_IRQ` writer - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type EodsIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS_IRQ` reader - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type EosIrqR = crate::BitReader;
#[doc = "Field `EOS_IRQ` writer - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type EosIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD_IRQ` reader - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type AwdIrqR = crate::BitReader;
#[doc = "Field `AWD_IRQ` writer - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type AwdIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_DS_IRQ` reader - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type OvrDsIrqR = crate::BitReader;
#[doc = "Field `OVR_DS_IRQ` writer - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
pub type OvrDsIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn eoc_irq(&self) -> EocIrqR {
        EocIrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn eods_irq(&self) -> EodsIrqR {
        EodsIrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn eos_irq(&self) -> EosIrqR {
        EosIrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn awd_irq(&self) -> AwdIrqR {
        AwdIrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn ovr_ds_irq(&self) -> OvrDsIrqR {
        OvrDsIrqR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn eoc_irq(&mut self) -> EocIrqW<'_, IrqEnableSpec> {
        EocIrqW::new(self, 0)
    }
    #[doc = "Bit 1 - EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn eods_irq(&mut self) -> EodsIrqW<'_, IrqEnableSpec> {
        EodsIrqW::new(self, 1)
    }
    #[doc = "Bit 3 - EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn eos_irq(&mut self) -> EosIrqW<'_, IrqEnableSpec> {
        EosIrqW::new(self, 3)
    }
    #[doc = "Bit 4 - AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn awd_irq(&mut self) -> AwdIrqW<'_, IrqEnableSpec> {
        AwdIrqW::new(self, 4)
    }
    #[doc = "Bit 5 - OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt"]
    #[inline(always)]
    pub fn ovr_ds_irq(&mut self) -> OvrDsIrqW<'_, IrqEnableSpec> {
        OvrDsIrqW::new(self, 5)
    }
}
#[doc = "IRQ_ENABLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnableSpec;
impl crate::RegisterSpec for IrqEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enable::R`](R) reader structure"]
impl crate::Readable for IrqEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure"]
impl crate::Writable for IrqEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENABLE to value 0"]
impl crate::Resettable for IrqEnableSpec {}
