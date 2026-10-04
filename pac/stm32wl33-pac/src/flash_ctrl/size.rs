#[doc = "Register `SIZE` reader"]
pub type R = crate::R<SizeSpec>;
#[doc = "Field `FLASH_SIZE` reader - Maximum valid address for flash memory: - 00 : 0x03FFF (64kb) - 01 : 0x07FFF (128kb) - 10 : 0x0BFFF (192kb) - 11 : 0x0FFFF (256kb)"]
pub type FlashSizeR = crate::FieldReader<u32>;
#[doc = "Field `RAM_SIZE` reader - RAM memory size selection: - 0 : 16kb - 1 : 32kb"]
pub type RamSizeR = crate::BitReader;
#[doc = "Field `FLASH_SECURE` reader - Flash memory protection (0: no key present, 1: key present)"]
pub type FlashSecureR = crate::BitReader;
#[doc = "Field `JTAG_DISABLE` reader - Flash+JTAG protection (0: no JTAG protection - see FLASH_SECURE, 1: Flash and JTAG protected)"]
pub type JtagDisableR = crate::BitReader;
#[doc = "Field `PACKAGE_SIZE` reader - Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins"]
pub type PackageSizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:16 - Maximum valid address for flash memory: - 00 : 0x03FFF (64kb) - 01 : 0x07FFF (128kb) - 10 : 0x0BFFF (192kb) - 11 : 0x0FFFF (256kb)"]
    #[inline(always)]
    pub fn flash_size(&self) -> FlashSizeR {
        FlashSizeR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - RAM memory size selection: - 0 : 16kb - 1 : 32kb"]
    #[inline(always)]
    pub fn ram_size(&self) -> RamSizeR {
        RamSizeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Flash memory protection (0: no key present, 1: key present)"]
    #[inline(always)]
    pub fn flash_secure(&self) -> FlashSecureR {
        FlashSecureR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flash+JTAG protection (0: no JTAG protection - see FLASH_SECURE, 1: Flash and JTAG protected)"]
    #[inline(always)]
    pub fn jtag_disable(&self) -> JtagDisableR {
        JtagDisableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins"]
    #[inline(always)]
    pub fn package_size(&self) -> PackageSizeR {
        PackageSizeR::new(((self.bits >> 21) & 3) as u8)
    }
}
#[doc = "SIZE register\n\nYou can [`read`](crate::Reg::read) this register and get [`size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeSpec;
impl crate::RegisterSpec for SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size::R`](R) reader structure"]
impl crate::Readable for SizeSpec {}
#[doc = "`reset()` method sets SIZE to value 0xffff"]
impl crate::Resettable for SizeSpec {
    const RESET_VALUE: u32 = 0xffff;
}
