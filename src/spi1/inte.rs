///Register `INTE` reader
pub type R = crate::R<INTErs>;
///Register `INTE` writer
pub type W = crate::W<INTErs>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TINTE` reader - Receiver Time-out Interrupt Enable 0: Receiver time-out interrupt is disabled 1: Receiver time-out interrupt is enabled
pub type TinteR = crate::BitReader;
///Field `TINTE` writer - Receiver Time-out Interrupt Enable 0: Receiver time-out interrupt is disabled 1: Receiver time-out interrupt is enabled
pub type TinteW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIE` reader - Receive FIFO Interrupt Enable 0: RxFIFO threshold-level-reached interrupt is disabled 1: RxFIFO threshold-level-reached interrupt is enabled
pub type RieR = crate::BitReader;
///Field `RIE` writer - Receive FIFO Interrupt Enable 0: RxFIFO threshold-level-reached interrupt is disabled 1: RxFIFO threshold-level-reached interrupt is enabled
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - Transmit FIFO Interrupt Enable 0: TxFIFO threshold-level-reached interrupt is disabled 1: TxFIFO threshold-level-reached interrupt is enabled
pub type TieR = crate::BitReader;
///Field `TIE` writer - Transmit FIFO Interrupt Enable 0: TxFIFO threshold-level-reached interrupt is disabled 1: TxFIFO threshold-level-reached interrupt is enabled
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIM` reader - Receive FIFO Overrun Interrupt Mask 0: ROR events generate an SPI interrupt 1: ROR events do NOT generate an SPI interrupt
pub type RimR = crate::BitReader;
///Field `RIM` writer - Receive FIFO Overrun Interrupt Mask 0: ROR events generate an SPI interrupt 1: ROR events do NOT generate an SPI interrupt
pub type RimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM` reader - Transmit FIFO Underrun Interrupt Mask 0 : TUR events generate an SPI interrupt 1 : TUR events do NOT generate an SPI interrupt
pub type TimR = crate::BitReader;
///Field `TIM` writer - Transmit FIFO Underrun Interrupt Mask 0 : TUR events generate an SPI interrupt 1 : TUR events do NOT generate an SPI interrupt
pub type TimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receiver Time-out Interrupt Enable 0: Receiver time-out interrupt is disabled 1: Receiver time-out interrupt is enabled
    #[inline(always)]
    pub fn tinte(&self) -> TinteR {
        TinteR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive FIFO Interrupt Enable 0: RxFIFO threshold-level-reached interrupt is disabled 1: RxFIFO threshold-level-reached interrupt is enabled
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit FIFO Interrupt Enable 0: TxFIFO threshold-level-reached interrupt is disabled 1: TxFIFO threshold-level-reached interrupt is enabled
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive FIFO Overrun Interrupt Mask 0: ROR events generate an SPI interrupt 1: ROR events do NOT generate an SPI interrupt
    #[inline(always)]
    pub fn rim(&self) -> RimR {
        RimR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit FIFO Underrun Interrupt Mask 0 : TUR events generate an SPI interrupt 1 : TUR events do NOT generate an SPI interrupt
    #[inline(always)]
    pub fn tim(&self) -> TimR {
        TimR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTE")
            .field("rsvd", &self.rsvd())
            .field("tim", &self.tim())
            .field("rim", &self.rim())
            .field("tie", &self.tie())
            .field("rie", &self.rie())
            .field("tinte", &self.tinte())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<INTErs> {
        Rsvd2W::new(self, 0)
    }
    ///Bit 1 - Receiver Time-out Interrupt Enable 0: Receiver time-out interrupt is disabled 1: Receiver time-out interrupt is enabled
    #[inline(always)]
    pub fn tinte(&mut self) -> TinteW<INTErs> {
        TinteW::new(self, 1)
    }
    ///Bit 2 - Receive FIFO Interrupt Enable 0: RxFIFO threshold-level-reached interrupt is disabled 1: RxFIFO threshold-level-reached interrupt is enabled
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<INTErs> {
        RieW::new(self, 2)
    }
    ///Bit 3 - Transmit FIFO Interrupt Enable 0: TxFIFO threshold-level-reached interrupt is disabled 1: TxFIFO threshold-level-reached interrupt is enabled
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<INTErs> {
        TieW::new(self, 3)
    }
    ///Bit 4 - Receive FIFO Overrun Interrupt Mask 0: ROR events generate an SPI interrupt 1: ROR events do NOT generate an SPI interrupt
    #[inline(always)]
    pub fn rim(&mut self) -> RimW<INTErs> {
        RimW::new(self, 4)
    }
    ///Bit 5 - Transmit FIFO Underrun Interrupt Mask 0 : TUR events generate an SPI interrupt 1 : TUR events do NOT generate an SPI interrupt
    #[inline(always)]
    pub fn tim(&mut self) -> TimW<INTErs> {
        TimW::new(self, 5)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<INTErs> {
        RsvdW::new(self, 6)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`inte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INTErs;
impl crate::RegisterSpec for INTErs {
    type Ux = u32;
}
///`read()` method returns [`inte::R`](R) reader structure
impl crate::Readable for INTErs {}
///`write(|w| ..)` method takes [`inte::W`](W) writer structure
impl crate::Writable for INTErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTE to value 0
impl crate::Resettable for INTErs {
    const RESET_VALUE: u32 = 0;
}
