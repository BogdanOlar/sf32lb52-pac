///Register `FRAME_AREA` reader
pub type R = crate::R<FRAME_AREArs>;
///Register `FRAME_AREA` writer
pub type W = crate::W<FRAME_AREArs>;
///Field `FRAME_HEIGHT` reader - AEZIP frame height
pub type FrameHeightR = crate::FieldReader<u16>;
///Field `FRAME_HEIGHT` writer - AEZIP frame height
pub type FrameHeightW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `FRAME_WIDTH` reader - AEZIP frame width
pub type FrameWidthR = crate::FieldReader<u16>;
///Field `FRAME_WIDTH` writer - AEZIP frame width
pub type FrameWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - AEZIP frame height
    #[inline(always)]
    pub fn frame_height(&self) -> FrameHeightR {
        FrameHeightR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - AEZIP frame width
    #[inline(always)]
    pub fn frame_width(&self) -> FrameWidthR {
        FrameWidthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_AREA")
            .field("frame_width", &self.frame_width())
            .field("frame_height", &self.frame_height())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - AEZIP frame height
    #[inline(always)]
    pub fn frame_height(&mut self) -> FrameHeightW<FRAME_AREArs> {
        FrameHeightW::new(self, 0)
    }
    ///Bits 16:31 - AEZIP frame width
    #[inline(always)]
    pub fn frame_width(&mut self) -> FrameWidthW<FRAME_AREArs> {
        FrameWidthW::new(self, 16)
    }
}
///Aezip frame area
///
///You can [`read`](crate::Reg::read) this register and get [`frame_area::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_area::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_AREArs;
impl crate::RegisterSpec for FRAME_AREArs {
    type Ux = u32;
}
///`read()` method returns [`frame_area::R`](R) reader structure
impl crate::Readable for FRAME_AREArs {}
///`write(|w| ..)` method takes [`frame_area::W`](W) writer structure
impl crate::Writable for FRAME_AREArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_AREA to value 0
impl crate::Resettable for FRAME_AREArs {
    const RESET_VALUE: u32 = 0;
}
