///Register `DIR0` reader
pub type R = crate::R<DIR0rs>;
///Register `DIR0` writer
pub type W = crate::W<DIR0rs>;
///Field `IN` reader - GPIO\[31:0\]
///input value
pub type InR = crate::FieldReader<u32>;
///Field `IN` writer - GPIO\[31:0\]
///input value
pub type InW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO\[31:0\]
    ///input value
    #[inline(always)]
    pub fn in_(&self) -> InR {
        InR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR0").field("in_", &self.in_()).finish()
    }
}
impl W {
    ///Bits 0:31 - GPIO\[31:0\]
    ///input value
    #[inline(always)]
    pub fn in_(&mut self) -> InW<DIR0rs> {
        InW::new(self, 0)
    }
}
///Data Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`dir0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DIR0rs;
impl crate::RegisterSpec for DIR0rs {
    type Ux = u32;
}
///`read()` method returns [`dir0::R`](R) reader structure
impl crate::Readable for DIR0rs {}
///`write(|w| ..)` method takes [`dir0::W`](W) writer structure
impl crate::Writable for DIR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIR0 to value 0
impl crate::Resettable for DIR0rs {
    const RESET_VALUE: u32 = 0;
}
