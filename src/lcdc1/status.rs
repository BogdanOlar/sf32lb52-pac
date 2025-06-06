///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Register `STATUS` writer
pub type W = crate::W<STATUSrs>;
///Field `LCD_BUSY` reader - LCS controll busy flag
pub type LcdBusyR = crate::BitReader;
///Field `LCD_BUSY` writer - LCS controll busy flag
pub type LcdBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_RUN` reader - DPI interface is running
pub type DpiRunR = crate::BitReader;
///Field `DPI_RUN` writer - DPI interface is running
pub type DpiRunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PAR_RUN` reader - JDI parallel interface is running
pub type JdiParRunR = crate::BitReader;
///Field `JDI_PAR_RUN` writer - JDI parallel interface is running
pub type JdiParRunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LCS controll busy flag
    #[inline(always)]
    pub fn lcd_busy(&self) -> LcdBusyR {
        LcdBusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DPI interface is running
    #[inline(always)]
    pub fn dpi_run(&self) -> DpiRunR {
        DpiRunR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - JDI parallel interface is running
    #[inline(always)]
    pub fn jdi_par_run(&self) -> JdiParRunR {
        JdiParRunR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("jdi_par_run", &self.jdi_par_run())
            .field("dpi_run", &self.dpi_run())
            .field("lcd_busy", &self.lcd_busy())
            .finish()
    }
}
impl W {
    ///Bit 0 - LCS controll busy flag
    #[inline(always)]
    pub fn lcd_busy(&mut self) -> LcdBusyW<STATUSrs> {
        LcdBusyW::new(self, 0)
    }
    ///Bit 1 - DPI interface is running
    #[inline(always)]
    pub fn dpi_run(&mut self) -> DpiRunW<STATUSrs> {
        DpiRunW::new(self, 1)
    }
    ///Bit 2 - JDI parallel interface is running
    #[inline(always)]
    pub fn jdi_par_run(&mut self) -> JdiParRunW<STATUSrs> {
        JdiParRunW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
