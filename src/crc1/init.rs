///Register `INIT` reader
pub type R = crate::R<INITrs>;
///Register `INIT` writer
pub type W = crate::W<INITrs>;
///Field `INIT` reader - Programmable initial CRC value
pub type InitR = crate::FieldReader<u32>;
///Field `INIT` writer - Programmable initial CRC value
pub type InitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT").field("init", &self.init()).finish()
    }
}
impl W {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn init(&mut self) -> InitW<INITrs> {
        InitW::new(self, 0)
    }
}
///Initial CRC value
///
///You can [`read`](crate::Reg::read) this register and get [`init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INITrs;
impl crate::RegisterSpec for INITrs {
    type Ux = u32;
}
///`read()` method returns [`init::R`](R) reader structure
impl crate::Readable for INITrs {}
///`write(|w| ..)` method takes [`init::W`](W) writer structure
impl crate::Writable for INITrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INIT to value 0
impl crate::Resettable for INITrs {
    const RESET_VALUE: u32 = 0;
}
