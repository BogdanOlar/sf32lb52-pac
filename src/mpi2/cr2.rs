///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `LOOP` reader - Repeat CMD1->CMD2 sequence for n times. This filed is only valid when CMD2E=1 and SME2=0. For example if LOOP=0, then the sequence is CMD1 -> CMD2. If LOOP=2, then the sequence is (CMD1->CMD2) -> (CMD1->CMD2) -> (CMD1->CMD2)
pub type LoopR = crate::FieldReader;
///Field `LOOP` writer - Repeat CMD1->CMD2 sequence for n times. This filed is only valid when CMD2E=1 and SME2=0. For example if LOOP=0, then the sequence is CMD1 -> CMD2. If LOOP=2, then the sequence is (CMD1->CMD2) -> (CMD1->CMD2) -> (CMD1->CMD2)
pub type LoopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Repeat CMD1->CMD2 sequence for n times. This filed is only valid when CMD2E=1 and SME2=0. For example if LOOP=0, then the sequence is CMD1 -> CMD2. If LOOP=2, then the sequence is (CMD1->CMD2) -> (CMD1->CMD2) -> (CMD1->CMD2)
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2").field("loop_", &self.loop_()).finish()
    }
}
impl W {
    ///Bits 0:7 - Repeat CMD1->CMD2 sequence for n times. This filed is only valid when CMD2E=1 and SME2=0. For example if LOOP=0, then the sequence is CMD1 -> CMD2. If LOOP=2, then the sequence is (CMD1->CMD2) -> (CMD1->CMD2) -> (CMD1->CMD2)
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<CR2rs> {
        LoopW::new(self, 0)
    }
}
///Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
