///Register `RTC_DR` reader
pub type R = crate::R<RTC_DRrs>;
///Register `RTC_DR` writer
pub type W = crate::W<RTC_DRrs>;
///Field `DU` reader - Date units in BCD format
pub type DuR = crate::FieldReader;
///Field `DU` writer - Date units in BCD format
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DT` reader - Date tens in BCD format
pub type DtR = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MU` reader - Month units in BCD format
pub type MuR = crate::FieldReader;
///Field `MU` writer - Month units in BCD format
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MT` reader - Month tens in BCD format
pub type MtR = crate::BitReader;
///Field `MT` writer - Month tens in BCD format
pub type MtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WD` reader - Week day units 000: forbidden 001: Monday ... 111: Sunday
pub type WdR = crate::FieldReader;
///Field `WD` writer - Week day units 000: forbidden 001: Monday ... 111: Sunday
pub type WdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `YU` reader - Year units in BCD format
pub type YuR = crate::FieldReader;
///Field `YU` writer - Year units in BCD format
pub type YuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `YT` reader - Year tens in BCD format
pub type YtR = crate::FieldReader;
///Field `YT` writer - Year tens in BCD format
pub type YtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CB` reader - Century flag
pub type CbR = crate::BitReader;
///Field `CB` writer - Century flag
pub type CbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR` reader - reserved for debug
pub type ErrR = crate::BitReader;
///Field `ERR` writer - reserved for debug
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Week day units 000: forbidden 001: Monday ... 111: Sunday
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&self) -> YuR {
        YuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&self) -> YtR {
        YtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Century flag
    #[inline(always)]
    pub fn cb(&self) -> CbR {
        CbR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 31 - reserved for debug
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_DR")
            .field("err", &self.err())
            .field("cb", &self.cb())
            .field("yt", &self.yt())
            .field("yu", &self.yu())
            .field("wd", &self.wd())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DuW<RTC_DRrs> {
        DuW::new(self, 0)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<RTC_DRrs> {
        DtW::new(self, 4)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&mut self) -> MuW<RTC_DRrs> {
        MuW::new(self, 8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&mut self) -> MtW<RTC_DRrs> {
        MtW::new(self, 12)
    }
    ///Bits 13:15 - Week day units 000: forbidden 001: Monday ... 111: Sunday
    #[inline(always)]
    pub fn wd(&mut self) -> WdW<RTC_DRrs> {
        WdW::new(self, 13)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&mut self) -> YuW<RTC_DRrs> {
        YuW::new(self, 16)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&mut self) -> YtW<RTC_DRrs> {
        YtW::new(self, 20)
    }
    ///Bit 24 - Century flag
    #[inline(always)]
    pub fn cb(&mut self) -> CbW<RTC_DRrs> {
        CbW::new(self, 24)
    }
    ///Bit 31 - reserved for debug
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<RTC_DRrs> {
        ErrW::new(self, 31)
    }
}
///Mirrored RTC Date Register
///
///You can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RTC_DRrs;
impl crate::RegisterSpec for RTC_DRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_dr::R`](R) reader structure
impl crate::Readable for RTC_DRrs {}
///`write(|w| ..)` method takes [`rtc_dr::W`](W) writer structure
impl crate::Writable for RTC_DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_DR to value 0
impl crate::Resettable for RTC_DRrs {
    const RESET_VALUE: u32 = 0;
}
