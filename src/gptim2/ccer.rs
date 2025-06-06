///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
///Field `CC1E` reader - Capture/Compare 1 output enable. CC1 channel configured as output: 0: Off - OC1 is not active 1: On - OC1 signal is output on the corresponding output pin CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled 1: Capture enabled
pub type Cc1eR = crate::BitReader;
///Field `CC1E` writer - Capture/Compare 1 output enable. CC1 channel configured as output: 0: Off - OC1 is not active 1: On - OC1 signal is output on the corresponding output pin CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled 1: Capture enabled
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode.
pub type Cc1pR = crate::BitReader;
///Field `CC1P` writer - Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode.
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::BitReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
pub type Cc1npR = crate::BitReader;
///Field `CC1NP` writer - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2E` reader - Capture/Compare 2 output enable.
pub type Cc2eR = crate::BitReader;
///Field `CC2E` writer - Capture/Compare 2 output enable.
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - Capture/Compare 2 output Polarity.
pub type Cc2pR = crate::BitReader;
///Field `CC2P` writer - Capture/Compare 2 output Polarity.
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NP` reader - Capture/Compare 2 output Polarity.
pub type Cc2npR = crate::BitReader;
///Field `CC2NP` writer - Capture/Compare 2 output Polarity.
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - Capture/Compare 3 output enable.
pub type Cc3eR = crate::BitReader;
///Field `CC3E` writer - Capture/Compare 3 output enable.
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - Capture/Compare 3 output Polarity.
pub type Cc3pR = crate::BitReader;
///Field `CC3P` writer - Capture/Compare 3 output Polarity.
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NP` reader - Capture/Compare 3 output Polarity.
pub type Cc3npR = crate::BitReader;
///Field `CC3NP` writer - Capture/Compare 3 output Polarity.
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - Capture/Compare 4 output enable.
pub type Cc4eR = crate::BitReader;
///Field `CC4E` writer - Capture/Compare 4 output enable.
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - Capture/Compare 4 output Polarity.
pub type Cc4pR = crate::BitReader;
///Field `CC4P` writer - Capture/Compare 4 output Polarity.
pub type Cc4pW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NP` reader - Capture/Compare 4 output Polarity.
pub type Cc4npR = crate::BitReader;
///Field `CC4NP` writer - Capture/Compare 4 output Polarity.
pub type Cc4npW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable. CC1 channel configured as output: 0: Off - OC1 is not active 1: On - OC1 signal is output on the corresponding output pin CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled 1: Capture enabled
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode.
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable.
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity.
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity.
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture/Compare 3 output enable.
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity.
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity.
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 output enable.
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/Compare 4 output Polarity.
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity.
    #[inline(always)]
    pub fn cc4np(&self) -> Cc4npR {
        Cc4npR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("rsvd", &self.rsvd())
            .field("cc4np", &self.cc4np())
            .field("rsvd2", &self.rsvd2())
            .field("cc4p", &self.cc4p())
            .field("cc4e", &self.cc4e())
            .field("cc3np", &self.cc3np())
            .field("rsvd3", &self.rsvd3())
            .field("cc3p", &self.cc3p())
            .field("cc3e", &self.cc3e())
            .field("cc2np", &self.cc2np())
            .field("rsvd4", &self.rsvd4())
            .field("cc2p", &self.cc2p())
            .field("cc2e", &self.cc2e())
            .field("cc1np", &self.cc1np())
            .field("rsvd5", &self.rsvd5())
            .field("cc1p", &self.cc1p())
            .field("cc1e", &self.cc1e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable. CC1 channel configured as output: 0: Off - OC1 is not active 1: On - OC1 signal is output on the corresponding output pin CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled 1: Capture enabled
    #[inline(always)]
    pub fn cc1e(&mut self) -> Cc1eW<CCERrs> {
        Cc1eW::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode.
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<CCERrs> {
        Cc1pW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<CCERrs> {
        Rsvd5W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
    #[inline(always)]
    pub fn cc1np(&mut self) -> Cc1npW<CCERrs> {
        Cc1npW::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 2 output enable.
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<CCERrs> {
        Cc2eW::new(self, 4)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity.
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<CCERrs> {
        Cc2pW::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<CCERrs> {
        Rsvd4W::new(self, 6)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity.
    #[inline(always)]
    pub fn cc2np(&mut self) -> Cc2npW<CCERrs> {
        Cc2npW::new(self, 7)
    }
    ///Bit 8 - Capture/Compare 3 output enable.
    #[inline(always)]
    pub fn cc3e(&mut self) -> Cc3eW<CCERrs> {
        Cc3eW::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity.
    #[inline(always)]
    pub fn cc3p(&mut self) -> Cc3pW<CCERrs> {
        Cc3pW::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CCERrs> {
        Rsvd3W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity.
    #[inline(always)]
    pub fn cc3np(&mut self) -> Cc3npW<CCERrs> {
        Cc3npW::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 output enable.
    #[inline(always)]
    pub fn cc4e(&mut self) -> Cc4eW<CCERrs> {
        Cc4eW::new(self, 12)
    }
    ///Bit 13 - Capture/Compare 4 output Polarity.
    #[inline(always)]
    pub fn cc4p(&mut self) -> Cc4pW<CCERrs> {
        Cc4pW::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CCERrs> {
        Rsvd2W::new(self, 14)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity.
    #[inline(always)]
    pub fn cc4np(&mut self) -> Cc4npW<CCERrs> {
        Cc4npW::new(self, 15)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CCERrs> {
        RsvdW::new(self, 16)
    }
}
///Capture/Compare enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u32;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {
    const RESET_VALUE: u32 = 0;
}
