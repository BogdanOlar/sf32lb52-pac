///Register `TIMR` reader
pub type R = crate::R<TIMRrs>;
///Register `TIMR` writer
pub type W = crate::W<TIMRrs>;
///Field `THRCK` reader - SCLK to CSB hold time into READ mode. Recmmended value > 500ns
pub type ThrckR = crate::FieldReader;
///Field `THRCK` writer - SCLK to CSB hold time into READ mode. Recmmended value > 500ns
pub type ThrckW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `THPCK` reader - SCLK to CSB hold time into PGM mode. Recommended value > 20ns
pub type ThpckR = crate::FieldReader;
///Field `THPCK` writer - SCLK to CSB hold time into PGM mode. Recommended value > 20ns
pub type ThpckW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TCKHP` reader - SCLK high period for PGM. Recommended value ~10us
pub type TckhpR = crate::FieldReader<u16>;
///Field `TCKHP` writer - SCLK high period for PGM. Recommended value ~10us
pub type TckhpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:6 - SCLK to CSB hold time into READ mode. Recmmended value > 500ns
    #[inline(always)]
    pub fn thrck(&self) -> ThrckR {
        ThrckR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:9 - SCLK to CSB hold time into PGM mode. Recommended value > 20ns
    #[inline(always)]
    pub fn thpck(&self) -> ThpckR {
        ThpckR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:20 - SCLK high period for PGM. Recommended value ~10us
    #[inline(always)]
    pub fn tckhp(&self) -> TckhpR {
        TckhpR::new(((self.bits >> 10) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMR")
            .field("tckhp", &self.tckhp())
            .field("thpck", &self.thpck())
            .field("thrck", &self.thrck())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - SCLK to CSB hold time into READ mode. Recmmended value > 500ns
    #[inline(always)]
    pub fn thrck(&mut self) -> ThrckW<TIMRrs> {
        ThrckW::new(self, 0)
    }
    ///Bits 7:9 - SCLK to CSB hold time into PGM mode. Recommended value > 20ns
    #[inline(always)]
    pub fn thpck(&mut self) -> ThpckW<TIMRrs> {
        ThpckW::new(self, 7)
    }
    ///Bits 10:20 - SCLK high period for PGM. Recommended value ~10us
    #[inline(always)]
    pub fn tckhp(&mut self) -> TckhpW<TIMRrs> {
        TckhpW::new(self, 10)
    }
}
///Timer Register
///
///You can [`read`](crate::Reg::read) this register and get [`timr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TIMRrs;
impl crate::RegisterSpec for TIMRrs {
    type Ux = u32;
}
///`read()` method returns [`timr::R`](R) reader structure
impl crate::Readable for TIMRrs {}
///`write(|w| ..)` method takes [`timr::W`](W) writer structure
impl crate::Writable for TIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMR to value 0
impl crate::Resettable for TIMRrs {
    const RESET_VALUE: u32 = 0;
}
