///Register `DLR2` reader
pub type R = crate::R<DLR2rs>;
///Register `DLR2` writer
pub type W = crate::W<DLR2rs>;
///Field `DLEN` reader - Data length in CMD2 sequence
pub type DlenR = crate::FieldReader<u32>;
///Field `DLEN` writer - Data length in CMD2 sequence
pub type DlenW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Data length in CMD2 sequence
    #[inline(always)]
    pub fn dlen(&self) -> DlenR {
        DlenR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLR2").field("dlen", &self.dlen()).finish()
    }
}
impl W {
    ///Bits 0:19 - Data length in CMD2 sequence
    #[inline(always)]
    pub fn dlen(&mut self) -> DlenW<DLR2rs> {
        DlenW::new(self, 0)
    }
}
///Data Length Register
///
///You can [`read`](crate::Reg::read) this register and get [`dlr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DLR2rs;
impl crate::RegisterSpec for DLR2rs {
    type Ux = u32;
}
///`read()` method returns [`dlr2::R`](R) reader structure
impl crate::Readable for DLR2rs {}
///`write(|w| ..)` method takes [`dlr2::W`](W) writer structure
impl crate::Writable for DLR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DLR2 to value 0
impl crate::Resettable for DLR2rs {
    const RESET_VALUE: u32 = 0;
}
