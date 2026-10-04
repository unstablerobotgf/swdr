#[doc = "Register `IWDG_SR` reader"]
pub type R = crate::R<IwdgSrSpec>;
#[doc = "Field `PVU` reader - Watchdog prescaler value update. Read only bit. This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to 5 RC 40 kHz cycles). Prescaler value can be updated only when PVU bit is reset"]
pub type PvuR = crate::BitReader;
#[doc = "Field `RVU` reader - Watchdog counter reload value update. Read only bit. This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to 5 RC 40 kHz cycles). Reload value can be updated only when RVU bit is reset"]
pub type RvuR = crate::BitReader;
#[doc = "Field `WVU` reader - Watchdog counter window value update. Read only bit. This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to 5 RC 40 kHz cycles). Window value can be updated only when WVU bit is reset. This bit is generated only if generic 'window' = 1"]
pub type WvuR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update. Read only bit. This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to 5 RC 40 kHz cycles). Prescaler value can be updated only when PVU bit is reset"]
    #[inline(always)]
    pub fn pvu(&self) -> PvuR {
        PvuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update. Read only bit. This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to 5 RC 40 kHz cycles). Reload value can be updated only when RVU bit is reset"]
    #[inline(always)]
    pub fn rvu(&self) -> RvuR {
        RvuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update. Read only bit. This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to 5 RC 40 kHz cycles). Window value can be updated only when WVU bit is reset. This bit is generated only if generic 'window' = 1"]
    #[inline(always)]
    pub fn wvu(&self) -> WvuR {
        WvuR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "IWDG_SR register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgSrSpec;
impl crate::RegisterSpec for IwdgSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_sr::R`](R) reader structure"]
impl crate::Readable for IwdgSrSpec {}
#[doc = "`reset()` method sets IWDG_SR to value 0"]
impl crate::Resettable for IwdgSrSpec {}
