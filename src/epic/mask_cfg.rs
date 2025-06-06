///Register `MASK_CFG` reader
pub type R = crate::R<MASK_CFGrs>;
///Register `MASK_CFG` writer
pub type W = crate::W<MASK_CFGrs>;
///Field `FORMAT` reader - mask input format 1'h0: A8 1'h1: A4
pub type FormatR = crate::BitReader;
///Field `FORMAT` writer - mask input format 1'h0: A8 1'h1: A4
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MIX_MODE` reader - mask mix mode 1'h0: mult mode 1'h1: overwrite mode
pub type MixModeR = crate::BitReader;
///Field `MIX_MODE` writer - mask mix mode 1'h0: mult mode 1'h1: overwrite mode
pub type MixModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L0_MASK_EN` reader - layer0 mask enable
pub type L0MaskEnR = crate::BitReader;
///Field `L0_MASK_EN` writer - layer0 mask enable
pub type L0MaskEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_MASK_EN` reader - layer1 mask enable
pub type L1MaskEnR = crate::BitReader;
///Field `L1_MASK_EN` writer - layer1 mask enable
pub type L1MaskEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_MASK_EN` reader - layer2 mask enable
pub type L2MaskEnR = crate::BitReader;
///Field `L2_MASK_EN` writer - layer2 mask enable
pub type L2MaskEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VL_MASK_EN` reader - video layer mask enable
pub type VlMaskEnR = crate::BitReader;
///Field `VL_MASK_EN` writer - video layer mask enable
pub type VlMaskEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIDTH` reader - source image width(including padding), unit is bytes
pub type WidthR = crate::FieldReader<u16>;
///Field `WIDTH` writer - source image width(including padding), unit is bytes
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `PREFETCH_EN` reader - preload 64 bytes extra data when reading pixel from memory
pub type PrefetchEnR = crate::BitReader;
///Field `PREFETCH_EN` writer - preload 64 bytes extra data when reading pixel from memory
pub type PrefetchEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACTIVE` reader - layer active flag
pub type ActiveR = crate::BitReader;
///Field `ACTIVE` writer - layer active flag
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - mask input format 1'h0: A8 1'h1: A4
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - mask mix mode 1'h0: mult mode 1'h1: overwrite mode
    #[inline(always)]
    pub fn mix_mode(&self) -> MixModeR {
        MixModeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - layer0 mask enable
    #[inline(always)]
    pub fn l0_mask_en(&self) -> L0MaskEnR {
        L0MaskEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - layer1 mask enable
    #[inline(always)]
    pub fn l1_mask_en(&self) -> L1MaskEnR {
        L1MaskEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - layer2 mask enable
    #[inline(always)]
    pub fn l2_mask_en(&self) -> L2MaskEnR {
        L2MaskEnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - video layer mask enable
    #[inline(always)]
    pub fn vl_mask_en(&self) -> VlMaskEnR {
        VlMaskEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 14:26 - source image width(including padding), unit is bytes
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 14) & 0x1fff) as u16)
    }
    ///Bit 27 - preload 64 bytes extra data when reading pixel from memory
    #[inline(always)]
    pub fn prefetch_en(&self) -> PrefetchEnR {
        PrefetchEnR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - layer active flag
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_CFG")
            .field("active", &self.active())
            .field("prefetch_en", &self.prefetch_en())
            .field("width", &self.width())
            .field("vl_mask_en", &self.vl_mask_en())
            .field("l2_mask_en", &self.l2_mask_en())
            .field("l1_mask_en", &self.l1_mask_en())
            .field("l0_mask_en", &self.l0_mask_en())
            .field("mix_mode", &self.mix_mode())
            .field("format", &self.format())
            .finish()
    }
}
impl W {
    ///Bit 0 - mask input format 1'h0: A8 1'h1: A4
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<MASK_CFGrs> {
        FormatW::new(self, 0)
    }
    ///Bit 1 - mask mix mode 1'h0: mult mode 1'h1: overwrite mode
    #[inline(always)]
    pub fn mix_mode(&mut self) -> MixModeW<MASK_CFGrs> {
        MixModeW::new(self, 1)
    }
    ///Bit 2 - layer0 mask enable
    #[inline(always)]
    pub fn l0_mask_en(&mut self) -> L0MaskEnW<MASK_CFGrs> {
        L0MaskEnW::new(self, 2)
    }
    ///Bit 3 - layer1 mask enable
    #[inline(always)]
    pub fn l1_mask_en(&mut self) -> L1MaskEnW<MASK_CFGrs> {
        L1MaskEnW::new(self, 3)
    }
    ///Bit 4 - layer2 mask enable
    #[inline(always)]
    pub fn l2_mask_en(&mut self) -> L2MaskEnW<MASK_CFGrs> {
        L2MaskEnW::new(self, 4)
    }
    ///Bit 5 - video layer mask enable
    #[inline(always)]
    pub fn vl_mask_en(&mut self) -> VlMaskEnW<MASK_CFGrs> {
        VlMaskEnW::new(self, 5)
    }
    ///Bits 14:26 - source image width(including padding), unit is bytes
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<MASK_CFGrs> {
        WidthW::new(self, 14)
    }
    ///Bit 27 - preload 64 bytes extra data when reading pixel from memory
    #[inline(always)]
    pub fn prefetch_en(&mut self) -> PrefetchEnW<MASK_CFGrs> {
        PrefetchEnW::new(self, 27)
    }
    ///Bit 28 - layer active flag
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<MASK_CFGrs> {
        ActiveW::new(self, 28)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`mask_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MASK_CFGrs;
impl crate::RegisterSpec for MASK_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`mask_cfg::R`](R) reader structure
impl crate::Readable for MASK_CFGrs {}
///`write(|w| ..)` method takes [`mask_cfg::W`](W) writer structure
impl crate::Writable for MASK_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MASK_CFG to value 0
impl crate::Resettable for MASK_CFGrs {
    const RESET_VALUE: u32 = 0;
}
