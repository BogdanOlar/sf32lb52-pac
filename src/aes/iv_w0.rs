///Register `IV_W0` reader
pub type R = crate::R<IV_W0rs>;
///Register `IV_W0` writer
pub type W = crate::W<IV_W0rs>;
///Field `DATA` reader - Initial Vector Word0
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - Initial Vector Word0
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Initial Vector Word0
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_W0").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - Initial Vector Word0
    #[inline(always)]
    pub fn data(&mut self) -> DataW<IV_W0rs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`iv_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IV_W0rs;
impl crate::RegisterSpec for IV_W0rs {
    type Ux = u32;
}
///`read()` method returns [`iv_w0::R`](R) reader structure
impl crate::Readable for IV_W0rs {}
///`write(|w| ..)` method takes [`iv_w0::W`](W) writer structure
impl crate::Writable for IV_W0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IV_W0 to value 0
impl crate::Resettable for IV_W0rs {
    const RESET_VALUE: u32 = 0;
}
