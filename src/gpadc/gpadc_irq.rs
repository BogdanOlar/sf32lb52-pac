///Register `GPADC_IRQ` reader
pub type R = crate::R<GPADC_IRQrs>;
///Register `GPADC_IRQ` writer
pub type W = crate::W<GPADC_IRQrs>;
///Field `GPADC_ICR` reader -
pub type GpadcIcrR = crate::BitReader;
///Field `GPADC_ICR` writer -
pub type GpadcIcrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPADC_IMR` reader -
pub type GpadcImrR = crate::BitReader;
///Field `GPADC_IMR` writer -
pub type GpadcImrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPADC_IRSR` reader -
pub type GpadcIrsrR = crate::BitReader;
///Field `GPADC_IRSR` writer -
pub type GpadcIrsrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPADC_ISR` reader -
pub type GpadcIsrR = crate::BitReader;
///Field `GPADC_ISR` writer -
pub type GpadcIsrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn gpadc_icr(&self) -> GpadcIcrR {
        GpadcIcrR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn gpadc_imr(&self) -> GpadcImrR {
        GpadcImrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn gpadc_irsr(&self) -> GpadcIrsrR {
        GpadcIrsrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn gpadc_isr(&self) -> GpadcIsrR {
        GpadcIsrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPADC_IRQ")
            .field("gpadc_isr", &self.gpadc_isr())
            .field("gpadc_irsr", &self.gpadc_irsr())
            .field("gpadc_imr", &self.gpadc_imr())
            .field("gpadc_icr", &self.gpadc_icr())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn gpadc_icr(&mut self) -> GpadcIcrW<GPADC_IRQrs> {
        GpadcIcrW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn gpadc_imr(&mut self) -> GpadcImrW<GPADC_IRQrs> {
        GpadcImrW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn gpadc_irsr(&mut self) -> GpadcIrsrW<GPADC_IRQrs> {
        GpadcIrsrW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn gpadc_isr(&mut self) -> GpadcIsrW<GPADC_IRQrs> {
        GpadcIsrW::new(self, 3)
    }
}
///GPADC IRQ Register
///
///You can [`read`](crate::Reg::read) this register and get [`gpadc_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GPADC_IRQrs;
impl crate::RegisterSpec for GPADC_IRQrs {
    type Ux = u32;
}
///`read()` method returns [`gpadc_irq::R`](R) reader structure
impl crate::Readable for GPADC_IRQrs {}
///`write(|w| ..)` method takes [`gpadc_irq::W`](W) writer structure
impl crate::Writable for GPADC_IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPADC_IRQ to value 0
impl crate::Resettable for GPADC_IRQrs {
    const RESET_VALUE: u32 = 0;
}
