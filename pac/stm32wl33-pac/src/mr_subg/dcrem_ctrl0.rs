#[doc = "Register `DCREM_CTRL0` reader"]
pub type R = crate::R<DcremCtrl0Spec>;
#[doc = "Register `DCREM_CTRL0` writer"]
pub type W = crate::W<DcremCtrl0Spec>;
#[doc = "Field `START_GAIN` reader - Filter gain in start mode for the DC removal block."]
pub type StartGainR = crate::FieldReader;
#[doc = "Field `START_GAIN` writer - Filter gain in start mode for the DC removal block."]
pub type StartGainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRACK_GAIN` reader - Filter gain in track mode for the DC removal block."]
pub type TrackGainR = crate::BitReader;
#[doc = "Field `TRACK_GAIN` writer - Filter gain in track mode for the DC removal block."]
pub type TrackGainW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Filter gain in start mode for the DC removal block."]
    #[inline(always)]
    pub fn start_gain(&self) -> StartGainR {
        StartGainR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Filter gain in track mode for the DC removal block."]
    #[inline(always)]
    pub fn track_gain(&self) -> TrackGainR {
        TrackGainR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter gain in start mode for the DC removal block."]
    #[inline(always)]
    pub fn start_gain(&mut self) -> StartGainW<'_, DcremCtrl0Spec> {
        StartGainW::new(self, 0)
    }
    #[doc = "Bit 7 - Filter gain in track mode for the DC removal block."]
    #[inline(always)]
    pub fn track_gain(&mut self) -> TrackGainW<'_, DcremCtrl0Spec> {
        TrackGainW::new(self, 7)
    }
}
#[doc = "DCREM_CTRL0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrem_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrem_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcremCtrl0Spec;
impl crate::RegisterSpec for DcremCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrem_ctrl0::R`](R) reader structure"]
impl crate::Readable for DcremCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcrem_ctrl0::W`](W) writer structure"]
impl crate::Writable for DcremCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCREM_CTRL0 to value 0xe8"]
impl crate::Resettable for DcremCtrl0Spec {
    const RESET_VALUE: u32 = 0xe8;
}
