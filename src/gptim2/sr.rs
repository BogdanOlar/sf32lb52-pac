///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow or underflow and if UDIS=0 in the CR1 register. When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register.
pub type UifR = crate::BitReader;
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow or underflow and if UDIS=0 in the CR1 register. When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register.
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IF` reader - Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register (An edge has been detected on IC1 which matches the selected polarity).
pub type Cc1ifR = crate::BitReader;
///Field `CC1IF` writer - Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register (An edge has been detected on IC1 which matches the selected polarity).
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag
pub type Cc2ifR = crate::BitReader;
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IF` reader - Capture/Compare 3 interrupt flag
pub type Cc3ifR = crate::BitReader;
///Field `CC3IF` writer - Capture/Compare 3 interrupt flag
pub type Cc3ifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IF` reader - Capture/Compare 4 interrupt flag
pub type Cc4ifR = crate::BitReader;
///Field `CC4IF` writer - Capture/Compare 4 interrupt flag
pub type Cc4ifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode). It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending.
pub type TifR = crate::BitReader;
///Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode). It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending.
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set
pub type Cc1ofR = crate::BitReader;
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OF` reader - Capture/Compare 2 overcapture flag
pub type Cc2ofR = crate::BitReader;
///Field `CC2OF` writer - Capture/Compare 2 overcapture flag
pub type Cc2ofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3OF` reader - Capture/Compare 3 overcapture flag
pub type Cc3ofR = crate::BitReader;
///Field `CC3OF` writer - Capture/Compare 3 overcapture flag
pub type Cc3ofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4OF` reader - Capture/Compare 4 overcapture flag
pub type Cc4ofR = crate::BitReader;
///Field `CC4OF` writer - Capture/Compare 4 overcapture flag
pub type Cc4ofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow or underflow and if UDIS=0 in the CR1 register. When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register (An edge has been detected on IC1 which matches the selected polarity).
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode). It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending.
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&self) -> Cc4ofR {
        Cc4ofR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("cc4of", &self.cc4of())
            .field("cc3of", &self.cc3of())
            .field("cc2of", &self.cc2of())
            .field("cc1of", &self.cc1of())
            .field("tif", &self.tif())
            .field("cc4if", &self.cc4if())
            .field("cc3if", &self.cc3if())
            .field("cc2if", &self.cc2if())
            .field("cc1if", &self.cc1if())
            .field("uif", &self.uif())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow or underflow and if UDIS=0 in the CR1 register. When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<SRrs> {
        UifW::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register (An edge has been detected on IC1 which matches the selected polarity).
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<SRrs> {
        Cc1ifW::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<SRrs> {
        Cc2ifW::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<SRrs> {
        Cc3ifW::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&mut self) -> Cc4ifW<SRrs> {
        Cc4ifW::new(self, 4)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode). It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending.
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<SRrs> {
        TifW::new(self, 6)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<SRrs> {
        Cc1ofW::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&mut self) -> Cc2ofW<SRrs> {
        Cc2ofW::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&mut self) -> Cc3ofW<SRrs> {
        Cc3ofW::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&mut self) -> Cc4ofW<SRrs> {
        Cc4ofW::new(self, 12)
    }
}
///TIM status register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
