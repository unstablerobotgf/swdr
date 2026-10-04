#[doc = "Register `DS_CONF` reader"]
pub type R = crate::R<DsConfSpec>;
#[doc = "Register `DS_CONF` writer"]
pub type W = crate::W<DsConfSpec>;
#[doc = "Field `DS_RATIO` reader - DS_RATIO\\[2:0\\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128"]
pub type DsRatioR = crate::FieldReader;
#[doc = "Field `DS_RATIO` writer - DS_RATIO\\[2:0\\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128"]
pub type DsRatioW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS_WIDTH` reader - DS_WIDTH\\[2:0\\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved"]
pub type DsWidthR = crate::FieldReader;
#[doc = "Field `DS_WIDTH` writer - DS_WIDTH\\[2:0\\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved"]
pub type DsWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DS_RATIO\\[2:0\\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128"]
    #[inline(always)]
    pub fn ds_ratio(&self) -> DsRatioR {
        DsRatioR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - DS_WIDTH\\[2:0\\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved"]
    #[inline(always)]
    pub fn ds_width(&self) -> DsWidthR {
        DsWidthR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DS_RATIO\\[2:0\\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128"]
    #[inline(always)]
    pub fn ds_ratio(&mut self) -> DsRatioW<'_, DsConfSpec> {
        DsRatioW::new(self, 0)
    }
    #[doc = "Bits 3:5 - DS_WIDTH\\[2:0\\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved"]
    #[inline(always)]
    pub fn ds_width(&mut self) -> DsWidthW<'_, DsConfSpec> {
        DsWidthW::new(self, 3)
    }
}
#[doc = "DS_CONF register\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsConfSpec;
impl crate::RegisterSpec for DsConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds_conf::R`](R) reader structure"]
impl crate::Readable for DsConfSpec {}
#[doc = "`write(|w| ..)` method takes [`ds_conf::W`](W) writer structure"]
impl crate::Writable for DsConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DS_CONF to value 0"]
impl crate::Resettable for DsConfSpec {}
