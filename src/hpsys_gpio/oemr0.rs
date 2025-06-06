///Register `OEMR0` reader
pub type R = crate::R<OEMR0rs>;
///Register `OEMR0` writer
pub type W = crate::W<OEMR0rs>;
///Field `OEM` reader - output mode of corresponding GPIO\[31:0\]
pub type OemR = crate::FieldReader<u32>;
///Field `OEM` writer - output mode of corresponding GPIO\[31:0\]
pub type OemW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - output mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn oem(&self) -> OemR {
        OemR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEMR0").field("oem", &self.oem()).finish()
    }
}
impl W {
    ///Bits 0:31 - output mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn oem(&mut self) -> OemW<OEMR0rs> {
        OemW::new(self, 0)
    }
}
///output mode Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OEMR0rs;
impl crate::RegisterSpec for OEMR0rs {
    type Ux = u32;
}
///`read()` method returns [`oemr0::R`](R) reader structure
impl crate::Readable for OEMR0rs {}
///`write(|w| ..)` method takes [`oemr0::W`](W) writer structure
impl crate::Writable for OEMR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OEMR0 to value 0
impl crate::Resettable for OEMR0rs {
    const RESET_VALUE: u32 = 0;
}
