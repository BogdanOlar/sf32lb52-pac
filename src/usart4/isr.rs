///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `PE` reader - Parity error. This bit is set when a parity error is detected in the received packet. 0: no parity error 1: parity error detected
pub type PeR = crate::BitReader;
///Field `PE` writer - Parity error. This bit is set when a parity error is detected in the received packet. 0: no parity error 1: parity error detected
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FE` reader - Framing error. This bit is set by hardware when stop bit is not correctly received 0: no framing error is detected 1: framing error is detected
pub type FeR = crate::BitReader;
///Field `FE` writer - Framing error. This bit is set by hardware when stop bit is not correctly received 0: no framing error is detected 1: framing error is detected
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NF` reader - Noise flag. Noise means the samping values in the 3-bit sampling mode are not the same. 0: no noise is detected 1: noise is detected
pub type NfR = crate::BitReader;
///Field `NF` writer - Noise flag. Noise means the samping values in the 3-bit sampling mode are not the same. 0: no noise is detected 1: noise is detected
pub type NfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORE` reader - Overrun error. When new data is received but Rx buffer is not empty (i.e. previous data is not read yet), ORE is asserted and current RDR content is not lost. This feature can be disabled by set CR3_OVRDIS to 1. 0: no overrun error 1: overrun error is detected
pub type OreR = crate::BitReader;
///Field `ORE` writer - Overrun error. When new data is received but Rx buffer is not empty (i.e. previous data is not read yet), ORE is asserted and current RDR content is not lost. This feature can be disabled by set CR3_OVRDIS to 1. 0: no overrun error 1: overrun error is detected
pub type OreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLE` reader - Idle line detected 0: no idle line is detected 1: idle line is detected
pub type IdleR = crate::BitReader;
///Field `IDLE` writer - Idle line detected 0: no idle line is detected 1: idle line is detected
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNE` reader - Rx data not empty. This bit is set by hardware when the received data is transferred into RDR register. 0: data is not received 1: data is ready in RDR to be read
pub type RxneR = crate::BitReader;
///Field `RXNE` writer - Rx data not empty. This bit is set by hardware when the received data is transferred into RDR register. 0: data is not received 1: data is ready in RDR to be read
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC` reader - transmission complete. This bit is set by hardware if the transmission is complete 0: transmission is not complete 1: transmission is complete
pub type TcR = crate::BitReader;
///Field `TC` writer - transmission complete. This bit is set by hardware if the transmission is complete 0: transmission is not complete 1: transmission is complete
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXE` reader - Tx data empty 0: data is ready in TDR 1: data is already transferred to shift register, i.e. transmission is in progress or complete
pub type TxeR = crate::BitReader;
///Field `TXE` writer - Tx data empty 0: data is ready in TDR 1: data is already transferred to shift register, i.e. transmission is in progress or complete
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD16` reader -
pub type Rsvd16R = crate::BitReader;
///Field `RSVD16` writer -
pub type Rsvd16W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSIF` reader - CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line
pub type CtsifR = crate::BitReader;
///Field `CTSIF` writer - CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line
pub type CtsifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTS` reader - CTS input. Read this bit to get the raw status of the CTS line.
pub type CtsR = crate::BitReader;
///Field `CTS` writer - CTS input. Read this bit to get the raw status of the CTS line.
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD15` reader -
pub type Rsvd15R = crate::BitReader;
///Field `RSVD15` writer -
pub type Rsvd15W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD14` reader -
pub type Rsvd14R = crate::BitReader;
///Field `RSVD14` writer -
pub type Rsvd14W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD13` reader -
pub type Rsvd13R = crate::BitReader;
///Field `RSVD13` writer -
pub type Rsvd13W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD12` reader -
pub type Rsvd12R = crate::BitReader;
///Field `RSVD12` writer -
pub type Rsvd12W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD11` reader -
pub type Rsvd11R = crate::BitReader;
///Field `RSVD11` writer -
pub type Rsvd11W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD10` reader -
pub type Rsvd10R = crate::BitReader;
///Field `RSVD10` writer -
pub type Rsvd10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD9` reader -
pub type Rsvd9R = crate::BitReader;
///Field `RSVD9` writer -
pub type Rsvd9W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD8` reader -
pub type Rsvd8R = crate::BitReader;
///Field `RSVD8` writer -
pub type Rsvd8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD7` reader -
pub type Rsvd7R = crate::BitReader;
///Field `RSVD7` writer -
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD6` reader -
pub type Rsvd6R = crate::BitReader;
///Field `RSVD6` writer -
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::BitReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Parity error. This bit is set when a parity error is detected in the received packet. 0: no parity error 1: parity error detected
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error. This bit is set by hardware when stop bit is not correctly received 0: no framing error is detected 1: framing error is detected
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise flag. Noise means the samping values in the 3-bit sampling mode are not the same. 0: no noise is detected 1: noise is detected
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error. When new data is received but Rx buffer is not empty (i.e. previous data is not read yet), ORE is asserted and current RDR content is not lost. This feature can be disabled by set CR3_OVRDIS to 1. 0: no overrun error 1: overrun error is detected
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected 0: no idle line is detected 1: idle line is detected
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx data not empty. This bit is set by hardware when the received data is transferred into RDR register. 0: data is not received 1: data is ready in RDR to be read
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - transmission complete. This bit is set by hardware if the transmission is complete 0: transmission is not complete 1: transmission is complete
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tx data empty 0: data is ready in TDR 1: data is already transferred to shift register, i.e. transmission is in progress or complete
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd16(&self) -> Rsvd16R {
        Rsvd16R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS input. Read this bit to get the raw status of the CTS line.
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd15(&self) -> Rsvd15R {
        Rsvd15R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn rsvd14(&self) -> Rsvd14R {
        Rsvd14R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn rsvd13(&self) -> Rsvd13R {
        Rsvd13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd12(&self) -> Rsvd12R {
        Rsvd12R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd11(&self) -> Rsvd11R {
        Rsvd11R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rsvd10(&self) -> Rsvd10R {
        Rsvd10R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd9(&self) -> Rsvd9R {
        Rsvd9R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn rsvd8(&self) -> Rsvd8R {
        Rsvd8R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn rsvd6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bit 25
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rsvd", &self.rsvd())
            .field("rsvd2", &self.rsvd2())
            .field("rsvd3", &self.rsvd3())
            .field("rsvd4", &self.rsvd4())
            .field("rsvd5", &self.rsvd5())
            .field("rsvd6", &self.rsvd6())
            .field("rsvd7", &self.rsvd7())
            .field("rsvd8", &self.rsvd8())
            .field("rsvd9", &self.rsvd9())
            .field("rsvd10", &self.rsvd10())
            .field("rsvd11", &self.rsvd11())
            .field("rsvd12", &self.rsvd12())
            .field("rsvd13", &self.rsvd13())
            .field("rsvd14", &self.rsvd14())
            .field("rsvd15", &self.rsvd15())
            .field("cts", &self.cts())
            .field("ctsif", &self.ctsif())
            .field("rsvd16", &self.rsvd16())
            .field("txe", &self.txe())
            .field("tc", &self.tc())
            .field("rxne", &self.rxne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("nf", &self.nf())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .finish()
    }
}
impl W {
    ///Bit 0 - Parity error. This bit is set when a parity error is detected in the received packet. 0: no parity error 1: parity error detected
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<ISRrs> {
        PeW::new(self, 0)
    }
    ///Bit 1 - Framing error. This bit is set by hardware when stop bit is not correctly received 0: no framing error is detected 1: framing error is detected
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<ISRrs> {
        FeW::new(self, 1)
    }
    ///Bit 2 - Noise flag. Noise means the samping values in the 3-bit sampling mode are not the same. 0: no noise is detected 1: noise is detected
    #[inline(always)]
    pub fn nf(&mut self) -> NfW<ISRrs> {
        NfW::new(self, 2)
    }
    ///Bit 3 - Overrun error. When new data is received but Rx buffer is not empty (i.e. previous data is not read yet), ORE is asserted and current RDR content is not lost. This feature can be disabled by set CR3_OVRDIS to 1. 0: no overrun error 1: overrun error is detected
    #[inline(always)]
    pub fn ore(&mut self) -> OreW<ISRrs> {
        OreW::new(self, 3)
    }
    ///Bit 4 - Idle line detected 0: no idle line is detected 1: idle line is detected
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<ISRrs> {
        IdleW::new(self, 4)
    }
    ///Bit 5 - Rx data not empty. This bit is set by hardware when the received data is transferred into RDR register. 0: data is not received 1: data is ready in RDR to be read
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<ISRrs> {
        RxneW::new(self, 5)
    }
    ///Bit 6 - transmission complete. This bit is set by hardware if the transmission is complete 0: transmission is not complete 1: transmission is complete
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<ISRrs> {
        TcW::new(self, 6)
    }
    ///Bit 7 - Tx data empty 0: data is ready in TDR 1: data is already transferred to shift register, i.e. transmission is in progress or complete
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<ISRrs> {
        TxeW::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd16(&mut self) -> Rsvd16W<ISRrs> {
        Rsvd16W::new(self, 8)
    }
    ///Bit 9 - CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line
    #[inline(always)]
    pub fn ctsif(&mut self) -> CtsifW<ISRrs> {
        CtsifW::new(self, 9)
    }
    ///Bit 10 - CTS input. Read this bit to get the raw status of the CTS line.
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<ISRrs> {
        CtsW::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd15(&mut self) -> Rsvd15W<ISRrs> {
        Rsvd15W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    pub fn rsvd14(&mut self) -> Rsvd14W<ISRrs> {
        Rsvd14W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    pub fn rsvd13(&mut self) -> Rsvd13W<ISRrs> {
        Rsvd13W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd12(&mut self) -> Rsvd12W<ISRrs> {
        Rsvd12W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd11(&mut self) -> Rsvd11W<ISRrs> {
        Rsvd11W::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rsvd10(&mut self) -> Rsvd10W<ISRrs> {
        Rsvd10W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd9(&mut self) -> Rsvd9W<ISRrs> {
        Rsvd9W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn rsvd8(&mut self) -> Rsvd8W<ISRrs> {
        Rsvd8W::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd7(&mut self) -> Rsvd7W<ISRrs> {
        Rsvd7W::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn rsvd6(&mut self) -> Rsvd6W<ISRrs> {
        Rsvd6W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<ISRrs> {
        Rsvd5W::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<ISRrs> {
        Rsvd4W::new(self, 22)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<ISRrs> {
        Rsvd3W::new(self, 23)
    }
    ///Bit 25
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ISRrs> {
        Rsvd2W::new(self, 25)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ISRrs> {
        RsvdW::new(self, 26)
    }
}
///Interrupt and Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR to value 0x0200_00c0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x0200_00c0;
}
