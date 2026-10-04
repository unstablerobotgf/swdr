#[doc = "Register `IRQSTAT` reader"]
pub type R = crate::R<IrqstatSpec>;
#[doc = "Register `IRQSTAT` writer"]
pub type W = crate::W<IrqstatSpec>;
#[doc = "Field `CMDDONE_MIS` reader - (1: clear, 0: inactive) CMDDONE_MIS flag"]
pub type CmddoneMisR = crate::BitReader;
#[doc = "Field `CMDDONE_MIS` writer - (1: clear, 0: inactive) CMDDONE_MIS flag"]
pub type CmddoneMisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTART_MIS` reader - (1: clear, 0: inactive) CMDSTART_MIS flag"]
pub type CmdstartMisR = crate::BitReader;
#[doc = "Field `CMDSTART_MIS` writer - (1: clear, 0: inactive) CMDSTART_MIS flag"]
pub type CmdstartMisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDBUSYERR_MIS` reader - (1: clear, 0: inactive) CMDBUSYERR_MIS flag"]
pub type CmdbusyerrMisR = crate::BitReader;
#[doc = "Field `CMDBUSYERR_MIS` writer - (1: clear, 0: inactive) CMDBUSYERR_MIS flag"]
pub type CmdbusyerrMisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLCMD_MIS` reader - (1: clear, 0: inactive) ILLCMD_MIS flag"]
pub type IllcmdMisR = crate::BitReader;
#[doc = "Field `ILLCMD_MIS` writer - (1: clear, 0: inactive) ILLCMD_MIS flag"]
pub type IllcmdMisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READOK_MIS` reader - (1: clear, 0: inactive) READOK_MIS flag"]
pub type ReadokMisR = crate::BitReader;
#[doc = "Field `READOK_MIS` writer - (1: clear, 0: inactive) READOK_MIS flag"]
pub type ReadokMisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNREADY_MIS` reader - (1: clear, 0: inactive) FNREADY_MIS flag"]
pub type FnreadyMisR = crate::BitReader;
#[doc = "Field `FNREADY_MIS` writer - (1: clear, 0: inactive) FNREADY_MIS flag"]
pub type FnreadyMisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (1: clear, 0: inactive) CMDDONE_MIS flag"]
    #[inline(always)]
    pub fn cmddone_mis(&self) -> CmddoneMisR {
        CmddoneMisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (1: clear, 0: inactive) CMDSTART_MIS flag"]
    #[inline(always)]
    pub fn cmdstart_mis(&self) -> CmdstartMisR {
        CmdstartMisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (1: clear, 0: inactive) CMDBUSYERR_MIS flag"]
    #[inline(always)]
    pub fn cmdbusyerr_mis(&self) -> CmdbusyerrMisR {
        CmdbusyerrMisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (1: clear, 0: inactive) ILLCMD_MIS flag"]
    #[inline(always)]
    pub fn illcmd_mis(&self) -> IllcmdMisR {
        IllcmdMisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (1: clear, 0: inactive) READOK_MIS flag"]
    #[inline(always)]
    pub fn readok_mis(&self) -> ReadokMisR {
        ReadokMisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (1: clear, 0: inactive) FNREADY_MIS flag"]
    #[inline(always)]
    pub fn fnready_mis(&self) -> FnreadyMisR {
        FnreadyMisR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (1: clear, 0: inactive) CMDDONE_MIS flag"]
    #[inline(always)]
    pub fn cmddone_mis(&mut self) -> CmddoneMisW<'_, IrqstatSpec> {
        CmddoneMisW::new(self, 0)
    }
    #[doc = "Bit 1 - (1: clear, 0: inactive) CMDSTART_MIS flag"]
    #[inline(always)]
    pub fn cmdstart_mis(&mut self) -> CmdstartMisW<'_, IrqstatSpec> {
        CmdstartMisW::new(self, 1)
    }
    #[doc = "Bit 2 - (1: clear, 0: inactive) CMDBUSYERR_MIS flag"]
    #[inline(always)]
    pub fn cmdbusyerr_mis(&mut self) -> CmdbusyerrMisW<'_, IrqstatSpec> {
        CmdbusyerrMisW::new(self, 2)
    }
    #[doc = "Bit 3 - (1: clear, 0: inactive) ILLCMD_MIS flag"]
    #[inline(always)]
    pub fn illcmd_mis(&mut self) -> IllcmdMisW<'_, IrqstatSpec> {
        IllcmdMisW::new(self, 3)
    }
    #[doc = "Bit 4 - (1: clear, 0: inactive) READOK_MIS flag"]
    #[inline(always)]
    pub fn readok_mis(&mut self) -> ReadokMisW<'_, IrqstatSpec> {
        ReadokMisW::new(self, 4)
    }
    #[doc = "Bit 5 - (1: clear, 0: inactive) FNREADY_MIS flag"]
    #[inline(always)]
    pub fn fnready_mis(&mut self) -> FnreadyMisW<'_, IrqstatSpec> {
        FnreadyMisW::new(self, 5)
    }
}
#[doc = "IRQSTAT register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqstatSpec;
impl crate::RegisterSpec for IrqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqstat::R`](R) reader structure"]
impl crate::Readable for IrqstatSpec {}
#[doc = "`write(|w| ..)` method takes [`irqstat::W`](W) writer structure"]
impl crate::Writable for IrqstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQSTAT to value 0"]
impl crate::Resettable for IrqstatSpec {}
