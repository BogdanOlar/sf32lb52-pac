///Register `GPADC_STATUS` reader
pub type R = crate::R<GPADC_STATUSrs>;
///Register `GPADC_STATUS` writer
pub type W = crate::W<GPADC_STATUSrs>;
///Field `ADC_DONE` reader -
pub type AdcDoneR = crate::BitReader;
///Field `ADC_DONE` writer -
pub type AdcDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOT_DONE` reader -
pub type SlotDoneR = crate::FieldReader;
///Field `SLOT_DONE` writer -
pub type SlotDoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CUR_SLOT` reader -
pub type CurSlotR = crate::FieldReader;
///Field `CUR_SLOT` writer -
pub type CurSlotW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new((self.bits & 1) != 0)
    }
    ///Bits 1:8
    #[inline(always)]
    pub fn slot_done(&self) -> SlotDoneR {
        SlotDoneR::new(((self.bits >> 1) & 0xff) as u8)
    }
    ///Bits 9:11
    #[inline(always)]
    pub fn cur_slot(&self) -> CurSlotR {
        CurSlotR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPADC_STATUS")
            .field("rsvd", &self.rsvd())
            .field("cur_slot", &self.cur_slot())
            .field("slot_done", &self.slot_done())
            .field("adc_done", &self.adc_done())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn adc_done(&mut self) -> AdcDoneW<GPADC_STATUSrs> {
        AdcDoneW::new(self, 0)
    }
    ///Bits 1:8
    #[inline(always)]
    pub fn slot_done(&mut self) -> SlotDoneW<GPADC_STATUSrs> {
        SlotDoneW::new(self, 1)
    }
    ///Bits 9:11
    #[inline(always)]
    pub fn cur_slot(&mut self) -> CurSlotW<GPADC_STATUSrs> {
        CurSlotW::new(self, 9)
    }
    ///Bits 12:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<GPADC_STATUSrs> {
        RsvdW::new(self, 12)
    }
}
///GPADC Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`gpadc_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GPADC_STATUSrs;
impl crate::RegisterSpec for GPADC_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`gpadc_status::R`](R) reader structure
impl crate::Readable for GPADC_STATUSrs {}
///`write(|w| ..)` method takes [`gpadc_status::W`](W) writer structure
impl crate::Writable for GPADC_STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPADC_STATUS to value 0
impl crate::Resettable for GPADC_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
