///Register `PERI_LDO` reader
pub type R = crate::R<PERI_LDOrs>;
///Register `PERI_LDO` writer
pub type W = crate::W<PERI_LDOrs>;
///Field `EN_LDO18` reader -
pub type EnLdo18R = crate::BitReader;
///Field `EN_LDO18` writer -
pub type EnLdo18W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO18_VREF_SEL` reader -
pub type Ldo18VrefSelR = crate::FieldReader;
///Field `LDO18_VREF_SEL` writer -
pub type Ldo18VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LDO18_PD` reader -
pub type Ldo18PdR = crate::BitReader;
///Field `LDO18_PD` writer -
pub type Ldo18PdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_VDD33_LDO2` reader -
pub type EnVdd33Ldo2R = crate::BitReader;
///Field `EN_VDD33_LDO2` writer -
pub type EnVdd33Ldo2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDD33_LDO2_SET_VOUT` reader -
pub type Vdd33Ldo2SetVoutR = crate::FieldReader;
///Field `VDD33_LDO2_SET_VOUT` writer -
pub type Vdd33Ldo2SetVoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VDD33_LDO2_PD` reader -
pub type Vdd33Ldo2PdR = crate::BitReader;
///Field `VDD33_LDO2_PD` writer -
pub type Vdd33Ldo2PdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_VDD33_LDO3` reader -
pub type EnVdd33Ldo3R = crate::BitReader;
///Field `EN_VDD33_LDO3` writer -
pub type EnVdd33Ldo3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDD33_LDO3_SET_VOUT` reader -
pub type Vdd33Ldo3SetVoutR = crate::FieldReader;
///Field `VDD33_LDO3_SET_VOUT` writer -
pub type Vdd33Ldo3SetVoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VDD33_LDO3_PD` reader -
pub type Vdd33Ldo3PdR = crate::BitReader;
///Field `VDD33_LDO3_PD` writer -
pub type Vdd33Ldo3PdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en_ldo18(&self) -> EnLdo18R {
        EnLdo18R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4
    #[inline(always)]
    pub fn ldo18_vref_sel(&self) -> Ldo18VrefSelR {
        Ldo18VrefSelR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ldo18_pd(&self) -> Ldo18PdR {
        Ldo18PdR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn en_vdd33_ldo2(&self) -> EnVdd33Ldo2R {
        EnVdd33Ldo2R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12
    #[inline(always)]
    pub fn vdd33_ldo2_set_vout(&self) -> Vdd33Ldo2SetVoutR {
        Vdd33Ldo2SetVoutR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bit 13
    #[inline(always)]
    pub fn vdd33_ldo2_pd(&self) -> Vdd33Ldo2PdR {
        Vdd33Ldo2PdR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn en_vdd33_ldo3(&self) -> EnVdd33Ldo3R {
        EnVdd33Ldo3R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20
    #[inline(always)]
    pub fn vdd33_ldo3_set_vout(&self) -> Vdd33Ldo3SetVoutR {
        Vdd33Ldo3SetVoutR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bit 21
    #[inline(always)]
    pub fn vdd33_ldo3_pd(&self) -> Vdd33Ldo3PdR {
        Vdd33Ldo3PdR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_LDO")
            .field("rsvd", &self.rsvd())
            .field("vdd33_ldo3_pd", &self.vdd33_ldo3_pd())
            .field("vdd33_ldo3_set_vout", &self.vdd33_ldo3_set_vout())
            .field("en_vdd33_ldo3", &self.en_vdd33_ldo3())
            .field("rsvd2", &self.rsvd2())
            .field("vdd33_ldo2_pd", &self.vdd33_ldo2_pd())
            .field("vdd33_ldo2_set_vout", &self.vdd33_ldo2_set_vout())
            .field("en_vdd33_ldo2", &self.en_vdd33_ldo2())
            .field("rsvd3", &self.rsvd3())
            .field("ldo18_pd", &self.ldo18_pd())
            .field("ldo18_vref_sel", &self.ldo18_vref_sel())
            .field("en_ldo18", &self.en_ldo18())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en_ldo18(&mut self) -> EnLdo18W<PERI_LDOrs> {
        EnLdo18W::new(self, 0)
    }
    ///Bits 1:4
    #[inline(always)]
    pub fn ldo18_vref_sel(&mut self) -> Ldo18VrefSelW<PERI_LDOrs> {
        Ldo18VrefSelW::new(self, 1)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ldo18_pd(&mut self) -> Ldo18PdW<PERI_LDOrs> {
        Ldo18PdW::new(self, 5)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<PERI_LDOrs> {
        Rsvd3W::new(self, 6)
    }
    ///Bit 8
    #[inline(always)]
    pub fn en_vdd33_ldo2(&mut self) -> EnVdd33Ldo2W<PERI_LDOrs> {
        EnVdd33Ldo2W::new(self, 8)
    }
    ///Bits 9:12
    #[inline(always)]
    pub fn vdd33_ldo2_set_vout(&mut self) -> Vdd33Ldo2SetVoutW<PERI_LDOrs> {
        Vdd33Ldo2SetVoutW::new(self, 9)
    }
    ///Bit 13
    #[inline(always)]
    pub fn vdd33_ldo2_pd(&mut self) -> Vdd33Ldo2PdW<PERI_LDOrs> {
        Vdd33Ldo2PdW::new(self, 13)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<PERI_LDOrs> {
        Rsvd2W::new(self, 14)
    }
    ///Bit 16
    #[inline(always)]
    pub fn en_vdd33_ldo3(&mut self) -> EnVdd33Ldo3W<PERI_LDOrs> {
        EnVdd33Ldo3W::new(self, 16)
    }
    ///Bits 17:20
    #[inline(always)]
    pub fn vdd33_ldo3_set_vout(&mut self) -> Vdd33Ldo3SetVoutW<PERI_LDOrs> {
        Vdd33Ldo3SetVoutW::new(self, 17)
    }
    ///Bit 21
    #[inline(always)]
    pub fn vdd33_ldo3_pd(&mut self) -> Vdd33Ldo3PdW<PERI_LDOrs> {
        Vdd33Ldo3PdW::new(self, 21)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PERI_LDOrs> {
        RsvdW::new(self, 22)
    }
}
///Peripherals LDO
///
///You can [`read`](crate::Reg::read) this register and get [`peri_ldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_ldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PERI_LDOrs;
impl crate::RegisterSpec for PERI_LDOrs {
    type Ux = u32;
}
///`read()` method returns [`peri_ldo::R`](R) reader structure
impl crate::Readable for PERI_LDOrs {}
///`write(|w| ..)` method takes [`peri_ldo::W`](W) writer structure
impl crate::Writable for PERI_LDOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_LDO to value 0
impl crate::Resettable for PERI_LDOrs {
    const RESET_VALUE: u32 = 0;
}
