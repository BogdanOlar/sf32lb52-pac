///Register `FRAME_START` reader
pub type R = crate::R<FRAME_STARTrs>;
///Register `FRAME_START` writer
pub type W = crate::W<FRAME_STARTrs>;
///Field `FRAME_START` reader - start number of frames,count from 1
pub type FrameStartR = crate::FieldReader<u32>;
///Field `FRAME_START` writer - start number of frames,count from 1
pub type FrameStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - start number of frames,count from 1
    #[inline(always)]
    pub fn frame_start(&self) -> FrameStartR {
        FrameStartR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_START")
            .field("frame_start", &self.frame_start())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - start number of frames,count from 1
    #[inline(always)]
    pub fn frame_start(&mut self) -> FrameStartW<FRAME_STARTrs> {
        FrameStartW::new(self, 0)
    }
}
///Aezip start number of frames
///
///You can [`read`](crate::Reg::read) this register and get [`frame_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_STARTrs;
impl crate::RegisterSpec for FRAME_STARTrs {
    type Ux = u32;
}
///`read()` method returns [`frame_start::R`](R) reader structure
impl crate::Readable for FRAME_STARTrs {}
///`write(|w| ..)` method takes [`frame_start::W`](W) writer structure
impl crate::Writable for FRAME_STARTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_START to value 0
impl crate::Resettable for FRAME_STARTrs {
    const RESET_VALUE: u32 = 0;
}
