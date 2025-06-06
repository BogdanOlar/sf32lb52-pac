///Register `TX_PCM_SAMPLE_CLK` reader
pub type R = crate::R<TX_PCM_SAMPLE_CLKrs>;
///Register `TX_PCM_SAMPLE_CLK` writer
pub type W = crate::W<TX_PCM_SAMPLE_CLKrs>;
///Field `FS_DUTY` reader - source PCM sample clock duty cycle(with GCLK=12MHz): 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS
pub type FsDutyR = crate::FieldReader<u16>;
///Field `FS_DUTY` writer - source PCM sample clock duty cycle(with GCLK=12MHz): 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS
pub type FsDutyW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - source PCM sample clock duty cycle(with GCLK=12MHz): 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS
    #[inline(always)]
    pub fn fs_duty(&self) -> FsDutyR {
        FsDutyR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PCM_SAMPLE_CLK")
            .field("fs_duty", &self.fs_duty())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - source PCM sample clock duty cycle(with GCLK=12MHz): 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS
    #[inline(always)]
    pub fn fs_duty(&mut self) -> FsDutyW<TX_PCM_SAMPLE_CLKrs> {
        FsDutyW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_pcm_sample_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm_sample_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_PCM_SAMPLE_CLKrs;
impl crate::RegisterSpec for TX_PCM_SAMPLE_CLKrs {
    type Ux = u32;
}
///`read()` method returns [`tx_pcm_sample_clk::R`](R) reader structure
impl crate::Readable for TX_PCM_SAMPLE_CLKrs {}
///`write(|w| ..)` method takes [`tx_pcm_sample_clk::W`](W) writer structure
impl crate::Writable for TX_PCM_SAMPLE_CLKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_PCM_SAMPLE_CLK to value 0xfa
impl crate::Resettable for TX_PCM_SAMPLE_CLKrs {
    const RESET_VALUE: u32 = 0xfa;
}
