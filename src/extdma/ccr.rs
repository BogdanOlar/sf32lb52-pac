///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `EN` reader - extdma enable. Will be cleared if ccr_reset is written
pub type EnR = crate::BitReader;
///Field `EN` writer - extdma enable. Will be cleared if ccr_reset is written
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - transfer complete interrupt enable 0: disabled 1: enabled
pub type TcieR = crate::BitReader;
///Field `TCIE` writer - transfer complete interrupt enable 0: disabled 1: enabled
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - half transfer interrupt enable 0: disabled 1: enabled
pub type HtieR = crate::BitReader;
///Field `HTIE` writer - half transfer interrupt enable 0: disabled 1: enabled
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - transfer error interrupt enable 0: disabled 1: enabled
pub type TeieR = crate::BitReader;
///Field `TEIE` writer - transfer error interrupt enable 0: disabled 1: enabled
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DSTINC` reader - destination increment mode Defines the increment mode for each DMA transfer to the destination memory. 0: disabled 1: enabled
pub type DstincR = crate::BitReader;
///Field `DSTINC` writer - destination increment mode Defines the increment mode for each DMA transfer to the destination memory. 0: disabled 1: enabled
pub type DstincW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRCINC` reader - source increment mode Defines the increment mode for each DMA transfer to the source memory. 0: disabled 1: enabled
pub type SrcincR = crate::BitReader;
///Field `SRCINC` writer - source increment mode Defines the increment mode for each DMA transfer to the source memory. 0: disabled 1: enabled
pub type SrcincW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSTSIZE` reader - destination size Defines the data size of each DMA transfer to the destination memory. Should be fixed to 10 (32 bits), word access allowed only.
pub type DstsizeR = crate::FieldReader;
///Field `DSTSIZE` writer - destination size Defines the data size of each DMA transfer to the destination memory. Should be fixed to 10 (32 bits), word access allowed only.
pub type DstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRCSIZE` reader - source size Defines the data size of each DMA transfer to the source memory. Should be fixed to 10 (32 bits), word access allowed only.
pub type SrcsizeR = crate::FieldReader;
///Field `SRCSIZE` writer - source size Defines the data size of each DMA transfer to the source memory. Should be fixed to 10 (32 bits), word access allowed only.
pub type SrcsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DSTBURST` reader - destination burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
pub type DstburstR = crate::FieldReader;
///Field `DSTBURST` writer - destination burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
pub type DstburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRCBURST` reader - source burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
pub type SrcburstR = crate::FieldReader;
///Field `SRCBURST` writer - source burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
pub type SrcburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RESET` reader - Software reset, will clear extdma status. Active high. Will be cleared by HW automatically
pub type ResetR = crate::BitReader;
///Field `RESET` writer - Software reset, will clear extdma status. Active high. Will be cleared by HW automatically
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - extdma enable. Will be cleared if ccr_reset is written
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - transfer complete interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - half transfer interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - transfer error interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - destination increment mode Defines the increment mode for each DMA transfer to the destination memory. 0: disabled 1: enabled
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - source increment mode Defines the increment mode for each DMA transfer to the source memory. 0: disabled 1: enabled
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - destination size Defines the data size of each DMA transfer to the destination memory. Should be fixed to 10 (32 bits), word access allowed only.
    #[inline(always)]
    pub fn dstsize(&self) -> DstsizeR {
        DstsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - source size Defines the data size of each DMA transfer to the source memory. Should be fixed to 10 (32 bits), word access allowed only.
    #[inline(always)]
    pub fn srcsize(&self) -> SrcsizeR {
        SrcsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:17 - destination burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
    #[inline(always)]
    pub fn dstburst(&self) -> DstburstR {
        DstburstR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - source burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
    #[inline(always)]
    pub fn srcburst(&self) -> SrcburstR {
        SrcburstR::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    ///Bit 31 - Software reset, will clear extdma status. Active high. Will be cleared by HW automatically
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("reset", &self.reset())
            .field("rsvd", &self.rsvd())
            .field("srcburst", &self.srcburst())
            .field("dstburst", &self.dstburst())
            .field("rsvd2", &self.rsvd2())
            .field("srcsize", &self.srcsize())
            .field("dstsize", &self.dstsize())
            .field("srcinc", &self.srcinc())
            .field("dstinc", &self.dstinc())
            .field("rsvd3", &self.rsvd3())
            .field("teie", &self.teie())
            .field("htie", &self.htie())
            .field("tcie", &self.tcie())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - extdma enable. Will be cleared if ccr_reset is written
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CCRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - transfer complete interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<CCRrs> {
        TcieW::new(self, 1)
    }
    ///Bit 2 - half transfer interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn htie(&mut self) -> HtieW<CCRrs> {
        HtieW::new(self, 2)
    }
    ///Bit 3 - transfer error interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<CCRrs> {
        TeieW::new(self, 3)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CCRrs> {
        Rsvd3W::new(self, 4)
    }
    ///Bit 6 - destination increment mode Defines the increment mode for each DMA transfer to the destination memory. 0: disabled 1: enabled
    #[inline(always)]
    pub fn dstinc(&mut self) -> DstincW<CCRrs> {
        DstincW::new(self, 6)
    }
    ///Bit 7 - source increment mode Defines the increment mode for each DMA transfer to the source memory. 0: disabled 1: enabled
    #[inline(always)]
    pub fn srcinc(&mut self) -> SrcincW<CCRrs> {
        SrcincW::new(self, 7)
    }
    ///Bits 8:9 - destination size Defines the data size of each DMA transfer to the destination memory. Should be fixed to 10 (32 bits), word access allowed only.
    #[inline(always)]
    pub fn dstsize(&mut self) -> DstsizeW<CCRrs> {
        DstsizeW::new(self, 8)
    }
    ///Bits 10:11 - source size Defines the data size of each DMA transfer to the source memory. Should be fixed to 10 (32 bits), word access allowed only.
    #[inline(always)]
    pub fn srcsize(&mut self) -> SrcsizeW<CCRrs> {
        SrcsizeW::new(self, 10)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CCRrs> {
        Rsvd2W::new(self, 12)
    }
    ///Bits 16:17 - destination burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
    #[inline(always)]
    pub fn dstburst(&mut self) -> DstburstW<CCRrs> {
        DstburstW::new(self, 16)
    }
    ///Bits 18:19 - source burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)
    #[inline(always)]
    pub fn srcburst(&mut self) -> SrcburstW<CCRrs> {
        SrcburstW::new(self, 18)
    }
    ///Bits 20:30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CCRrs> {
        RsvdW::new(self, 20)
    }
    ///Bit 31 - Software reset, will clear extdma status. Active high. Will be cleared by HW automatically
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<CCRrs> {
        ResetW::new(self, 31)
    }
}
///channel control register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR to value 0x000f_0ac0
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0x000f_0ac0;
}
