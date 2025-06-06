///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EIE` reader - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
pub type EieR = crate::BitReader;
///Field `EIE` writer - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
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
pub type Rsvd6R = crate::FieldReader;
///Field `RSVD6` writer -
pub type Rsvd6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::FieldReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsvd15(&self) -> Rsvd15R {
        Rsvd15R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd14(&self) -> Rsvd14R {
        Rsvd14R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn rsvd13(&self) -> Rsvd13R {
        Rsvd13R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd12(&self) -> Rsvd12R {
        Rsvd12R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd11(&self) -> Rsvd11R {
        Rsvd11R::new(((self.bits >> 5) & 1) != 0)
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
    ///Bit 13
    #[inline(always)]
    pub fn rsvd10(&self) -> Rsvd10R {
        Rsvd10R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd9(&self) -> Rsvd9R {
        Rsvd9R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd8(&self) -> Rsvd8R {
        Rsvd8R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn rsvd6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
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
            .field("ovrdis", &self.ovrdis())
            .field("onebit", &self.onebit())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("rsvd11", &self.rsvd11())
            .field("rsvd12", &self.rsvd12())
            .field("rsvd13", &self.rsvd13())
            .field("rsvd14", &self.rsvd14())
            .field("rsvd15", &self.rsvd15())
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
    ///Bit 1
    #[inline(always)]
    pub fn rsvd15(&mut self) -> Rsvd15W<CR3rs> {
        Rsvd15W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rsvd14(&mut self) -> Rsvd14W<CR3rs> {
        Rsvd14W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn rsvd13(&mut self) -> Rsvd13W<CR3rs> {
        Rsvd13W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd12(&mut self) -> Rsvd12W<CR3rs> {
        Rsvd12W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd11(&mut self) -> Rsvd11W<CR3rs> {
        Rsvd11W::new(self, 5)
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
    ///Bit 13
    #[inline(always)]
    pub fn rsvd10(&mut self) -> Rsvd10W<CR3rs> {
        Rsvd10W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd9(&mut self) -> Rsvd9W<CR3rs> {
        Rsvd9W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd8(&mut self) -> Rsvd8W<CR3rs> {
        Rsvd8W::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rsvd7(&mut self) -> Rsvd7W<CR3rs> {
        Rsvd7W::new(self, 16)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn rsvd6(&mut self) -> Rsvd6W<CR3rs> {
        Rsvd6W::new(self, 17)
    }
    ///Bits 20:21
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<CR3rs> {
        Rsvd5W::new(self, 20)
    }
    ///Bit 22
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<CR3rs> {
        Rsvd4W::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CR3rs> {
        Rsvd3W::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CR3rs> {
        Rsvd2W::new(self, 24)
    }
    ///Bits 25:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CR3rs> {
        RsvdW::new(self, 25)
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
