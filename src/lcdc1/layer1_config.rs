///Register `LAYER1_CONFIG` reader
pub type R = crate::R<LAYER1_CONFIGrs>;
///Register `LAYER1_CONFIG` writer
pub type W = crate::W<LAYER1_CONFIGrs>;
///Field `FORMAT` reader - overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved
pub type FormatR = crate::FieldReader;
///Field `FORMAT` writer - overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ALPHA_SEL` reader - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
pub type AlphaSelR = crate::BitReader;
///Field `ALPHA_SEL` writer - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
pub type AlphaSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALPHA` reader - layer alpha value
pub type AlphaR = crate::FieldReader;
///Field `ALPHA` writer - layer alpha value
pub type AlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
///Field `LINE_FETCH_MODE` reader - line fetch mode 0: address skip every single line 1: address skip every two line
pub type LineFetchModeR = crate::BitReader;
///Field `LINE_FETCH_MODE` writer - line fetch mode 0: address skip every single line 1: address skip every two line
pub type LineFetchModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACTIVE` reader - layer active flag
pub type ActiveR = crate::BitReader;
///Field `ACTIVE` writer - layer active flag
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALPHA_BLEND` reader - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
pub type AlphaBlendR = crate::BitReader;
///Field `ALPHA_BLEND` writer - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
pub type AlphaBlendW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `V_MIRROR` reader - set 1 to do vertical mirror for the layer
pub type VMirrorR = crate::BitReader;
///Field `V_MIRROR` writer - set 1 to do vertical mirror for the layer
pub type VMirrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
    #[inline(always)]
    pub fn alpha_sel(&self) -> AlphaSelR {
        AlphaSelR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:11 - layer alpha value
    #[inline(always)]
    pub fn alpha(&self) -> AlphaR {
        AlphaR::new(((self.bits >> 4) & 0xff) as u8)
    }
    ///Bit 12 - layer color filter enable
    #[inline(always)]
    pub fn filter_en(&self) -> FilterEnR {
        FilterEnR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:25 - source image width(including padding), unit is bytes
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    ///Bit 26 - preload 64 bytes extra data when reading pixel from memory
    #[inline(always)]
    pub fn prefetch_en(&self) -> PrefetchEnR {
        PrefetchEnR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - line fetch mode 0: address skip every single line 1: address skip every two line
    #[inline(always)]
    pub fn line_fetch_mode(&self) -> LineFetchModeR {
        LineFetchModeR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - layer active flag
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
    #[inline(always)]
    pub fn alpha_blend(&self) -> AlphaBlendR {
        AlphaBlendR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - set 1 to do vertical mirror for the layer
    #[inline(always)]
    pub fn v_mirror(&self) -> VMirrorR {
        VMirrorR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER1_CONFIG")
            .field("v_mirror", &self.v_mirror())
            .field("alpha_blend", &self.alpha_blend())
            .field("active", &self.active())
            .field("line_fetch_mode", &self.line_fetch_mode())
            .field("prefetch_en", &self.prefetch_en())
            .field("width", &self.width())
            .field("filter_en", &self.filter_en())
            .field("alpha", &self.alpha())
            .field("alpha_sel", &self.alpha_sel())
            .field("format", &self.format())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<LAYER1_CONFIGrs> {
        FormatW::new(self, 0)
    }
    ///Bit 3 - alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha
    #[inline(always)]
    pub fn alpha_sel(&mut self) -> AlphaSelW<LAYER1_CONFIGrs> {
        AlphaSelW::new(self, 3)
    }
    ///Bits 4:11 - layer alpha value
    #[inline(always)]
    pub fn alpha(&mut self) -> AlphaW<LAYER1_CONFIGrs> {
        AlphaW::new(self, 4)
    }
    ///Bit 12 - layer color filter enable
    #[inline(always)]
    pub fn filter_en(&mut self) -> FilterEnW<LAYER1_CONFIGrs> {
        FilterEnW::new(self, 12)
    }
    ///Bits 13:25 - source image width(including padding), unit is bytes
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<LAYER1_CONFIGrs> {
        WidthW::new(self, 13)
    }
    ///Bit 26 - preload 64 bytes extra data when reading pixel from memory
    #[inline(always)]
    pub fn prefetch_en(&mut self) -> PrefetchEnW<LAYER1_CONFIGrs> {
        PrefetchEnW::new(self, 26)
    }
    ///Bit 27 - line fetch mode 0: address skip every single line 1: address skip every two line
    #[inline(always)]
    pub fn line_fetch_mode(&mut self) -> LineFetchModeW<LAYER1_CONFIGrs> {
        LineFetchModeW::new(self, 27)
    }
    ///Bit 28 - layer active flag
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<LAYER1_CONFIGrs> {
        ActiveW::new(self, 28)
    }
    ///Bit 29 - set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha
    #[inline(always)]
    pub fn alpha_blend(&mut self) -> AlphaBlendW<LAYER1_CONFIGrs> {
        AlphaBlendW::new(self, 29)
    }
    ///Bit 30 - set 1 to do vertical mirror for the layer
    #[inline(always)]
    pub fn v_mirror(&mut self) -> VMirrorW<LAYER1_CONFIGrs> {
        VMirrorW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER1_CONFIGrs;
impl crate::RegisterSpec for LAYER1_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`layer1_config::R`](R) reader structure
impl crate::Readable for LAYER1_CONFIGrs {}
///`write(|w| ..)` method takes [`layer1_config::W`](W) writer structure
impl crate::Writable for LAYER1_CONFIGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER1_CONFIG to value 0
impl crate::Resettable for LAYER1_CONFIGrs {
    const RESET_VALUE: u32 = 0;
}
