///Register `TAR2` reader
pub type R = crate::R<TAR2rs>;
///Register `TAR2` writer
pub type W = crate::W<TAR2rs>;
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
        f.debug_struct("TAR2").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address to access to
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<TAR2rs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TAR2rs;
impl crate::RegisterSpec for TAR2rs {
    type Ux = u32;
}
///`read()` method returns [`tar2::R`](R) reader structure
impl crate::Readable for TAR2rs {}
///`write(|w| ..)` method takes [`tar2::W`](W) writer structure
impl crate::Writable for TAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAR2 to value 0
impl crate::Resettable for TAR2rs {
    const RESET_VALUE: u32 = 0;
}
