///Register `PLL_CAL_CFG` reader
pub type R = crate::R<PLL_CAL_CFGrs>;
///Register `PLL_CAL_CFG` writer
pub type W = crate::W<PLL_CAL_CFGrs>;
///Field `EN` reader - calibration enable
pub type EnR = crate::BitReader;
///Field `EN` writer - calibration enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DONE` reader - calibration done
pub type DoneR = crate::BitReader;
///Field `DONE` writer - calibration done
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEN` reader - calibration length
pub type LenR = crate::FieldReader<u16>;
///Field `LEN` writer - calibration length
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - calibration enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - calibration done
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:31 - calibration length
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CAL_CFG")
            .field("len", &self.len())
            .field("done", &self.done())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - calibration enable
    #[inline(always)]
    pub fn en(&mut self) -> EnW<PLL_CAL_CFGrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - calibration done
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<PLL_CAL_CFGrs> {
        DoneW::new(self, 1)
    }
    ///Bits 16:31 - calibration length
    #[inline(always)]
    pub fn len(&mut self) -> LenW<PLL_CAL_CFGrs> {
        LenW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cal_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cal_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CAL_CFGrs;
impl crate::RegisterSpec for PLL_CAL_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`pll_cal_cfg::R`](R) reader structure
impl crate::Readable for PLL_CAL_CFGrs {}
///`write(|w| ..)` method takes [`pll_cal_cfg::W`](W) writer structure
impl crate::Writable for PLL_CAL_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CAL_CFG to value 0
impl crate::Resettable for PLL_CAL_CFGrs {
    const RESET_VALUE: u32 = 0;
}
