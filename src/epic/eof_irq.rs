///Register `EOF_IRQ` reader
pub type R = crate::R<EOF_IRQrs>;
///Register `EOF_IRQ` writer
pub type W = crate::W<EOF_IRQrs>;
///Field `IRQ_CAUSE` reader - end of frame interrupt, can be masked by EOF_IRQ_MASK
pub type IrqCauseR = crate::BitReader;
///Field `IRQ_CAUSE` writer - end of frame interrupt, can be masked by EOF_IRQ_MASK
pub type IrqCauseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_HIT_CAUSE` reader - line hit interrupt, can be masked by LINE_IRQ_MASK
pub type LineHitCauseR = crate::BitReader;
///Field `LINE_HIT_CAUSE` writer - line hit interrupt, can be masked by LINE_IRQ_MASK
pub type LineHitCauseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader<u16>;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `IRQ_STATUS` reader - raw status of end of frame interrupt
pub type IrqStatusR = crate::BitReader;
///Field `IRQ_STATUS` writer - raw status of end of frame interrupt
pub type IrqStatusW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_HIT_STATUS` reader - raw status of line hit interrupt
pub type LineHitStatusR = crate::BitReader;
///Field `LINE_HIT_STATUS` writer - raw status of line hit interrupt
pub type LineHitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0 - end of frame interrupt, can be masked by EOF_IRQ_MASK
    #[inline(always)]
    pub fn irq_cause(&self) -> IrqCauseR {
        IrqCauseR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - line hit interrupt, can be masked by LINE_IRQ_MASK
    #[inline(always)]
    pub fn line_hit_cause(&self) -> LineHitCauseR {
        LineHitCauseR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bit 16 - raw status of end of frame interrupt
    #[inline(always)]
    pub fn irq_status(&self) -> IrqStatusR {
        IrqStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - raw status of line hit interrupt
    #[inline(always)]
    pub fn line_hit_status(&self) -> LineHitStatusR {
        LineHitStatusR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EOF_IRQ")
            .field("rsvd", &self.rsvd())
            .field("line_hit_status", &self.line_hit_status())
            .field("irq_status", &self.irq_status())
            .field("rsvd2", &self.rsvd2())
            .field("line_hit_cause", &self.line_hit_cause())
            .field("irq_cause", &self.irq_cause())
            .finish()
    }
}
impl W {
    ///Bit 0 - end of frame interrupt, can be masked by EOF_IRQ_MASK
    #[inline(always)]
    pub fn irq_cause(&mut self) -> IrqCauseW<EOF_IRQrs> {
        IrqCauseW::new(self, 0)
    }
    ///Bit 1 - line hit interrupt, can be masked by LINE_IRQ_MASK
    #[inline(always)]
    pub fn line_hit_cause(&mut self) -> LineHitCauseW<EOF_IRQrs> {
        LineHitCauseW::new(self, 1)
    }
    ///Bits 2:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<EOF_IRQrs> {
        Rsvd2W::new(self, 2)
    }
    ///Bit 16 - raw status of end of frame interrupt
    #[inline(always)]
    pub fn irq_status(&mut self) -> IrqStatusW<EOF_IRQrs> {
        IrqStatusW::new(self, 16)
    }
    ///Bit 17 - raw status of line hit interrupt
    #[inline(always)]
    pub fn line_hit_status(&mut self) -> LineHitStatusW<EOF_IRQrs> {
        LineHitStatusW::new(self, 17)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<EOF_IRQrs> {
        RsvdW::new(self, 18)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`eof_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eof_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EOF_IRQrs;
impl crate::RegisterSpec for EOF_IRQrs {
    type Ux = u32;
}
///`read()` method returns [`eof_irq::R`](R) reader structure
impl crate::Readable for EOF_IRQrs {}
///`write(|w| ..)` method takes [`eof_irq::W`](W) writer structure
impl crate::Writable for EOF_IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EOF_IRQ to value 0
impl crate::Resettable for EOF_IRQrs {
    const RESET_VALUE: u32 = 0;
}
