///Register `LPSYS_LDO` reader
pub type R = crate::R<LPSYS_LDOrs>;
///Register `LPSYS_LDO` writer
pub type W = crate::W<LPSYS_LDOrs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BP` reader -
pub type BpR = crate::BitReader;
///Field `BP` writer -
pub type BpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREF` reader - optional voltage (1.0V)
pub type VrefR = crate::FieldReader;
///Field `VREF` writer - optional voltage (1.0V)
pub type VrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VREF2` reader - Lower voltage for deep sleep mode (0.6V)
pub type Vref2R = crate::FieldReader;
///Field `VREF2` writer - Lower voltage for deep sleep mode (0.6V)
pub type Vref2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DLY` reader - LPSYS_LDO power up delay in CLK_LP cycles
pub type DlyR = crate::FieldReader;
///Field `DLY` writer - LPSYS_LDO power up delay in CLK_LP cycles
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RDY` reader -
pub type RdyR = crate::BitReader;
///Field `RDY` writer -
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn bp(&self) -> BpR {
        BpR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - optional voltage (1.0V)
    #[inline(always)]
    pub fn vref(&self) -> VrefR {
        VrefR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:9 - Lower voltage for deep sleep mode (0.6V)
    #[inline(always)]
    pub fn vref2(&self) -> Vref2R {
        Vref2R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 10:15 - LPSYS_LDO power up delay in CLK_LP cycles
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSYS_LDO")
            .field("rdy", &self.rdy())
            .field("dly", &self.dly())
            .field("vref2", &self.vref2())
            .field("vref", &self.vref())
            .field("bp", &self.bp())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<LPSYS_LDOrs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn bp(&mut self) -> BpW<LPSYS_LDOrs> {
        BpW::new(self, 1)
    }
    ///Bits 2:5 - optional voltage (1.0V)
    #[inline(always)]
    pub fn vref(&mut self) -> VrefW<LPSYS_LDOrs> {
        VrefW::new(self, 2)
    }
    ///Bits 6:9 - Lower voltage for deep sleep mode (0.6V)
    #[inline(always)]
    pub fn vref2(&mut self) -> Vref2W<LPSYS_LDOrs> {
        Vref2W::new(self, 6)
    }
    ///Bits 10:15 - LPSYS_LDO power up delay in CLK_LP cycles
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<LPSYS_LDOrs> {
        DlyW::new(self, 10)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<LPSYS_LDOrs> {
        RdyW::new(self, 16)
    }
}
///LPSYS LDO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lpsys_ldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsys_ldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LPSYS_LDOrs;
impl crate::RegisterSpec for LPSYS_LDOrs {
    type Ux = u32;
}
///`read()` method returns [`lpsys_ldo::R`](R) reader structure
impl crate::Readable for LPSYS_LDOrs {}
///`write(|w| ..)` method takes [`lpsys_ldo::W`](W) writer structure
impl crate::Writable for LPSYS_LDOrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPSYS_LDO to value 0
impl crate::Resettable for LPSYS_LDOrs {
    const RESET_VALUE: u32 = 0;
}
