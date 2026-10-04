#[doc = "Register `RRM_UDRA_CTRL` writer"]
pub type W = crate::W<RrmUdraCtrlSpec>;
#[doc = "Field `RRM_CMD_REQ` writer - Action bit: write 1 to request a RRM-UDRA command."]
pub type RrmCmdReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Action bit: write 1 to request a RRM-UDRA command."]
    #[inline(always)]
    pub fn rrm_cmd_req(&mut self) -> RrmCmdReqW<'_, RrmUdraCtrlSpec> {
        RrmCmdReqW::new(self, 0)
    }
}
#[doc = "RRM_UDRA_CTRL register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_udra_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrmUdraCtrlSpec;
impl crate::RegisterSpec for RrmUdraCtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rrm_udra_ctrl::W`](W) writer structure"]
impl crate::Writable for RrmUdraCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRM_UDRA_CTRL to value 0"]
impl crate::Resettable for RrmUdraCtrlSpec {}
