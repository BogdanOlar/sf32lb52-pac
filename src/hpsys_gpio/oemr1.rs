///Register `OEMR1` reader
pub type R = crate::R<OEMR1rs>;
///Register `OEMR1` writer
pub type W = crate::W<OEMR1rs>;
///Field `OEM` reader - output mode of corresponding GPIO\[44:32\]
pub type OemR = crate::FieldReader<u16>;
///Field `OEM` writer - output mode of corresponding GPIO\[44:32\]
pub type OemW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - output mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn oem(&self) -> OemR {
        OemR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEMR1")
            .field("rsvd", &self.rsvd())
            .field("oem", &self.oem())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - output mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn oem(&mut self) -> OemW<OEMR1rs> {
        OemW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<OEMR1rs> {
        RsvdW::new(self, 13)
    }
}
///output mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OEMR1rs;
impl crate::RegisterSpec for OEMR1rs {
    type Ux = u32;
}
///`read()` method returns [`oemr1::R`](R) reader structure
impl crate::Readable for OEMR1rs {}
///`write(|w| ..)` method takes [`oemr1::W`](W) writer structure
impl crate::Writable for OEMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OEMR1 to value 0
impl crate::Resettable for OEMR1rs {
    const RESET_VALUE: u32 = 0;
}
