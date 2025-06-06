///Register `YUV_ENG_CFG0` reader
pub type R = crate::R<YUV_ENG_CFG0rs>;
///Register `YUV_ENG_CFG0` writer
pub type W = crate::W<YUV_ENG_CFG0rs>;
///Field `WIDTH_U` reader - yuv u vector line width, unit is bytes
pub type WidthUR = crate::FieldReader<u16>;
///Field `WIDTH_U` writer - yuv u vector line width, unit is bytes
pub type WidthUW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `WIDTH_Y` reader - yuv y vector line width, unit is bytes
pub type WidthYR = crate::FieldReader<u16>;
///Field `WIDTH_Y` writer - yuv y vector line width, unit is bytes
pub type WidthYW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `FORMAT` reader - yuv format
pub type FormatR = crate::FieldReader;
///Field `FORMAT` writer - yuv format
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RANGE_SEL` reader - yuv input range 1'h0: tv range 1'h1: pc range
pub type RangeSelR = crate::BitReader;
///Field `RANGE_SEL` writer - yuv input range 1'h0: tv range 1'h1: pc range
pub type RangeSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:12 - yuv u vector line width, unit is bytes
    #[inline(always)]
    pub fn width_u(&self) -> WidthUR {
        WidthUR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:25 - yuv y vector line width, unit is bytes
    #[inline(always)]
    pub fn width_y(&self) -> WidthYR {
        WidthYR::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    ///Bits 26:28 - yuv format
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bit 29 - yuv input range 1'h0: tv range 1'h1: pc range
    #[inline(always)]
    pub fn range_sel(&self) -> RangeSelR {
        RangeSelR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YUV_ENG_CFG0")
            .field("rsvd", &self.rsvd())
            .field("range_sel", &self.range_sel())
            .field("format", &self.format())
            .field("width_y", &self.width_y())
            .field("width_u", &self.width_u())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - yuv u vector line width, unit is bytes
    #[inline(always)]
    pub fn width_u(&mut self) -> WidthUW<YUV_ENG_CFG0rs> {
        WidthUW::new(self, 0)
    }
    ///Bits 13:25 - yuv y vector line width, unit is bytes
    #[inline(always)]
    pub fn width_y(&mut self) -> WidthYW<YUV_ENG_CFG0rs> {
        WidthYW::new(self, 13)
    }
    ///Bits 26:28 - yuv format
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<YUV_ENG_CFG0rs> {
        FormatW::new(self, 26)
    }
    ///Bit 29 - yuv input range 1'h0: tv range 1'h1: pc range
    #[inline(always)]
    pub fn range_sel(&mut self) -> RangeSelW<YUV_ENG_CFG0rs> {
        RangeSelW::new(self, 29)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<YUV_ENG_CFG0rs> {
        RsvdW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`yuv_eng_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yuv_eng_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct YUV_ENG_CFG0rs;
impl crate::RegisterSpec for YUV_ENG_CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`yuv_eng_cfg0::R`](R) reader structure
impl crate::Readable for YUV_ENG_CFG0rs {}
///`write(|w| ..)` method takes [`yuv_eng_cfg0::W`](W) writer structure
impl crate::Writable for YUV_ENG_CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets YUV_ENG_CFG0 to value 0
impl crate::Resettable for YUV_ENG_CFG0rs {
    const RESET_VALUE: u32 = 0;
}
