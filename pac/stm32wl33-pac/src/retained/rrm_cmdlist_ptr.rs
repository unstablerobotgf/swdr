#[doc = "Register `RRM_CMDLIST_PTR` reader"]
pub type R = crate::R<RrmCmdlistPtrSpec>;
#[doc = "Register `RRM_CMDLIST_PTR` writer"]
pub type W = crate::W<RrmCmdlistPtrSpec>;
#[doc = "Field `CMDLIST_PTR_OFFSET` reader - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point."]
pub type CmdlistPtrOffsetR = crate::FieldReader<u16>;
#[doc = "Field `CMDLIST_PTR_OFFSET` writer - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point."]
pub type CmdlistPtrOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CMDLIST_PTR_VALID` reader - Indicate if a command list has to be executed or not"]
pub type CmdlistPtrValidR = crate::BitReader;
#[doc = "Field `CMDLIST_PTR_VALID` writer - Indicate if a command list has to be executed or not"]
pub type CmdlistPtrValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point."]
    #[inline(always)]
    pub fn cmdlist_ptr_offset(&self) -> CmdlistPtrOffsetR {
        CmdlistPtrOffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Indicate if a command list has to be executed or not"]
    #[inline(always)]
    pub fn cmdlist_ptr_valid(&self) -> CmdlistPtrValidR {
        CmdlistPtrValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Contain the offset versus the SoC RAM base address where to find the RRM-UDRA command list entry point."]
    #[inline(always)]
    pub fn cmdlist_ptr_offset(&mut self) -> CmdlistPtrOffsetW<'_, RrmCmdlistPtrSpec> {
        CmdlistPtrOffsetW::new(self, 0)
    }
    #[doc = "Bit 31 - Indicate if a command list has to be executed or not"]
    #[inline(always)]
    pub fn cmdlist_ptr_valid(&mut self) -> CmdlistPtrValidW<'_, RrmCmdlistPtrSpec> {
        CmdlistPtrValidW::new(self, 31)
    }
}
#[doc = "RRM_CMDLIST_PTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrm_cmdlist_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_cmdlist_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrmCmdlistPtrSpec;
impl crate::RegisterSpec for RrmCmdlistPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrm_cmdlist_ptr::R`](R) reader structure"]
impl crate::Readable for RrmCmdlistPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`rrm_cmdlist_ptr::W`](W) writer structure"]
impl crate::Writable for RrmCmdlistPtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRM_CMDLIST_PTR to value 0"]
impl crate::Resettable for RrmCmdlistPtrSpec {}
