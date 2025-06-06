///Register `V_SRC` reader
pub type R = crate::R<V_SRCrs>;
///Register `V_SRC` writer
pub type W = crate::W<V_SRCrs>;
///Field `ADDR` reader - v vector address
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - v vector address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - v vector address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("V_SRC").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:31 - v vector address
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<V_SRCrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`v_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`v_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct V_SRCrs;
impl crate::RegisterSpec for V_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`v_src::R`](R) reader structure
impl crate::Readable for V_SRCrs {}
///`write(|w| ..)` method takes [`v_src::W`](W) writer structure
impl crate::Writable for V_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets V_SRC to value 0
impl crate::Resettable for V_SRCrs {
    const RESET_VALUE: u32 = 0;
}
