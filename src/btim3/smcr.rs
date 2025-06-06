///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::FieldReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TS` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
pub type TsR = crate::FieldReader;
///Field `TS` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSM` reader - Master/Slave mode. This bit should be asserted on master timer if synchronization if needed. 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
pub type MsmR = crate::BitReader;
///Field `MSM` writer - Master/Slave mode. This bit should be asserted on master timer if synchronization if needed. 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SMS` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 000: Slave mode disabled. 001: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 010: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 011: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. 100: External Clock Mode - Rising edges of the selected trigger (TRGI) clock the counter.
pub type SmsR = crate::FieldReader;
///Field `SMS` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 000: Slave mode disabled. 001: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 010: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 011: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. 100: External Clock Mode - Rising edges of the selected trigger (TRGI) clock the counter.
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTS` reader - Gating trigger selection in gated mode This bit-field selects the trigger input to be used to enable the counter gating. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
pub type GtsR = crate::FieldReader;
///Field `GTS` writer - Gating trigger selection in gated mode This bit-field selects the trigger input to be used to enable the counter gating. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
pub type GtsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GTP` reader - Gating trigger polarity invert 0: active at high level 1: active at low level
pub type GtpR = crate::BitReader;
///Field `GTP` writer - Gating trigger polarity invert 0: active at high level 1: active at low level
pub type GtpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GM` reader - Gated Mode. The counter clock is enabled when the selected trigger input (TRGI) is active (according to gating trigger polarity). The counter stops (but is not reset) as soon as the trigger becomes inactive. Both start and stop of the counter are controlled. Gated mode and slave mode can be enabled simutanuously with different trigger selection.
pub type GmR = crate::BitReader;
///Field `GM` writer - Gated Mode. The counter clock is enabled when the selected trigger input (TRGI) is active (according to gating trigger polarity). The counter stops (but is not reset) as soon as the trigger becomes inactive. Both start and stop of the counter are controlled. Gated mode and slave mode can be enabled simutanuously with different trigger selection.
pub type GmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master/Slave mode. This bit should be asserted on master timer if synchronization if needed. 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:18 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 000: Slave mode disabled. 001: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 010: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 011: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. 100: External Clock Mode - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Gating trigger selection in gated mode This bit-field selects the trigger input to be used to enable the counter gating. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
    #[inline(always)]
    pub fn gts(&self) -> GtsR {
        GtsR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Gating trigger polarity invert 0: active at high level 1: active at low level
    #[inline(always)]
    pub fn gtp(&self) -> GtpR {
        GtpR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Gated Mode. The counter clock is enabled when the selected trigger input (TRGI) is active (according to gating trigger polarity). The counter stops (but is not reset) as soon as the trigger becomes inactive. Both start and stop of the counter are controlled. Gated mode and slave mode can be enabled simutanuously with different trigger selection.
    #[inline(always)]
    pub fn gm(&self) -> GmR {
        GmR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("rsvd", &self.rsvd())
            .field("gm", &self.gm())
            .field("gtp", &self.gtp())
            .field("gts", &self.gts())
            .field("rsvd2", &self.rsvd2())
            .field("sms", &self.sms())
            .field("rsvd3", &self.rsvd3())
            .field("msm", &self.msm())
            .field("rsvd4", &self.rsvd4())
            .field("ts", &self.ts())
            .field("rsvd5", &self.rsvd5())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<SMCRrs> {
        Rsvd5W::new(self, 0)
    }
    ///Bits 4:5 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<SMCRrs> {
        TsW::new(self, 4)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<SMCRrs> {
        Rsvd4W::new(self, 6)
    }
    ///Bit 7 - Master/Slave mode. This bit should be asserted on master timer if synchronization if needed. 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<SMCRrs> {
        MsmW::new(self, 7)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<SMCRrs> {
        Rsvd3W::new(self, 8)
    }
    ///Bits 16:18 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 000: Slave mode disabled. 001: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 010: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 011: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. 100: External Clock Mode - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<SMCRrs> {
        SmsW::new(self, 16)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<SMCRrs> {
        Rsvd2W::new(self, 19)
    }
    ///Bits 20:21 - Gating trigger selection in gated mode This bit-field selects the trigger input to be used to enable the counter gating. 00: Internal Trigger 0 (ITR0) 01: Internal Trigger 1 (ITR1) 10: Internal Trigger 2 (ITR2) 11: Internal Trigger 3 (ITR3)
    #[inline(always)]
    pub fn gts(&mut self) -> GtsW<SMCRrs> {
        GtsW::new(self, 20)
    }
    ///Bit 22 - Gating trigger polarity invert 0: active at high level 1: active at low level
    #[inline(always)]
    pub fn gtp(&mut self) -> GtpW<SMCRrs> {
        GtpW::new(self, 22)
    }
    ///Bit 23 - Gated Mode. The counter clock is enabled when the selected trigger input (TRGI) is active (according to gating trigger polarity). The counter stops (but is not reset) as soon as the trigger becomes inactive. Both start and stop of the counter are controlled. Gated mode and slave mode can be enabled simutanuously with different trigger selection.
    #[inline(always)]
    pub fn gm(&mut self) -> GmW<SMCRrs> {
        GmW::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SMCRrs> {
        RsvdW::new(self, 24)
    }
}
///TIM slave mode control register
///
///You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
