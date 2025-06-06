///Register `RX_RE_SAMPLE_CLK_DIV` reader
pub type R = crate::R<RX_RE_SAMPLE_CLK_DIVrs>;
///Register `RX_RE_SAMPLE_CLK_DIV` writer
pub type W = crate::W<RX_RE_SAMPLE_CLK_DIVrs>;
///Field `RS_DUTY` reader - source PCM sample clock duty cycle: 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS Note: 1)duty_cycle = 12M/FS
pub type RsDutyR = crate::FieldReader<u16>;
///Field `RS_DUTY` writer - source PCM sample clock duty cycle: 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS Note: 1)duty_cycle = 12M/FS
pub type RsDutyW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - source PCM sample clock duty cycle: 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS Note: 1)duty_cycle = 12M/FS
    #[inline(always)]
    pub fn rs_duty(&self) -> RsDutyR {
        RsDutyR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_RE_SAMPLE_CLK_DIV")
            .field("rs_duty", &self.rs_duty())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - source PCM sample clock duty cycle: 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS Note: 1)duty_cycle = 12M/FS
    #[inline(always)]
    pub fn rs_duty(&mut self) -> RsDutyW<RX_RE_SAMPLE_CLK_DIVrs> {
        RsDutyW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_re_sample_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_re_sample_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_RE_SAMPLE_CLK_DIVrs;
impl crate::RegisterSpec for RX_RE_SAMPLE_CLK_DIVrs {
    type Ux = u32;
}
///`read()` method returns [`rx_re_sample_clk_div::R`](R) reader structure
impl crate::Readable for RX_RE_SAMPLE_CLK_DIVrs {}
///`write(|w| ..)` method takes [`rx_re_sample_clk_div::W`](W) writer structure
impl crate::Writable for RX_RE_SAMPLE_CLK_DIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_RE_SAMPLE_CLK_DIV to value 0x7d
impl crate::Resettable for RX_RE_SAMPLE_CLK_DIVrs {
    const RESET_VALUE: u32 = 0x7d;
}
