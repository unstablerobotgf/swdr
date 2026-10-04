#[doc = "Register `LCD_SR` reader"]
pub type R = crate::R<LcdSrSpec>;
#[doc = "Field `ENS` reader - LCD enabled status"]
pub type EnsR = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame flag"]
pub type SofR = crate::BitReader;
#[doc = "Field `UDR` reader - Update display request"]
pub type UdrR = crate::BitReader;
#[doc = "Field `UDD` reader - Update Display Done"]
pub type UddR = crate::BitReader;
#[doc = "Field `RDY` reader - Ready flag"]
pub type RdyR = crate::BitReader;
#[doc = "Field `FCRSF` reader - LCD Frame Control Register Synchronization flag"]
pub type FcrsfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LCD enabled status"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame flag"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update display request"]
    #[inline(always)]
    pub fn udr(&self) -> UdrR {
        UdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update Display Done"]
    #[inline(always)]
    pub fn udd(&self) -> UddR {
        UddR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ready flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Frame Control Register Synchronization flag"]
    #[inline(always)]
    pub fn fcrsf(&self) -> FcrsfR {
        FcrsfR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "LCD_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdSrSpec;
impl crate::RegisterSpec for LcdSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_sr::R`](R) reader structure"]
impl crate::Readable for LcdSrSpec {}
#[doc = "`reset()` method sets LCD_SR to value 0x20"]
impl crate::Resettable for LcdSrSpec {
    const RESET_VALUE: u32 = 0x20;
}
