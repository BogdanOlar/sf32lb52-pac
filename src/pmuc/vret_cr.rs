///Register `VRET_CR` reader
pub type R = crate::R<VRET_CRrs>;
///Register `VRET_CR` writer
pub type W = crate::W<VRET_CRrs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BM` reader -
pub type BmR = crate::BitReader;
///Field `BM` writer -
pub type BmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBIT` reader -
pub type VbitR = crate::FieldReader;
///Field `VBIT` writer -
pub type VbitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIM` reader -
pub type TrimR = crate::FieldReader;
///Field `TRIM` writer -
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DLY` reader - VRET_LDO power up delay in number of CLK_LP cycles
pub type DlyR = crate::FieldReader;
///Field `DLY` writer - VRET_LDO power up delay in number of CLK_LP cycles
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
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn vbit(&self) -> VbitR {
        VbitR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bits 16:21 - VRET_LDO power up delay in number of CLK_LP cycles
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VRET_CR")
            .field("rdy", &self.rdy())
            .field("dly", &self.dly())
            .field("trim", &self.trim())
            .field("vbit", &self.vbit())
            .field("bm", &self.bm())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<VRET_CRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn bm(&mut self) -> BmW<VRET_CRrs> {
        BmW::new(self, 1)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn vbit(&mut self) -> VbitW<VRET_CRrs> {
        VbitW::new(self, 2)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<VRET_CRrs> {
        TrimW::new(self, 10)
    }
    ///Bits 16:21 - VRET_LDO power up delay in number of CLK_LP cycles
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<VRET_CRrs> {
        DlyW::new(self, 16)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<VRET_CRrs> {
        RdyW::new(self, 31)
    }
}
///VRET Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`vret_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vret_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VRET_CRrs;
impl crate::RegisterSpec for VRET_CRrs {
    type Ux = u32;
}
///`read()` method returns [`vret_cr::R`](R) reader structure
impl crate::Readable for VRET_CRrs {}
///`write(|w| ..)` method takes [`vret_cr::W`](W) writer structure
impl crate::Writable for VRET_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VRET_CR to value 0
impl crate::Resettable for VRET_CRrs {
    const RESET_VALUE: u32 = 0;
}
