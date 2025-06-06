///Register `YUV_ENG_CFG1` reader
pub type R = crate::R<YUV_ENG_CFG1rs>;
///Register `YUV_ENG_CFG1` writer
pub type W = crate::W<YUV_ENG_CFG1rs>;
///Field `WIDTH_V` reader - yuv v vector line width, unit is bytes
pub type WidthVR = crate::FieldReader<u16>;
///Field `WIDTH_V` writer - yuv v vector line width, unit is bytes
pub type WidthVW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - yuv v vector line width, unit is bytes
    #[inline(always)]
    pub fn width_v(&self) -> WidthVR {
        WidthVR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YUV_ENG_CFG1")
            .field("width_v", &self.width_v())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - yuv v vector line width, unit is bytes
    #[inline(always)]
    pub fn width_v(&mut self) -> WidthVW<YUV_ENG_CFG1rs> {
        WidthVW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`yuv_eng_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yuv_eng_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct YUV_ENG_CFG1rs;
impl crate::RegisterSpec for YUV_ENG_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`yuv_eng_cfg1::R`](R) reader structure
impl crate::Readable for YUV_ENG_CFG1rs {}
///`write(|w| ..)` method takes [`yuv_eng_cfg1::W`](W) writer structure
impl crate::Writable for YUV_ENG_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets YUV_ENG_CFG1 to value 0
impl crate::Resettable for YUV_ENG_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
