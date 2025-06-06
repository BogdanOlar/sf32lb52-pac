///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `SEL_SYS` reader - select clk_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
pub type SelSysR = crate::BitReader;
///Field `SEL_SYS` writer - select clk_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
pub type SelSysW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_SYS_LP` reader - select clk_lpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
pub type SelSysLpR = crate::BitReader;
///Field `SEL_SYS_LP` writer - select clk_lpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
pub type SelSysLpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_PERI` reader - select clk_peri_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
pub type SelPeriR = crate::BitReader;
///Field `SEL_PERI` writer - select clk_peri_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
pub type SelPeriW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_TICK` reader - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
pub type SelTickR = crate::FieldReader;
///Field `SEL_TICK` writer - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
pub type SelTickW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - select clk_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
    #[inline(always)]
    pub fn sel_sys(&self) -> SelSysR {
        SelSysR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - select clk_lpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
    #[inline(always)]
    pub fn sel_sys_lp(&self) -> SelSysLpR {
        SelSysLpR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - select clk_peri_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
    #[inline(always)]
    pub fn sel_peri(&self) -> SelPeriR {
        SelPeriR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
    #[inline(always)]
    pub fn sel_tick(&self) -> SelTickR {
        SelTickR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("sel_tick", &self.sel_tick())
            .field("sel_peri", &self.sel_peri())
            .field("sel_sys_lp", &self.sel_sys_lp())
            .field("sel_sys", &self.sel_sys())
            .finish()
    }
}
impl W {
    ///Bit 0 - select clk_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
    #[inline(always)]
    pub fn sel_sys(&mut self) -> SelSysW<CSRrs> {
        SelSysW::new(self, 0)
    }
    ///Bit 2 - select clk_lpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
    #[inline(always)]
    pub fn sel_sys_lp(&mut self) -> SelSysLpW<CSRrs> {
        SelSysLpW::new(self, 2)
    }
    ///Bit 4 - select clk_peri_lpsys source 0 - clk_hrc48; 1 - clk_hxt48
    #[inline(always)]
    pub fn sel_peri(&mut self) -> SelPeriW<CSRrs> {
        SelPeriW::new(self, 4)
    }
    ///Bits 5:6 - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
    #[inline(always)]
    pub fn sel_tick(&mut self) -> SelTickW<CSRrs> {
        SelTickW::new(self, 5)
    }
}
///Clock Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
