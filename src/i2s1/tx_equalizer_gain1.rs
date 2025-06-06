///Register `TX_EQUALIZER_GAIN1` reader
pub type R = crate::R<TX_EQUALIZER_GAIN1rs>;
///Register `TX_EQUALIZER_GAIN1` writer
pub type W = crate::W<TX_EQUALIZER_GAIN1rs>;
///Field `BAND1_GAIN` reader -
pub type Band1GainR = crate::FieldReader;
///Field `BAND1_GAIN` writer -
pub type Band1GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND2_GAIN` reader -
pub type Band2GainR = crate::FieldReader;
///Field `BAND2_GAIN` writer -
pub type Band2GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND3_GAIN` reader -
pub type Band3GainR = crate::FieldReader;
///Field `BAND3_GAIN` writer -
pub type Band3GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND4_GAIN` reader -
pub type Band4GainR = crate::FieldReader;
///Field `BAND4_GAIN` writer -
pub type Band4GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND5_GAIN` reader -
pub type Band5GainR = crate::FieldReader;
///Field `BAND5_GAIN` writer -
pub type Band5GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND6_GAIN` reader -
pub type Band6GainR = crate::FieldReader;
///Field `BAND6_GAIN` writer -
pub type Band6GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn band1_gain(&self) -> Band1GainR {
        Band1GainR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9
    #[inline(always)]
    pub fn band2_gain(&self) -> Band2GainR {
        Band2GainR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14
    #[inline(always)]
    pub fn band3_gain(&self) -> Band3GainR {
        Band3GainR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19
    #[inline(always)]
    pub fn band4_gain(&self) -> Band4GainR {
        Band4GainR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24
    #[inline(always)]
    pub fn band5_gain(&self) -> Band5GainR {
        Band5GainR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29
    #[inline(always)]
    pub fn band6_gain(&self) -> Band6GainR {
        Band6GainR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_EQUALIZER_GAIN1")
            .field("band6_gain", &self.band6_gain())
            .field("band5_gain", &self.band5_gain())
            .field("band4_gain", &self.band4_gain())
            .field("band3_gain", &self.band3_gain())
            .field("band2_gain", &self.band2_gain())
            .field("band1_gain", &self.band1_gain())
            .finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    pub fn band1_gain(&mut self) -> Band1GainW<TX_EQUALIZER_GAIN1rs> {
        Band1GainW::new(self, 0)
    }
    ///Bits 5:9
    #[inline(always)]
    pub fn band2_gain(&mut self) -> Band2GainW<TX_EQUALIZER_GAIN1rs> {
        Band2GainW::new(self, 5)
    }
    ///Bits 10:14
    #[inline(always)]
    pub fn band3_gain(&mut self) -> Band3GainW<TX_EQUALIZER_GAIN1rs> {
        Band3GainW::new(self, 10)
    }
    ///Bits 15:19
    #[inline(always)]
    pub fn band4_gain(&mut self) -> Band4GainW<TX_EQUALIZER_GAIN1rs> {
        Band4GainW::new(self, 15)
    }
    ///Bits 20:24
    #[inline(always)]
    pub fn band5_gain(&mut self) -> Band5GainW<TX_EQUALIZER_GAIN1rs> {
        Band5GainW::new(self, 20)
    }
    ///Bits 25:29
    #[inline(always)]
    pub fn band6_gain(&mut self) -> Band6GainW<TX_EQUALIZER_GAIN1rs> {
        Band6GainW::new(self, 25)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_equalizer_gain1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_equalizer_gain1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_EQUALIZER_GAIN1rs;
impl crate::RegisterSpec for TX_EQUALIZER_GAIN1rs {
    type Ux = u32;
}
///`read()` method returns [`tx_equalizer_gain1::R`](R) reader structure
impl crate::Readable for TX_EQUALIZER_GAIN1rs {}
///`write(|w| ..)` method takes [`tx_equalizer_gain1::W`](W) writer structure
impl crate::Writable for TX_EQUALIZER_GAIN1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_EQUALIZER_GAIN1 to value 0
impl crate::Resettable for TX_EQUALIZER_GAIN1rs {
    const RESET_VALUE: u32 = 0;
}
