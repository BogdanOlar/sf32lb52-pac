///Register `SMR` reader
pub type R = crate::R<SMRrs>;
///Register `SMR` writer
pub type W = crate::W<SMRrs>;
///Field `STATUS` reader - If status match is enabled, this register is compared with the data read from external memory. Together with SMKR, only the bits with mask=1 will be considered to compare in AND or OR mode as configured in SMM field.
pub type StatusR = crate::FieldReader<u32>;
///Field `STATUS` writer - If status match is enabled, this register is compared with the data read from external memory. Together with SMKR, only the bits with mask=1 will be considered to compare in AND or OR mode as configured in SMM field.
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - If status match is enabled, this register is compared with the data read from external memory. Together with SMKR, only the bits with mask=1 will be considered to compare in AND or OR mode as configured in SMM field.
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMR")
            .field("status", &self.status())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - If status match is enabled, this register is compared with the data read from external memory. Together with SMKR, only the bits with mask=1 will be considered to compare in AND or OR mode as configured in SMM field.
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<SMRrs> {
        StatusW::new(self, 0)
    }
}
///Status Match Register
///
///You can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SMRrs;
impl crate::RegisterSpec for SMRrs {
    type Ux = u32;
}
///`read()` method returns [`smr::R`](R) reader structure
impl crate::Readable for SMRrs {}
///`write(|w| ..)` method takes [`smr::W`](W) writer structure
impl crate::Writable for SMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMR to value 0
impl crate::Resettable for SMRrs {
    const RESET_VALUE: u32 = 0;
}
