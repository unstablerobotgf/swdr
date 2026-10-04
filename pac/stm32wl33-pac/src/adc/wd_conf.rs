#[doc = "Register `WD_CONF` reader"]
pub type R = crate::R<WdConfSpec>;
#[doc = "Register `WD_CONF` writer"]
pub type W = crate::W<WdConfSpec>;
#[doc = "Field `AWD_CHX` reader - AWD_CHX\\[15:0\\]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM\\[0\\] to ADC negative input Bit1: VINM\\[1\\] to ADC negative input Bit2: VINM\\[2\\] to ADC negative input Bit3: VINM\\[3\\] to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input"]
pub type AwdChxR = crate::FieldReader<u16>;
#[doc = "Field `AWD_CHX` writer - AWD_CHX\\[15:0\\]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM\\[0\\] to ADC negative input Bit1: VINM\\[1\\] to ADC negative input Bit2: VINM\\[2\\] to ADC negative input Bit3: VINM\\[3\\] to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input"]
pub type AwdChxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AWD_CHX\\[15:0\\]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM\\[0\\] to ADC negative input Bit1: VINM\\[1\\] to ADC negative input Bit2: VINM\\[2\\] to ADC negative input Bit3: VINM\\[3\\] to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input"]
    #[inline(always)]
    pub fn awd_chx(&self) -> AwdChxR {
        AwdChxR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AWD_CHX\\[15:0\\]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM\\[0\\] to ADC negative input Bit1: VINM\\[1\\] to ADC negative input Bit2: VINM\\[2\\] to ADC negative input Bit3: VINM\\[3\\] to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\] to ADC positive input Bit9: VINP\\[1\\] to ADC positive input Bit10: VINP\\[2\\] to ADC positive input Bit11: VINP\\[3\\] to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input"]
    #[inline(always)]
    pub fn awd_chx(&mut self) -> AwdChxW<'_, WdConfSpec> {
        AwdChxW::new(self, 0)
    }
}
#[doc = "WD_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdConfSpec;
impl crate::RegisterSpec for WdConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd_conf::R`](R) reader structure"]
impl crate::Readable for WdConfSpec {}
#[doc = "`write(|w| ..)` method takes [`wd_conf::W`](W) writer structure"]
impl crate::Writable for WdConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD_CONF to value 0"]
impl crate::Resettable for WdConfSpec {}
