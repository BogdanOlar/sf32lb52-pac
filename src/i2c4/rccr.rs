///Register `RCCR` reader
pub type R = crate::R<RCCRrs>;
///Register `RCCR` writer
pub type W = crate::W<RCCRrs>;
///Field `RSTCYC` reader - The cycles of SCL during bus reset
pub type RstcycR = crate::FieldReader;
///Field `RSTCYC` writer - The cycles of SCL during bus reset
pub type RstcycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:3 - The cycles of SCL during bus reset
    #[inline(always)]
    pub fn rstcyc(&self) -> RstcycR {
        RstcycR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCCR")
            .field("rsvd", &self.rsvd())
            .field("rstcyc", &self.rstcyc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - The cycles of SCL during bus reset
    #[inline(always)]
    pub fn rstcyc(&mut self) -> RstcycW<RCCRrs> {
        RstcycW::new(self, 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RCCRrs> {
        RsvdW::new(self, 4)
    }
}
///Bus Reset Cycle Counter Register
///
///You can [`read`](crate::Reg::read) this register and get [`rccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RCCRrs;
impl crate::RegisterSpec for RCCRrs {
    type Ux = u32;
}
///`read()` method returns [`rccr::R`](R) reader structure
impl crate::Readable for RCCRrs {}
///`write(|w| ..)` method takes [`rccr::W`](W) writer structure
impl crate::Writable for RCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCCR to value 0x09
impl crate::Resettable for RCCRrs {
    const RESET_VALUE: u32 = 0x09;
}
