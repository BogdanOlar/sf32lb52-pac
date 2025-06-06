///Register `ANACR` reader
pub type R = crate::R<ANACRrs>;
///Register `ANACR` writer
pub type W = crate::W<ANACRrs>;
///Field `LDO_EN` reader -
pub type LdoEnR = crate::BitReader;
///Field `LDO_EN` writer -
pub type LdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO_VREF_SEL` reader -
pub type LdoVrefSelR = crate::FieldReader;
///Field `LDO_VREF_SEL` writer -
pub type LdoVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LDO_MODE` reader -
pub type LdoModeR = crate::BitReader;
///Field `LDO_MODE` writer -
pub type LdoModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LDO_DC_TR` reader -
pub type LdoDcTrR = crate::FieldReader;
///Field `LDO_DC_TR` writer -
pub type LdoDcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RESERVE0` reader -
pub type Reserve0R = crate::FieldReader;
///Field `RESERVE0` writer -
pub type Reserve0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RESERVE1` reader -
pub type Reserve1R = crate::FieldReader;
///Field `RESERVE1` writer -
pub type Reserve1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn ldo_en(&self) -> LdoEnR {
        LdoEnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3
    #[inline(always)]
    pub fn ldo_vref_sel(&self) -> LdoVrefSelR {
        LdoVrefSelR::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ldo_mode(&self) -> LdoModeR {
        LdoModeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn ldo_dc_tr(&self) -> LdoDcTrR {
        LdoDcTrR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn reserve0(&self) -> Reserve0R {
        Reserve0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn reserve1(&self) -> Reserve1R {
        Reserve1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACR")
            .field("reserve1", &self.reserve1())
            .field("reserve0", &self.reserve0())
            .field("rsvd", &self.rsvd())
            .field("ldo_dc_tr", &self.ldo_dc_tr())
            .field("rsvd2", &self.rsvd2())
            .field("ldo_mode", &self.ldo_mode())
            .field("ldo_vref_sel", &self.ldo_vref_sel())
            .field("ldo_en", &self.ldo_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn ldo_en(&mut self) -> LdoEnW<ANACRrs> {
        LdoEnW::new(self, 0)
    }
    ///Bits 1:3
    #[inline(always)]
    pub fn ldo_vref_sel(&mut self) -> LdoVrefSelW<ANACRrs> {
        LdoVrefSelW::new(self, 1)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ldo_mode(&mut self) -> LdoModeW<ANACRrs> {
        LdoModeW::new(self, 4)
    }
    ///Bits 5:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ANACRrs> {
        Rsvd2W::new(self, 5)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn ldo_dc_tr(&mut self) -> LdoDcTrW<ANACRrs> {
        LdoDcTrW::new(self, 8)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ANACRrs> {
        RsvdW::new(self, 11)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn reserve0(&mut self) -> Reserve0W<ANACRrs> {
        Reserve0W::new(self, 16)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn reserve1(&mut self) -> Reserve1W<ANACRrs> {
        Reserve1W::new(self, 24)
    }
}
///Bank3 Data7
///
///You can [`read`](crate::Reg::read) this register and get [`anacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ANACRrs;
impl crate::RegisterSpec for ANACRrs {
    type Ux = u32;
}
///`read()` method returns [`anacr::R`](R) reader structure
impl crate::Readable for ANACRrs {}
///`write(|w| ..)` method takes [`anacr::W`](W) writer structure
impl crate::Writable for ANACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ANACR to value 0
impl crate::Resettable for ANACRrs {
    const RESET_VALUE: u32 = 0;
}
