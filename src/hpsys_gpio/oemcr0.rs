///Register `OEMCR0` reader
pub type R = crate::R<OEMCR0rs>;
///Register `OEMCR0` writer
pub type W = crate::W<OEMCR0rs>;
///Field `OEMC` reader - output mode Clear of corresponding GPIO\[31:0\]
pub type OemcR = crate::FieldReader<u32>;
///Field `OEMC` writer - output mode Clear of corresponding GPIO\[31:0\]
pub type OemcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - output mode Clear of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn oemc(&self) -> OemcR {
        OemcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEMCR0")
            .field("oemc", &self.oemc())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - output mode Clear of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn oemc(&mut self) -> OemcW<OEMCR0rs> {
        OemcW::new(self, 0)
    }
}
///output mode Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OEMCR0rs;
impl crate::RegisterSpec for OEMCR0rs {
    type Ux = u32;
}
///`read()` method returns [`oemcr0::R`](R) reader structure
impl crate::Readable for OEMCR0rs {}
///`write(|w| ..)` method takes [`oemcr0::W`](W) writer structure
impl crate::Writable for OEMCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OEMCR0 to value 0
impl crate::Resettable for OEMCR0rs {
    const RESET_VALUE: u32 = 0;
}
