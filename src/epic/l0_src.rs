///Register `L0_SRC` reader
pub type R = crate::R<L0_SRCrs>;
///Register `L0_SRC` writer
pub type W = crate::W<L0_SRCrs>;
///Field `ADDR` reader - source image RGB data address\[31:0\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word.
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - source image RGB data address\[31:0\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word.
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - source image RGB data address\[31:0\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word.
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0_SRC")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - source image RGB data address\[31:0\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word.
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<L0_SRCrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l0_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L0_SRCrs;
impl crate::RegisterSpec for L0_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`l0_src::R`](R) reader structure
impl crate::Readable for L0_SRCrs {}
///`write(|w| ..)` method takes [`l0_src::W`](W) writer structure
impl crate::Writable for L0_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L0_SRC to value 0
impl crate::Resettable for L0_SRCrs {
    const RESET_VALUE: u32 = 0;
}
