///Register `EXT_KEY_W6` reader
pub type R = crate::R<EXT_KEY_W6rs>;
///Register `EXT_KEY_W6` writer
pub type W = crate::W<EXT_KEY_W6rs>;
///Field `DATA` reader - External Key Word6
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - External Key Word6
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - External Key Word6
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_KEY_W6")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - External Key Word6
    #[inline(always)]
    pub fn data(&mut self) -> DataW<EXT_KEY_W6rs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ext_key_w6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_key_w6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EXT_KEY_W6rs;
impl crate::RegisterSpec for EXT_KEY_W6rs {
    type Ux = u32;
}
///`read()` method returns [`ext_key_w6::R`](R) reader structure
impl crate::Readable for EXT_KEY_W6rs {}
///`write(|w| ..)` method takes [`ext_key_w6::W`](W) writer structure
impl crate::Writable for EXT_KEY_W6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXT_KEY_W6 to value 0
impl crate::Resettable for EXT_KEY_W6rs {
    const RESET_VALUE: u32 = 0;
}
