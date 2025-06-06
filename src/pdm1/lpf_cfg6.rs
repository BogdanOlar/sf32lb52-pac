///Register `LPF_CFG6` reader
pub type R = crate::R<LPF_CFG6rs>;
///Register `LPF_CFG6` writer
pub type W = crate::W<LPF_CFG6rs>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader<u16>;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `LPF_DS` reader - 1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter
pub type LpfDsR = crate::BitReader;
///Field `LPF_DS` writer - 1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter
pub type LpfDsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPF_BYPASS` reader - 1:bypass low-pass filter ; 0: enable low-pass filter
pub type LpfBypassR = crate::BitReader;
///Field `LPF_BYPASS` writer - 1:bypass low-pass filter ; 0: enable low-pass filter
pub type LpfBypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new((self.bits & 0x0fff) as u16)
    }
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
    ///Bits 14:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPF_CFG6")
            .field("rsvd", &self.rsvd())
            .field("lpf_bypass", &self.lpf_bypass())
            .field("lpf_ds", &self.lpf_ds())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<LPF_CFG6rs> {
        Rsvd2W::new(self, 0)
    }
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
    ///Bits 14:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<LPF_CFG6rs> {
        RsvdW::new(self, 14)
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
