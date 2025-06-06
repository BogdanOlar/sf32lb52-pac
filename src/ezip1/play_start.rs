///Register `PLAY_START` reader
pub type R = crate::R<PLAY_STARTrs>;
///Register `PLAY_START` writer
pub type W = crate::W<PLAY_STARTrs>;
///Field `PLAY_START` reader - start number of times to loop this AEZIP,,count from 1
pub type PlayStartR = crate::FieldReader<u32>;
///Field `PLAY_START` writer - start number of times to loop this AEZIP,,count from 1
pub type PlayStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - start number of times to loop this AEZIP,,count from 1
    #[inline(always)]
    pub fn play_start(&self) -> PlayStartR {
        PlayStartR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLAY_START")
            .field("play_start", &self.play_start())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - start number of times to loop this AEZIP,,count from 1
    #[inline(always)]
    pub fn play_start(&mut self) -> PlayStartW<PLAY_STARTrs> {
        PlayStartW::new(self, 0)
    }
}
///Aezip start number of play
///
///You can [`read`](crate::Reg::read) this register and get [`play_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`play_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLAY_STARTrs;
impl crate::RegisterSpec for PLAY_STARTrs {
    type Ux = u32;
}
///`read()` method returns [`play_start::R`](R) reader structure
impl crate::Readable for PLAY_STARTrs {}
///`write(|w| ..)` method takes [`play_start::W`](W) writer structure
impl crate::Writable for PLAY_STARTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLAY_START to value 0
impl crate::Resettable for PLAY_STARTrs {
    const RESET_VALUE: u32 = 0;
}
