///Register `EGR` reader
pub type R = crate::R<EGRrs>;
///Register `EGR` writer
pub type W = crate::W<EGRrs>;
///Field `UG` reader - Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. The prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting).
pub type UgR = crate::BitReader;
///Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. The prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting).
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` reader - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type Cc1gR = crate::BitReader;
///Field `CC1G` writer - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2G` reader - Capture/compare 2 generation
pub type Cc2gR = crate::BitReader;
///Field `CC2G` writer - Capture/compare 2 generation
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` reader - Capture/compare 3 generation
pub type Cc3gR = crate::BitReader;
///Field `CC3G` writer - Capture/compare 3 generation
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` reader - Capture/compare 4 generation
pub type Cc4gR = crate::BitReader;
///Field `CC4G` writer - Capture/compare 4 generation
pub type Cc4gW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMG` reader - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware 0: No action 1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits This bit acts only on channels having a complementary output.
pub type ComgR = crate::BitReader;
///Field `COMG` writer - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware 0: No action 1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits This bit acts only on channels having a complementary output.
pub type ComgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` reader - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled.
pub type TgR = crate::BitReader;
///Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled.
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG` reader - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
pub type BgR = crate::BitReader;
///Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
pub type BgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2G` reader - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled.
pub type B2gR = crate::BitReader;
///Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled.
pub type B2gW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. The prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting).
    #[inline(always)]
    pub fn ug(&self) -> UgR {
        UgR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&self) -> Cc1gR {
        Cc1gR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/compare 2 generation
    #[inline(always)]
    pub fn cc2g(&self) -> Cc2gR {
        Cc2gR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare 3 generation
    #[inline(always)]
    pub fn cc3g(&self) -> Cc3gR {
        Cc3gR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/compare 4 generation
    #[inline(always)]
    pub fn cc4g(&self) -> Cc4gR {
        Cc4gR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware 0: No action 1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits This bit acts only on channels having a complementary output.
    #[inline(always)]
    pub fn comg(&self) -> ComgR {
        ComgR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn bg(&self) -> BgR {
        BgR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled.
    #[inline(always)]
    pub fn b2g(&self) -> B2gR {
        B2gR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EGR")
            .field("b2g", &self.b2g())
            .field("bg", &self.bg())
            .field("tg", &self.tg())
            .field("comg", &self.comg())
            .field("cc4g", &self.cc4g())
            .field("cc3g", &self.cc3g())
            .field("cc2g", &self.cc2g())
            .field("cc1g", &self.cc1g())
            .field("ug", &self.ug())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. The prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting).
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<EGRrs> {
        UgW::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<EGRrs> {
        Cc1gW::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 generation
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<EGRrs> {
        Cc2gW::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 generation
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<EGRrs> {
        Cc3gW::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 generation
    #[inline(always)]
    pub fn cc4g(&mut self) -> Cc4gW<EGRrs> {
        Cc4gW::new(self, 4)
    }
    ///Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware 0: No action 1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits This bit acts only on channels having a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> ComgW<EGRrs> {
        ComgW::new(self, 5)
    }
    ///Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<EGRrs> {
        TgW::new(self, 6)
    }
    ///Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn bg(&mut self) -> BgW<EGRrs> {
        BgW::new(self, 7)
    }
    ///Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled.
    #[inline(always)]
    pub fn b2g(&mut self) -> B2gW<EGRrs> {
        B2gW::new(self, 8)
    }
}
///Event generation register
///
///You can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
///`read()` method returns [`egr::R`](R) reader structure
impl crate::Readable for EGRrs {}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u32 = 0;
}
