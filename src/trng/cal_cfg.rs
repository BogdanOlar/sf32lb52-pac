///Register `CAL_CFG` reader
pub type R = crate::R<CAL_CFGrs>;
///Register `CAL_CFG` writer
pub type W = crate::W<CAL_CFGrs>;
///Field `OSC_CLK_FORCE_ON` reader - osc force enable
pub type OscClkForceOnR = crate::BitReader;
///Field `OSC_CLK_FORCE_ON` writer - osc force enable
pub type OscClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSC_CLK_SEL` reader - osc clock select
pub type OscClkSelR = crate::FieldReader;
///Field `OSC_CLK_SEL` writer - osc clock select
pub type OscClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ENABLE` reader - calibration enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - calibration enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DONE` reader - calibration done
pub type DoneR = crate::BitReader;
///Field `DONE` writer - calibration done
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `LENGTH` reader - calibration length
pub type LengthR = crate::FieldReader<u16>;
///Field `LENGTH` writer - calibration length
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - osc force enable
    #[inline(always)]
    pub fn osc_clk_force_on(&self) -> OscClkForceOnR {
        OscClkForceOnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - osc clock select
    #[inline(always)]
    pub fn osc_clk_sel(&self) -> OscClkSelR {
        OscClkSelR::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - calibration enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - calibration done
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:15
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    ///Bits 16:31 - calibration length
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAL_CFG")
            .field("length", &self.length())
            .field("rsvd", &self.rsvd())
            .field("done", &self.done())
            .field("enable", &self.enable())
            .field("osc_clk_sel", &self.osc_clk_sel())
            .field("osc_clk_force_on", &self.osc_clk_force_on())
            .finish()
    }
}
impl W {
    ///Bit 0 - osc force enable
    #[inline(always)]
    pub fn osc_clk_force_on(&mut self) -> OscClkForceOnW<CAL_CFGrs> {
        OscClkForceOnW::new(self, 0)
    }
    ///Bits 1:3 - osc clock select
    #[inline(always)]
    pub fn osc_clk_sel(&mut self) -> OscClkSelW<CAL_CFGrs> {
        OscClkSelW::new(self, 1)
    }
    ///Bit 4 - calibration enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CAL_CFGrs> {
        EnableW::new(self, 4)
    }
    ///Bit 5 - calibration done
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<CAL_CFGrs> {
        DoneW::new(self, 5)
    }
    ///Bits 6:15
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CAL_CFGrs> {
        RsvdW::new(self, 6)
    }
    ///Bits 16:31 - calibration length
    #[inline(always)]
    pub fn length(&mut self) -> LengthW<CAL_CFGrs> {
        LengthW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cal_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CAL_CFGrs;
impl crate::RegisterSpec for CAL_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`cal_cfg::R`](R) reader structure
impl crate::Readable for CAL_CFGrs {}
///`write(|w| ..)` method takes [`cal_cfg::W`](W) writer structure
impl crate::Writable for CAL_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAL_CFG to value 0
impl crate::Resettable for CAL_CFGrs {
    const RESET_VALUE: u32 = 0;
}
