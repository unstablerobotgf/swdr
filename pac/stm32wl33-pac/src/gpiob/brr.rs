#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR0` writer - BR0: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - BR1: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - BR2: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - BR3: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - BR4: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - BR5: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - BR6: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - BR7: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - BR8: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR9` writer - BR9: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR10` writer - BR10: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR11` writer - BR11: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR12` writer - BR12: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR13` writer - BR13: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR14` writer - BR14: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR15` writer - BR15: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
pub type Br15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - BR0: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - BR1: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BrrSpec> {
        Br1W::new(self, 1)
    }
    #[doc = "Bit 2 - BR2: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BrrSpec> {
        Br2W::new(self, 2)
    }
    #[doc = "Bit 3 - BR3: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BrrSpec> {
        Br3W::new(self, 3)
    }
    #[doc = "Bit 4 - BR4: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BrrSpec> {
        Br4W::new(self, 4)
    }
    #[doc = "Bit 5 - BR5: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BrrSpec> {
        Br5W::new(self, 5)
    }
    #[doc = "Bit 6 - BR6: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br6(&mut self) -> Br6W<'_, BrrSpec> {
        Br6W::new(self, 6)
    }
    #[doc = "Bit 7 - BR7: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br7(&mut self) -> Br7W<'_, BrrSpec> {
        Br7W::new(self, 7)
    }
    #[doc = "Bit 8 - BR8: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br8(&mut self) -> Br8W<'_, BrrSpec> {
        Br8W::new(self, 8)
    }
    #[doc = "Bit 9 - BR9: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br9(&mut self) -> Br9W<'_, BrrSpec> {
        Br9W::new(self, 9)
    }
    #[doc = "Bit 10 - BR10: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br10(&mut self) -> Br10W<'_, BrrSpec> {
        Br10W::new(self, 10)
    }
    #[doc = "Bit 11 - BR11: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br11(&mut self) -> Br11W<'_, BrrSpec> {
        Br11W::new(self, 11)
    }
    #[doc = "Bit 12 - BR12: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br12(&mut self) -> Br12W<'_, BrrSpec> {
        Br12W::new(self, 12)
    }
    #[doc = "Bit 13 - BR13: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br13(&mut self) -> Br13W<'_, BrrSpec> {
        Br13W::new(self, 13)
    }
    #[doc = "Bit 14 - BR14: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br14(&mut self) -> Br14W<'_, BrrSpec> {
        Br14W::new(self, 14)
    }
    #[doc = "Bit 15 - BR15: Port B reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. -0: No action on the corresponding ODx bit -1: Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn br15(&mut self) -> Br15W<'_, BrrSpec> {
        Br15W::new(self, 15)
    }
}
#[doc = "BRR register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
