///Register `AUDIO_TX_LRCK_DIV` reader
pub type R = crate::R<AUDIO_TX_LRCK_DIVrs>;
///Register `AUDIO_TX_LRCK_DIV` writer
pub type W = crate::W<AUDIO_TX_LRCK_DIVrs>;
///Field `DUTY_LOW` reader - TX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS
pub type DutyLowR = crate::FieldReader<u16>;
///Field `DUTY_LOW` writer - TX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS
pub type DutyLowW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DUTY_HIGH` reader - TX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS
pub type DutyHighR = crate::FieldReader<u16>;
///Field `DUTY_HIGH` writer - TX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS
pub type DutyHighW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:11 - TX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS
    #[inline(always)]
    pub fn duty_low(&self) -> DutyLowR {
        DutyLowR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:27 - TX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS
    #[inline(always)]
    pub fn duty_high(&self) -> DutyHighR {
        DutyHighR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_TX_LRCK_DIV")
            .field("rsvd", &self.rsvd())
            .field("duty_high", &self.duty_high())
            .field("rsvd2", &self.rsvd2())
            .field("duty_low", &self.duty_low())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - TX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS
    #[inline(always)]
    pub fn duty_low(&mut self) -> DutyLowW<AUDIO_TX_LRCK_DIVrs> {
        DutyLowW::new(self, 0)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<AUDIO_TX_LRCK_DIVrs> {
        Rsvd2W::new(self, 12)
    }
    ///Bits 16:27 - TX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS
    #[inline(always)]
    pub fn duty_high(&mut self) -> DutyHighW<AUDIO_TX_LRCK_DIVrs> {
        DutyHighW::new(self, 16)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AUDIO_TX_LRCK_DIVrs> {
        RsvdW::new(self, 28)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_tx_lrck_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_tx_lrck_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_TX_LRCK_DIVrs;
impl crate::RegisterSpec for AUDIO_TX_LRCK_DIVrs {
    type Ux = u32;
}
///`read()` method returns [`audio_tx_lrck_div::R`](R) reader structure
impl crate::Readable for AUDIO_TX_LRCK_DIVrs {}
///`write(|w| ..)` method takes [`audio_tx_lrck_div::W`](W) writer structure
impl crate::Writable for AUDIO_TX_LRCK_DIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_TX_LRCK_DIV to value 0x007d_007d
impl crate::Resettable for AUDIO_TX_LRCK_DIVrs {
    const RESET_VALUE: u32 = 0x007d_007d;
}
