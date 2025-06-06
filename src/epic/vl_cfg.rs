///Register `VL_CFG` reader
pub type R = crate::R<VL_CFGrs>;
///Register `VL_CFG` writer
pub type W = crate::W<VL_CFGrs>;
///Field `FORMAT` reader - video layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2
pub type FormatR = crate::FieldReader;
///Field `FORMAT` writer - video layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ALPHA_SEL` reader - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
pub type AlphaSelR = crate::BitReader;
///Field `ALPHA_SEL` writer - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
pub type AlphaSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALPHA` reader - layer alpha value
pub type AlphaR = crate::FieldReader;
///Field `ALPHA` writer - layer alpha value
pub type AlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BLEND_DEPTH` reader - video layer blending depth
pub type BlendDepthR = crate::FieldReader;
///Field `BLEND_DEPTH` writer - video layer blending depth
pub type BlendDepthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FILTER_EN` reader - layer color filter enable
pub type FilterEnR = crate::BitReader;
///Field `FILTER_EN` writer - layer color filter enable
pub type FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `ALPHA_BLEND` reader - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
pub type AlphaBlendR = crate::BitReader;
///Field `ALPHA_BLEND` writer - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
pub type AlphaBlendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - video layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
    #[inline(always)]
    pub fn alpha_sel(&self) -> AlphaSelR {
        AlphaSelR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:12 - layer alpha value
    #[inline(always)]
    pub fn alpha(&self) -> AlphaR {
        AlphaR::new(((self.bits >> 5) & 0xff) as u8)
    }
    ///Bits 13:14 - video layer blending depth
    #[inline(always)]
    pub fn blend_depth(&self) -> BlendDepthR {
        BlendDepthR::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - layer color filter enable
    #[inline(always)]
    pub fn filter_en(&self) -> FilterEnR {
        FilterEnR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:28 - source image width(including padding), unit is bytes
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    ///Bit 29 - preload 64 bytes extra data when reading pixel from memory
    #[inline(always)]
    pub fn prefetch_en(&self) -> PrefetchEnR {
        PrefetchEnR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - layer active flag
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
    #[inline(always)]
    pub fn alpha_blend(&self) -> AlphaBlendR {
        AlphaBlendR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_CFG")
            .field("alpha_blend", &self.alpha_blend())
            .field("active", &self.active())
            .field("prefetch_en", &self.prefetch_en())
            .field("width", &self.width())
            .field("filter_en", &self.filter_en())
            .field("blend_depth", &self.blend_depth())
            .field("alpha", &self.alpha())
            .field("alpha_sel", &self.alpha_sel())
            .field("format", &self.format())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - video layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<VL_CFGrs> {
        FormatW::new(self, 0)
    }
    ///Bit 4 - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
    #[inline(always)]
    pub fn alpha_sel(&mut self) -> AlphaSelW<VL_CFGrs> {
        AlphaSelW::new(self, 4)
    }
    ///Bits 5:12 - layer alpha value
    #[inline(always)]
    pub fn alpha(&mut self) -> AlphaW<VL_CFGrs> {
        AlphaW::new(self, 5)
    }
    ///Bits 13:14 - video layer blending depth
    #[inline(always)]
    pub fn blend_depth(&mut self) -> BlendDepthW<VL_CFGrs> {
        BlendDepthW::new(self, 13)
    }
    ///Bit 15 - layer color filter enable
    #[inline(always)]
    pub fn filter_en(&mut self) -> FilterEnW<VL_CFGrs> {
        FilterEnW::new(self, 15)
    }
    ///Bits 16:28 - source image width(including padding), unit is bytes
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<VL_CFGrs> {
        WidthW::new(self, 16)
    }
    ///Bit 29 - preload 64 bytes extra data when reading pixel from memory
    #[inline(always)]
    pub fn prefetch_en(&mut self) -> PrefetchEnW<VL_CFGrs> {
        PrefetchEnW::new(self, 29)
    }
    ///Bit 30 - layer active flag
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<VL_CFGrs> {
        ActiveW::new(self, 30)
    }
    ///Bit 31 - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
    #[inline(always)]
    pub fn alpha_blend(&mut self) -> AlphaBlendW<VL_CFGrs> {
        AlphaBlendW::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_CFGrs;
impl crate::RegisterSpec for VL_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`vl_cfg::R`](R) reader structure
impl crate::Readable for VL_CFGrs {}
///`write(|w| ..)` method takes [`vl_cfg::W`](W) writer structure
impl crate::Writable for VL_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_CFG to value 0
impl crate::Resettable for VL_CFGrs {
    const RESET_VALUE: u32 = 0;
}
