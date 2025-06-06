///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `HDIV` reader - hclk_hpsys = clk_hpsys / HDIV if HDIV=0, hclk_hpsys = clk_hpsys
pub type HdivR = crate::FieldReader;
///Field `HDIV` writer - hclk_hpsys = clk_hpsys / HDIV if HDIV=0, hclk_hpsys = clk_hpsys
pub type HdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PDIV1` reader - pclk_hpsys = hclk_hpsys / (2^PDIV1), by default divided by 2
pub type Pdiv1R = crate::FieldReader;
///Field `PDIV1` writer - pclk_hpsys = hclk_hpsys / (2^PDIV1), by default divided by 2
pub type Pdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PDIV2` reader - pclk2_hpsys = hclk_hpsys / (2^PDIV2), by default divided by 16
pub type Pdiv2R = crate::FieldReader;
///Field `PDIV2` writer - pclk2_hpsys = hclk_hpsys / (2^PDIV2), by default divided by 16
pub type Pdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TICKDIV` reader - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
pub type TickdivR = crate::FieldReader;
///Field `TICKDIV` writer - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
pub type TickdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:7 - hclk_hpsys = clk_hpsys / HDIV if HDIV=0, hclk_hpsys = clk_hpsys
    #[inline(always)]
    pub fn hdiv(&self) -> HdivR {
        HdivR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - pclk_hpsys = hclk_hpsys / (2^PDIV1), by default divided by 2
    #[inline(always)]
    pub fn pdiv1(&self) -> Pdiv1R {
        Pdiv1R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - pclk2_hpsys = hclk_hpsys / (2^PDIV2), by default divided by 16
    #[inline(always)]
    pub fn pdiv2(&self) -> Pdiv2R {
        Pdiv2R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:21 - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
    #[inline(always)]
    pub fn tickdiv(&self) -> TickdivR {
        TickdivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("tickdiv", &self.tickdiv())
            .field("pdiv2", &self.pdiv2())
            .field("pdiv1", &self.pdiv1())
            .field("hdiv", &self.hdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - hclk_hpsys = clk_hpsys / HDIV if HDIV=0, hclk_hpsys = clk_hpsys
    #[inline(always)]
    pub fn hdiv(&mut self) -> HdivW<CFGRrs> {
        HdivW::new(self, 0)
    }
    ///Bits 8:10 - pclk_hpsys = hclk_hpsys / (2^PDIV1), by default divided by 2
    #[inline(always)]
    pub fn pdiv1(&mut self) -> Pdiv1W<CFGRrs> {
        Pdiv1W::new(self, 8)
    }
    ///Bits 12:14 - pclk2_hpsys = hclk_hpsys / (2^PDIV2), by default divided by 16
    #[inline(always)]
    pub fn pdiv2(&mut self) -> Pdiv2W<CFGRrs> {
        Pdiv2W::new(self, 12)
    }
    ///Bits 16:21 - systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV
    #[inline(always)]
    pub fn tickdiv(&mut self) -> TickdivW<CFGRrs> {
        TickdivW::new(self, 16)
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
