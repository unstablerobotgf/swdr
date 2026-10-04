#[doc = "Register `LCSC_ISR` reader"]
pub type R = crate::R<LcscIsrSpec>;
#[doc = "Register `LCSC_ISR` writer"]
pub type W = crate::W<LcscIsrSpec>;
#[doc = "Field `CLKWISE_F` reader - Clock Wise Flag:"]
pub type ClkwiseFR = crate::BitReader;
#[doc = "Field `CLKWISE_F` writer - Clock Wise Flag:"]
pub type ClkwiseFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLKWISE_F` reader - Anti Clock Wise Flag:"]
pub type AclkwiseFR = crate::BitReader;
#[doc = "Field `ACLKWISE_F` writer - Anti Clock Wise Flag:"]
pub type AclkwiseFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP_F` reader - Tamper Flag"]
pub type TampFR = crate::BitReader;
#[doc = "Field `TAMP_F` writer - Tamper Flag"]
pub type TampFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_OFB_F` reader - Out of Bound Counter Flag"]
pub type CntOfbFR = crate::BitReader;
#[doc = "Field `CNT_OFB_F` writer - Out of Bound Counter Flag"]
pub type CntOfbFW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Wise Flag:"]
    #[inline(always)]
    pub fn clkwise_f(&self) -> ClkwiseFR {
        ClkwiseFR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Anti Clock Wise Flag:"]
    #[inline(always)]
    pub fn aclkwise_f(&self) -> AclkwiseFR {
        AclkwiseFR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper Flag"]
    #[inline(always)]
    pub fn tamp_f(&self) -> TampFR {
        TampFR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Out of Bound Counter Flag"]
    #[inline(always)]
    pub fn cnt_ofb_f(&self) -> CntOfbFR {
        CntOfbFR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Wise Flag:"]
    #[inline(always)]
    pub fn clkwise_f(&mut self) -> ClkwiseFW<'_, LcscIsrSpec> {
        ClkwiseFW::new(self, 0)
    }
    #[doc = "Bit 1 - Anti Clock Wise Flag:"]
    #[inline(always)]
    pub fn aclkwise_f(&mut self) -> AclkwiseFW<'_, LcscIsrSpec> {
        AclkwiseFW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper Flag"]
    #[inline(always)]
    pub fn tamp_f(&mut self) -> TampFW<'_, LcscIsrSpec> {
        TampFW::new(self, 2)
    }
    #[doc = "Bit 3 - Out of Bound Counter Flag"]
    #[inline(always)]
    pub fn cnt_ofb_f(&mut self) -> CntOfbFW<'_, LcscIsrSpec> {
        CntOfbFW::new(self, 3)
    }
}
#[doc = "LCSC_ISR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscIsrSpec;
impl crate::RegisterSpec for LcscIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_isr::R`](R) reader structure"]
impl crate::Readable for LcscIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_isr::W`](W) writer structure"]
impl crate::Writable for LcscIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_ISR to value 0"]
impl crate::Resettable for LcscIsrSpec {}
