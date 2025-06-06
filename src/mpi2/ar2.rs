///Register `AR2` reader
pub type R = crate::R<AR2rs>;
///Register `AR2` writer
pub type W = crate::W<AR2rs>;
///Field `ADDR` reader - Address byte in CMD2 sequence
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - Address byte in CMD2 sequence
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Address byte in CMD2 sequence
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR2").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Address byte in CMD2 sequence
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<AR2rs> {
        AddrW::new(self, 0)
    }
}
///Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AR2rs;
impl crate::RegisterSpec for AR2rs {
    type Ux = u32;
}
///`read()` method returns [`ar2::R`](R) reader structure
impl crate::Readable for AR2rs {}
///`write(|w| ..)` method takes [`ar2::W`](W) writer structure
impl crate::Writable for AR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AR2 to value 0
impl crate::Resettable for AR2rs {
    const RESET_VALUE: u32 = 0;
}
