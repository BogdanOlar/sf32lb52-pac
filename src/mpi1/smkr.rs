///Register `SMKR` reader
pub type R = crate::R<SMKRrs>;
///Register `SMKR` writer
pub type W = crate::W<SMKRrs>;
///Field `MASK` reader - Status mask 0: the corresponding bit is not considered to compare 1: the corresponding bit is considered to compare
pub type MaskR = crate::FieldReader<u32>;
///Field `MASK` writer - Status mask 0: the corresponding bit is not considered to compare 1: the corresponding bit is considered to compare
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Status mask 0: the corresponding bit is not considered to compare 1: the corresponding bit is considered to compare
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMKR").field("mask", &self.mask()).finish()
    }
}
impl W {
    ///Bits 0:31 - Status mask 0: the corresponding bit is not considered to compare 1: the corresponding bit is considered to compare
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<SMKRrs> {
        MaskW::new(self, 0)
    }
}
///Status Mask Register
///
///You can [`read`](crate::Reg::read) this register and get [`smkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SMKRrs;
impl crate::RegisterSpec for SMKRrs {
    type Ux = u32;
}
///`read()` method returns [`smkr::R`](R) reader structure
impl crate::Readable for SMKRrs {}
///`write(|w| ..)` method takes [`smkr::W`](W) writer structure
impl crate::Writable for SMKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMKR to value 0
impl crate::Resettable for SMKRrs {
    const RESET_VALUE: u32 = 0;
}
