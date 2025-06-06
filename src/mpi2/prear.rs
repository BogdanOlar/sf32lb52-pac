///Register `PREAR` reader
pub type R = crate::R<PREARrs>;
///Register `PREAR` writer
pub type W = crate::W<PREARrs>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `EA` reader - Ending address of the prefetch area
pub type EaR = crate::FieldReader<u32>;
///Field `EA` writer - Ending address of the prefetch area
pub type EaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:31 - Ending address of the prefetch area
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PREAR")
            .field("ea", &self.ea())
            .field("rsvd", &self.rsvd())
            .finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PREARrs> {
        RsvdW::new(self, 0)
    }
    ///Bits 10:31 - Ending address of the prefetch area
    #[inline(always)]
    pub fn ea(&mut self) -> EaW<PREARrs> {
        EaW::new(self, 10)
    }
}
///Prefetch Ending Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`prear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PREARrs;
impl crate::RegisterSpec for PREARrs {
    type Ux = u32;
}
///`read()` method returns [`prear::R`](R) reader structure
impl crate::Readable for PREARrs {}
///`write(|w| ..)` method takes [`prear::W`](W) writer structure
impl crate::Writable for PREARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PREAR to value 0
impl crate::Resettable for PREARrs {
    const RESET_VALUE: u32 = 0;
}
