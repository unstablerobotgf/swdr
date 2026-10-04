#[doc = "Register `SWITCH` reader"]
pub type R = crate::R<SwitchSpec>;
#[doc = "Register `SWITCH` writer"]
pub type W = crate::W<SwitchSpec>;
#[doc = "Field `SE_VIN_0` reader - SE_VIN_0\\[1:0\\]: input voltage for VINM\\[0\\] / VINP\\[0\\]-VINM\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin0R = crate::FieldReader;
#[doc = "Field `SE_VIN_0` writer - SE_VIN_0\\[1:0\\]: input voltage for VINM\\[0\\] / VINP\\[0\\]-VINM\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_1` reader - SE_VIN_1\\[1:0\\]: input voltage for VINM\\[1\\] / VINP\\[1\\]-VINM\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin1R = crate::FieldReader;
#[doc = "Field `SE_VIN_1` writer - SE_VIN_1\\[1:0\\]: input voltage for VINM\\[1\\] / VINP\\[1\\]-VINM\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_2` reader - SE_VIN_2\\[1:0\\]: input voltage for VINM\\[2\\] / VINP\\[2\\]-VINM\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin2R = crate::FieldReader;
#[doc = "Field `SE_VIN_2` writer - SE_VIN_2\\[1:0\\]: input voltage for VINM\\[2\\] / VINP\\[2\\]-VINM\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_3` reader - SE_VIN_3\\[1:0\\]: input voltage for VINM\\[3\\] / VINP\\[3\\]-VINM\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin3R = crate::FieldReader;
#[doc = "Field `SE_VIN_3` writer - SE_VIN_3\\[1:0\\]: input voltage for VINM\\[3\\] / VINP\\[3\\]-VINM\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_4` reader - SE_VIN_4\\[1:0\\]: input voltage for VINP\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin4R = crate::FieldReader;
#[doc = "Field `SE_VIN_4` writer - SE_VIN_4\\[1:0\\]: input voltage for VINP\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_5` reader - SE_VIN_5\\[1:0\\]: input voltage for VINP\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin5R = crate::FieldReader;
#[doc = "Field `SE_VIN_5` writer - SE_VIN_5\\[1:0\\]: input voltage for VINP\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_6` reader - SE_VIN_6\\[1:0\\]: input voltage for VINP\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin6R = crate::FieldReader;
#[doc = "Field `SE_VIN_6` writer - SE_VIN_6\\[1:0\\]: input voltage for VINP\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE_VIN_7` reader - SE_VIN_7\\[1:0\\]: input voltage for VINP\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin7R = crate::FieldReader;
#[doc = "Field `SE_VIN_7` writer - SE_VIN_7\\[1:0\\]: input voltage for VINP\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
pub type SeVin7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SE_VIN_0\\[1:0\\]: input voltage for VINM\\[0\\] / VINP\\[0\\]-VINM\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_0(&self) -> SeVin0R {
        SeVin0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SE_VIN_1\\[1:0\\]: input voltage for VINM\\[1\\] / VINP\\[1\\]-VINM\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_1(&self) -> SeVin1R {
        SeVin1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SE_VIN_2\\[1:0\\]: input voltage for VINM\\[2\\] / VINP\\[2\\]-VINM\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_2(&self) -> SeVin2R {
        SeVin2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SE_VIN_3\\[1:0\\]: input voltage for VINM\\[3\\] / VINP\\[3\\]-VINM\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_3(&self) -> SeVin3R {
        SeVin3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SE_VIN_4\\[1:0\\]: input voltage for VINP\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_4(&self) -> SeVin4R {
        SeVin4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SE_VIN_5\\[1:0\\]: input voltage for VINP\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_5(&self) -> SeVin5R {
        SeVin5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SE_VIN_6\\[1:0\\]: input voltage for VINP\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_6(&self) -> SeVin6R {
        SeVin6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SE_VIN_7\\[1:0\\]: input voltage for VINP\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_7(&self) -> SeVin7R {
        SeVin7R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SE_VIN_0\\[1:0\\]: input voltage for VINM\\[0\\] / VINP\\[0\\]-VINM\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_0(&mut self) -> SeVin0W<'_, SwitchSpec> {
        SeVin0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SE_VIN_1\\[1:0\\]: input voltage for VINM\\[1\\] / VINP\\[1\\]-VINM\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_1(&mut self) -> SeVin1W<'_, SwitchSpec> {
        SeVin1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - SE_VIN_2\\[1:0\\]: input voltage for VINM\\[2\\] / VINP\\[2\\]-VINM\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_2(&mut self) -> SeVin2W<'_, SwitchSpec> {
        SeVin2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SE_VIN_3\\[1:0\\]: input voltage for VINM\\[3\\] / VINP\\[3\\]-VINM\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_3(&mut self) -> SeVin3W<'_, SwitchSpec> {
        SeVin3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SE_VIN_4\\[1:0\\]: input voltage for VINP\\[0\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_4(&mut self) -> SeVin4W<'_, SwitchSpec> {
        SeVin4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - SE_VIN_5\\[1:0\\]: input voltage for VINP\\[1\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_5(&mut self) -> SeVin5W<'_, SwitchSpec> {
        SeVin5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - SE_VIN_6\\[1:0\\]: input voltage for VINP\\[2\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_6(&mut self) -> SeVin6W<'_, SwitchSpec> {
        SeVin6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - SE_VIN_7\\[1:0\\]: input voltage for VINP\\[3\\] 00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V"]
    #[inline(always)]
    pub fn se_vin_7(&mut self) -> SeVin7W<'_, SwitchSpec> {
        SeVin7W::new(self, 14)
    }
}
#[doc = "SWITCH register\n\nYou can [`read`](crate::Reg::read) this register and get [`switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwitchSpec;
impl crate::RegisterSpec for SwitchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`switch::R`](R) reader structure"]
impl crate::Readable for SwitchSpec {}
#[doc = "`write(|w| ..)` method takes [`switch::W`](W) writer structure"]
impl crate::Writable for SwitchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWITCH to value 0"]
impl crate::Resettable for SwitchSpec {}
