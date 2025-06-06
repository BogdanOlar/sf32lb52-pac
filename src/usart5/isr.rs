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
///Field `CTSIF` reader - CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line
pub type CtsifR = crate::BitReader;
///Field `CTSIF` writer - CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line
pub type CtsifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTS` reader - CTS input. Read this bit to get the raw status of the CTS line.
pub type CtsR = crate::BitReader;
///Field `CTS` writer - CTS input. Read this bit to get the raw status of the CTS line.
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("cts", &self.cts())
            .field("ctsif", &self.ctsif())
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
