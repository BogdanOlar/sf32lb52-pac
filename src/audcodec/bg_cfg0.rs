///Register `BG_CFG0` reader
pub type R = crate::R<BG_CFG0rs>;
///Register `BG_CFG0` writer
pub type W = crate::W<BG_CFG0rs>;
///Field `EN` reader - enable bandgap
pub type EnR = crate::BitReader;
///Field `EN` writer - enable bandgap
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_MODE` reader - 1: bandgap lp mode
pub type LpModeR = crate::BitReader;
///Field `LP_MODE` writer - 1: bandgap lp mode
pub type LpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREF_SEL` reader - set vref, 12: 2.2V
pub type VrefSelR = crate::FieldReader;
///Field `VREF_SEL` writer - set vref, 12: 2.2V
pub type VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN_CHOP` reader - enable bandgap chop
pub type EnChopR = crate::BitReader;
///Field `EN_CHOP` writer - enable bandgap chop
pub type EnChopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_SMPL` reader - enable bandgap sample
pub type EnSmplR = crate::BitReader;
///Field `EN_SMPL` writer - enable bandgap sample
pub type EnSmplW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_RCFLT` reader - enable bandgap rc filter
pub type EnRcfltR = crate::BitReader;
///Field `EN_RCFLT` writer - enable bandgap rc filter
pub type EnRcfltW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MIC_VREF_SEL` reader - select mic vref
pub type MicVrefSelR = crate::FieldReader;
///Field `MIC_VREF_SEL` writer - select mic vref
pub type MicVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EN_AMP` reader - enable bg opamp
pub type EnAmpR = crate::BitReader;
///Field `EN_AMP` writer - enable bg opamp
pub type EnAmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SET_VC` reader - set vc
pub type SetVcR = crate::BitReader;
///Field `SET_VC` writer - set vc
pub type SetVcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable bandgap
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: bandgap lp mode
    #[inline(always)]
    pub fn lp_mode(&self) -> LpModeR {
        LpModeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - set vref, 12: 2.2V
    #[inline(always)]
    pub fn vref_sel(&self) -> VrefSelR {
        VrefSelR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - enable bandgap chop
    #[inline(always)]
    pub fn en_chop(&self) -> EnChopR {
        EnChopR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable bandgap sample
    #[inline(always)]
    pub fn en_smpl(&self) -> EnSmplR {
        EnSmplR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - enable bandgap rc filter
    #[inline(always)]
    pub fn en_rcflt(&self) -> EnRcfltR {
        EnRcfltR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - select mic vref
    #[inline(always)]
    pub fn mic_vref_sel(&self) -> MicVrefSelR {
        MicVrefSelR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12 - enable bg opamp
    #[inline(always)]
    pub fn en_amp(&self) -> EnAmpR {
        EnAmpR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - set vc
    #[inline(always)]
    pub fn set_vc(&self) -> SetVcR {
        SetVcR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BG_CFG0")
            .field("set_vc", &self.set_vc())
            .field("en_amp", &self.en_amp())
            .field("mic_vref_sel", &self.mic_vref_sel())
            .field("en_rcflt", &self.en_rcflt())
            .field("en_smpl", &self.en_smpl())
            .field("en_chop", &self.en_chop())
            .field("vref_sel", &self.vref_sel())
            .field("lp_mode", &self.lp_mode())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable bandgap
    #[inline(always)]
    pub fn en(&mut self) -> EnW<BG_CFG0rs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - 1: bandgap lp mode
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LpModeW<BG_CFG0rs> {
        LpModeW::new(self, 1)
    }
    ///Bits 2:5 - set vref, 12: 2.2V
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VrefSelW<BG_CFG0rs> {
        VrefSelW::new(self, 2)
    }
    ///Bit 6 - enable bandgap chop
    #[inline(always)]
    pub fn en_chop(&mut self) -> EnChopW<BG_CFG0rs> {
        EnChopW::new(self, 6)
    }
    ///Bit 7 - enable bandgap sample
    #[inline(always)]
    pub fn en_smpl(&mut self) -> EnSmplW<BG_CFG0rs> {
        EnSmplW::new(self, 7)
    }
    ///Bit 8 - enable bandgap rc filter
    #[inline(always)]
    pub fn en_rcflt(&mut self) -> EnRcfltW<BG_CFG0rs> {
        EnRcfltW::new(self, 8)
    }
    ///Bits 9:11 - select mic vref
    #[inline(always)]
    pub fn mic_vref_sel(&mut self) -> MicVrefSelW<BG_CFG0rs> {
        MicVrefSelW::new(self, 9)
    }
    ///Bit 12 - enable bg opamp
    #[inline(always)]
    pub fn en_amp(&mut self) -> EnAmpW<BG_CFG0rs> {
        EnAmpW::new(self, 12)
    }
    ///Bit 13 - set vc
    #[inline(always)]
    pub fn set_vc(&mut self) -> SetVcW<BG_CFG0rs> {
        SetVcW::new(self, 13)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bg_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BG_CFG0rs;
impl crate::RegisterSpec for BG_CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`bg_cfg0::R`](R) reader structure
impl crate::Readable for BG_CFG0rs {}
///`write(|w| ..)` method takes [`bg_cfg0::W`](W) writer structure
impl crate::Writable for BG_CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BG_CFG0 to value 0
impl crate::Resettable for BG_CFG0rs {
    const RESET_VALUE: u32 = 0;
}
