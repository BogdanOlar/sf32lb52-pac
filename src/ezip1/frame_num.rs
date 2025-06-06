///Register `FRAME_NUM` reader
pub type R = crate::R<FRAME_NUMrs>;
///Register `FRAME_NUM` writer
pub type W = crate::W<FRAME_NUMrs>;
///Field `FRAME_NUM` reader - number of frames
pub type FrameNumR = crate::FieldReader<u32>;
///Field `FRAME_NUM` writer - number of frames
pub type FrameNumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - number of frames
    #[inline(always)]
    pub fn frame_num(&self) -> FrameNumR {
        FrameNumR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_NUM")
            .field("frame_num", &self.frame_num())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - number of frames
    #[inline(always)]
    pub fn frame_num(&mut self) -> FrameNumW<FRAME_NUMrs> {
        FrameNumW::new(self, 0)
    }
}
///Aezip number of frames
///
///You can [`read`](crate::Reg::read) this register and get [`frame_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_NUMrs;
impl crate::RegisterSpec for FRAME_NUMrs {
    type Ux = u32;
}
///`read()` method returns [`frame_num::R`](R) reader structure
impl crate::Readable for FRAME_NUMrs {}
///`write(|w| ..)` method takes [`frame_num::W`](W) writer structure
impl crate::Writable for FRAME_NUMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_NUM to value 0
impl crate::Resettable for FRAME_NUMrs {
    const RESET_VALUE: u32 = 0;
}
