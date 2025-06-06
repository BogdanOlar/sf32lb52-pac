///Register `TSEN_IRQ` reader
pub type R = crate::R<TSEN_IRQrs>;
///Register `TSEN_IRQ` writer
pub type W = crate::W<TSEN_IRQrs>;
///Field `TSEN_ICR` reader -
pub type TsenIcrR = crate::BitReader;
///Field `TSEN_ICR` writer -
pub type TsenIcrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN_IMR` reader -
pub type TsenImrR = crate::BitReader;
///Field `TSEN_IMR` writer -
pub type TsenImrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN_IRSR` reader -
pub type TsenIrsrR = crate::BitReader;
///Field `TSEN_IRSR` writer -
pub type TsenIrsrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN_ISR` reader -
pub type TsenIsrR = crate::BitReader;
///Field `TSEN_ISR` writer -
pub type TsenIsrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn tsen_icr(&self) -> TsenIcrR {
        TsenIcrR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn tsen_imr(&self) -> TsenImrR {
        TsenImrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn tsen_irsr(&self) -> TsenIrsrR {
        TsenIrsrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn tsen_isr(&self) -> TsenIsrR {
        TsenIsrR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSEN_IRQ")
            .field("rsvd", &self.rsvd())
            .field("tsen_isr", &self.tsen_isr())
            .field("tsen_irsr", &self.tsen_irsr())
            .field("tsen_imr", &self.tsen_imr())
            .field("tsen_icr", &self.tsen_icr())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn tsen_icr(&mut self) -> TsenIcrW<TSEN_IRQrs> {
        TsenIcrW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn tsen_imr(&mut self) -> TsenImrW<TSEN_IRQrs> {
        TsenImrW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn tsen_irsr(&mut self) -> TsenIrsrW<TSEN_IRQrs> {
        TsenIrsrW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn tsen_isr(&mut self) -> TsenIsrW<TSEN_IRQrs> {
        TsenIsrW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TSEN_IRQrs> {
        RsvdW::new(self, 4)
    }
}
///Tsen IRQ Register
///
///You can [`read`](crate::Reg::read) this register and get [`tsen_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TSEN_IRQrs;
impl crate::RegisterSpec for TSEN_IRQrs {
    type Ux = u32;
}
///`read()` method returns [`tsen_irq::R`](R) reader structure
impl crate::Readable for TSEN_IRQrs {}
///`write(|w| ..)` method takes [`tsen_irq::W`](W) writer structure
impl crate::Writable for TSEN_IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSEN_IRQ to value 0
impl crate::Resettable for TSEN_IRQrs {
    const RESET_VALUE: u32 = 0;
}
