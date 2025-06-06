///Register `AON_LDO` reader
pub type R = crate::R<AON_LDOrs>;
///Register `AON_LDO` writer
pub type W = crate::W<AON_LDOrs>;
///Field `VBAT_LDO_SET_VOUT` reader -
pub type VbatLdoSetVoutR = crate::FieldReader;
///Field `VBAT_LDO_SET_VOUT` writer -
pub type VbatLdoSetVoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VBAT_POR_TH` reader -
pub type VbatPorThR = crate::FieldReader;
///Field `VBAT_POR_TH` writer -
pub type VbatPorThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn vbat_ldo_set_vout(&self) -> VbatLdoSetVoutR {
        VbatLdoSetVoutR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6
    #[inline(always)]
    pub fn vbat_por_th(&self) -> VbatPorThR {
        VbatPorThR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AON_LDO")
            .field("rsvd", &self.rsvd())
            .field("vbat_por_th", &self.vbat_por_th())
            .field("vbat_ldo_set_vout", &self.vbat_ldo_set_vout())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn vbat_ldo_set_vout(&mut self) -> VbatLdoSetVoutW<AON_LDOrs> {
        VbatLdoSetVoutW::new(self, 0)
    }
    ///Bits 4:6
    #[inline(always)]
    pub fn vbat_por_th(&mut self) -> VbatPorThW<AON_LDOrs> {
        VbatPorThW::new(self, 4)
    }
    ///Bits 7:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AON_LDOrs> {
        RsvdW::new(self, 7)
    }
}
///AON LDO Register
///
///You can [`read`](crate::Reg::read) this register and get [`aon_ldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_ldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AON_LDOrs;
impl crate::RegisterSpec for AON_LDOrs {
    type Ux = u32;
}
///`read()` method returns [`aon_ldo::R`](R) reader structure
impl crate::Readable for AON_LDOrs {}
///`write(|w| ..)` method takes [`aon_ldo::W`](W) writer structure
impl crate::Writable for AON_LDOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AON_LDO to value 0
impl crate::Resettable for AON_LDOrs {
    const RESET_VALUE: u32 = 0;
}
