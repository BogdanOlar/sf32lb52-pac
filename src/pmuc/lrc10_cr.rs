///Register `LRC10_CR` reader
pub type R = crate::R<LRC10_CRrs>;
///Register `LRC10_CR` writer
pub type W = crate::W<LRC10_CRrs>;
///Field `EN` reader - Enabled by default
pub type EnR = crate::BitReader;
///Field `EN` writer - Enabled by default
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPBM1` reader -
pub type Cmpbm1R = crate::FieldReader;
///Field `CMPBM1` writer -
pub type Cmpbm1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CMPBM2` reader -
pub type Cmpbm2R = crate::BitReader;
///Field `CMPBM2` writer -
pub type Cmpbm2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHGCRT` reader -
pub type ChgcrtR = crate::FieldReader;
///Field `CHGCRT` writer -
pub type ChgcrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHGCAP` reader -
pub type ChgcapR = crate::FieldReader;
///Field `CHGCAP` writer -
pub type ChgcapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REFRES` reader -
pub type RefresR = crate::BitReader;
///Field `REFRES` writer -
pub type RefresW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDY` reader -
pub type RdyR = crate::BitReader;
///Field `RDY` writer -
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enabled by default
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2
    #[inline(always)]
    pub fn cmpbm1(&self) -> Cmpbm1R {
        Cmpbm1R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3
    #[inline(always)]
    pub fn cmpbm2(&self) -> Cmpbm2R {
        Cmpbm2R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn chgcrt(&self) -> ChgcrtR {
        ChgcrtR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn chgcap(&self) -> ChgcapR {
        ChgcapR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn refres(&self) -> RefresR {
        RefresR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LRC10_CR")
            .field("rdy", &self.rdy())
            .field("refres", &self.refres())
            .field("chgcap", &self.chgcap())
            .field("chgcrt", &self.chgcrt())
            .field("cmpbm2", &self.cmpbm2())
            .field("cmpbm1", &self.cmpbm1())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enabled by default
    #[inline(always)]
    pub fn en(&mut self) -> EnW<LRC10_CRrs> {
        EnW::new(self, 0)
    }
    ///Bits 1:2
    #[inline(always)]
    pub fn cmpbm1(&mut self) -> Cmpbm1W<LRC10_CRrs> {
        Cmpbm1W::new(self, 1)
    }
    ///Bit 3
    #[inline(always)]
    pub fn cmpbm2(&mut self) -> Cmpbm2W<LRC10_CRrs> {
        Cmpbm2W::new(self, 3)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn chgcrt(&mut self) -> ChgcrtW<LRC10_CRrs> {
        ChgcrtW::new(self, 4)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn chgcap(&mut self) -> ChgcapW<LRC10_CRrs> {
        ChgcapW::new(self, 6)
    }
    ///Bit 8
    #[inline(always)]
    pub fn refres(&mut self) -> RefresW<LRC10_CRrs> {
        RefresW::new(self, 8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<LRC10_CRrs> {
        RdyW::new(self, 31)
    }
}
///RC10K Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lrc10_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrc10_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LRC10_CRrs;
impl crate::RegisterSpec for LRC10_CRrs {
    type Ux = u32;
}
///`read()` method returns [`lrc10_cr::R`](R) reader structure
impl crate::Readable for LRC10_CRrs {}
///`write(|w| ..)` method takes [`lrc10_cr::W`](W) writer structure
impl crate::Writable for LRC10_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LRC10_CR to value 0
impl crate::Resettable for LRC10_CRrs {
    const RESET_VALUE: u32 = 0;
}
