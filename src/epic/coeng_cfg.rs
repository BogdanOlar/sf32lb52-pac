///Register `COENG_CFG` reader
pub type R = crate::R<COENG_CFGrs>;
///Register `COENG_CFG` writer
pub type W = crate::W<COENG_CFGrs>;
///Field `EZIP_EN` reader - ezip enable
pub type EzipEnR = crate::BitReader;
///Field `EZIP_EN` writer - ezip enable
pub type EzipEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EZIP_CH_SEL` reader - ezip channel select
pub type EzipChSelR = crate::FieldReader;
///Field `EZIP_CH_SEL` writer - ezip channel select
pub type EzipChSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `YUV_EN` reader - yuv enable
pub type YuvEnR = crate::BitReader;
///Field `YUV_EN` writer - yuv enable
pub type YuvEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YUV_CH_SEL` reader - yuv engine channel select
pub type YuvChSelR = crate::FieldReader;
///Field `YUV_CH_SEL` writer - yuv engine channel select
pub type YuvChSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - ezip enable
    #[inline(always)]
    pub fn ezip_en(&self) -> EzipEnR {
        EzipEnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - ezip channel select
    #[inline(always)]
    pub fn ezip_ch_sel(&self) -> EzipChSelR {
        EzipChSelR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - yuv enable
    #[inline(always)]
    pub fn yuv_en(&self) -> YuvEnR {
        YuvEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - yuv engine channel select
    #[inline(always)]
    pub fn yuv_ch_sel(&self) -> YuvChSelR {
        YuvChSelR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COENG_CFG")
            .field("yuv_ch_sel", &self.yuv_ch_sel())
            .field("yuv_en", &self.yuv_en())
            .field("ezip_ch_sel", &self.ezip_ch_sel())
            .field("ezip_en", &self.ezip_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - ezip enable
    #[inline(always)]
    pub fn ezip_en(&mut self) -> EzipEnW<COENG_CFGrs> {
        EzipEnW::new(self, 0)
    }
    ///Bits 1:2 - ezip channel select
    #[inline(always)]
    pub fn ezip_ch_sel(&mut self) -> EzipChSelW<COENG_CFGrs> {
        EzipChSelW::new(self, 1)
    }
    ///Bit 3 - yuv enable
    #[inline(always)]
    pub fn yuv_en(&mut self) -> YuvEnW<COENG_CFGrs> {
        YuvEnW::new(self, 3)
    }
    ///Bits 4:5 - yuv engine channel select
    #[inline(always)]
    pub fn yuv_ch_sel(&mut self) -> YuvChSelW<COENG_CFGrs> {
        YuvChSelW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`coeng_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coeng_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COENG_CFGrs;
impl crate::RegisterSpec for COENG_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`coeng_cfg::R`](R) reader structure
impl crate::Readable for COENG_CFGrs {}
///`write(|w| ..)` method takes [`coeng_cfg::W`](W) writer structure
impl crate::Writable for COENG_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COENG_CFG to value 0
impl crate::Resettable for COENG_CFGrs {
    const RESET_VALUE: u32 = 0;
}
