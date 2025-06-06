///Register `SETTING` reader
pub type R = crate::R<SETTINGrs>;
///Register `SETTING` writer
pub type W = crate::W<SETTINGrs>;
///Field `EOF_MASK` reader - end of frame interrupt mask, 0: mask the interrupt
pub type EofMaskR = crate::BitReader;
///Field `EOF_MASK` writer - end of frame interrupt mask, 0: mask the interrupt
pub type EofMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICB_OF_MASK` reader - icb overflow interrupt mask, 0: mask the interrupt
pub type IcbOfMaskR = crate::BitReader;
///Field `ICB_OF_MASK` writer - icb overflow interrupt mask, 0: mask the interrupt
pub type IcbOfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPIL_INTR_MASK` reader - dpi line interrupt, 0: mask the interrupt
pub type DpilIntrMaskR = crate::BitReader;
///Field `DPIL_INTR_MASK` writer - dpi line interrupt, 0: mask the interrupt
pub type DpilIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_UDR_MASK` reader - dpi under run interrupt mask, 0: mask the interrupt
pub type DpiUdrMaskR = crate::BitReader;
///Field `DPI_UDR_MASK` writer - dpi under run interrupt mask, 0: mask the interrupt
pub type DpiUdrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PARL_INTR_MASK` reader - jdi parallel interface line interrupt, 0: mask the interrupt
pub type JdiParlIntrMaskR = crate::BitReader;
///Field `JDI_PARL_INTR_MASK` writer - jdi parallel interface line interrupt, 0: mask the interrupt
pub type JdiParlIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDI_PAR_UDR_MASK` reader - jdi parallel interface under run interrupt mask, 0: mask the interrupt
pub type JdiParUdrMaskR = crate::BitReader;
///Field `JDI_PAR_UDR_MASK` writer - jdi parallel interface under run interrupt mask, 0: mask the interrupt
pub type JdiParUdrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_DONE_MASK` reader - line process done interrupt, 0: mask the interrupt
pub type LineDoneMaskR = crate::BitReader;
///Field `LINE_DONE_MASK` writer - line process done interrupt, 0: mask the interrupt
pub type LineDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTO_GATE_EN` reader - auto clock gating enable
pub type AutoGateEnR = crate::BitReader;
///Field `AUTO_GATE_EN` writer - auto clock gating enable
pub type AutoGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_DONE_NUM` reader - line number of line process done interrupt
pub type LineDoneNumR = crate::FieldReader<u16>;
///Field `LINE_DONE_NUM` writer - line number of line process done interrupt
pub type LineDoneNumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bit 0 - end of frame interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn eof_mask(&self) -> EofMaskR {
        EofMaskR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - icb overflow interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn icb_of_mask(&self) -> IcbOfMaskR {
        IcbOfMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dpi line interrupt, 0: mask the interrupt
    #[inline(always)]
    pub fn dpil_intr_mask(&self) -> DpilIntrMaskR {
        DpilIntrMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - dpi under run interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn dpi_udr_mask(&self) -> DpiUdrMaskR {
        DpiUdrMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - jdi parallel interface line interrupt, 0: mask the interrupt
    #[inline(always)]
    pub fn jdi_parl_intr_mask(&self) -> JdiParlIntrMaskR {
        JdiParlIntrMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - jdi parallel interface under run interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn jdi_par_udr_mask(&self) -> JdiParUdrMaskR {
        JdiParUdrMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - line process done interrupt, 0: mask the interrupt
    #[inline(always)]
    pub fn line_done_mask(&self) -> LineDoneMaskR {
        LineDoneMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - auto clock gating enable
    #[inline(always)]
    pub fn auto_gate_en(&self) -> AutoGateEnR {
        AutoGateEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:26 - line number of line process done interrupt
    #[inline(always)]
    pub fn line_done_num(&self) -> LineDoneNumR {
        LineDoneNumR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETTING")
            .field("line_done_num", &self.line_done_num())
            .field("auto_gate_en", &self.auto_gate_en())
            .field("line_done_mask", &self.line_done_mask())
            .field("jdi_par_udr_mask", &self.jdi_par_udr_mask())
            .field("jdi_parl_intr_mask", &self.jdi_parl_intr_mask())
            .field("dpi_udr_mask", &self.dpi_udr_mask())
            .field("dpil_intr_mask", &self.dpil_intr_mask())
            .field("icb_of_mask", &self.icb_of_mask())
            .field("eof_mask", &self.eof_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - end of frame interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn eof_mask(&mut self) -> EofMaskW<SETTINGrs> {
        EofMaskW::new(self, 0)
    }
    ///Bit 1 - icb overflow interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn icb_of_mask(&mut self) -> IcbOfMaskW<SETTINGrs> {
        IcbOfMaskW::new(self, 1)
    }
    ///Bit 2 - dpi line interrupt, 0: mask the interrupt
    #[inline(always)]
    pub fn dpil_intr_mask(&mut self) -> DpilIntrMaskW<SETTINGrs> {
        DpilIntrMaskW::new(self, 2)
    }
    ///Bit 3 - dpi under run interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn dpi_udr_mask(&mut self) -> DpiUdrMaskW<SETTINGrs> {
        DpiUdrMaskW::new(self, 3)
    }
    ///Bit 4 - jdi parallel interface line interrupt, 0: mask the interrupt
    #[inline(always)]
    pub fn jdi_parl_intr_mask(&mut self) -> JdiParlIntrMaskW<SETTINGrs> {
        JdiParlIntrMaskW::new(self, 4)
    }
    ///Bit 5 - jdi parallel interface under run interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn jdi_par_udr_mask(&mut self) -> JdiParUdrMaskW<SETTINGrs> {
        JdiParUdrMaskW::new(self, 5)
    }
    ///Bit 6 - line process done interrupt, 0: mask the interrupt
    #[inline(always)]
    pub fn line_done_mask(&mut self) -> LineDoneMaskW<SETTINGrs> {
        LineDoneMaskW::new(self, 6)
    }
    ///Bit 8 - auto clock gating enable
    #[inline(always)]
    pub fn auto_gate_en(&mut self) -> AutoGateEnW<SETTINGrs> {
        AutoGateEnW::new(self, 8)
    }
    ///Bits 16:26 - line number of line process done interrupt
    #[inline(always)]
    pub fn line_done_num(&mut self) -> LineDoneNumW<SETTINGrs> {
        LineDoneNumW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SETTINGrs;
impl crate::RegisterSpec for SETTINGrs {
    type Ux = u32;
}
///`read()` method returns [`setting::R`](R) reader structure
impl crate::Readable for SETTINGrs {}
///`write(|w| ..)` method takes [`setting::W`](W) writer structure
impl crate::Writable for SETTINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SETTING to value 0
impl crate::Resettable for SETTINGrs {
    const RESET_VALUE: u32 = 0;
}
