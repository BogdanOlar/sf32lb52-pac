///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `HDIV1` reader - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
pub type Hdiv1R = crate::FieldReader;
///Field `HDIV1` writer - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
pub type Hdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PDIV1` reader - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
pub type Pdiv1R = crate::FieldReader;
///Field `PDIV1` writer - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
pub type Pdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PDIV2` reader - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
pub type Pdiv2R = crate::FieldReader;
///Field `PDIV2` writer - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
pub type Pdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
impl R {
    ///Bits 0:5 - hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys
    #[inline(always)]
    pub fn hdiv1(&self) -> Hdiv1R {
        Hdiv1R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:10 - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
    #[inline(always)]
    pub fn pdiv1(&self) -> Pdiv1R {
        Pdiv1R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
    #[inline(always)]
    pub fn pdiv2(&self) -> Pdiv2R {
        Pdiv2R::new(((self.bits >> 12) & 7) as u8)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("tickdiv", &self.tickdiv())
            .field("macfreq", &self.macfreq())
            .field("macdiv", &self.macdiv())
            .field("pdiv2", &self.pdiv2())
            .field("pdiv1", &self.pdiv1())
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
    ///Bits 8:10 - pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2
    #[inline(always)]
    pub fn pdiv1(&mut self) -> Pdiv1W<CFGRrs> {
        Pdiv1W::new(self, 8)
    }
    ///Bits 12:14 - pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32
    #[inline(always)]
    pub fn pdiv2(&mut self) -> Pdiv2W<CFGRrs> {
        Pdiv2W::new(self, 12)
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
