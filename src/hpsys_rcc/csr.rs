///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `SEL_SYS` reader - select clk_hpsys source 0 - clk_hrc48; 1 - clk_hxt48; 2 - reserved; 3 - clk_dll1
pub type SelSysR = crate::FieldReader;
///Field `SEL_SYS` writer - select clk_hpsys source 0 - clk_hrc48; 1 - clk_hxt48; 2 - reserved; 3 - clk_dll1
pub type SelSysW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_SYS_LP` reader - select clk_hpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
pub type SelSysLpR = crate::BitReader;
///Field `SEL_SYS_LP` writer - select clk_hpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
pub type SelSysLpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_MPI1` reader - selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi1R = crate::FieldReader;
///Field `SEL_MPI1` writer - selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_MPI2` reader - selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi2R = crate::FieldReader;
///Field `SEL_MPI2` writer - selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_PERI` reader - select clk_peri_hpsys source used by USART/SPI/I2C/GPTIM2/BTIM2 0 - clk_hrc48; 1 - clk_hxt48
pub type SelPeriR = crate::BitReader;
///Field `SEL_PERI` writer - select clk_peri_hpsys source used by USART/SPI/I2C/GPTIM2/BTIM2 0 - clk_hrc48; 1 - clk_hxt48
pub type SelPeriW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_TICK` reader - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
pub type SelTickR = crate::FieldReader;
///Field `SEL_TICK` writer - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
pub type SelTickW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_USBC` reader - select USB source clock 0 - clk_hpsys; 1 - clk_dll2
pub type SelUsbcR = crate::BitReader;
///Field `SEL_USBC` writer - select USB source clock 0 - clk_hpsys; 1 - clk_dll2
pub type SelUsbcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - select clk_hpsys source 0 - clk_hrc48; 1 - clk_hxt48; 2 - reserved; 3 - clk_dll1
    #[inline(always)]
    pub fn sel_sys(&self) -> SelSysR {
        SelSysR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - select clk_hpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
    #[inline(always)]
    pub fn sel_sys_lp(&self) -> SelSysLpR {
        SelSysLpR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
    #[inline(always)]
    pub fn sel_mpi1(&self) -> SelMpi1R {
        SelMpi1R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
    #[inline(always)]
    pub fn sel_mpi2(&self) -> SelMpi2R {
        SelMpi2R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 12 - select clk_peri_hpsys source used by USART/SPI/I2C/GPTIM2/BTIM2 0 - clk_hrc48; 1 - clk_hxt48
    #[inline(always)]
    pub fn sel_peri(&self) -> SelPeriR {
        SelPeriR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
    #[inline(always)]
    pub fn sel_tick(&self) -> SelTickR {
        SelTickR::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - select USB source clock 0 - clk_hpsys; 1 - clk_dll2
    #[inline(always)]
    pub fn sel_usbc(&self) -> SelUsbcR {
        SelUsbcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("sel_usbc", &self.sel_usbc())
            .field("sel_tick", &self.sel_tick())
            .field("sel_peri", &self.sel_peri())
            .field("sel_mpi2", &self.sel_mpi2())
            .field("sel_mpi1", &self.sel_mpi1())
            .field("sel_sys_lp", &self.sel_sys_lp())
            .field("sel_sys", &self.sel_sys())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - select clk_hpsys source 0 - clk_hrc48; 1 - clk_hxt48; 2 - reserved; 3 - clk_dll1
    #[inline(always)]
    pub fn sel_sys(&mut self) -> SelSysW<CSRrs> {
        SelSysW::new(self, 0)
    }
    ///Bit 2 - select clk_hpsys source 0 - selected by SEL_SYS; 1 - clk_wdt
    #[inline(always)]
    pub fn sel_sys_lp(&mut self) -> SelSysLpW<CSRrs> {
        SelSysLpW::new(self, 2)
    }
    ///Bits 4:5 - selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
    #[inline(always)]
    pub fn sel_mpi1(&mut self) -> SelMpi1W<CSRrs> {
        SelMpi1W::new(self, 4)
    }
    ///Bits 6:7 - selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
    #[inline(always)]
    pub fn sel_mpi2(&mut self) -> SelMpi2W<CSRrs> {
        SelMpi2W::new(self, 6)
    }
    ///Bit 12 - select clk_peri_hpsys source used by USART/SPI/I2C/GPTIM2/BTIM2 0 - clk_hrc48; 1 - clk_hxt48
    #[inline(always)]
    pub fn sel_peri(&mut self) -> SelPeriW<CSRrs> {
        SelPeriW::new(self, 12)
    }
    ///Bits 13:14 - select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48
    #[inline(always)]
    pub fn sel_tick(&mut self) -> SelTickW<CSRrs> {
        SelTickW::new(self, 13)
    }
    ///Bit 15 - select USB source clock 0 - clk_hpsys; 1 - clk_dll2
    #[inline(always)]
    pub fn sel_usbc(&mut self) -> SelUsbcW<CSRrs> {
        SelUsbcW::new(self, 15)
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
