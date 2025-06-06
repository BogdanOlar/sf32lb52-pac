///Register `DOR1` reader
pub type R = crate::R<DOR1rs>;
///Register `DOR1` writer
pub type W = crate::W<DOR1rs>;
///Field `OUT` reader - GPIO\[44:32\]
///output value if output enabled
pub type OutR = crate::FieldReader<u16>;
///Field `OUT` writer - GPIO\[44:32\]
///output value if output enabled
pub type OutW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - GPIO\[44:32\]
    ///output value if output enabled
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOR1").field("out", &self.out()).finish()
    }
}
impl W {
    ///Bits 0:12 - GPIO\[44:32\]
    ///output value if output enabled
    #[inline(always)]
    pub fn out(&mut self) -> OutW<DOR1rs> {
        OutW::new(self, 0)
    }
}
///Data Output Register
///
///You can [`read`](crate::Reg::read) this register and get [`dor1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOR1rs;
impl crate::RegisterSpec for DOR1rs {
    type Ux = u32;
}
///`read()` method returns [`dor1::R`](R) reader structure
impl crate::Readable for DOR1rs {}
///`write(|w| ..)` method takes [`dor1::W`](W) writer structure
impl crate::Writable for DOR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOR1 to value 0
impl crate::Resettable for DOR1rs {
    const RESET_VALUE: u32 = 0;
}
