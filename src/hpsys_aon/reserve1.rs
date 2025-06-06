///Register `RESERVE1` reader
pub type R = crate::R<RESERVE1rs>;
///Register `RESERVE1` writer
pub type W = crate::W<RESERVE1rs>;
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
        f.debug_struct("RESERVE1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - for debug only
    #[inline(always)]
    pub fn data(&mut self) -> DataW<RESERVE1rs> {
        DataW::new(self, 0)
    }
}
///Reserved Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`reserve1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserve1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVE1rs;
impl crate::RegisterSpec for RESERVE1rs {
    type Ux = u32;
}
///`read()` method returns [`reserve1::R`](R) reader structure
impl crate::Readable for RESERVE1rs {}
///`write(|w| ..)` method takes [`reserve1::W`](W) writer structure
impl crate::Writable for RESERVE1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVE1 to value 0
impl crate::Resettable for RESERVE1rs {
    const RESET_VALUE: u32 = 0;
}
