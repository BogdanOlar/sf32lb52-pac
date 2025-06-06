///Register `TAR8` reader
pub type R = crate::R<TAR8rs>;
///Register `TAR8` writer
pub type W = crate::W<TAR8rs>;
///Field `ADDR` reader - peripheral address to access to
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - peripheral address to access to
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - peripheral address to access to
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAR8").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address to access to
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<TAR8rs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tar8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TAR8rs;
impl crate::RegisterSpec for TAR8rs {
    type Ux = u32;
}
///`read()` method returns [`tar8::R`](R) reader structure
impl crate::Readable for TAR8rs {}
///`write(|w| ..)` method takes [`tar8::W`](W) writer structure
impl crate::Writable for TAR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAR8 to value 0
impl crate::Resettable for TAR8rs {
    const RESET_VALUE: u32 = 0;
}
