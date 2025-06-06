///Register `DNR` reader
pub type R = crate::R<DNRrs>;
///Register `DNR` writer
pub type W = crate::W<DNRrs>;
///Field `NDT` reader - Write as number of data to transfer in byte. Read as left data number to transfer
pub type NdtR = crate::FieldReader<u16>;
///Field `NDT` writer - Write as number of data to transfer in byte. Read as left data number to transfer
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Write as number of data to transfer in byte. Read as left data number to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DNR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:8 - Write as number of data to transfer in byte. Read as left data number to transfer
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<DNRrs> {
        NdtW::new(self, 0)
    }
}
///DMA number register
///
///You can [`read`](crate::Reg::read) this register and get [`dnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DNRrs;
impl crate::RegisterSpec for DNRrs {
    type Ux = u32;
}
///`read()` method returns [`dnr::R`](R) reader structure
impl crate::Readable for DNRrs {}
///`write(|w| ..)` method takes [`dnr::W`](W) writer structure
impl crate::Writable for DNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DNR to value 0
impl crate::Resettable for DNRrs {
    const RESET_VALUE: u32 = 0;
}
