///Register `CMP` reader
pub type R = crate::R<CMPrs>;
///Register `CMP` writer
pub type W = crate::W<CMPrs>;
///Field `CMP` reader - Compare value CMP is the compare value used by the LPTIM.
pub type CmpR = crate::FieldReader<u32>;
///Field `CMP` writer - Compare value CMP is the compare value used by the LPTIM.
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - Compare value CMP is the compare value used by the LPTIM.
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP")
            .field("rsvd", &self.rsvd())
            .field("cmp", &self.cmp())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Compare value CMP is the compare value used by the LPTIM.
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<CMPrs> {
        CmpW::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CMPrs> {
        RsvdW::new(self, 24)
    }
}
///LPTIM compare register
///
///You can [`read`](crate::Reg::read) this register and get [`cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMPrs;
impl crate::RegisterSpec for CMPrs {
    type Ux = u32;
}
///`read()` method returns [`cmp::R`](R) reader structure
impl crate::Readable for CMPrs {}
///`write(|w| ..)` method takes [`cmp::W`](W) writer structure
impl crate::Writable for CMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMP to value 0
impl crate::Resettable for CMPrs {
    const RESET_VALUE: u32 = 0;
}
