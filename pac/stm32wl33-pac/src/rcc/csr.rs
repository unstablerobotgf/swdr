#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `RMVF` writer - Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags 0: Nothing done 1: Reset the value of the reset flags"]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADRSTF` reader - SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs. 0: No reset from pad occurred 1: Reset from pad occurred"]
pub type PadrstfR = crate::BitReader;
#[doc = "Field `PORRSTF` reader - POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block. 0: No POWER reset occurred 1: POWER reset occurred"]
pub type PorrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` reader - Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs. 0: No software reset occurred 1: Software reset occurred"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `WDGRSTF` reader - Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs. 0: No watchdog reset occurred 1: Watchdog reset occurred"]
pub type WdgrstfR = crate::BitReader;
#[doc = "Field `LOCKUPRSTF` reader - LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals. 0: No lockup reset occurred 1: lockup reset occurred"]
pub type LockuprstfR = crate::BitReader;
impl R {
    #[doc = "Bit 26 - SYSTEM reset flag Reset by software by writing the RMVF bit. Set by hardware when a reset from pad occurs. 0: No reset from pad occurred 1: Reset from pad occurred"]
    #[inline(always)]
    pub fn padrstf(&self) -> PadrstfR {
        PadrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POWER reset flag Reset by software by writing the RMVF bit. Set by hardware when a power reset occurs from LPMURESET block. 0: No POWER reset occurred 1: POWER reset occurred"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag Reset by software by writing the RMVF bit. Set by hardware when a software reset occurs. 0: No software reset occurred 1: Software reset occurred"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a watchdog reset from V33 domain occurs. 0: No watchdog reset occurred 1: Watchdog reset occurred"]
    #[inline(always)]
    pub fn wdgrstf(&self) -> WdgrstfR {
        WdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LOCK UP reset flag from CM0 Reset by software by writing the RMVF bit. Set by hardware from unrecoverable exception CPU. It reset V12i domain, FLASH controller and peripherals. 0: No lockup reset occurred 1: lockup reset occurred"]
    #[inline(always)]
    pub fn lockuprstf(&self) -> LockuprstfR {
        LockuprstfR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Remove reset flag Set by software to clear the value of the reset flags. It auto clears by HW after clearing reason flags 0: Nothing done 1: Reset the value of the reset flags"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RmvfW<'_, CsrSpec> {
        RmvfW::new(self, 23)
    }
}
#[doc = "CSR register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
