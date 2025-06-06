///Register `DAC_EQ_CFG18` reader
pub type R = crate::R<DAC_EQ_CFG18rs>;
///Register `DAC_EQ_CFG18` writer
pub type W = crate::W<DAC_EQ_CFG18rs>;
///Field `COEF` reader -
pub type CoefR = crate::FieldReader<u32>;
///Field `COEF` writer -
pub type CoefW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23
    #[inline(always)]
    pub fn coef(&self) -> CoefR {
        CoefR::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_EQ_CFG18")
            .field("rsvd", &self.rsvd())
            .field("coef", &self.coef())
            .finish()
    }
}
impl W {
    ///Bits 0:23
    #[inline(always)]
    pub fn coef(&mut self) -> CoefW<DAC_EQ_CFG18rs> {
        CoefW::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DAC_EQ_CFG18rs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_eq_cfg18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_eq_cfg18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_EQ_CFG18rs;
impl crate::RegisterSpec for DAC_EQ_CFG18rs {
    type Ux = u32;
}
///`read()` method returns [`dac_eq_cfg18::R`](R) reader structure
impl crate::Readable for DAC_EQ_CFG18rs {}
///`write(|w| ..)` method takes [`dac_eq_cfg18::W`](W) writer structure
impl crate::Writable for DAC_EQ_CFG18rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_EQ_CFG18 to value 0
impl crate::Resettable for DAC_EQ_CFG18rs {
    const RESET_VALUE: u32 = 0;
}
