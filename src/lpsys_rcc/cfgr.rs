///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `HDIV1` reader - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
pub type Hdiv1R = crate::FieldReader;
///Field `HDIV1` writer - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
pub type Hdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::FieldReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PDIV1` reader - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
pub type Pdiv1R = crate::FieldReader;
///Field `PDIV1` writer - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
pub type Pdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDIV2` reader - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
pub type Pdiv2R = crate::FieldReader;
///Field `PDIV2` writer - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
pub type Pdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MACDIV` reader - MAC clock divider MACCLK = hclk_lpsys / MACDIV
pub type MacdivR = crate::FieldReader;
///Field `MACDIV` writer - MAC clock divider MACCLK = hclk_lpsys / MACDIV
pub type MacdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MACFREQ` reader - clock frequency of MAC clock
pub type MacfreqR = crate::FieldReader;
///Field `MACFREQ` writer - clock frequency of MAC clock
pub type MacfreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TICKDIV` reader - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
pub type TickdivR = crate::FieldReader;
///Field `TICKDIV` writer - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
pub type TickdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::BitReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
    #[inline(always)]
    pub fn hdiv1(&self) -> Hdiv1R {
        Hdiv1R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
    #[inline(always)]
    pub fn pdiv1(&self) -> Pdiv1R {
        Pdiv1R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
    #[inline(always)]
    pub fn pdiv2(&self) -> Pdiv2R {
        Pdiv2R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - MAC clock divider MACCLK = hclk_lpsys / MACDIV
    #[inline(always)]
    pub fn macdiv(&self) -> MacdivR {
        MacdivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:24 - clock frequency of MAC clock
    #[inline(always)]
    pub fn macfreq(&self) -> MacfreqR {
        MacfreqR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:30 - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
    #[inline(always)]
    pub fn tickdiv(&self) -> TickdivR {
        TickdivR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("rsvd", &self.rsvd())
            .field("tickdiv", &self.tickdiv())
            .field("macfreq", &self.macfreq())
            .field("macdiv", &self.macdiv())
            .field("rsvd2", &self.rsvd2())
            .field("pdiv2", &self.pdiv2())
            .field("rsvd3", &self.rsvd3())
            .field("pdiv1", &self.pdiv1())
            .field("rsvd4", &self.rsvd4())
            .field("hdiv1", &self.hdiv1())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
    #[inline(always)]
    pub fn hdiv1(&mut self) -> Hdiv1W<CFGRrs> {
        Hdiv1W::new(self, 0)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<CFGRrs> {
        Rsvd4W::new(self, 6)
    }
    ///Bits 8:10 - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
    #[inline(always)]
    pub fn pdiv1(&mut self) -> Pdiv1W<CFGRrs> {
        Pdiv1W::new(self, 8)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CFGRrs> {
        Rsvd3W::new(self, 11)
    }
    ///Bits 12:14 - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
    #[inline(always)]
    pub fn pdiv2(&mut self) -> Pdiv2W<CFGRrs> {
        Pdiv2W::new(self, 12)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CFGRrs> {
        Rsvd2W::new(self, 15)
    }
    ///Bits 16:19 - MAC clock divider MACCLK = hclk_lpsys / MACDIV
    #[inline(always)]
    pub fn macdiv(&mut self) -> MacdivW<CFGRrs> {
        MacdivW::new(self, 16)
    }
    ///Bits 20:24 - clock frequency of MAC clock
    #[inline(always)]
    pub fn macfreq(&mut self) -> MacfreqW<CFGRrs> {
        MacfreqW::new(self, 20)
    }
    ///Bits 25:30 - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
    #[inline(always)]
    pub fn tickdiv(&mut self) -> TickdivW<CFGRrs> {
        TickdivW::new(self, 25)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CFGRrs> {
        RsvdW::new(self, 31)
    }
}
///Clock Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
