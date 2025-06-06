///Register `DOR0` reader
pub type R = crate::R<DOR0rs>;
///Register `DOR0` writer
pub type W = crate::W<DOR0rs>;
///Field `OUT` reader - GPIO\[31:0\]
///output value if output enabled
pub type OutR = crate::FieldReader<u32>;
///Field `OUT` writer - GPIO\[31:0\]
///output value if output enabled
pub type OutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO\[31:0\]
    ///output value if output enabled
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOR0").field("out", &self.out()).finish()
    }
}
impl W {
    ///Bits 0:31 - GPIO\[31:0\]
    ///output value if output enabled
    #[inline(always)]
    pub fn out(&mut self) -> OutW<DOR0rs> {
        OutW::new(self, 0)
    }
}
///Data Output Register
///
///You can [`read`](crate::Reg::read) this register and get [`dor0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOR0rs;
impl crate::RegisterSpec for DOR0rs {
    type Ux = u32;
}
///`read()` method returns [`dor0::R`](R) reader structure
impl crate::Readable for DOR0rs {}
///`write(|w| ..)` method takes [`dor0::W`](W) writer structure
impl crate::Writable for DOR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOR0 to value 0
impl crate::Resettable for DOR0rs {
    const RESET_VALUE: u32 = 0;
}
