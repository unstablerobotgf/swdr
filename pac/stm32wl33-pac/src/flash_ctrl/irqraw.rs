#[doc = "Register `IRQRAW` reader"]
pub type R = crate::R<IrqrawSpec>;
#[doc = "Register `IRQRAW` writer"]
pub type W = crate::W<IrqrawSpec>;
#[doc = "Field `CMDDONE_RIS` reader - (1: active, 0: inactive) COMMAND sequence ended"]
pub type CmddoneRisR = crate::BitReader;
#[doc = "Field `CMDDONE_RIS` writer - (1: active, 0: inactive) COMMAND sequence ended"]
pub type CmddoneRisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTART_RIS` reader - (1: active, 0: inactive) COMMAND sequence started"]
pub type CmdstartRisR = crate::BitReader;
#[doc = "Field `CMDSTART_RIS` writer - (1: active, 0: inactive) COMMAND sequence started"]
pub type CmdstartRisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDBUSYERR_RIS` reader - (1: active, 0: inactive) COMMAND issued while flash busy"]
pub type CmdbusyerrRisR = crate::BitReader;
#[doc = "Field `CMDBUSYERR_RIS` writer - (1: active, 0: inactive) COMMAND issued while flash busy"]
pub type CmdbusyerrRisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLCMD_RIS` reader - (1: active, 0: inactive) Illegal command issued"]
pub type IllcmdRisR = crate::BitReader;
#[doc = "Field `ILLCMD_RIS` writer - (1: active, 0: inactive) Illegal command issued"]
pub type IllcmdRisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READOK_RIS` reader - (1: active, 0: inactive) READ COMMAND completed successfully"]
pub type ReadokRisR = crate::BitReader;
#[doc = "Field `READOK_RIS` writer - (1: active, 0: inactive) READ COMMAND completed successfully"]
pub type ReadokRisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSLEEPERR_RIS` reader - (1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1)"]
pub type CmdsleeperrRisR = crate::BitReader;
#[doc = "Field `CMDSLEEPERR_RIS` writer - (1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1)"]
pub type CmdsleeperrRisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (1: active, 0: inactive) COMMAND sequence ended"]
    #[inline(always)]
    pub fn cmddone_ris(&self) -> CmddoneRisR {
        CmddoneRisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (1: active, 0: inactive) COMMAND sequence started"]
    #[inline(always)]
    pub fn cmdstart_ris(&self) -> CmdstartRisR {
        CmdstartRisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (1: active, 0: inactive) COMMAND issued while flash busy"]
    #[inline(always)]
    pub fn cmdbusyerr_ris(&self) -> CmdbusyerrRisR {
        CmdbusyerrRisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (1: active, 0: inactive) Illegal command issued"]
    #[inline(always)]
    pub fn illcmd_ris(&self) -> IllcmdRisR {
        IllcmdRisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (1: active, 0: inactive) READ COMMAND completed successfully"]
    #[inline(always)]
    pub fn readok_ris(&self) -> ReadokRisR {
        ReadokRisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1)"]
    #[inline(always)]
    pub fn cmdsleeperr_ris(&self) -> CmdsleeperrRisR {
        CmdsleeperrRisR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (1: active, 0: inactive) COMMAND sequence ended"]
    #[inline(always)]
    pub fn cmddone_ris(&mut self) -> CmddoneRisW<'_, IrqrawSpec> {
        CmddoneRisW::new(self, 0)
    }
    #[doc = "Bit 1 - (1: active, 0: inactive) COMMAND sequence started"]
    #[inline(always)]
    pub fn cmdstart_ris(&mut self) -> CmdstartRisW<'_, IrqrawSpec> {
        CmdstartRisW::new(self, 1)
    }
    #[doc = "Bit 2 - (1: active, 0: inactive) COMMAND issued while flash busy"]
    #[inline(always)]
    pub fn cmdbusyerr_ris(&mut self) -> CmdbusyerrRisW<'_, IrqrawSpec> {
        CmdbusyerrRisW::new(self, 2)
    }
    #[doc = "Bit 3 - (1: active, 0: inactive) Illegal command issued"]
    #[inline(always)]
    pub fn illcmd_ris(&mut self) -> IllcmdRisW<'_, IrqrawSpec> {
        IllcmdRisW::new(self, 3)
    }
    #[doc = "Bit 4 - (1: active, 0: inactive) READ COMMAND completed successfully"]
    #[inline(always)]
    pub fn readok_ris(&mut self) -> ReadokRisW<'_, IrqrawSpec> {
        ReadokRisW::new(self, 4)
    }
    #[doc = "Bit 5 - (1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1)"]
    #[inline(always)]
    pub fn cmdsleeperr_ris(&mut self) -> CmdsleeperrRisW<'_, IrqrawSpec> {
        CmdsleeperrRisW::new(self, 5)
    }
}
#[doc = "IRQRAW register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqraw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqraw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqrawSpec;
impl crate::RegisterSpec for IrqrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqraw::R`](R) reader structure"]
impl crate::Readable for IrqrawSpec {}
#[doc = "`write(|w| ..)` method takes [`irqraw::W`](W) writer structure"]
impl crate::Writable for IrqrawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQRAW to value 0x01"]
impl crate::Resettable for IrqrawSpec {
    const RESET_VALUE: u32 = 0x01;
}
