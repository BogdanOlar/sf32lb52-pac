///Register `SETTING` reader
pub type R = crate::R<SETTINGrs>;
///Register `SETTING` writer
pub type W = crate::W<SETTINGrs>;
///Field `EOF_IRQ_MASK` reader - end of frame interrupt mask, 0: mask the interrupt
pub type EofIrqMaskR = crate::BitReader;
///Field `EOF_IRQ_MASK` writer - end of frame interrupt mask, 0: mask the interrupt
pub type EofIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_IRQ_MASK` reader - canvas line hit interrupt mask, 0: mask the interrupt
pub type LineIrqMaskR = crate::BitReader;
///Field `LINE_IRQ_MASK` writer - canvas line hit interrupt mask, 0: mask the interrupt
pub type LineIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTO_GATE_EN` reader - auto clock gating enable
pub type AutoGateEnR = crate::BitReader;
///Field `AUTO_GATE_EN` writer - auto clock gating enable
pub type AutoGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_IRQ_NUM` reader - canvas line hit interrupt line number
pub type LineIrqNumR = crate::FieldReader<u16>;
///Field `LINE_IRQ_NUM` writer - canvas line hit interrupt line number
pub type LineIrqNumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bit 0 - end of frame interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn eof_irq_mask(&self) -> EofIrqMaskR {
        EofIrqMaskR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - canvas line hit interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn line_irq_mask(&self) -> LineIrqMaskR {
        LineIrqMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - auto clock gating enable
    #[inline(always)]
    pub fn auto_gate_en(&self) -> AutoGateEnR {
        AutoGateEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 16:25 - canvas line hit interrupt line number
    #[inline(always)]
    pub fn line_irq_num(&self) -> LineIrqNumR {
        LineIrqNumR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETTING")
            .field("line_irq_num", &self.line_irq_num())
            .field("auto_gate_en", &self.auto_gate_en())
            .field("line_irq_mask", &self.line_irq_mask())
            .field("eof_irq_mask", &self.eof_irq_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - end of frame interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn eof_irq_mask(&mut self) -> EofIrqMaskW<SETTINGrs> {
        EofIrqMaskW::new(self, 0)
    }
    ///Bit 1 - canvas line hit interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn line_irq_mask(&mut self) -> LineIrqMaskW<SETTINGrs> {
        LineIrqMaskW::new(self, 1)
    }
    ///Bit 2 - auto clock gating enable
    #[inline(always)]
    pub fn auto_gate_en(&mut self) -> AutoGateEnW<SETTINGrs> {
        AutoGateEnW::new(self, 2)
    }
    ///Bits 16:25 - canvas line hit interrupt line number
    #[inline(always)]
    pub fn line_irq_num(&mut self) -> LineIrqNumW<SETTINGrs> {
        LineIrqNumW::new(self, 16)
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
