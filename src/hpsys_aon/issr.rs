///Register `ISSR` reader
pub type R = crate::R<ISSRrs>;
///Register `ISSR` writer
pub type W = crate::W<ISSRrs>;
///Field `HP2LP_REQ` reader - Write 1 to request LPSYS to stay in active mode
pub type Hp2lpReqR = crate::BitReader;
///Field `HP2LP_REQ` writer - Write 1 to request LPSYS to stay in active mode
pub type Hp2lpReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_REQ` reader - Indicate LPSYS request exists
pub type Lp2hpReqR = crate::BitReader;
///Field `LP2HP_REQ` writer - Indicate LPSYS request exists
pub type Lp2hpReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HP_ACTIVE` reader - Write 1 to indicates HPSYS is active
pub type HpActiveR = crate::BitReader;
///Field `HP_ACTIVE` writer - Write 1 to indicates HPSYS is active
pub type HpActiveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_ACTIVE` reader - Read 1 indicates LPSYS is active
pub type LpActiveR = crate::BitReader;
///Field `LP_ACTIVE` writer - Read 1 indicates LPSYS is active
pub type LpActiveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bit 0 - Write 1 to request LPSYS to stay in active mode
    #[inline(always)]
    pub fn hp2lp_req(&self) -> Hp2lpReqR {
        Hp2lpReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indicate LPSYS request exists
    #[inline(always)]
    pub fn lp2hp_req(&self) -> Lp2hpReqR {
        Lp2hpReqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Write 1 to indicates HPSYS is active
    #[inline(always)]
    pub fn hp_active(&self) -> HpActiveR {
        HpActiveR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read 1 indicates LPSYS is active
    #[inline(always)]
    pub fn lp_active(&self) -> LpActiveR {
        LpActiveR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISSR")
            .field("rsvd", &self.rsvd())
            .field("lp_active", &self.lp_active())
            .field("hp_active", &self.hp_active())
            .field("rsvd2", &self.rsvd2())
            .field("lp2hp_req", &self.lp2hp_req())
            .field("hp2lp_req", &self.hp2lp_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - Write 1 to request LPSYS to stay in active mode
    #[inline(always)]
    pub fn hp2lp_req(&mut self) -> Hp2lpReqW<ISSRrs> {
        Hp2lpReqW::new(self, 0)
    }
    ///Bit 1 - Indicate LPSYS request exists
    #[inline(always)]
    pub fn lp2hp_req(&mut self) -> Lp2hpReqW<ISSRrs> {
        Lp2hpReqW::new(self, 1)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ISSRrs> {
        Rsvd2W::new(self, 2)
    }
    ///Bit 4 - Write 1 to indicates HPSYS is active
    #[inline(always)]
    pub fn hp_active(&mut self) -> HpActiveW<ISSRrs> {
        HpActiveW::new(self, 4)
    }
    ///Bit 5 - Read 1 indicates LPSYS is active
    #[inline(always)]
    pub fn lp_active(&mut self) -> LpActiveW<ISSRrs> {
        LpActiveW::new(self, 5)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ISSRrs> {
        RsvdW::new(self, 6)
    }
}
///Inter System Wakeup Register
///
///You can [`read`](crate::Reg::read) this register and get [`issr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`issr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISSRrs;
impl crate::RegisterSpec for ISSRrs {
    type Ux = u32;
}
///`read()` method returns [`issr::R`](R) reader structure
impl crate::Readable for ISSRrs {}
///`write(|w| ..)` method takes [`issr::W`](W) writer structure
impl crate::Writable for ISSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISSR to value 0
impl crate::Resettable for ISSRrs {
    const RESET_VALUE: u32 = 0;
}
