///Register `AUDIO_RX_BCLK_DIV` reader
pub type R = crate::R<AUDIO_RX_BCLK_DIVrs>;
///Register `AUDIO_RX_BCLK_DIV` writer
pub type W = crate::W<AUDIO_RX_BCLK_DIVrs>;
///Field `DUTY` reader - RX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs
pub type DutyR = crate::FieldReader<u16>;
///Field `DUTY` writer - RX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:9 - RX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_RX_BCLK_DIV")
            .field("rsvd", &self.rsvd())
            .field("duty", &self.duty())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - RX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs
    #[inline(always)]
    pub fn duty(&mut self) -> DutyW<AUDIO_RX_BCLK_DIVrs> {
        DutyW::new(self, 0)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AUDIO_RX_BCLK_DIVrs> {
        RsvdW::new(self, 10)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_bclk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_bclk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_RX_BCLK_DIVrs;
impl crate::RegisterSpec for AUDIO_RX_BCLK_DIVrs {
    type Ux = u32;
}
///`read()` method returns [`audio_rx_bclk_div::R`](R) reader structure
impl crate::Readable for AUDIO_RX_BCLK_DIVrs {}
///`write(|w| ..)` method takes [`audio_rx_bclk_div::W`](W) writer structure
impl crate::Writable for AUDIO_RX_BCLK_DIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_RX_BCLK_DIV to value 0x05
impl crate::Resettable for AUDIO_RX_BCLK_DIVrs {
    const RESET_VALUE: u32 = 0x05;
}
