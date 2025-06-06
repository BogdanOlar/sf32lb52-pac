///Register `SRCAR` reader
pub type R = crate::R<SRCARrs>;
///Register `SRCAR` writer
pub type W = crate::W<SRCARrs>;
///Field `SRCADDR` reader - source address It contains the base address of the source data to be read. Should be word aligned
pub type SrcaddrR = crate::FieldReader<u32>;
///Field `SRCADDR` writer - source address It contains the base address of the source data to be read. Should be word aligned
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - source address It contains the base address of the source data to be read. Should be word aligned
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRCAR")
            .field("srcaddr", &self.srcaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - source address It contains the base address of the source data to be read. Should be word aligned
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SrcaddrW<SRCARrs> {
        SrcaddrW::new(self, 0)
    }
}
///source address register
///
///You can [`read`](crate::Reg::read) this register and get [`srcar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRCARrs;
impl crate::RegisterSpec for SRCARrs {
    type Ux = u32;
}
///`read()` method returns [`srcar::R`](R) reader structure
impl crate::Readable for SRCARrs {}
///`write(|w| ..)` method takes [`srcar::W`](W) writer structure
impl crate::Writable for SRCARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRCAR to value 0
impl crate::Resettable for SRCARrs {
    const RESET_VALUE: u32 = 0;
}
