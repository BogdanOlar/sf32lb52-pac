///Register `RWOT_CTRL` reader
pub type R = crate::R<RWOT_CTRLrs>;
///Register `RWOT_CTRL` writer
pub type W = crate::W<RWOT_CTRLrs>;
///Field `RWOT` reader - Receive Without Transmit 0: Transmit/receive mode 1: Receive without transmit mode
pub type RwotR = crate::BitReader;
///Field `RWOT` writer - Receive Without Transmit 0: Transmit/receive mode 1: Receive without transmit mode
pub type RwotW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CYCLE_RWOT_EN` reader - Enable RWOT Cycle Counter Mode 1: Enable
pub type CycleRwotEnR = crate::BitReader;
///Field `CYCLE_RWOT_EN` writer - Enable RWOT Cycle Counter Mode 1: Enable
pub type CycleRwotEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SET_RWOT_CYCLE` reader - Set RWOT Cycle This field is used to set the value of the RWOT_CCM register to the internal rwot_counter. This field is self-cleared after SSE = 1. 1: Set rwot_counter
pub type SetRwotCycleR = crate::BitReader;
///Field `SET_RWOT_CYCLE` writer - Set RWOT Cycle This field is used to set the value of the RWOT_CCM register to the internal rwot_counter. This field is self-cleared after SSE = 1. 1: Set rwot_counter
pub type SetRwotCycleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_RWOT_CYCLE` reader - Clear Internal rwot_counter This field clears the rwot_counter to 0. This field is self cleared after SSE = 1. 1: Clear rwot_counter
pub type ClrRwotCycleR = crate::BitReader;
///Field `CLR_RWOT_CYCLE` writer - Clear Internal rwot_counter This field clears the rwot_counter to 0. This field is self cleared after SSE = 1. 1: Clear rwot_counter
pub type ClrRwotCycleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_RWOT_LAST_SAMPLE` reader - Mask last_sample_flag in RWOT Mode 1: Mask 0: Unmask
pub type MaskRwotLastSampleR = crate::BitReader;
///Field `MASK_RWOT_LAST_SAMPLE` writer - Mask last_sample_flag in RWOT Mode 1: Mask 0: Unmask
pub type MaskRwotLastSampleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bit 0 - Receive Without Transmit 0: Transmit/receive mode 1: Receive without transmit mode
    #[inline(always)]
    pub fn rwot(&self) -> RwotR {
        RwotR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable RWOT Cycle Counter Mode 1: Enable
    #[inline(always)]
    pub fn cycle_rwot_en(&self) -> CycleRwotEnR {
        CycleRwotEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set RWOT Cycle This field is used to set the value of the RWOT_CCM register to the internal rwot_counter. This field is self-cleared after SSE = 1. 1: Set rwot_counter
    #[inline(always)]
    pub fn set_rwot_cycle(&self) -> SetRwotCycleR {
        SetRwotCycleR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear Internal rwot_counter This field clears the rwot_counter to 0. This field is self cleared after SSE = 1. 1: Clear rwot_counter
    #[inline(always)]
    pub fn clr_rwot_cycle(&self) -> ClrRwotCycleR {
        ClrRwotCycleR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mask last_sample_flag in RWOT Mode 1: Mask 0: Unmask
    #[inline(always)]
    pub fn mask_rwot_last_sample(&self) -> MaskRwotLastSampleR {
        MaskRwotLastSampleR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWOT_CTRL")
            .field("rsvd", &self.rsvd())
            .field("mask_rwot_last_sample", &self.mask_rwot_last_sample())
            .field("clr_rwot_cycle", &self.clr_rwot_cycle())
            .field("set_rwot_cycle", &self.set_rwot_cycle())
            .field("cycle_rwot_en", &self.cycle_rwot_en())
            .field("rwot", &self.rwot())
            .finish()
    }
}
impl W {
    ///Bit 0 - Receive Without Transmit 0: Transmit/receive mode 1: Receive without transmit mode
    #[inline(always)]
    pub fn rwot(&mut self) -> RwotW<RWOT_CTRLrs> {
        RwotW::new(self, 0)
    }
    ///Bit 1 - Enable RWOT Cycle Counter Mode 1: Enable
    #[inline(always)]
    pub fn cycle_rwot_en(&mut self) -> CycleRwotEnW<RWOT_CTRLrs> {
        CycleRwotEnW::new(self, 1)
    }
    ///Bit 2 - Set RWOT Cycle This field is used to set the value of the RWOT_CCM register to the internal rwot_counter. This field is self-cleared after SSE = 1. 1: Set rwot_counter
    #[inline(always)]
    pub fn set_rwot_cycle(&mut self) -> SetRwotCycleW<RWOT_CTRLrs> {
        SetRwotCycleW::new(self, 2)
    }
    ///Bit 3 - Clear Internal rwot_counter This field clears the rwot_counter to 0. This field is self cleared after SSE = 1. 1: Clear rwot_counter
    #[inline(always)]
    pub fn clr_rwot_cycle(&mut self) -> ClrRwotCycleW<RWOT_CTRLrs> {
        ClrRwotCycleW::new(self, 3)
    }
    ///Bit 4 - Mask last_sample_flag in RWOT Mode 1: Mask 0: Unmask
    #[inline(always)]
    pub fn mask_rwot_last_sample(&mut self) -> MaskRwotLastSampleW<RWOT_CTRLrs> {
        MaskRwotLastSampleW::new(self, 4)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RWOT_CTRLrs> {
        RsvdW::new(self, 5)
    }
}
///RWOT Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`rwot_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwot_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RWOT_CTRLrs;
impl crate::RegisterSpec for RWOT_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`rwot_ctrl::R`](R) reader structure
impl crate::Readable for RWOT_CTRLrs {}
///`write(|w| ..)` method takes [`rwot_ctrl::W`](W) writer structure
impl crate::Writable for RWOT_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RWOT_CTRL to value 0
impl crate::Resettable for RWOT_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
