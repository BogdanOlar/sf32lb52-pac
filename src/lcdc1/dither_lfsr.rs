///Register `DITHER_LFSR` reader
pub type R = crate::R<DITHER_LFSRrs>;
///Register `DITHER_LFSR` writer
pub type W = crate::W<DITHER_LFSRrs>;
///Field `INIT_VAL` reader - lfsr init load value
pub type InitValR = crate::FieldReader<u32>;
///Field `INIT_VAL` writer - lfsr init load value
pub type InitValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - lfsr init load value
    #[inline(always)]
    pub fn init_val(&self) -> InitValR {
        InitValR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DITHER_LFSR")
            .field("init_val", &self.init_val())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - lfsr init load value
    #[inline(always)]
    pub fn init_val(&mut self) -> InitValW<DITHER_LFSRrs> {
        InitValW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dither_lfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dither_lfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DITHER_LFSRrs;
impl crate::RegisterSpec for DITHER_LFSRrs {
    type Ux = u32;
}
///`read()` method returns [`dither_lfsr::R`](R) reader structure
impl crate::Readable for DITHER_LFSRrs {}
///`write(|w| ..)` method takes [`dither_lfsr::W`](W) writer structure
impl crate::Writable for DITHER_LFSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DITHER_LFSR to value 0
impl crate::Resettable for DITHER_LFSRrs {
    const RESET_VALUE: u32 = 0;
}
