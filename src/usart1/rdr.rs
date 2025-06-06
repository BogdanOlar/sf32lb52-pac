///Register `RDR` reader
pub type R = crate::R<RDRrs>;
///Register `RDR` writer
pub type W = crate::W<RDRrs>;
///Field `RDR` reader - Received data
pub type RdrR = crate::FieldReader<u16>;
///Field `RDR` writer - Received data
pub type RdrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Received data
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDR").field("rdr", &self.rdr()).finish()
    }
}
impl W {
    ///Bits 0:8 - Received data
    #[inline(always)]
    pub fn rdr(&mut self) -> RdrW<RDRrs> {
        RdrW::new(self, 0)
    }
}
///Receive Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RDRrs;
impl crate::RegisterSpec for RDRrs {
    type Ux = u32;
}
///`read()` method returns [`rdr::R`](R) reader structure
impl crate::Readable for RDRrs {}
///`write(|w| ..)` method takes [`rdr::W`](W) writer structure
impl crate::Writable for RDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDRrs {
    const RESET_VALUE: u32 = 0;
}
