///Register `DAC_CH0_DC` reader
pub type R = crate::R<DAC_CH0_DCrs>;
///Register `DAC_CH0_DC` writer
pub type W = crate::W<DAC_CH0_DCrs>;
///Field `OFFSET` reader - dac ch0 dc offset
pub type OffsetR = crate::FieldReader<u32>;
///Field `OFFSET` writer - dac ch0 dc offset
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - dac ch0 dc offset
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH0_DC")
            .field("rsvd", &self.rsvd())
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - dac ch0 dc offset
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<DAC_CH0_DCrs> {
        OffsetW::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DAC_CH0_DCrs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch0_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch0_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH0_DCrs;
impl crate::RegisterSpec for DAC_CH0_DCrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch0_dc::R`](R) reader structure
impl crate::Readable for DAC_CH0_DCrs {}
///`write(|w| ..)` method takes [`dac_ch0_dc::W`](W) writer structure
impl crate::Writable for DAC_CH0_DCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH0_DC to value 0
impl crate::Resettable for DAC_CH0_DCrs {
    const RESET_VALUE: u32 = 0;
}
