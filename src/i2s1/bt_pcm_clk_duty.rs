///Register `BT_PCM_CLK_DUTY` reader
pub type R = crate::R<BT_PCM_CLK_DUTYrs>;
///Register `BT_PCM_CLK_DUTY` writer
pub type W = crate::W<BT_PCM_CLK_DUTYrs>;
///Field `CLK_DUTY` reader - BT_PCM_CLK duty cycle = (GCLK/(bt_pcm_sync*bt_pcm_dw))
pub type ClkDutyR = crate::FieldReader<u16>;
///Field `CLK_DUTY` writer - BT_PCM_CLK duty cycle = (GCLK/(bt_pcm_sync*bt_pcm_dw))
pub type ClkDutyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:9 - BT_PCM_CLK duty cycle = (GCLK/(bt_pcm_sync*bt_pcm_dw))
    #[inline(always)]
    pub fn clk_duty(&self) -> ClkDutyR {
        ClkDutyR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_PCM_CLK_DUTY")
            .field("rsvd", &self.rsvd())
            .field("clk_duty", &self.clk_duty())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - BT_PCM_CLK duty cycle = (GCLK/(bt_pcm_sync*bt_pcm_dw))
    #[inline(always)]
    pub fn clk_duty(&mut self) -> ClkDutyW<BT_PCM_CLK_DUTYrs> {
        ClkDutyW::new(self, 0)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<BT_PCM_CLK_DUTYrs> {
        RsvdW::new(self, 10)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_clk_duty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_clk_duty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BT_PCM_CLK_DUTYrs;
impl crate::RegisterSpec for BT_PCM_CLK_DUTYrs {
    type Ux = u32;
}
///`read()` method returns [`bt_pcm_clk_duty::R`](R) reader structure
impl crate::Readable for BT_PCM_CLK_DUTYrs {}
///`write(|w| ..)` method takes [`bt_pcm_clk_duty::W`](W) writer structure
impl crate::Writable for BT_PCM_CLK_DUTYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_PCM_CLK_DUTY to value 0
impl crate::Resettable for BT_PCM_CLK_DUTYrs {
    const RESET_VALUE: u32 = 0;
}
