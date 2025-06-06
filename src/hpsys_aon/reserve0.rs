///Register `RESERVE0` reader
pub type R = crate::R<RESERVE0rs>;
///Register `RESERVE0` writer
pub type W = crate::W<RESERVE0rs>;
///Field `DATA` reader - for debug only
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - for debug only
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - for debug only
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVE0")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - for debug only
    #[inline(always)]
    pub fn data(&mut self) -> DataW<RESERVE0rs> {
        DataW::new(self, 0)
    }
}
///Reserved Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`reserve0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserve0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVE0rs;
impl crate::RegisterSpec for RESERVE0rs {
    type Ux = u32;
}
///`read()` method returns [`reserve0::R`](R) reader structure
impl crate::Readable for RESERVE0rs {}
///`write(|w| ..)` method takes [`reserve0::W`](W) writer structure
impl crate::Writable for RESERVE0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVE0 to value 0
impl crate::Resettable for RESERVE0rs {
    const RESET_VALUE: u32 = 0;
}
