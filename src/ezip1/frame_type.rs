///Register `FRAME_TYPE` reader
pub type R = crate::R<FRAME_TYPErs>;
///Register `FRAME_TYPE` writer
pub type W = crate::W<FRAME_TYPErs>;
///Field `BLEND_OP` reader - AEZIP type of frame area renndering for this frame
pub type BlendOpR = crate::FieldReader;
///Field `BLEND_OP` writer - AEZIP type of frame area renndering for this frame
pub type BlendOpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DISPOSE_OP` reader - AEZIP type of frame area disposal to be done after rendering this frame
pub type DisposeOpR = crate::FieldReader;
///Field `DISPOSE_OP` writer - AEZIP type of frame area disposal to be done after rendering this frame
pub type DisposeOpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - AEZIP type of frame area renndering for this frame
    #[inline(always)]
    pub fn blend_op(&self) -> BlendOpR {
        BlendOpR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - AEZIP type of frame area disposal to be done after rendering this frame
    #[inline(always)]
    pub fn dispose_op(&self) -> DisposeOpR {
        DisposeOpR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_TYPE")
            .field("dispose_op", &self.dispose_op())
            .field("blend_op", &self.blend_op())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - AEZIP type of frame area renndering for this frame
    #[inline(always)]
    pub fn blend_op(&mut self) -> BlendOpW<FRAME_TYPErs> {
        BlendOpW::new(self, 0)
    }
    ///Bits 8:15 - AEZIP type of frame area disposal to be done after rendering this frame
    #[inline(always)]
    pub fn dispose_op(&mut self) -> DisposeOpW<FRAME_TYPErs> {
        DisposeOpW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`frame_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_TYPErs;
impl crate::RegisterSpec for FRAME_TYPErs {
    type Ux = u32;
}
///`read()` method returns [`frame_type::R`](R) reader structure
impl crate::Readable for FRAME_TYPErs {}
///`write(|w| ..)` method takes [`frame_type::W`](W) writer structure
impl crate::Writable for FRAME_TYPErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_TYPE to value 0
impl crate::Resettable for FRAME_TYPErs {
    const RESET_VALUE: u32 = 0;
}
