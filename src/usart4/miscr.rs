///Register `MISCR` reader
pub type R = crate::R<MISCRrs>;
///Register `MISCR` writer
pub type W = crate::W<MISCRrs>;
///Field `SMPLINI` reader - initial sample count, count down from this value to zero to reach the middle of the start bit in RxReserved-Do not modify
pub type SmpliniR = crate::FieldReader;
///Field `SMPLINI` writer - initial sample count, count down from this value to zero to reach the middle of the start bit in RxReserved-Do not modify
pub type SmpliniW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RTSBIT` reader - assert RTS ahead of the frame completion (in number of bits)Reserved-Do not modify
pub type RtsbitR = crate::FieldReader;
///Field `RTSBIT` writer - assert RTS ahead of the frame completion (in number of bits)Reserved-Do not modify
pub type RtsbitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AUTOCAL` reader -
pub type AutocalR = crate::BitReader;
///Field `AUTOCAL` writer -
pub type AutocalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - initial sample count, count down from this value to zero to reach the middle of the start bit in RxReserved-Do not modify
    #[inline(always)]
    pub fn smplini(&self) -> SmpliniR {
        SmpliniR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - assert RTS ahead of the frame completion (in number of bits)Reserved-Do not modify
    #[inline(always)]
    pub fn rtsbit(&self) -> RtsbitR {
        RtsbitR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn autocal(&self) -> AutocalR {
        AutocalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISCR")
            .field("autocal", &self.autocal())
            .field("rtsbit", &self.rtsbit())
            .field("smplini", &self.smplini())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - initial sample count, count down from this value to zero to reach the middle of the start bit in RxReserved-Do not modify
    #[inline(always)]
    pub fn smplini(&mut self) -> SmpliniW<MISCRrs> {
        SmpliniW::new(self, 0)
    }
    ///Bits 4:7 - assert RTS ahead of the frame completion (in number of bits)Reserved-Do not modify
    #[inline(always)]
    pub fn rtsbit(&mut self) -> RtsbitW<MISCRrs> {
        RtsbitW::new(self, 4)
    }
    ///Bit 31
    #[inline(always)]
    pub fn autocal(&mut self) -> AutocalW<MISCRrs> {
        AutocalW::new(self, 31)
    }
}
///Miscellaneous Register
///
///You can [`read`](crate::Reg::read) this register and get [`miscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MISCRrs;
impl crate::RegisterSpec for MISCRrs {
    type Ux = u32;
}
///`read()` method returns [`miscr::R`](R) reader structure
impl crate::Readable for MISCRrs {}
///`write(|w| ..)` method takes [`miscr::W`](W) writer structure
impl crate::Writable for MISCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MISCR to value 0
impl crate::Resettable for MISCRrs {
    const RESET_VALUE: u32 = 0;
}
