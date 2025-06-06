///Register `TX_EQUALIZER_GAIN2` reader
pub type R = crate::R<TX_EQUALIZER_GAIN2rs>;
///Register `TX_EQUALIZER_GAIN2` writer
pub type W = crate::W<TX_EQUALIZER_GAIN2rs>;
///Field `BAND7_GAIN` reader -
pub type Band7GainR = crate::FieldReader;
///Field `BAND7_GAIN` writer -
pub type Band7GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND8_GAIN` reader -
pub type Band8GainR = crate::FieldReader;
///Field `BAND8_GAIN` writer -
pub type Band8GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND9_GAIN` reader -
pub type Band9GainR = crate::FieldReader;
///Field `BAND9_GAIN` writer -
pub type Band9GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BAND10_GAIN` reader -
pub type Band10GainR = crate::FieldReader;
///Field `BAND10_GAIN` writer -
pub type Band10GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn band7_gain(&self) -> Band7GainR {
        Band7GainR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9
    #[inline(always)]
    pub fn band8_gain(&self) -> Band8GainR {
        Band8GainR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14
    #[inline(always)]
    pub fn band9_gain(&self) -> Band9GainR {
        Band9GainR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19
    #[inline(always)]
    pub fn band10_gain(&self) -> Band10GainR {
        Band10GainR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_EQUALIZER_GAIN2")
            .field("band10_gain", &self.band10_gain())
            .field("band9_gain", &self.band9_gain())
            .field("band8_gain", &self.band8_gain())
            .field("band7_gain", &self.band7_gain())
            .finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    pub fn band7_gain(&mut self) -> Band7GainW<TX_EQUALIZER_GAIN2rs> {
        Band7GainW::new(self, 0)
    }
    ///Bits 5:9
    #[inline(always)]
    pub fn band8_gain(&mut self) -> Band8GainW<TX_EQUALIZER_GAIN2rs> {
        Band8GainW::new(self, 5)
    }
    ///Bits 10:14
    #[inline(always)]
    pub fn band9_gain(&mut self) -> Band9GainW<TX_EQUALIZER_GAIN2rs> {
        Band9GainW::new(self, 10)
    }
    ///Bits 15:19
    #[inline(always)]
    pub fn band10_gain(&mut self) -> Band10GainW<TX_EQUALIZER_GAIN2rs> {
        Band10GainW::new(self, 15)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_equalizer_gain2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_equalizer_gain2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_EQUALIZER_GAIN2rs;
impl crate::RegisterSpec for TX_EQUALIZER_GAIN2rs {
    type Ux = u32;
}
///`read()` method returns [`tx_equalizer_gain2::R`](R) reader structure
impl crate::Readable for TX_EQUALIZER_GAIN2rs {}
///`write(|w| ..)` method takes [`tx_equalizer_gain2::W`](W) writer structure
impl crate::Writable for TX_EQUALIZER_GAIN2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_EQUALIZER_GAIN2 to value 0
impl crate::Resettable for TX_EQUALIZER_GAIN2rs {
    const RESET_VALUE: u32 = 0;
}
