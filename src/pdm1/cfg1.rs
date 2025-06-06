///Register `CFG1` reader
pub type R = crate::R<CFG1rs>;
///Register `CFG1` writer
pub type W = crate::W<CFG1rs>;
///Field `SAMPLE_DLY_L` reader - The number of delay dff before the left data stream in processing
pub type SampleDlyLR = crate::FieldReader;
///Field `SAMPLE_DLY_L` writer - The number of delay dff before the left data stream in processing
pub type SampleDlyLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SAMPLE_DLY_R` reader - The number of delay dff before the right data stream in processing
pub type SampleDlyRR = crate::FieldReader;
///Field `SAMPLE_DLY_R` writer - The number of delay dff before the right data stream in processing
pub type SampleDlyRW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 5:7 - The number of delay dff before the left data stream in processing
    #[inline(always)]
    pub fn sample_dly_l(&self) -> SampleDlyLR {
        SampleDlyLR::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - The number of delay dff before the right data stream in processing
    #[inline(always)]
    pub fn sample_dly_r(&self) -> SampleDlyRR {
        SampleDlyRR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("sample_dly_r", &self.sample_dly_r())
            .field("sample_dly_l", &self.sample_dly_l())
            .finish()
    }
}
impl W {
    ///Bits 5:7 - The number of delay dff before the left data stream in processing
    #[inline(always)]
    pub fn sample_dly_l(&mut self) -> SampleDlyLW<CFG1rs> {
        SampleDlyLW::new(self, 5)
    }
    ///Bits 8:10 - The number of delay dff before the right data stream in processing
    #[inline(always)]
    pub fn sample_dly_r(&mut self) -> SampleDlyRW<CFG1rs> {
        SampleDlyRW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFG1rs;
impl crate::RegisterSpec for CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`cfg1::R`](R) reader structure
impl crate::Readable for CFG1rs {}
///`write(|w| ..)` method takes [`cfg1::W`](W) writer structure
impl crate::Writable for CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG1 to value 0
impl crate::Resettable for CFG1rs {
    const RESET_VALUE: u32 = 0;
}
