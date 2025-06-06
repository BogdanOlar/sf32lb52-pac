///Register `PLAY_NUM` reader
pub type R = crate::R<PLAY_NUMrs>;
///Register `PLAY_NUM` writer
pub type W = crate::W<PLAY_NUMrs>;
///Field `PLAY_NUM` reader - number of times to loop this AEZIP,0 indicates infinite looping
pub type PlayNumR = crate::FieldReader<u32>;
///Field `PLAY_NUM` writer - number of times to loop this AEZIP,0 indicates infinite looping
pub type PlayNumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - number of times to loop this AEZIP,0 indicates infinite looping
    #[inline(always)]
    pub fn play_num(&self) -> PlayNumR {
        PlayNumR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLAY_NUM")
            .field("play_num", &self.play_num())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - number of times to loop this AEZIP,0 indicates infinite looping
    #[inline(always)]
    pub fn play_num(&mut self) -> PlayNumW<PLAY_NUMrs> {
        PlayNumW::new(self, 0)
    }
}
///Aezip number of times to loop this AEZIP
///
///You can [`read`](crate::Reg::read) this register and get [`play_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`play_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLAY_NUMrs;
impl crate::RegisterSpec for PLAY_NUMrs {
    type Ux = u32;
}
///`read()` method returns [`play_num::R`](R) reader structure
impl crate::Readable for PLAY_NUMrs {}
///`write(|w| ..)` method takes [`play_num::W`](W) writer structure
impl crate::Writable for PLAY_NUMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLAY_NUM to value 0
impl crate::Resettable for PLAY_NUMrs {
    const RESET_VALUE: u32 = 0;
}
