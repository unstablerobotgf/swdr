#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `REMAP` reader - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed"]
pub type RemapR = crate::BitReader;
#[doc = "Field `REMAP` writer - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed"]
pub type RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_GROUP_WRITE` reader - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden"]
pub type DisGroupWriteR = crate::BitReader;
#[doc = "Field `DIS_GROUP_WRITE` writer - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden"]
pub type DisGroupWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT_STATE` reader - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency"]
pub type WaitStateR = crate::FieldReader;
#[doc = "Field `WAIT_STATE` writer - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency"]
pub type WaitStateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLEEP_SM` reader - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode."]
pub type SleepSmR = crate::BitReader;
#[doc = "Field `SLEEP_SM` writer - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode."]
pub type SleepSmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed"]
    #[inline(always)]
    pub fn remap(&self) -> RemapR {
        RemapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden"]
    #[inline(always)]
    pub fn dis_group_write(&self) -> DisGroupWriteR {
        DisGroupWriteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency"]
    #[inline(always)]
    pub fn wait_state(&self) -> WaitStateR {
        WaitStateR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode."]
    #[inline(always)]
    pub fn sleep_sm(&self) -> SleepSmR {
        SleepSmR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CPU access routing (it supersedes PREMAP configuration): - 0 : FLASH memory addressed - 1 : SRAM0 memory addressed"]
    #[inline(always)]
    pub fn remap(&mut self) -> RemapW<'_, ConfigSpec> {
        RemapW::new(self, 1)
    }
    #[doc = "Bit 2 - Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden"]
    #[inline(always)]
    pub fn dis_group_write(&mut self) -> DisGroupWriteW<'_, ConfigSpec> {
        DisGroupWriteW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Add latency to flash read opeations: - 00 : no latency - 01 : 1 clock cycle latency - 10 : 2 clock cycles latency - 11 : 3 clock cycles latency"]
    #[inline(always)]
    pub fn wait_state(&mut self) -> WaitStateW<'_, ConfigSpec> {
        WaitStateW::new(self, 4)
    }
    #[doc = "Bit 6 - Flash memory power-down mode enable in SLEEP mode This bit allows to have the Flash memory in power-down mode or in idle mode when the device is in Sleep mode. - 0: When the device is in Sleep mode, the NVM is in Idle mode. - 1: When the device is in Sleep mode, the NVM is in power-down mode."]
    #[inline(always)]
    pub fn sleep_sm(&mut self) -> SleepSmW<'_, ConfigSpec> {
        SleepSmW::new(self, 6)
    }
}
#[doc = "CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x10"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x10;
}
