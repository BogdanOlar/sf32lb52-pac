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
///Field `LDO_DC_TR` reader -
pub type LdoDcTrR = crate::FieldReader;
///Field `LDO_DC_TR` writer -
pub type LdoDcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    ///Bits 8:10
    #[inline(always)]
    pub fn ldo_dc_tr(&self) -> LdoDcTrR {
        LdoDcTrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACR")
            .field("ldo_dc_tr", &self.ldo_dc_tr())
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
    ///Bits 8:10
    #[inline(always)]
    pub fn ldo_dc_tr(&mut self) -> LdoDcTrW<ANACRrs> {
        LdoDcTrW::new(self, 8)
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
