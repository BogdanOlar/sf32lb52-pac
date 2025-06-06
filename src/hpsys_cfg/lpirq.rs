///Register `LPIRQ` reader
pub type R = crate::R<LPIRQrs>;
///Register `LPIRQ` writer
pub type W = crate::W<LPIRQrs>;
///Field `SEL0` reader - select hp2lp0 interrupt source
pub type Sel0R = crate::FieldReader;
///Field `SEL0` writer - select hp2lp0 interrupt source
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IF0` reader - hp2lp0 interrupt status. Write 1 to clear.
pub type If0R = crate::BitReader;
///Field `IF0` writer - hp2lp0 interrupt status. Write 1 to clear.
pub type If0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL1` reader - select hp2lp1 interrupt source
pub type Sel1R = crate::FieldReader;
///Field `SEL1` writer - select hp2lp1 interrupt source
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IF1` reader - hp2lp1 interrupt status. Write 1 to clear.
pub type If1R = crate::BitReader;
///Field `IF1` writer - hp2lp1 interrupt status. Write 1 to clear.
pub type If1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:5 - select hp2lp0 interrupt source
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - hp2lp0 interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn if0(&self) -> If0R {
        If0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - select hp2lp1 interrupt source
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - hp2lp1 interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn if1(&self) -> If1R {
        If1R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIRQ")
            .field("rsvd", &self.rsvd())
            .field("if1", &self.if1())
            .field("rsvd2", &self.rsvd2())
            .field("sel1", &self.sel1())
            .field("if0", &self.if0())
            .field("rsvd3", &self.rsvd3())
            .field("sel0", &self.sel0())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - select hp2lp0 interrupt source
    #[inline(always)]
    pub fn sel0(&mut self) -> Sel0W<LPIRQrs> {
        Sel0W::new(self, 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<LPIRQrs> {
        Rsvd3W::new(self, 6)
    }
    ///Bit 7 - hp2lp0 interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn if0(&mut self) -> If0W<LPIRQrs> {
        If0W::new(self, 7)
    }
    ///Bits 8:13 - select hp2lp1 interrupt source
    #[inline(always)]
    pub fn sel1(&mut self) -> Sel1W<LPIRQrs> {
        Sel1W::new(self, 8)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<LPIRQrs> {
        Rsvd2W::new(self, 14)
    }
    ///Bit 15 - hp2lp1 interrupt status. Write 1 to clear.
    #[inline(always)]
    pub fn if1(&mut self) -> If1W<LPIRQrs> {
        If1W::new(self, 15)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<LPIRQrs> {
        RsvdW::new(self, 16)
    }
}
///Interrupt Selection for LCPU
///
///You can [`read`](crate::Reg::read) this register and get [`lpirq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpirq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LPIRQrs;
impl crate::RegisterSpec for LPIRQrs {
    type Ux = u32;
}
///`read()` method returns [`lpirq::R`](R) reader structure
impl crate::Readable for LPIRQrs {}
///`write(|w| ..)` method takes [`lpirq::W`](W) writer structure
impl crate::Writable for LPIRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPIRQ to value 0
impl crate::Resettable for LPIRQrs {
    const RESET_VALUE: u32 = 0;
}
