///Register `HPF_CFG` reader
pub type R = crate::R<HPF_CFGrs>;
///Register `HPF_CFG` writer
pub type W = crate::W<HPF_CFGrs>;
///Field `HPF_COEFF` reader - coefficient of high-pass filter
pub type HpfCoeffR = crate::FieldReader;
///Field `HPF_COEFF` writer - coefficient of high-pass filter
pub type HpfCoeffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HPF_BYPASS` reader - 1:bypass-high pass filter ; 0: enable high-pass filter
pub type HpfBypassR = crate::BitReader;
///Field `HPF_BYPASS` writer - 1:bypass-high pass filter ; 0: enable high-pass filter
pub type HpfBypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPF_RST` reader - 1:high-pass filter normal operation ; 0:reset high-pass filter
pub type HpfRstR = crate::BitReader;
///Field `HPF_RST` writer - 1:high-pass filter normal operation ; 0:reset high-pass filter
pub type HpfRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - coefficient of high-pass filter
    #[inline(always)]
    pub fn hpf_coeff(&self) -> HpfCoeffR {
        HpfCoeffR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - 1:bypass-high pass filter ; 0: enable high-pass filter
    #[inline(always)]
    pub fn hpf_bypass(&self) -> HpfBypassR {
        HpfBypassR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 1:high-pass filter normal operation ; 0:reset high-pass filter
    #[inline(always)]
    pub fn hpf_rst(&self) -> HpfRstR {
        HpfRstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPF_CFG")
            .field("hpf_rst", &self.hpf_rst())
            .field("hpf_bypass", &self.hpf_bypass())
            .field("hpf_coeff", &self.hpf_coeff())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - coefficient of high-pass filter
    #[inline(always)]
    pub fn hpf_coeff(&mut self) -> HpfCoeffW<HPF_CFGrs> {
        HpfCoeffW::new(self, 0)
    }
    ///Bit 4 - 1:bypass-high pass filter ; 0: enable high-pass filter
    #[inline(always)]
    pub fn hpf_bypass(&mut self) -> HpfBypassW<HPF_CFGrs> {
        HpfBypassW::new(self, 4)
    }
    ///Bit 5 - 1:high-pass filter normal operation ; 0:reset high-pass filter
    #[inline(always)]
    pub fn hpf_rst(&mut self) -> HpfRstW<HPF_CFGrs> {
        HpfRstW::new(self, 5)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hpf_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpf_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HPF_CFGrs;
impl crate::RegisterSpec for HPF_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`hpf_cfg::R`](R) reader structure
impl crate::Readable for HPF_CFGrs {}
///`write(|w| ..)` method takes [`hpf_cfg::W`](W) writer structure
impl crate::Writable for HPF_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HPF_CFG to value 0
impl crate::Resettable for HPF_CFGrs {
    const RESET_VALUE: u32 = 0;
}
