///Register `REFGEN_CFG` reader
pub type R = crate::R<REFGEN_CFGrs>;
///Register `REFGEN_CFG` writer
pub type W = crate::W<REFGEN_CFGrs>;
///Field `EN` reader - enable ref gen
pub type EnR = crate::BitReader;
///Field `EN` writer - enable ref gen
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_CHOP` reader - enable ref gen chop
pub type EnChopR = crate::BitReader;
///Field `EN_CHOP` writer - enable ref gen chop
pub type EnChopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BM` reader - bias mode
pub type BmR = crate::FieldReader;
///Field `BM` writer - bias mode
pub type BmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LP_MODE` reader - 1: lpmode(adc), 0:dac
pub type LpModeR = crate::BitReader;
///Field `LP_MODE` writer - 1: lpmode(adc), 0:dac
pub type LpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LV_MODE` reader - low vol mode
pub type LvModeR = crate::BitReader;
///Field `LV_MODE` writer - low vol mode
pub type LvModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RZSEL` reader - sel Rz, 0: 1uF cap
pub type RzselR = crate::FieldReader;
///Field `RZSEL` writer - sel Rz, 0: 1uF cap
pub type RzselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DISCHG` reader - discharge vref
pub type DischgR = crate::BitReader;
///Field `DISCHG` writer - discharge vref
pub type DischgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bit 0 - enable ref gen
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enable ref gen chop
    #[inline(always)]
    pub fn en_chop(&self) -> EnChopR {
        EnChopR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - bias mode
    #[inline(always)]
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - 1: lpmode(adc), 0:dac
    #[inline(always)]
    pub fn lp_mode(&self) -> LpModeR {
        LpModeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - low vol mode
    #[inline(always)]
    pub fn lv_mode(&self) -> LvModeR {
        LvModeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - sel Rz, 0: 1uF cap
    #[inline(always)]
    pub fn rzsel(&self) -> RzselR {
        RzselR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - discharge vref
    #[inline(always)]
    pub fn dischg(&self) -> DischgR {
        DischgR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REFGEN_CFG")
            .field("rsvd", &self.rsvd())
            .field("dischg", &self.dischg())
            .field("rzsel", &self.rzsel())
            .field("lv_mode", &self.lv_mode())
            .field("lp_mode", &self.lp_mode())
            .field("bm", &self.bm())
            .field("en_chop", &self.en_chop())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable ref gen
    #[inline(always)]
    pub fn en(&mut self) -> EnW<REFGEN_CFGrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - enable ref gen chop
    #[inline(always)]
    pub fn en_chop(&mut self) -> EnChopW<REFGEN_CFGrs> {
        EnChopW::new(self, 1)
    }
    ///Bits 2:3 - bias mode
    #[inline(always)]
    pub fn bm(&mut self) -> BmW<REFGEN_CFGrs> {
        BmW::new(self, 2)
    }
    ///Bit 4 - 1: lpmode(adc), 0:dac
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LpModeW<REFGEN_CFGrs> {
        LpModeW::new(self, 4)
    }
    ///Bit 5 - low vol mode
    #[inline(always)]
    pub fn lv_mode(&mut self) -> LvModeW<REFGEN_CFGrs> {
        LvModeW::new(self, 5)
    }
    ///Bits 6:7 - sel Rz, 0: 1uF cap
    #[inline(always)]
    pub fn rzsel(&mut self) -> RzselW<REFGEN_CFGrs> {
        RzselW::new(self, 6)
    }
    ///Bit 8 - discharge vref
    #[inline(always)]
    pub fn dischg(&mut self) -> DischgW<REFGEN_CFGrs> {
        DischgW::new(self, 8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<REFGEN_CFGrs> {
        RsvdW::new(self, 9)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`refgen_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refgen_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct REFGEN_CFGrs;
impl crate::RegisterSpec for REFGEN_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`refgen_cfg::R`](R) reader structure
impl crate::Readable for REFGEN_CFGrs {}
///`write(|w| ..)` method takes [`refgen_cfg::W`](W) writer structure
impl crate::Writable for REFGEN_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REFGEN_CFG to value 0
impl crate::Resettable for REFGEN_CFGrs {
    const RESET_VALUE: u32 = 0;
}
