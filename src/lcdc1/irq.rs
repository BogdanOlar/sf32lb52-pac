///Register `IRQ` reader
pub type R = crate::R<IRQrs>;
///Register `IRQ` writer
pub type W = crate::W<IRQrs>;
///Field `EOF_STAT` reader - end of frame interrupt, masked by mask register
pub type EofStatR = crate::BitReader;
///Field `EOF_STAT` writer - end of frame interrupt, masked by mask register
pub type EofStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICB_OF_STAT` reader - icb overflow interrupt, masked by mask register
pub type IcbOfStatR = crate::BitReader;
///Field `ICB_OF_STAT` writer - icb overflow interrupt, masked by mask register
pub type IcbOfStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPIL_INTR_STAT` reader - dpi line interrupt, masked by mask register
pub type DpilIntrStatR = crate::BitReader;
///Field `DPIL_INTR_STAT` writer - dpi line interrupt, masked by mask register
pub type DpilIntrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_UDR_STAT` reader - dpi under run interrupt, masked by mask register
pub type DpiUdrStatR = crate::BitReader;
///Field `DPI_UDR_STAT` writer - dpi under run interrupt, masked by mask register
pub type DpiUdrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PARL_INTR_STAT` reader - jdi parallel interface line interrupt, masked by mask register
pub type JdiParlIntrStatR = crate::BitReader;
///Field `JDI_PARL_INTR_STAT` writer - jdi parallel interface line interrupt, masked by mask register
pub type JdiParlIntrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PAR_UDR_STAT` reader - jdi parallel interface under run interrupt, masked by mask register
pub type JdiParUdrStatR = crate::BitReader;
///Field `JDI_PAR_UDR_STAT` writer - jdi parallel interface under run interrupt, masked by mask register
pub type JdiParUdrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_DONE_STAT` reader - line process done interrupt, masked by mask register
pub type LineDoneStatR = crate::BitReader;
///Field `LINE_DONE_STAT` writer - line process done interrupt, masked by mask register
pub type LineDoneStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOF_RAW_STAT` reader - raw status of end of frame interrupt
pub type EofRawStatR = crate::BitReader;
///Field `EOF_RAW_STAT` writer - raw status of end of frame interrupt
pub type EofRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICB_OF_RAW_STAT` reader - raw status of icb overflow interrupt
pub type IcbOfRawStatR = crate::BitReader;
///Field `ICB_OF_RAW_STAT` writer - raw status of icb overflow interrupt
pub type IcbOfRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPIL_INTR_RAW_STAT` reader - raw status of dpi line interrupt
pub type DpilIntrRawStatR = crate::BitReader;
///Field `DPIL_INTR_RAW_STAT` writer - raw status of dpi line interrupt
pub type DpilIntrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_UDR_RAW_STAT` reader - raw status of dpi under run interrupt
pub type DpiUdrRawStatR = crate::BitReader;
///Field `DPI_UDR_RAW_STAT` writer - raw status of dpi under run interrupt
pub type DpiUdrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PARL_INTR_RAW_STAT` reader - raw_status of jdi parallel interface line interrupt
pub type JdiParlIntrRawStatR = crate::BitReader;
///Field `JDI_PARL_INTR_RAW_STAT` writer - raw_status of jdi parallel interface line interrupt
pub type JdiParlIntrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PAR_UDR_RAW_STAT` reader - raw_status of jdi parallel interface under run interrupt
pub type JdiParUdrRawStatR = crate::BitReader;
///Field `JDI_PAR_UDR_RAW_STAT` writer - raw_status of jdi parallel interface under run interrupt
pub type JdiParUdrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_DONE_RAW_STAT` reader - raw_status of line process done interrupt
pub type LineDoneRawStatR = crate::BitReader;
///Field `LINE_DONE_RAW_STAT` writer - raw_status of line process done interrupt
pub type LineDoneRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - end of frame interrupt, masked by mask register
    #[inline(always)]
    pub fn eof_stat(&self) -> EofStatR {
        EofStatR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - icb overflow interrupt, masked by mask register
    #[inline(always)]
    pub fn icb_of_stat(&self) -> IcbOfStatR {
        IcbOfStatR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dpi line interrupt, masked by mask register
    #[inline(always)]
    pub fn dpil_intr_stat(&self) -> DpilIntrStatR {
        DpilIntrStatR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - dpi under run interrupt, masked by mask register
    #[inline(always)]
    pub fn dpi_udr_stat(&self) -> DpiUdrStatR {
        DpiUdrStatR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - jdi parallel interface line interrupt, masked by mask register
    #[inline(always)]
    pub fn jdi_parl_intr_stat(&self) -> JdiParlIntrStatR {
        JdiParlIntrStatR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - jdi parallel interface under run interrupt, masked by mask register
    #[inline(always)]
    pub fn jdi_par_udr_stat(&self) -> JdiParUdrStatR {
        JdiParUdrStatR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - line process done interrupt, masked by mask register
    #[inline(always)]
    pub fn line_done_stat(&self) -> LineDoneStatR {
        LineDoneStatR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - raw status of end of frame interrupt
    #[inline(always)]
    pub fn eof_raw_stat(&self) -> EofRawStatR {
        EofRawStatR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - raw status of icb overflow interrupt
    #[inline(always)]
    pub fn icb_of_raw_stat(&self) -> IcbOfRawStatR {
        IcbOfRawStatR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - raw status of dpi line interrupt
    #[inline(always)]
    pub fn dpil_intr_raw_stat(&self) -> DpilIntrRawStatR {
        DpilIntrRawStatR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - raw status of dpi under run interrupt
    #[inline(always)]
    pub fn dpi_udr_raw_stat(&self) -> DpiUdrRawStatR {
        DpiUdrRawStatR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - raw_status of jdi parallel interface line interrupt
    #[inline(always)]
    pub fn jdi_parl_intr_raw_stat(&self) -> JdiParlIntrRawStatR {
        JdiParlIntrRawStatR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - raw_status of jdi parallel interface under run interrupt
    #[inline(always)]
    pub fn jdi_par_udr_raw_stat(&self) -> JdiParUdrRawStatR {
        JdiParUdrRawStatR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - raw_status of line process done interrupt
    #[inline(always)]
    pub fn line_done_raw_stat(&self) -> LineDoneRawStatR {
        LineDoneRawStatR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("line_done_raw_stat", &self.line_done_raw_stat())
            .field("jdi_par_udr_raw_stat", &self.jdi_par_udr_raw_stat())
            .field("jdi_parl_intr_raw_stat", &self.jdi_parl_intr_raw_stat())
            .field("dpi_udr_raw_stat", &self.dpi_udr_raw_stat())
            .field("dpil_intr_raw_stat", &self.dpil_intr_raw_stat())
            .field("icb_of_raw_stat", &self.icb_of_raw_stat())
            .field("eof_raw_stat", &self.eof_raw_stat())
            .field("line_done_stat", &self.line_done_stat())
            .field("jdi_par_udr_stat", &self.jdi_par_udr_stat())
            .field("jdi_parl_intr_stat", &self.jdi_parl_intr_stat())
            .field("dpi_udr_stat", &self.dpi_udr_stat())
            .field("dpil_intr_stat", &self.dpil_intr_stat())
            .field("icb_of_stat", &self.icb_of_stat())
            .field("eof_stat", &self.eof_stat())
            .finish()
    }
}
impl W {
    ///Bit 0 - end of frame interrupt, masked by mask register
    #[inline(always)]
    pub fn eof_stat(&mut self) -> EofStatW<IRQrs> {
        EofStatW::new(self, 0)
    }
    ///Bit 1 - icb overflow interrupt, masked by mask register
    #[inline(always)]
    pub fn icb_of_stat(&mut self) -> IcbOfStatW<IRQrs> {
        IcbOfStatW::new(self, 1)
    }
    ///Bit 2 - dpi line interrupt, masked by mask register
    #[inline(always)]
    pub fn dpil_intr_stat(&mut self) -> DpilIntrStatW<IRQrs> {
        DpilIntrStatW::new(self, 2)
    }
    ///Bit 3 - dpi under run interrupt, masked by mask register
    #[inline(always)]
    pub fn dpi_udr_stat(&mut self) -> DpiUdrStatW<IRQrs> {
        DpiUdrStatW::new(self, 3)
    }
    ///Bit 4 - jdi parallel interface line interrupt, masked by mask register
    #[inline(always)]
    pub fn jdi_parl_intr_stat(&mut self) -> JdiParlIntrStatW<IRQrs> {
        JdiParlIntrStatW::new(self, 4)
    }
    ///Bit 5 - jdi parallel interface under run interrupt, masked by mask register
    #[inline(always)]
    pub fn jdi_par_udr_stat(&mut self) -> JdiParUdrStatW<IRQrs> {
        JdiParUdrStatW::new(self, 5)
    }
    ///Bit 6 - line process done interrupt, masked by mask register
    #[inline(always)]
    pub fn line_done_stat(&mut self) -> LineDoneStatW<IRQrs> {
        LineDoneStatW::new(self, 6)
    }
    ///Bit 16 - raw status of end of frame interrupt
    #[inline(always)]
    pub fn eof_raw_stat(&mut self) -> EofRawStatW<IRQrs> {
        EofRawStatW::new(self, 16)
    }
    ///Bit 17 - raw status of icb overflow interrupt
    #[inline(always)]
    pub fn icb_of_raw_stat(&mut self) -> IcbOfRawStatW<IRQrs> {
        IcbOfRawStatW::new(self, 17)
    }
    ///Bit 18 - raw status of dpi line interrupt
    #[inline(always)]
    pub fn dpil_intr_raw_stat(&mut self) -> DpilIntrRawStatW<IRQrs> {
        DpilIntrRawStatW::new(self, 18)
    }
    ///Bit 19 - raw status of dpi under run interrupt
    #[inline(always)]
    pub fn dpi_udr_raw_stat(&mut self) -> DpiUdrRawStatW<IRQrs> {
        DpiUdrRawStatW::new(self, 19)
    }
    ///Bit 20 - raw_status of jdi parallel interface line interrupt
    #[inline(always)]
    pub fn jdi_parl_intr_raw_stat(&mut self) -> JdiParlIntrRawStatW<IRQrs> {
        JdiParlIntrRawStatW::new(self, 20)
    }
    ///Bit 21 - raw_status of jdi parallel interface under run interrupt
    #[inline(always)]
    pub fn jdi_par_udr_raw_stat(&mut self) -> JdiParUdrRawStatW<IRQrs> {
        JdiParUdrRawStatW::new(self, 21)
    }
    ///Bit 22 - raw_status of line process done interrupt
    #[inline(always)]
    pub fn line_done_raw_stat(&mut self) -> LineDoneRawStatW<IRQrs> {
        LineDoneRawStatW::new(self, 22)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRQrs;
impl crate::RegisterSpec for IRQrs {
    type Ux = u32;
}
///`read()` method returns [`irq::R`](R) reader structure
impl crate::Readable for IRQrs {}
///`write(|w| ..)` method takes [`irq::W`](W) writer structure
impl crate::Writable for IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ to value 0
impl crate::Resettable for IRQrs {
    const RESET_VALUE: u32 = 0;
}
