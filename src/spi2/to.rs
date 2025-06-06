///Register `TO` reader
pub type R = crate::R<TOrs>;
///Register `TO` writer
pub type W = crate::W<TOrs>;
///Field `TIMEOUT` reader - Timeout Value TIMEOUT value is the value (0 to 2^24-1) that defines the time-out interval. The time-out interval is given by the equation shown in the TIMEOUT Interval Equation.
pub type TimeoutR = crate::FieldReader<u32>;
///Field `TIMEOUT` writer - Timeout Value TIMEOUT value is the value (0 to 2^24-1) that defines the time-out interval. The time-out interval is given by the equation shown in the TIMEOUT Interval Equation.
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - Timeout Value TIMEOUT value is the value (0 to 2^24-1) that defines the time-out interval. The time-out interval is given by the equation shown in the TIMEOUT Interval Equation.
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field("rsvd", &self.rsvd())
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Timeout Value TIMEOUT value is the value (0 to 2^24-1) that defines the time-out interval. The time-out interval is given by the equation shown in the TIMEOUT Interval Equation.
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<TOrs> {
        TimeoutW::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TOrs> {
        RsvdW::new(self, 24)
    }
}
///SPI Time Out Register
///
///You can [`read`](crate::Reg::read) this register and get [`to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TOrs;
impl crate::RegisterSpec for TOrs {
    type Ux = u32;
}
///`read()` method returns [`to::R`](R) reader structure
impl crate::Readable for TOrs {}
///`write(|w| ..)` method takes [`to::W`](W) writer structure
impl crate::Writable for TOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TO to value 0
impl crate::Resettable for TOrs {
    const RESET_VALUE: u32 = 0;
}
