///Register `DAC_CH0_DEBUG` reader
pub type R = crate::R<DAC_CH0_DEBUGrs>;
///Register `DAC_CH0_DEBUG` writer
pub type W = crate::W<DAC_CH0_DEBUGrs>;
///Field `DATA_OUT` reader - debug dac output
pub type DataOutR = crate::FieldReader<u16>;
///Field `DATA_OUT` writer - debug dac output
pub type DataOutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BYPASS` reader - debug bypass mode
pub type BypassR = crate::BitReader;
///Field `BYPASS` writer - debug bypass mode
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:15 - debug dac output
    #[inline(always)]
    pub fn data_out(&self) -> DataOutR {
        DataOutR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - debug bypass mode
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH0_DEBUG")
            .field("rsvd", &self.rsvd())
            .field("bypass", &self.bypass())
            .field("data_out", &self.data_out())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - debug dac output
    #[inline(always)]
    pub fn data_out(&mut self) -> DataOutW<DAC_CH0_DEBUGrs> {
        DataOutW::new(self, 0)
    }
    ///Bit 16 - debug bypass mode
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<DAC_CH0_DEBUGrs> {
        BypassW::new(self, 16)
    }
    ///Bits 17:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DAC_CH0_DEBUGrs> {
        RsvdW::new(self, 17)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH0_DEBUGrs;
impl crate::RegisterSpec for DAC_CH0_DEBUGrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch0_debug::R`](R) reader structure
impl crate::Readable for DAC_CH0_DEBUGrs {}
///`write(|w| ..)` method takes [`dac_ch0_debug::W`](W) writer structure
impl crate::Writable for DAC_CH0_DEBUGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH0_DEBUG to value 0
impl crate::Resettable for DAC_CH0_DEBUGrs {
    const RESET_VALUE: u32 = 0;
}
