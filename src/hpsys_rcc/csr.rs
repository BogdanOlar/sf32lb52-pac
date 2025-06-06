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
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_MPI1` reader - selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi1R = crate::FieldReader;
///Field `SEL_MPI1` writer - selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEL_MPI2` reader - selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi2R = crate::FieldReader;
///Field `SEL_MPI2` writer - selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved
pub type SelMpi2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    ///Bit 3
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bits 8:9
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 3) as u8)
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
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("rsvd", &self.rsvd())
            .field("sel_usbc", &self.sel_usbc())
            .field("sel_tick", &self.sel_tick())
            .field("sel_peri", &self.sel_peri())
            .field("rsvd2", &self.rsvd2())
            .field("rsvd3", &self.rsvd3())
            .field("sel_mpi2", &self.sel_mpi2())
            .field("sel_mpi1", &self.sel_mpi1())
            .field("rsvd4", &self.rsvd4())
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
    ///Bit 3
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<CSRrs> {
        Rsvd4W::new(self, 3)
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
    ///Bits 8:9
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CSRrs> {
        Rsvd3W::new(self, 8)
    }
    ///Bits 10:11
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CSRrs> {
        Rsvd2W::new(self, 10)
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
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CSRrs> {
        RsvdW::new(self, 16)
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
