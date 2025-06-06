///Register `SEQ_NUM` reader
pub type R = crate::R<SEQ_NUMrs>;
///Register `SEQ_NUM` writer
pub type W = crate::W<SEQ_NUMrs>;
///Field `SEQ_NUM` reader - sequence number of the animation chunk,starting from 0
pub type SeqNumR = crate::FieldReader<u32>;
///Field `SEQ_NUM` writer - sequence number of the animation chunk,starting from 0
pub type SeqNumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - sequence number of the animation chunk,starting from 0
    #[inline(always)]
    pub fn seq_num(&self) -> SeqNumR {
        SeqNumR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_NUM")
            .field("seq_num", &self.seq_num())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - sequence number of the animation chunk,starting from 0
    #[inline(always)]
    pub fn seq_num(&mut self) -> SeqNumW<SEQ_NUMrs> {
        SeqNumW::new(self, 0)
    }
}
///Aezip sequence number
///
///You can [`read`](crate::Reg::read) this register and get [`seq_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQ_NUMrs;
impl crate::RegisterSpec for SEQ_NUMrs {
    type Ux = u32;
}
///`read()` method returns [`seq_num::R`](R) reader structure
impl crate::Readable for SEQ_NUMrs {}
///`write(|w| ..)` method takes [`seq_num::W`](W) writer structure
impl crate::Writable for SEQ_NUMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQ_NUM to value 0
impl crate::Resettable for SEQ_NUMrs {
    const RESET_VALUE: u32 = 0;
}
