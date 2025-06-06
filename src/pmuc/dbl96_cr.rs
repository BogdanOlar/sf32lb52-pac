///Register `DBL96_CR` reader
pub type R = crate::R<DBL96_CRrs>;
///Register `DBL96_CR` writer
pub type W = crate::W<DBL96_CRrs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EN` reader -
pub type OutEnR = crate::BitReader;
///Field `OUT_EN` writer -
pub type OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TODIG_EN` reader -
pub type TodigEnR = crate::BitReader;
///Field `TODIG_EN` writer -
pub type TodigEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TODIG_STR` reader -
pub type TodigStrR = crate::FieldReader;
///Field `TODIG_STR` writer -
pub type TodigStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TORF_EN` reader -
pub type TorfEnR = crate::BitReader;
///Field `TORF_EN` writer -
pub type TorfEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOOSLO_EN` reader -
pub type ToosloEnR = crate::BitReader;
///Field `TOOSLO_EN` writer -
pub type ToosloEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOP_RSTB` reader -
pub type LoopRstbR = crate::BitReader;
///Field `LOOP_RSTB` writer -
pub type LoopRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PH_EN` reader -
pub type PhEnR = crate::FieldReader;
///Field `PH_EN` writer -
pub type PhEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DLY_EN` reader -
pub type DlyEnR = crate::FieldReader;
///Field `DLY_EN` writer -
pub type DlyEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DLY_EXT_EN` reader -
pub type DlyExtEnR = crate::BitReader;
///Field `DLY_EXT_EN` writer -
pub type DlyExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLY_SEL_EXT_EN` reader -
pub type DlySelExtEnR = crate::BitReader;
///Field `DLY_SEL_EXT_EN` writer -
pub type DlySelExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLY_SEL_EXT` reader -
pub type DlySelExtR = crate::FieldReader<u16>;
///Field `DLY_SEL_EXT` writer -
pub type DlySelExtW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn out_en(&self) -> OutEnR {
        OutEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn todig_en(&self) -> TodigEnR {
        TodigEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn todig_str(&self) -> TodigStrR {
        TodigStrR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5
    #[inline(always)]
    pub fn torf_en(&self) -> TorfEnR {
        TorfEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn tooslo_en(&self) -> ToosloEnR {
        ToosloEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn loop_rstb(&self) -> LoopRstbR {
        LoopRstbR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn ph_en(&self) -> PhEnR {
        PhEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn dly_en(&self) -> DlyEnR {
        DlyEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn dly_ext_en(&self) -> DlyExtEnR {
        DlyExtEnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn dly_sel_ext_en(&self) -> DlySelExtEnR {
        DlySelExtEnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:28
    #[inline(always)]
    pub fn dly_sel_ext(&self) -> DlySelExtR {
        DlySelExtR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBL96_CR")
            .field("dly_sel_ext", &self.dly_sel_ext())
            .field("dly_sel_ext_en", &self.dly_sel_ext_en())
            .field("dly_ext_en", &self.dly_ext_en())
            .field("dly_en", &self.dly_en())
            .field("ph_en", &self.ph_en())
            .field("loop_rstb", &self.loop_rstb())
            .field("tooslo_en", &self.tooslo_en())
            .field("torf_en", &self.torf_en())
            .field("todig_str", &self.todig_str())
            .field("todig_en", &self.todig_en())
            .field("out_en", &self.out_en())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DBL96_CRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn out_en(&mut self) -> OutEnW<DBL96_CRrs> {
        OutEnW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn todig_en(&mut self) -> TodigEnW<DBL96_CRrs> {
        TodigEnW::new(self, 2)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn todig_str(&mut self) -> TodigStrW<DBL96_CRrs> {
        TodigStrW::new(self, 3)
    }
    ///Bit 5
    #[inline(always)]
    pub fn torf_en(&mut self) -> TorfEnW<DBL96_CRrs> {
        TorfEnW::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn tooslo_en(&mut self) -> ToosloEnW<DBL96_CRrs> {
        ToosloEnW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn loop_rstb(&mut self) -> LoopRstbW<DBL96_CRrs> {
        LoopRstbW::new(self, 7)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn ph_en(&mut self) -> PhEnW<DBL96_CRrs> {
        PhEnW::new(self, 8)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn dly_en(&mut self) -> DlyEnW<DBL96_CRrs> {
        DlyEnW::new(self, 12)
    }
    ///Bit 16
    #[inline(always)]
    pub fn dly_ext_en(&mut self) -> DlyExtEnW<DBL96_CRrs> {
        DlyExtEnW::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn dly_sel_ext_en(&mut self) -> DlySelExtEnW<DBL96_CRrs> {
        DlySelExtEnW::new(self, 17)
    }
    ///Bits 18:28
    #[inline(always)]
    pub fn dly_sel_ext(&mut self) -> DlySelExtW<DBL96_CRrs> {
        DlySelExtW::new(self, 18)
    }
}
///DBL96 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbl96_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbl96_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBL96_CRrs;
impl crate::RegisterSpec for DBL96_CRrs {
    type Ux = u32;
}
///`read()` method returns [`dbl96_cr::R`](R) reader structure
impl crate::Readable for DBL96_CRrs {}
///`write(|w| ..)` method takes [`dbl96_cr::W`](W) writer structure
impl crate::Writable for DBL96_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBL96_CR to value 0
impl crate::Resettable for DBL96_CRrs {
    const RESET_VALUE: u32 = 0;
}
