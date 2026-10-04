#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RtorSpec>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RtorSpec>;
#[doc = "Field `RTO` reader - RTO\\[23:0\\]: Receiver timeout value This bit-field gives the Receiver timeout value in terms of number of baud clocks. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the Start Bit of the last received character."]
pub type RtoR = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - RTO\\[23:0\\]: Receiver timeout value This bit-field gives the Receiver timeout value in terms of number of baud clocks. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the Start Bit of the last received character."]
pub type RtoW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BLEN` reader - BLEN\\[7:0\\]: Block Length This bit-field gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bit-field can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1."]
pub type BlenR = crate::FieldReader;
#[doc = "Field `BLEN` writer - BLEN\\[7:0\\]: Block Length This bit-field gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bit-field can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1."]
pub type BlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - RTO\\[23:0\\]: Receiver timeout value This bit-field gives the Receiver timeout value in terms of number of baud clocks. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the Start Bit of the last received character."]
    #[inline(always)]
    pub fn rto(&self) -> RtoR {
        RtoR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - BLEN\\[7:0\\]: Block Length This bit-field gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bit-field can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1."]
    #[inline(always)]
    pub fn blen(&self) -> BlenR {
        BlenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - RTO\\[23:0\\]: Receiver timeout value This bit-field gives the Receiver timeout value in terms of number of baud clocks. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the Start Bit of the last received character."]
    #[inline(always)]
    pub fn rto(&mut self) -> RtoW<'_, RtorSpec> {
        RtoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - BLEN\\[7:0\\]: Block Length This bit-field gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bit-field can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1."]
    #[inline(always)]
    pub fn blen(&mut self) -> BlenW<'_, RtorSpec> {
        BlenW::new(self, 24)
    }
}
#[doc = "RTOR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtorSpec;
impl crate::RegisterSpec for RtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RtorSpec {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RtorSpec {}
