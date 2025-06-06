///Register `L2_SRC` reader
pub type R = crate::R<L2_SRCrs>;
///Register `L2_SRC` writer
pub type W = crate::W<L2_SRCrs>;
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
        f.debug_struct("L2_SRC")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - source image RGB data address\[31:0\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word.
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<L2_SRCrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_SRCrs;
impl crate::RegisterSpec for L2_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`l2_src::R`](R) reader structure
impl crate::Readable for L2_SRCrs {}
///`write(|w| ..)` method takes [`l2_src::W`](W) writer structure
impl crate::Writable for L2_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_SRC to value 0
impl crate::Resettable for L2_SRCrs {
    const RESET_VALUE: u32 = 0;
}
