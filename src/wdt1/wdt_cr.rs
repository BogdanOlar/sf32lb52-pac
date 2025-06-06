///Register `WDT_CR` reader
pub type R = crate::R<WDT_CRrs>;
///Register `WDT_CR` writer
pub type W = crate::W<WDT_CRrs>;
///Field `RESET_LENGTH` reader - reset pulse length in number of wdt clock cycles
pub type ResetLengthR = crate::FieldReader;
///Field `RESET_LENGTH` writer - reset pulse length in number of wdt clock cycles
pub type ResetLengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RESPONSE_MODE` reader - 0:reset only, 1:interrupt and reset
pub type ResponseModeR = crate::BitReader;
///Field `RESPONSE_MODE` writer - 0:reset only, 1:interrupt and reset
pub type ResponseModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - reset pulse length in number of wdt clock cycles
    #[inline(always)]
    pub fn reset_length(&self) -> ResetLengthR {
        ResetLengthR::new((self.bits & 7) as u8)
    }
    ///Bit 4 - 0:reset only, 1:interrupt and reset
    #[inline(always)]
    pub fn response_mode(&self) -> ResponseModeR {
        ResponseModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_CR")
            .field("response_mode", &self.response_mode())
            .field("reset_length", &self.reset_length())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - reset pulse length in number of wdt clock cycles
    #[inline(always)]
    pub fn reset_length(&mut self) -> ResetLengthW<WDT_CRrs> {
        ResetLengthW::new(self, 0)
    }
    ///Bit 4 - 0:reset only, 1:interrupt and reset
    #[inline(always)]
    pub fn response_mode(&mut self) -> ResponseModeW<WDT_CRrs> {
        ResponseModeW::new(self, 4)
    }
}
///WatchDog Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_CRrs;
impl crate::RegisterSpec for WDT_CRrs {
    type Ux = u32;
}
///`read()` method returns [`wdt_cr::R`](R) reader structure
impl crate::Readable for WDT_CRrs {}
///`write(|w| ..)` method takes [`wdt_cr::W`](W) writer structure
impl crate::Writable for WDT_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_CR to value 0
impl crate::Resettable for WDT_CRrs {
    const RESET_VALUE: u32 = 0;
}
