///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `UE` reader - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated when overflow occurred while repetition counter reached zero. UE flag can be cleared by writing 1 to the UECLR bit in the LPTIM_ICR register.
pub type UeR = crate::BitReader;
///Field `UE` writer - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated when overflow occurred while repetition counter reached zero. UE flag can be cleared by writing 1 to the UECLR bit in the LPTIM_ICR register.
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OF` reader - Overflow occurred OF is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. OF flag can be cleared by writing 1 to the OFCLR bit in the LPTIM_ICR register.
pub type OfR = crate::BitReader;
///Field `OF` writer - Overflow occurred OF is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. OF flag can be cleared by writing 1 to the OFCLR bit in the LPTIM_ICR register.
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC` reader - Output compare match The OC bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register’s value. OC flag can be cleared by writing 1 to the OCCLR bit in the LPTIM_ICR register.
pub type OcR = crate::BitReader;
///Field `OC` writer - Output compare match The OC bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register’s value. OC flag can be cleared by writing 1 to the OCCLR bit in the LPTIM_ICR register.
pub type OcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ET` reader - External trigger edge event ET is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. ET flag can be cleared by writing 1 to the ETCLR bit in the LPTIM_ICR register.
pub type EtR = crate::BitReader;
///Field `ET` writer - External trigger edge event ET is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. ET flag can be cleared by writing 1 to the ETCLR bit in the LPTIM_ICR register.
pub type EtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `UEWKUP` reader - Indicates update event wakeup occurred UEWKUP is set by hardware when an update event was generated (overflow occurred while repetition counter reached zero). To clear UEWKUP, first write 0 to the UEWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
pub type UewkupR = crate::BitReader;
///Field `UEWKUP` writer - Indicates update event wakeup occurred UEWKUP is set by hardware when an update event was generated (overflow occurred while repetition counter reached zero). To clear UEWKUP, first write 0 to the UEWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
pub type UewkupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFWKUP` reader - Indicates overflow wakeup occurred OFWKUP is set by hardware when LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. To clear OFWKUP, first write 0 to the OFWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
pub type OfwkupR = crate::BitReader;
///Field `OFWKUP` writer - Indicates overflow wakeup occurred OFWKUP is set by hardware when LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. To clear OFWKUP, first write 0 to the OFWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
pub type OfwkupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCWKUP` reader - Indicates output compare wakeup occurred The OCWKUP bit is set by hardware when LPTIM_CNT register value reached the LPTIM_CMP register’s value. To clear OCWKUP, first write 0 to the OCWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
pub type OcwkupR = crate::BitReader;
///Field `OCWKUP` writer - Indicates output compare wakeup occurred The OCWKUP bit is set by hardware when LPTIM_CNT register value reached the LPTIM_CMP register’s value. To clear OCWKUP, first write 0 to the OCWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
pub type OcwkupW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    ///Bit 0 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated when overflow occurred while repetition counter reached zero. UE flag can be cleared by writing 1 to the UECLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overflow occurred OF is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. OF flag can be cleared by writing 1 to the OFCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output compare match The OC bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register’s value. OC flag can be cleared by writing 1 to the OCCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - External trigger edge event ET is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. ET flag can be cleared by writing 1 to the ETCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn et(&self) -> EtR {
        EtR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Indicates update event wakeup occurred UEWKUP is set by hardware when an update event was generated (overflow occurred while repetition counter reached zero). To clear UEWKUP, first write 0 to the UEWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn uewkup(&self) -> UewkupR {
        UewkupR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Indicates overflow wakeup occurred OFWKUP is set by hardware when LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. To clear OFWKUP, first write 0 to the OFWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ofwkup(&self) -> OfwkupR {
        OfwkupR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Indicates output compare wakeup occurred The OCWKUP bit is set by hardware when LPTIM_CNT register value reached the LPTIM_CMP register’s value. To clear OCWKUP, first write 0 to the OCWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ocwkup(&self) -> OcwkupR {
        OcwkupR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rsvd", &self.rsvd())
            .field("ocwkup", &self.ocwkup())
            .field("ofwkup", &self.ofwkup())
            .field("uewkup", &self.uewkup())
            .field("rsvd2", &self.rsvd2())
            .field("et", &self.et())
            .field("oc", &self.oc())
            .field("of", &self.of())
            .field("ue", &self.ue())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated when overflow occurred while repetition counter reached zero. UE flag can be cleared by writing 1 to the UECLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<ISRrs> {
        UeW::new(self, 0)
    }
    ///Bit 1 - Overflow occurred OF is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. OF flag can be cleared by writing 1 to the OFCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn of(&mut self) -> OfW<ISRrs> {
        OfW::new(self, 1)
    }
    ///Bit 2 - Output compare match The OC bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register’s value. OC flag can be cleared by writing 1 to the OCCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<ISRrs> {
        OcW::new(self, 2)
    }
    ///Bit 3 - External trigger edge event ET is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. ET flag can be cleared by writing 1 to the ETCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn et(&mut self) -> EtW<ISRrs> {
        EtW::new(self, 3)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ISRrs> {
        Rsvd2W::new(self, 4)
    }
    ///Bit 8 - Indicates update event wakeup occurred UEWKUP is set by hardware when an update event was generated (overflow occurred while repetition counter reached zero). To clear UEWKUP, first write 0 to the UEWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn uewkup(&mut self) -> UewkupW<ISRrs> {
        UewkupW::new(self, 8)
    }
    ///Bit 9 - Indicates overflow wakeup occurred OFWKUP is set by hardware when LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. To clear OFWKUP, first write 0 to the OFWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ofwkup(&mut self) -> OfwkupW<ISRrs> {
        OfwkupW::new(self, 9)
    }
    ///Bit 10 - Indicates output compare wakeup occurred The OCWKUP bit is set by hardware when LPTIM_CNT register value reached the LPTIM_CMP register’s value. To clear OCWKUP, first write 0 to the OCWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ocwkup(&mut self) -> OcwkupW<ISRrs> {
        OcwkupW::new(self, 10)
    }
    ///Bits 11:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ISRrs> {
        RsvdW::new(self, 11)
    }
}
///LPTIM interrupt and status register
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
