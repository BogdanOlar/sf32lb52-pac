///Register `TAR6` reader
pub type R = crate::R<TAR6rs>;
///Register `TAR6` writer
pub type W = crate::W<TAR6rs>;
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
        f.debug_struct("TAR6").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address to access to
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<TAR6rs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tar6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TAR6rs;
impl crate::RegisterSpec for TAR6rs {
    type Ux = u32;
}
///`read()` method returns [`tar6::R`](R) reader structure
impl crate::Readable for TAR6rs {}
///`write(|w| ..)` method takes [`tar6::W`](W) writer structure
impl crate::Writable for TAR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAR6 to value 0
impl crate::Resettable for TAR6rs {
    const RESET_VALUE: u32 = 0;
}
