///Register `ADC2_CFG2` reader
pub type R = crate::R<ADC2_CFG2rs>;
///Register `ADC2_CFG2` writer
pub type W = crate::W<ADC2_CFG2rs>;
///Field `CLEAR` reader - clear adc
pub type ClearR = crate::BitReader;
///Field `CLEAR` writer - clear adc
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHOP_EN` reader - chopping enable
pub type ChopEnR = crate::BitReader;
///Field `CHOP_EN` writer - chopping enable
pub type ChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTB` reader - reset adc
pub type RstbR = crate::BitReader;
///Field `RSTB` writer - reset adc
pub type RstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN` reader - enable adc
pub type EnR = crate::BitReader;
///Field `EN` writer - enable adc
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bit 0 - clear adc
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - chopping enable
    #[inline(always)]
    pub fn chop_en(&self) -> ChopEnR {
        ChopEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - reset adc
    #[inline(always)]
    pub fn rstb(&self) -> RstbR {
        RstbR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - enable adc
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC2_CFG2")
            .field("rsvd", &self.rsvd())
            .field("en", &self.en())
            .field("rstb", &self.rstb())
            .field("chop_en", &self.chop_en())
            .field("clear", &self.clear())
            .finish()
    }
}
impl W {
    ///Bit 0 - clear adc
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<ADC2_CFG2rs> {
        ClearW::new(self, 0)
    }
    ///Bit 1 - chopping enable
    #[inline(always)]
    pub fn chop_en(&mut self) -> ChopEnW<ADC2_CFG2rs> {
        ChopEnW::new(self, 1)
    }
    ///Bit 2 - reset adc
    #[inline(always)]
    pub fn rstb(&mut self) -> RstbW<ADC2_CFG2rs> {
        RstbW::new(self, 2)
    }
    ///Bit 3 - enable adc
    #[inline(always)]
    pub fn en(&mut self) -> EnW<ADC2_CFG2rs> {
        EnW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ADC2_CFG2rs> {
        RsvdW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`adc2_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC2_CFG2rs;
impl crate::RegisterSpec for ADC2_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`adc2_cfg2::R`](R) reader structure
impl crate::Readable for ADC2_CFG2rs {}
///`write(|w| ..)` method takes [`adc2_cfg2::W`](W) writer structure
impl crate::Writable for ADC2_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC2_CFG2 to value 0
impl crate::Resettable for ADC2_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
