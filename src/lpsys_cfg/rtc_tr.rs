///Register `RTC_TR` reader
pub type R = crate::R<RTC_TRrs>;
///Register `RTC_TR` writer
pub type W = crate::W<RTC_TRrs>;
///Field `SS` reader - Sub-second counter
pub type SsR = crate::FieldReader<u16>;
///Field `SS` writer - Sub-second counter
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `SU` reader - Second units in BCD format
pub type SuR = crate::FieldReader;
///Field `SU` writer - Second units in BCD format
pub type SuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ST` reader - Second tens in BCD format
pub type StR = crate::FieldReader;
///Field `ST` writer - Second tens in BCD format
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MNU` reader - Minute units in BCD format
pub type MnuR = crate::FieldReader;
///Field `MNU` writer - Minute units in BCD format
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MNT` reader - Minute tens in BCD format
pub type MntR = crate::FieldReader;
///Field `MNT` writer - Minute tens in BCD format
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HU` reader - Hour units in BCD format
pub type HuR = crate::FieldReader;
///Field `HU` writer - Hour units in BCD format
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HT` reader - Hour tens in BCD format
pub type HtR = crate::FieldReader;
///Field `HT` writer - Hour tens in BCD format
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PM` reader - AM/PM notation 0: AM 1: PM
pub type PmR = crate::BitReader;
///Field `PM` writer - AM/PM notation 0: AM 1: PM
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - Sub-second counter
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 11:14 - Second units in BCD format
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bits 15:17 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:21 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:24 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:28 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bits 29:30 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - AM/PM notation 0: AM 1: PM
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_TR")
            .field("pm", &self.pm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mnt", &self.mnt())
            .field("mnu", &self.mnu())
            .field("st", &self.st())
            .field("su", &self.su())
            .field("ss", &self.ss())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Sub-second counter
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<RTC_TRrs> {
        SsW::new(self, 0)
    }
    ///Bits 11:14 - Second units in BCD format
    #[inline(always)]
    pub fn su(&mut self) -> SuW<RTC_TRrs> {
        SuW::new(self, 11)
    }
    ///Bits 15:17 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&mut self) -> StW<RTC_TRrs> {
        StW::new(self, 15)
    }
    ///Bits 18:21 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&mut self) -> MnuW<RTC_TRrs> {
        MnuW::new(self, 18)
    }
    ///Bits 22:24 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&mut self) -> MntW<RTC_TRrs> {
        MntW::new(self, 22)
    }
    ///Bits 25:28 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&mut self) -> HuW<RTC_TRrs> {
        HuW::new(self, 25)
    }
    ///Bits 29:30 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&mut self) -> HtW<RTC_TRrs> {
        HtW::new(self, 29)
    }
    ///Bit 31 - AM/PM notation 0: AM 1: PM
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<RTC_TRrs> {
        PmW::new(self, 31)
    }
}
///Mirrored RTC Time Register
///
///You can [`read`](crate::Reg::read) this register and get [`rtc_tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RTC_TRrs;
impl crate::RegisterSpec for RTC_TRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_tr::R`](R) reader structure
impl crate::Readable for RTC_TRrs {}
///`write(|w| ..)` method takes [`rtc_tr::W`](W) writer structure
impl crate::Writable for RTC_TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_TR to value 0
impl crate::Resettable for RTC_TRrs {
    const RESET_VALUE: u32 = 0;
}
