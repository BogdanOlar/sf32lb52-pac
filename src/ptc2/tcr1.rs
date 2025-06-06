///Register `TCR1` reader
pub type R = crate::R<TCR1rs>;
///Register `TCR1` writer
pub type W = crate::W<TCR1rs>;
///Field `TRIGSEL` reader - select trigger source 0: task will only be triggered by SWTRIG others: task will be triggered by selected source or SWTRIG
pub type TrigselR = crate::FieldReader;
///Field `TRIGSEL` writer - select trigger source 0: task will only be triggered by SWTRIG others: task will be triggered by selected source or SWTRIG
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `OP` reader - task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back
pub type OpR = crate::FieldReader;
///Field `OP` writer - task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back
pub type OpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGPOL` reader - trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger
pub type TrigpolR = crate::BitReader;
///Field `TRIGPOL` writer - trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger
pub type TrigpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIG` reader - software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically.
pub type SwtrigR = crate::BitReader;
///Field `SWTRIG` writer - software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically.
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPEN` reader - repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0
pub type RepenR = crate::BitReader;
///Field `REPEN` writer - repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0
pub type RepenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPTRIG` reader - repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times
pub type ReptrigR = crate::BitReader;
///Field `REPTRIG` writer - repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times
pub type ReptrigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPIRQ` reader - repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times
pub type RepirqR = crate::BitReader;
///Field `REPIRQ` writer - repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times
pub type RepirqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - select trigger source 0: task will only be triggered by SWTRIG others: task will be triggered by selected source or SWTRIG
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back
    #[inline(always)]
    pub fn op(&self) -> OpR {
        OpR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger
    #[inline(always)]
    pub fn trigpol(&self) -> TrigpolR {
        TrigpolR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically.
    #[inline(always)]
    pub fn swtrig(&self) -> SwtrigR {
        SwtrigR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0
    #[inline(always)]
    pub fn repen(&self) -> RepenR {
        RepenR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times
    #[inline(always)]
    pub fn reptrig(&self) -> ReptrigR {
        ReptrigR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times
    #[inline(always)]
    pub fn repirq(&self) -> RepirqR {
        RepirqR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR1")
            .field("repirq", &self.repirq())
            .field("reptrig", &self.reptrig())
            .field("repen", &self.repen())
            .field("swtrig", &self.swtrig())
            .field("trigpol", &self.trigpol())
            .field("op", &self.op())
            .field("trigsel", &self.trigsel())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - select trigger source 0: task will only be triggered by SWTRIG others: task will be triggered by selected source or SWTRIG
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<TCR1rs> {
        TrigselW::new(self, 0)
    }
    ///Bits 16:18 - task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back
    #[inline(always)]
    pub fn op(&mut self) -> OpW<TCR1rs> {
        OpW::new(self, 16)
    }
    ///Bit 19 - trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger
    #[inline(always)]
    pub fn trigpol(&mut self) -> TrigpolW<TCR1rs> {
        TrigpolW::new(self, 19)
    }
    ///Bit 20 - software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically.
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<TCR1rs> {
        SwtrigW::new(self, 20)
    }
    ///Bit 21 - repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0
    #[inline(always)]
    pub fn repen(&mut self) -> RepenW<TCR1rs> {
        RepenW::new(self, 21)
    }
    ///Bit 22 - repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times
    #[inline(always)]
    pub fn reptrig(&mut self) -> ReptrigW<TCR1rs> {
        ReptrigW::new(self, 22)
    }
    ///Bit 23 - repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times
    #[inline(always)]
    pub fn repirq(&mut self) -> RepirqW<TCR1rs> {
        RepirqW::new(self, 23)
    }
}
///task 1 control register
///
///You can [`read`](crate::Reg::read) this register and get [`tcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TCR1rs;
impl crate::RegisterSpec for TCR1rs {
    type Ux = u32;
}
///`read()` method returns [`tcr1::R`](R) reader structure
impl crate::Readable for TCR1rs {}
///`write(|w| ..)` method takes [`tcr1::W`](W) writer structure
impl crate::Writable for TCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TCR1 to value 0
impl crate::Resettable for TCR1rs {
    const RESET_VALUE: u32 = 0;
}
