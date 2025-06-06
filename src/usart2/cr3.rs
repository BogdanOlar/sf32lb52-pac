///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EIE` reader - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
pub type EieR = crate::BitReader;
///Field `EIE` writer - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAR` reader - Receiver DMA enable 0: DMA mode disabled for reception 1: DMA mode enabled for reception
pub type DmarR = crate::BitReader;
///Field `DMAR` writer - Receiver DMA enable 0: DMA mode disabled for reception 1: DMA mode enabled for reception
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAT` reader - Transmitter DMA enable 0: DMA mode disabled for transmission 1: DMA mode enabled for transmission
pub type DmatR = crate::BitReader;
///Field `DMAT` writer - Transmitter DMA enable 0: DMA mode disabled for transmission 1: DMA mode enabled for transmission
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTSE` reader - RTS enable 0: RTS hardware flow control disabled 1: RTS hardware flow control enabled, RTS output is asserted low when new data can be received
pub type RtseR = crate::BitReader;
///Field `RTSE` writer - RTS enable 0: RTS hardware flow control disabled 1: RTS hardware flow control enabled, RTS output is asserted low when new data can be received
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSE` reader - CTS enable 0: CTS hardware flow control disabled 1: CTS hardware flow control enabled, data is transmitted only when CTS input is asserted low
pub type CtseR = crate::BitReader;
///Field `CTSE` writer - CTS enable 0: CTS hardware flow control disabled 1: CTS hardware flow control enabled, data is transmitted only when CTS input is asserted low
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSIE` reader - CTS interrupt enable 0: interrupt disabled 1: interrupt is generated whenever CTSIF=1 in the ISR register
pub type CtsieR = crate::BitReader;
///Field `CTSIE` writer - CTS interrupt enable 0: interrupt disabled 1: interrupt is generated whenever CTSIF=1 in the ISR register
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ONEBIT` reader - One bit sampling mode 0: 3-bit sampling mode, the sampling value is determined by the voted result out of 3 bits 1: 1-bit sampling mode
pub type OnebitR = crate::BitReader;
///Field `ONEBIT` writer - One bit sampling mode 0: 3-bit sampling mode, the sampling value is determined by the voted result out of 3 bits 1: 1-bit sampling mode
pub type OnebitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRDIS` reader - Overrun disable 0: overrun error flag (ORE) will be set if new data received but previous data not read. New data will not overwrite the content in RDR register. 1: overrun disabled. If new data is received before previous data is read, the new data will overwrite the content in RDR register and ORE flag remains unset.
pub type OvrdisR = crate::BitReader;
///Field `OVRDIS` writer - Overrun disable 0: overrun error flag (ORE) will be set if new data received but previous data not read. New data will not overwrite the content in RDR register. 1: overrun disabled. If new data is received before previous data is read, the new data will overwrite the content in RDR register and ORE flag remains unset.
pub type OvrdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    ///Bit 6 - Receiver DMA enable 0: DMA mode disabled for reception 1: DMA mode enabled for reception
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmitter DMA enable 0: DMA mode disabled for transmission 1: DMA mode enabled for transmission
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable 0: RTS hardware flow control disabled 1: RTS hardware flow control enabled, RTS output is asserted low when new data can be received
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable 0: CTS hardware flow control disabled 1: CTS hardware flow control enabled, data is transmitted only when CTS input is asserted low
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable 0: interrupt disabled 1: interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - One bit sampling mode 0: 3-bit sampling mode, the sampling value is determined by the voted result out of 3 bits 1: 1-bit sampling mode
    #[inline(always)]
    pub fn onebit(&self) -> OnebitR {
        OnebitR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Overrun disable 0: overrun error flag (ORE) will be set if new data received but previous data not read. New data will not overwrite the content in RDR register. 1: overrun disabled. If new data is received before previous data is read, the new data will overwrite the content in RDR register and ORE flag remains unset.
    #[inline(always)]
    pub fn ovrdis(&self) -> OvrdisR {
        OvrdisR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("ovrdis", &self.ovrdis())
            .field("onebit", &self.onebit())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<CR3rs> {
        EieW::new(self, 0)
    }
    ///Bit 6 - Receiver DMA enable 0: DMA mode disabled for reception 1: DMA mode enabled for reception
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<CR3rs> {
        DmarW::new(self, 6)
    }
    ///Bit 7 - Transmitter DMA enable 0: DMA mode disabled for transmission 1: DMA mode enabled for transmission
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<CR3rs> {
        DmatW::new(self, 7)
    }
    ///Bit 8 - RTS enable 0: RTS hardware flow control disabled 1: RTS hardware flow control enabled, RTS output is asserted low when new data can be received
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<CR3rs> {
        RtseW::new(self, 8)
    }
    ///Bit 9 - CTS enable 0: CTS hardware flow control disabled 1: CTS hardware flow control enabled, data is transmitted only when CTS input is asserted low
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<CR3rs> {
        CtseW::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable 0: interrupt disabled 1: interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<CR3rs> {
        CtsieW::new(self, 10)
    }
    ///Bit 11 - One bit sampling mode 0: 3-bit sampling mode, the sampling value is determined by the voted result out of 3 bits 1: 1-bit sampling mode
    #[inline(always)]
    pub fn onebit(&mut self) -> OnebitW<CR3rs> {
        OnebitW::new(self, 11)
    }
    ///Bit 12 - Overrun disable 0: overrun error flag (ORE) will be set if new data received but previous data not read. New data will not overwrite the content in RDR register. 1: overrun disabled. If new data is received before previous data is read, the new data will overwrite the content in RDR register and ORE flag remains unset.
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OvrdisW<CR3rs> {
        OvrdisW::new(self, 12)
    }
}
///Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
