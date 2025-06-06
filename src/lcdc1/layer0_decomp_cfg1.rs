///Register `LAYER0_DECOMP_CFG1` reader
pub type R = crate::R<LAYER0_DECOMP_CFG1rs>;
///Register `LAYER0_DECOMP_CFG1` writer
pub type W = crate::W<LAYER0_DECOMP_CFG1rs>;
///Field `BLOCK_WIDTH` reader - block_size in pixel unit. 0: 16 pixels 1: 32 pixels Small block size will cause more blocks and more bits to store block information.
pub type BlockWidthR = crate::BitReader;
///Field `BLOCK_WIDTH` writer - block_size in pixel unit. 0: 16 pixels 1: 32 pixels Small block size will cause more blocks and more bits to store block information.
pub type BlockWidthW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DITHER` reader - dithering function 0: off 1: on
pub type DitherR = crate::BitReader;
///Field `DITHER` writer - dithering function 0: off 1: on
pub type DitherW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFG1_RESERVED` reader -
pub type Cfg1ReservedR = crate::FieldReader;
///Field `CFG1_RESERVED` writer -
pub type Cfg1ReservedW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `FAILOVER_BITS_R` reader - failover compression mode target bits(Red)
pub type FailoverBitsRR = crate::FieldReader;
///Field `FAILOVER_BITS_R` writer - failover compression mode target bits(Red)
pub type FailoverBitsRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FAILOVER_BITS_G` reader - failover compression mode target bits(Green)
pub type FailoverBitsGR = crate::FieldReader;
///Field `FAILOVER_BITS_G` writer - failover compression mode target bits(Green)
pub type FailoverBitsGW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FAILOVER_BITS_B` reader - failover compression mode target bits(Blue)
pub type FailoverBitsBR = crate::FieldReader;
///Field `FAILOVER_BITS_B` writer - failover compression mode target bits(Blue)
pub type FailoverBitsBW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LINE_MIN_QIDX` reader - minimum qidx for line mode
pub type LineMinQidxR = crate::FieldReader;
///Field `LINE_MIN_QIDX` writer - minimum qidx for line mode
pub type LineMinQidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BLOCK_MIN_QIDX` reader - minimum qidx for block mode
pub type BlockMinQidxR = crate::FieldReader;
///Field `BLOCK_MIN_QIDX` writer - minimum qidx for block mode
pub type BlockMinQidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EXTRA_LOW` reader - extra bit for low quality block
pub type ExtraLowR = crate::FieldReader;
///Field `EXTRA_LOW` writer - extra bit for low quality block
pub type ExtraLowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - block_size in pixel unit. 0: 16 pixels 1: 32 pixels Small block size will cause more blocks and more bits to store block information.
    #[inline(always)]
    pub fn block_width(&self) -> BlockWidthR {
        BlockWidthR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - dithering function 0: off 1: on
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn cfg1_reserved(&self) -> Cfg1ReservedR {
        Cfg1ReservedR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    ///Bits 8:11 - failover compression mode target bits(Red)
    #[inline(always)]
    pub fn failover_bits_r(&self) -> FailoverBitsRR {
        FailoverBitsRR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - failover compression mode target bits(Green)
    #[inline(always)]
    pub fn failover_bits_g(&self) -> FailoverBitsGR {
        FailoverBitsGR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - failover compression mode target bits(Blue)
    #[inline(always)]
    pub fn failover_bits_b(&self) -> FailoverBitsBR {
        FailoverBitsBR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - minimum qidx for line mode
    #[inline(always)]
    pub fn line_min_qidx(&self) -> LineMinQidxR {
        LineMinQidxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - minimum qidx for block mode
    #[inline(always)]
    pub fn block_min_qidx(&self) -> BlockMinQidxR {
        BlockMinQidxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - extra bit for low quality block
    #[inline(always)]
    pub fn extra_low(&self) -> ExtraLowR {
        ExtraLowR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER0_DECOMP_CFG1")
            .field("extra_low", &self.extra_low())
            .field("block_min_qidx", &self.block_min_qidx())
            .field("line_min_qidx", &self.line_min_qidx())
            .field("failover_bits_b", &self.failover_bits_b())
            .field("failover_bits_g", &self.failover_bits_g())
            .field("failover_bits_r", &self.failover_bits_r())
            .field("cfg1_reserved", &self.cfg1_reserved())
            .field("dither", &self.dither())
            .field("block_width", &self.block_width())
            .finish()
    }
}
impl W {
    ///Bit 0 - block_size in pixel unit. 0: 16 pixels 1: 32 pixels Small block size will cause more blocks and more bits to store block information.
    #[inline(always)]
    pub fn block_width(&mut self) -> BlockWidthW<LAYER0_DECOMP_CFG1rs> {
        BlockWidthW::new(self, 0)
    }
    ///Bit 1 - dithering function 0: off 1: on
    #[inline(always)]
    pub fn dither(&mut self) -> DitherW<LAYER0_DECOMP_CFG1rs> {
        DitherW::new(self, 1)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn cfg1_reserved(&mut self) -> Cfg1ReservedW<LAYER0_DECOMP_CFG1rs> {
        Cfg1ReservedW::new(self, 2)
    }
    ///Bits 8:11 - failover compression mode target bits(Red)
    #[inline(always)]
    pub fn failover_bits_r(&mut self) -> FailoverBitsRW<LAYER0_DECOMP_CFG1rs> {
        FailoverBitsRW::new(self, 8)
    }
    ///Bits 12:15 - failover compression mode target bits(Green)
    #[inline(always)]
    pub fn failover_bits_g(&mut self) -> FailoverBitsGW<LAYER0_DECOMP_CFG1rs> {
        FailoverBitsGW::new(self, 12)
    }
    ///Bits 16:19 - failover compression mode target bits(Blue)
    #[inline(always)]
    pub fn failover_bits_b(&mut self) -> FailoverBitsBW<LAYER0_DECOMP_CFG1rs> {
        FailoverBitsBW::new(self, 16)
    }
    ///Bits 20:23 - minimum qidx for line mode
    #[inline(always)]
    pub fn line_min_qidx(&mut self) -> LineMinQidxW<LAYER0_DECOMP_CFG1rs> {
        LineMinQidxW::new(self, 20)
    }
    ///Bits 24:27 - minimum qidx for block mode
    #[inline(always)]
    pub fn block_min_qidx(&mut self) -> BlockMinQidxW<LAYER0_DECOMP_CFG1rs> {
        BlockMinQidxW::new(self, 24)
    }
    ///Bits 28:31 - extra bit for low quality block
    #[inline(always)]
    pub fn extra_low(&mut self) -> ExtraLowW<LAYER0_DECOMP_CFG1rs> {
        ExtraLowW::new(self, 28)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER0_DECOMP_CFG1rs;
impl crate::RegisterSpec for LAYER0_DECOMP_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`layer0_decomp_cfg1::R`](R) reader structure
impl crate::Readable for LAYER0_DECOMP_CFG1rs {}
///`write(|w| ..)` method takes [`layer0_decomp_cfg1::W`](W) writer structure
impl crate::Writable for LAYER0_DECOMP_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER0_DECOMP_CFG1 to value 0x8002_3307
impl crate::Resettable for LAYER0_DECOMP_CFG1rs {
    const RESET_VALUE: u32 = 0x8002_3307;
}
