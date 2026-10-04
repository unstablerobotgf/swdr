#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `COMMAND_ID` reader - Opcode coresponding to a command:"]
pub type CommandIdR = crate::FieldReader;
#[doc = "Field `COMMAND_ID` writer - Opcode coresponding to a command:"]
pub type CommandIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BACK2ACTIVE` reader - Select the default/return state for the Radio FSM to be ACTIVE2"]
pub type Back2activeR = crate::BitReader;
#[doc = "Field `BACK2ACTIVE` writer - Select the default/return state for the Radio FSM to be ACTIVE2"]
pub type Back2activeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2LOCKON` reader - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX"]
pub type Back2lockonR = crate::BitReader;
#[doc = "Field `BACK2LOCKON` writer - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX"]
pub type Back2lockonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Opcode coresponding to a command:"]
    #[inline(always)]
    pub fn command_id(&self) -> CommandIdR {
        CommandIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Select the default/return state for the Radio FSM to be ACTIVE2"]
    #[inline(always)]
    pub fn back2active(&self) -> Back2activeR {
        Back2activeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX"]
    #[inline(always)]
    pub fn back2lockon(&self) -> Back2lockonR {
        Back2lockonR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Opcode coresponding to a command:"]
    #[inline(always)]
    pub fn command_id(&mut self) -> CommandIdW<'_, CommandSpec> {
        CommandIdW::new(self, 0)
    }
    #[doc = "Bit 25 - Select the default/return state for the Radio FSM to be ACTIVE2"]
    #[inline(always)]
    pub fn back2active(&mut self) -> Back2activeW<'_, CommandSpec> {
        Back2activeW::new(self, 25)
    }
    #[doc = "Bit 26 - Request to the Radio FSM to stay in LOCKON state when exiting a RX or a TX"]
    #[inline(always)]
    pub fn back2lockon(&mut self) -> Back2lockonW<'_, CommandSpec> {
        Back2lockonW::new(self, 26)
    }
}
#[doc = "COMMAND register\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {}
