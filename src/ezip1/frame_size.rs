///Register `FRAME_SIZE` reader
pub type R = crate::R<FRAME_SIZErs>;
///Register `FRAME_SIZE` writer
pub type W = crate::W<FRAME_SIZErs>;
///Field `FRAME_SIZE` reader - frame size
pub type FrameSizeR = crate::FieldReader<u32>;
///Field `FRAME_SIZE` writer - frame size
pub type FrameSizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - frame size
    #[inline(always)]
    pub fn frame_size(&self) -> FrameSizeR {
        FrameSizeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_SIZE")
            .field("frame_size", &self.frame_size())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - frame size
    #[inline(always)]
    pub fn frame_size(&mut self) -> FrameSizeW<FRAME_SIZErs> {
        FrameSizeW::new(self, 0)
    }
}
///Aezip frame size
///
///You can [`read`](crate::Reg::read) this register and get [`frame_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_SIZErs;
impl crate::RegisterSpec for FRAME_SIZErs {
    type Ux = u32;
}
///`read()` method returns [`frame_size::R`](R) reader structure
impl crate::Readable for FRAME_SIZErs {}
///`write(|w| ..)` method takes [`frame_size::W`](W) writer structure
impl crate::Writable for FRAME_SIZErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_SIZE to value 0
impl crate::Resettable for FRAME_SIZErs {
    const RESET_VALUE: u32 = 0;
}
