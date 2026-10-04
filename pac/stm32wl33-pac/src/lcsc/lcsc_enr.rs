#[doc = "Register `LCSC_ENR` reader"]
pub type R = crate::R<LcscEnrSpec>;
#[doc = "Register `LCSC_ENR` writer"]
pub type W = crate::W<LcscEnrSpec>;
#[doc = "Field `CLKWISE_IE` reader - Clock Wise Interrupt and Wakeup Enable"]
pub type ClkwiseIeR = crate::BitReader;
#[doc = "Field `CLKWISE_IE` writer - Clock Wise Interrupt and Wakeup Enable"]
pub type ClkwiseIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLKWISE_IE` reader - Anti Clock Wise Interrupt and Wakeup Enable"]
pub type AclkwiseIeR = crate::BitReader;
#[doc = "Field `ACLKWISE_IE` writer - Anti Clock Wise Interrupt and Wakeup Enable"]
pub type AclkwiseIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP_IE` reader - Tamper Interrupt and Wakeup Enable"]
pub type TampIeR = crate::BitReader;
#[doc = "Field `TAMP_IE` writer - Tamper Interrupt and Wakeup Enable"]
pub type TampIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_OFB_WKP_IE` reader - LCAB Counter Out Of Bound wakeup enable"]
pub type CntOfbWkpIeR = crate::BitReader;
#[doc = "Field `CNT_OFB_WKP_IE` writer - LCAB Counter Out Of Bound wakeup enable"]
pub type CntOfbWkpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCSC_EN` reader - LCSC Enable"]
pub type LcscEnR = crate::BitReader;
#[doc = "Field `LCSC_EN` writer - LCSC Enable"]
pub type LcscEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Wise Interrupt and Wakeup Enable"]
    #[inline(always)]
    pub fn clkwise_ie(&self) -> ClkwiseIeR {
        ClkwiseIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Anti Clock Wise Interrupt and Wakeup Enable"]
    #[inline(always)]
    pub fn aclkwise_ie(&self) -> AclkwiseIeR {
        AclkwiseIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper Interrupt and Wakeup Enable"]
    #[inline(always)]
    pub fn tamp_ie(&self) -> TampIeR {
        TampIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCAB Counter Out Of Bound wakeup enable"]
    #[inline(always)]
    pub fn cnt_ofb_wkp_ie(&self) -> CntOfbWkpIeR {
        CntOfbWkpIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - LCSC Enable"]
    #[inline(always)]
    pub fn lcsc_en(&self) -> LcscEnR {
        LcscEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Wise Interrupt and Wakeup Enable"]
    #[inline(always)]
    pub fn clkwise_ie(&mut self) -> ClkwiseIeW<'_, LcscEnrSpec> {
        ClkwiseIeW::new(self, 0)
    }
    #[doc = "Bit 1 - Anti Clock Wise Interrupt and Wakeup Enable"]
    #[inline(always)]
    pub fn aclkwise_ie(&mut self) -> AclkwiseIeW<'_, LcscEnrSpec> {
        AclkwiseIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper Interrupt and Wakeup Enable"]
    #[inline(always)]
    pub fn tamp_ie(&mut self) -> TampIeW<'_, LcscEnrSpec> {
        TampIeW::new(self, 2)
    }
    #[doc = "Bit 3 - LCAB Counter Out Of Bound wakeup enable"]
    #[inline(always)]
    pub fn cnt_ofb_wkp_ie(&mut self) -> CntOfbWkpIeW<'_, LcscEnrSpec> {
        CntOfbWkpIeW::new(self, 3)
    }
    #[doc = "Bit 31 - LCSC Enable"]
    #[inline(always)]
    pub fn lcsc_en(&mut self) -> LcscEnW<'_, LcscEnrSpec> {
        LcscEnW::new(self, 31)
    }
}
#[doc = "LCSC_ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscEnrSpec;
impl crate::RegisterSpec for LcscEnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_enr::R`](R) reader structure"]
impl crate::Readable for LcscEnrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_enr::W`](W) writer structure"]
impl crate::Writable for LcscEnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_ENR to value 0"]
impl crate::Resettable for LcscEnrSpec {}
