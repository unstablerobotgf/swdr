#[doc = "Register `DATABUFFER_INFO` reader"]
pub type R = crate::R<DatabufferInfoSpec>;
#[doc = "Field `CURRENT_DATABUFFER_COUNT` reader - Indicates the number of bytes used in the last used DATA BUFFER."]
pub type CurrentDatabufferCountR = crate::FieldReader<u16>;
#[doc = "Field `NB_DATABUFFER_USED` reader - Provides the number of data buffers which have been fully used"]
pub type NbDatabufferUsedR = crate::FieldReader<u16>;
#[doc = "Field `CURRENT_DATABUFFER` reader - Indicates which Data Buffer is currently used by the HW"]
pub type CurrentDatabufferR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Indicates the number of bytes used in the last used DATA BUFFER."]
    #[inline(always)]
    pub fn current_databuffer_count(&self) -> CurrentDatabufferCountR {
        CurrentDatabufferCountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - Provides the number of data buffers which have been fully used"]
    #[inline(always)]
    pub fn nb_databuffer_used(&self) -> NbDatabufferUsedR {
        NbDatabufferUsedR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Indicates which Data Buffer is currently used by the HW"]
    #[inline(always)]
    pub fn current_databuffer(&self) -> CurrentDatabufferR {
        CurrentDatabufferR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DATABUFFER_INFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`databuffer_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufferInfoSpec;
impl crate::RegisterSpec for DatabufferInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databuffer_info::R`](R) reader structure"]
impl crate::Readable for DatabufferInfoSpec {}
#[doc = "`reset()` method sets DATABUFFER_INFO to value 0"]
impl crate::Resettable for DatabufferInfoSpec {}
