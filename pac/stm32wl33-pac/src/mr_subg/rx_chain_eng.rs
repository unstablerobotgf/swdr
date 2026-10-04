#[doc = "Register `RX_CHAIN_ENG` reader"]
pub type R = crate::R<RxChainEngSpec>;
#[doc = "Register `RX_CHAIN_ENG` writer"]
pub type W = crate::W<RxChainEngSpec>;
#[doc = "Field `LNA_ISOL_ENA` reader - Option for LNA during the EN_RX state of the Radio FSM:"]
pub type LnaIsolEnaR = crate::BitReader;
#[doc = "Field `LNA_ISOL_ENA` writer - Option for LNA during the EN_RX state of the Radio FSM:"]
pub type LnaIsolEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGA_PRECH_ENA` reader - Option for PGA precharge during the EN_RX state of the Radio FSM:"]
pub type PgaPrechEnaR = crate::BitReader;
#[doc = "Field `PGA_PRECH_ENA` writer - Option for PGA precharge during the EN_RX state of the Radio FSM:"]
pub type PgaPrechEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Option for LNA during the EN_RX state of the Radio FSM:"]
    #[inline(always)]
    pub fn lna_isol_ena(&self) -> LnaIsolEnaR {
        LnaIsolEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option for PGA precharge during the EN_RX state of the Radio FSM:"]
    #[inline(always)]
    pub fn pga_prech_ena(&self) -> PgaPrechEnaR {
        PgaPrechEnaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option for LNA during the EN_RX state of the Radio FSM:"]
    #[inline(always)]
    pub fn lna_isol_ena(&mut self) -> LnaIsolEnaW<'_, RxChainEngSpec> {
        LnaIsolEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Option for PGA precharge during the EN_RX state of the Radio FSM:"]
    #[inline(always)]
    pub fn pga_prech_ena(&mut self) -> PgaPrechEnaW<'_, RxChainEngSpec> {
        PgaPrechEnaW::new(self, 1)
    }
}
#[doc = "RX_CHAIN_ENG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_chain_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_chain_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxChainEngSpec;
impl crate::RegisterSpec for RxChainEngSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_chain_eng::R`](R) reader structure"]
impl crate::Readable for RxChainEngSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_chain_eng::W`](W) writer structure"]
impl crate::Writable for RxChainEngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CHAIN_ENG to value 0x03"]
impl crate::Resettable for RxChainEngSpec {
    const RESET_VALUE: u32 = 0x03;
}
