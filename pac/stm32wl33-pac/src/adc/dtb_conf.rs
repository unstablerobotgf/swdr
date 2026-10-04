#[doc = "Register `DTB_CONF` reader"]
pub type R = crate::R<DtbConfSpec>;
#[doc = "Register `DTB_CONF` writer"]
pub type W = crate::W<DtbConfSpec>;
#[doc = "Field `ADC_DBG_CONF` reader - ADC_DBG_CONF\\[3:0\\]: use for debug purpose."]
pub type AdcDbgConfR = crate::FieldReader;
#[doc = "Field `ADC_DBG_CONF` writer - ADC_DBG_CONF\\[3:0\\]: use for debug purpose."]
pub type AdcDbgConfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADC_DTB_CONF` reader - ADC_DTB_CONF\\[1:0\\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\\[11:0\\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM"]
pub type AdcDtbConfR = crate::FieldReader;
#[doc = "Field `ADC_DTB_CONF` writer - ADC_DTB_CONF\\[1:0\\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\\[11:0\\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM"]
pub type AdcDtbConfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTB_SER_SEL` reader - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\\[1:0\\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data"]
pub type DtbSerSelR = crate::BitReader;
#[doc = "Field `DTB_SER_SEL` writer - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\\[1:0\\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data"]
pub type DtbSerSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSM_STATE` reader - FSM_STATE\\[7:0\\]: show the state of the state machine. Bit 0: IDLE Bit 1: Reserved Bit 2: ADC setup phase Bit 3: Reserved Bit 4: ADC_START_CONV resynchronization Bit 5: Reserved Bit 6: ADC mode Bit 7: sequence mode"]
pub type FsmStateR = crate::FieldReader;
#[doc = "Field `FSM_CUR_STATE` reader - FSM_CUR_STATE\\[2:0\\]: show the last executed state by the state machine. 000: IDLE mode 001: Reserved 010: ADC setup phase 011: Reserved 100: ADC_START_CONV resynchronization 101: Reserved 110: ADC mode 111: sequence mode"]
pub type FsmCurStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - ADC_DBG_CONF\\[3:0\\]: use for debug purpose."]
    #[inline(always)]
    pub fn adc_dbg_conf(&self) -> AdcDbgConfR {
        AdcDbgConfR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - ADC_DTB_CONF\\[1:0\\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\\[11:0\\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM"]
    #[inline(always)]
    pub fn adc_dtb_conf(&self) -> AdcDtbConfR {
        AdcDtbConfR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\\[1:0\\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data"]
    #[inline(always)]
    pub fn dtb_ser_sel(&self) -> DtbSerSelR {
        DtbSerSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - FSM_STATE\\[7:0\\]: show the state of the state machine. Bit 0: IDLE Bit 1: Reserved Bit 2: ADC setup phase Bit 3: Reserved Bit 4: ADC_START_CONV resynchronization Bit 5: Reserved Bit 6: ADC mode Bit 7: sequence mode"]
    #[inline(always)]
    pub fn fsm_state(&self) -> FsmStateR {
        FsmStateR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - FSM_CUR_STATE\\[2:0\\]: show the last executed state by the state machine. 000: IDLE mode 001: Reserved 010: ADC setup phase 011: Reserved 100: ADC_START_CONV resynchronization 101: Reserved 110: ADC mode 111: sequence mode"]
    #[inline(always)]
    pub fn fsm_cur_state(&self) -> FsmCurStateR {
        FsmCurStateR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC_DBG_CONF\\[3:0\\]: use for debug purpose."]
    #[inline(always)]
    pub fn adc_dbg_conf(&mut self) -> AdcDbgConfW<'_, DtbConfSpec> {
        AdcDbgConfW::new(self, 0)
    }
    #[doc = "Bits 8:9 - ADC_DTB_CONF\\[1:0\\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\\[11:0\\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM"]
    #[inline(always)]
    pub fn adc_dtb_conf(&mut self) -> AdcDtbConfW<'_, DtbConfSpec> {
        AdcDtbConfW::new(self, 8)
    }
    #[doc = "Bit 10 - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\\[1:0\\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data"]
    #[inline(always)]
    pub fn dtb_ser_sel(&mut self) -> DtbSerSelW<'_, DtbConfSpec> {
        DtbSerSelW::new(self, 10)
    }
}
#[doc = "DTB_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtb_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbConfSpec;
impl crate::RegisterSpec for DtbConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtb_conf::R`](R) reader structure"]
impl crate::Readable for DtbConfSpec {}
#[doc = "`write(|w| ..)` method takes [`dtb_conf::W`](W) writer structure"]
impl crate::Writable for DtbConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTB_CONF to value 0"]
impl crate::Resettable for DtbConfSpec {}
