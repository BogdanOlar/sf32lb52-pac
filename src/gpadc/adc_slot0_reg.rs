///Register `ADC_SLOT0_REG` reader
pub type R = crate::R<ADC_SLOT0_REGrs>;
///Register `ADC_SLOT0_REG` writer
pub type W = crate::W<ADC_SLOT0_REGrs>;
///Field `SLOT_EN` reader -
pub type SlotEnR = crate::BitReader;
///Field `SLOT_EN` writer -
pub type SlotEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCHNL_SEL` reader -
pub type PchnlSelR = crate::FieldReader;
///Field `PCHNL_SEL` writer -
pub type PchnlSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NCHNL_SEL` reader -
pub type NchnlSelR = crate::FieldReader;
///Field `NCHNL_SEL` writer -
pub type NchnlSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn slot_en(&self) -> SlotEnR {
        SlotEnR::new((self.bits & 1) != 0)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn pchnl_sel(&self) -> PchnlSelR {
        PchnlSelR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13
    #[inline(always)]
    pub fn nchnl_sel(&self) -> NchnlSelR {
        NchnlSelR::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SLOT0_REG")
            .field("nchnl_sel", &self.nchnl_sel())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("slot_en", &self.slot_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn slot_en(&mut self) -> SlotEnW<ADC_SLOT0_REGrs> {
        SlotEnW::new(self, 0)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn pchnl_sel(&mut self) -> PchnlSelW<ADC_SLOT0_REGrs> {
        PchnlSelW::new(self, 8)
    }
    ///Bits 11:13
    #[inline(always)]
    pub fn nchnl_sel(&mut self) -> NchnlSelW<ADC_SLOT0_REGrs> {
        NchnlSelW::new(self, 11)
    }
}
///ADC Slot0 Config Register
///
///You can [`read`](crate::Reg::read) this register and get [`adc_slot0_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_slot0_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_SLOT0_REGrs;
impl crate::RegisterSpec for ADC_SLOT0_REGrs {
    type Ux = u32;
}
///`read()` method returns [`adc_slot0_reg::R`](R) reader structure
impl crate::Readable for ADC_SLOT0_REGrs {}
///`write(|w| ..)` method takes [`adc_slot0_reg::W`](W) writer structure
impl crate::Writable for ADC_SLOT0_REGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_SLOT0_REG to value 0x0002_0801
impl crate::Resettable for ADC_SLOT0_REGrs {
    const RESET_VALUE: u32 = 0x0002_0801;
}
