///Register `DOER0` reader
pub type R = crate::R<DOER0rs>;
///Register `DOER0` writer
pub type W = crate::W<DOER0rs>;
///Field `DOE` reader - GPIO\[31:0\]
///output enable
pub type DoeR = crate::FieldReader<u32>;
///Field `DOE` writer - GPIO\[31:0\]
///output enable
pub type DoeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO\[31:0\]
    ///output enable
    #[inline(always)]
    pub fn doe(&self) -> DoeR {
        DoeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOER0").field("doe", &self.doe()).finish()
    }
}
impl W {
    ///Bits 0:31 - GPIO\[31:0\]
    ///output enable
    #[inline(always)]
    pub fn doe(&mut self) -> DoeW<DOER0rs> {
        DoeW::new(self, 0)
    }
}
///Data Output Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`doer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOER0rs;
impl crate::RegisterSpec for DOER0rs {
    type Ux = u32;
}
///`read()` method returns [`doer0::R`](R) reader structure
impl crate::Readable for DOER0rs {}
///`write(|w| ..)` method takes [`doer0::W`](W) writer structure
impl crate::Writable for DOER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOER0 to value 0
impl crate::Resettable for DOER0rs {
    const RESET_VALUE: u32 = 0;
}
