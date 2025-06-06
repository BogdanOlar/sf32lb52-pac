///Register `TSEN_RDATA` reader
pub type R = crate::R<TSEN_RDATArs>;
///Register `TSEN_RDATA` writer
pub type W = crate::W<TSEN_RDATArs>;
///Field `TSEN_RDATA` reader -
pub type TsenRdataR = crate::FieldReader<u16>;
///Field `TSEN_RDATA` writer -
pub type TsenRdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn tsen_rdata(&self) -> TsenRdataR {
        TsenRdataR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSEN_RDATA")
            .field("rsvd", &self.rsvd())
            .field("tsen_rdata", &self.tsen_rdata())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn tsen_rdata(&mut self) -> TsenRdataW<TSEN_RDATArs> {
        TsenRdataW::new(self, 0)
    }
    ///Bits 12:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TSEN_RDATArs> {
        RsvdW::new(self, 12)
    }
}
///Tsen Read Data
///
///You can [`read`](crate::Reg::read) this register and get [`tsen_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TSEN_RDATArs;
impl crate::RegisterSpec for TSEN_RDATArs {
    type Ux = u32;
}
///`read()` method returns [`tsen_rdata::R`](R) reader structure
impl crate::Readable for TSEN_RDATArs {}
///`write(|w| ..)` method takes [`tsen_rdata::W`](W) writer structure
impl crate::Writable for TSEN_RDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSEN_RDATA to value 0
impl crate::Resettable for TSEN_RDATArs {
    const RESET_VALUE: u32 = 0;
}
