///Register `RIR` reader
pub type R = crate::R<RIRrs>;
///Register `RIR` writer
pub type W = crate::W<RIRrs>;
///Field `RSP_INDEX` reader - Response command index
pub type RspIndexR = crate::FieldReader;
///Field `RSP_INDEX` writer - Response command index
pub type RspIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:5 - Response command index
    #[inline(always)]
    pub fn rsp_index(&self) -> RspIndexR {
        RspIndexR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIR")
            .field("rsvd", &self.rsvd())
            .field("rsp_index", &self.rsp_index())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Response command index
    #[inline(always)]
    pub fn rsp_index(&mut self) -> RspIndexW<RIRrs> {
        RspIndexW::new(self, 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RIRrs> {
        RsvdW::new(self, 6)
    }
}
///response command index register
///
///You can [`read`](crate::Reg::read) this register and get [`rir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RIRrs;
impl crate::RegisterSpec for RIRrs {
    type Ux = u32;
}
///`read()` method returns [`rir::R`](R) reader structure
impl crate::Readable for RIRrs {}
///`write(|w| ..)` method takes [`rir::W`](W) writer structure
impl crate::Writable for RIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RIR to value 0
impl crate::Resettable for RIRrs {
    const RESET_VALUE: u32 = 0;
}
