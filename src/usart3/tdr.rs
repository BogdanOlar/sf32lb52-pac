///Register `TDR` reader
pub type R = crate::R<TDRrs>;
///Register `TDR` writer
pub type W = crate::W<TDRrs>;
///Field `TDR` reader - Transmit data
pub type TdrR = crate::FieldReader<u16>;
///Field `TDR` writer - Transmit data
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:8 - Transmit data
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDR")
            .field("rsvd", &self.rsvd())
            .field("tdr", &self.tdr())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Transmit data
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<TDRrs> {
        TdrW::new(self, 0)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TDRrs> {
        RsvdW::new(self, 9)
    }
}
///Transmit Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
///`read()` method returns [`tdr::R`](R) reader structure
impl crate::Readable for TDRrs {}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TDR to value 0
impl crate::Resettable for TDRrs {
    const RESET_VALUE: u32 = 0;
}
