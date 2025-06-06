///Register `LPF_CFG6` reader
pub type R = crate::R<LPF_CFG6rs>;
///Register `LPF_CFG6` writer
pub type W = crate::W<LPF_CFG6rs>;
///Field `LPF_DS` reader - 1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter
pub type LpfDsR = crate::BitReader;
///Field `LPF_DS` writer - 1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter
pub type LpfDsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPF_BYPASS` reader - 1:bypass low-pass filter ; 0: enable low-pass filter
pub type LpfBypassR = crate::BitReader;
///Field `LPF_BYPASS` writer - 1:bypass low-pass filter ; 0: enable low-pass filter
pub type LpfBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - 1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter
    #[inline(always)]
    pub fn lpf_ds(&self) -> LpfDsR {
        LpfDsR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 1:bypass low-pass filter ; 0: enable low-pass filter
    #[inline(always)]
    pub fn lpf_bypass(&self) -> LpfBypassR {
        LpfBypassR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPF_CFG6")
            .field("lpf_bypass", &self.lpf_bypass())
            .field("lpf_ds", &self.lpf_ds())
            .finish()
    }
}
impl W {
    ///Bit 12 - 1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter
    #[inline(always)]
    pub fn lpf_ds(&mut self) -> LpfDsW<LPF_CFG6rs> {
        LpfDsW::new(self, 12)
    }
    ///Bit 13 - 1:bypass low-pass filter ; 0: enable low-pass filter
    #[inline(always)]
    pub fn lpf_bypass(&mut self) -> LpfBypassW<LPF_CFG6rs> {
        LpfBypassW::new(self, 13)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lpf_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpf_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LPF_CFG6rs;
impl crate::RegisterSpec for LPF_CFG6rs {
    type Ux = u32;
}
///`read()` method returns [`lpf_cfg6::R`](R) reader structure
impl crate::Readable for LPF_CFG6rs {}
///`write(|w| ..)` method takes [`lpf_cfg6::W`](W) writer structure
impl crate::Writable for LPF_CFG6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPF_CFG6 to value 0
impl crate::Resettable for LPF_CFG6rs {
    const RESET_VALUE: u32 = 0;
}
