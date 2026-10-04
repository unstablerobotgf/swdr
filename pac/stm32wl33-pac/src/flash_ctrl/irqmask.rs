#[doc = "Register `IRQMASK` reader"]
pub type R = crate::R<IrqmaskSpec>;
#[doc = "Register `IRQMASK` writer"]
pub type W = crate::W<IrqmaskSpec>;
#[doc = "Field `CMDDONEM` reader - (1: mask, 0: inactive) CMDDONE_MIS mask"]
pub type CmddonemR = crate::BitReader;
#[doc = "Field `CMDDONEM` writer - (1: mask, 0: inactive) CMDDONE_MIS mask"]
pub type CmddonemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTARTM` reader - (1: mask, 0: inactive) CMDSTART_MIS mask"]
pub type CmdstartmR = crate::BitReader;
#[doc = "Field `CMDSTARTM` writer - (1: mask, 0: inactive) CMDSTART_MIS mask"]
pub type CmdstartmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDBUSYERRM` reader - (1: mask, 0: inactive) CMDBUSYERR_MIS mask"]
pub type CmdbusyerrmR = crate::BitReader;
#[doc = "Field `CMDBUSYERRM` writer - (1: mask, 0: inactive) CMDBUSYERR_MIS mask"]
pub type CmdbusyerrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLCMDM` reader - (1: mask, 0: inactive) ILLCMD_MIS mask"]
pub type IllcmdmR = crate::BitReader;
#[doc = "Field `ILLCMDM` writer - (1: mask, 0: inactive) ILLCMD_MIS mask"]
pub type IllcmdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READOKM` reader - (1: mask, 0: inactive) READOK_MIS mask"]
pub type ReadokmR = crate::BitReader;
#[doc = "Field `READOKM` writer - (1: mask, 0: inactive) READOK_MIS mask"]
pub type ReadokmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNREADYM` reader - (1: mask, 0: inactive) FNREADY_MIS mask"]
pub type FnreadymR = crate::BitReader;
#[doc = "Field `FNREADYM` writer - (1: mask, 0: inactive) FNREADY_MIS mask"]
pub type FnreadymW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (1: mask, 0: inactive) CMDDONE_MIS mask"]
    #[inline(always)]
    pub fn cmddonem(&self) -> CmddonemR {
        CmddonemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (1: mask, 0: inactive) CMDSTART_MIS mask"]
    #[inline(always)]
    pub fn cmdstartm(&self) -> CmdstartmR {
        CmdstartmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (1: mask, 0: inactive) CMDBUSYERR_MIS mask"]
    #[inline(always)]
    pub fn cmdbusyerrm(&self) -> CmdbusyerrmR {
        CmdbusyerrmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (1: mask, 0: inactive) ILLCMD_MIS mask"]
    #[inline(always)]
    pub fn illcmdm(&self) -> IllcmdmR {
        IllcmdmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (1: mask, 0: inactive) READOK_MIS mask"]
    #[inline(always)]
    pub fn readokm(&self) -> ReadokmR {
        ReadokmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (1: mask, 0: inactive) FNREADY_MIS mask"]
    #[inline(always)]
    pub fn fnreadym(&self) -> FnreadymR {
        FnreadymR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (1: mask, 0: inactive) CMDDONE_MIS mask"]
    #[inline(always)]
    pub fn cmddonem(&mut self) -> CmddonemW<'_, IrqmaskSpec> {
        CmddonemW::new(self, 0)
    }
    #[doc = "Bit 1 - (1: mask, 0: inactive) CMDSTART_MIS mask"]
    #[inline(always)]
    pub fn cmdstartm(&mut self) -> CmdstartmW<'_, IrqmaskSpec> {
        CmdstartmW::new(self, 1)
    }
    #[doc = "Bit 2 - (1: mask, 0: inactive) CMDBUSYERR_MIS mask"]
    #[inline(always)]
    pub fn cmdbusyerrm(&mut self) -> CmdbusyerrmW<'_, IrqmaskSpec> {
        CmdbusyerrmW::new(self, 2)
    }
    #[doc = "Bit 3 - (1: mask, 0: inactive) ILLCMD_MIS mask"]
    #[inline(always)]
    pub fn illcmdm(&mut self) -> IllcmdmW<'_, IrqmaskSpec> {
        IllcmdmW::new(self, 3)
    }
    #[doc = "Bit 4 - (1: mask, 0: inactive) READOK_MIS mask"]
    #[inline(always)]
    pub fn readokm(&mut self) -> ReadokmW<'_, IrqmaskSpec> {
        ReadokmW::new(self, 4)
    }
    #[doc = "Bit 5 - (1: mask, 0: inactive) FNREADY_MIS mask"]
    #[inline(always)]
    pub fn fnreadym(&mut self) -> FnreadymW<'_, IrqmaskSpec> {
        FnreadymW::new(self, 5)
    }
}
#[doc = "IRQMASK register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqmaskSpec;
impl crate::RegisterSpec for IrqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqmask::R`](R) reader structure"]
impl crate::Readable for IrqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`irqmask::W`](W) writer structure"]
impl crate::Writable for IrqmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQMASK to value 0x3f"]
impl crate::Resettable for IrqmaskSpec {
    const RESET_VALUE: u32 = 0x3f;
}
