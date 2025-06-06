///Register `DSR` reader
pub type R = crate::R<DSRrs>;
///Register `DSR` writer
pub type W = crate::W<DSRrs>;
///Field `SD_DATA_I_LL` reader - The status of each sd data pad status
pub type SdDataILlR = crate::FieldReader;
///Field `SD_DATA_I_LL` writer - The status of each sd data pad status
pub type SdDataILlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:7 - The status of each sd data pad status
    #[inline(always)]
    pub fn sd_data_i_ll(&self) -> SdDataILlR {
        SdDataILlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSR")
            .field("rsvd", &self.rsvd())
            .field("sd_data_i_ll", &self.sd_data_i_ll())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - The status of each sd data pad status
    #[inline(always)]
    pub fn sd_data_i_ll(&mut self) -> SdDataILlW<DSRrs> {
        SdDataILlW::new(self, 0)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DSRrs> {
        RsvdW::new(self, 8)
    }
}
///data status register
///
///You can [`read`](crate::Reg::read) this register and get [`dsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DSRrs;
impl crate::RegisterSpec for DSRrs {
    type Ux = u32;
}
///`read()` method returns [`dsr::R`](R) reader structure
impl crate::Readable for DSRrs {}
///`write(|w| ..)` method takes [`dsr::W`](W) writer structure
impl crate::Writable for DSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSR to value 0
impl crate::Resettable for DSRrs {
    const RESET_VALUE: u32 = 0;
}
