///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM will use: 0: LPTIM is clocked by internal clock source, according to INTCKSEL 1: LPTIM is clocked by external clock source, according to EXTCKSEL
pub type CkselR = crate::BitReader;
///Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM will use: 0: LPTIM is clocked by internal clock source, according to INTCKSEL 1: LPTIM is clocked by external clock source, according to EXTCKSEL
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKPOL` reader - Clock Polarity If LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: 00: the rising edge is the active edge used for counting 01: the falling edge is the active edge used for counting 10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four time the external clock frequency. 11: not allowed
pub type CkpolR = crate::FieldReader;
///Field `CKPOL` writer - Clock Polarity If LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: 00: the rising edge is the active edge used for counting 01: the falling edge is the active edge used for counting 10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four time the external clock frequency. 11: not allowed
pub type CkpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any external clock signal level change is considered as a valid transition 01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition. 10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition. 11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition.
pub type CkfltR = crate::FieldReader;
///Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any external clock signal level change is considered as a valid transition 01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition. 10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition. 11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition.
pub type CkfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INTCKSEL` reader - Internal clock source selector 0: internal clock source is clk_lp 1: internal clock source is pclk2
pub type IntckselR = crate::BitReader;
///Field `INTCKSEL` writer - Internal clock source selector 0: internal clock source is clk_lp 1: internal clock source is pclk2
pub type IntckselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any trigger active level change is considered as a valid trigger 01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger. 10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger. 11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger.
pub type TrgfltR = crate::FieldReader;
///Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any trigger active level change is considered as a valid trigger 01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger. 10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger. 11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger.
pub type TrgfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EXTCKSEL` reader - External clock source selector 0: external clock source is from lptim_in 1: external clock source is from LPCOMP (if LPCOMP integrated)
pub type ExtckselR = crate::BitReader;
///Field `EXTCKSEL` writer - External clock source selector 0: external clock source is from lptim_in 1: external clock source is from LPCOMP (if LPCOMP integrated)
pub type ExtckselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors: 000: /1 001: /2 010: /4 011: /8 100: /16 101: /32 110: /64 111: /128
pub type PrescR = crate::FieldReader;
///Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors: 000: /1 001: /2 010: /4 011: /8 100: /16 101: /32 110: /64 111: /128
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: 000: lptim_ext0 001: lptim_ext1 010: lptim_ext2 011: lptim_ext3 100: lptim_ext4 101: lptim_ext5 110: lptim_ext6 111: lptim_ext7
pub type TrigselR = crate::FieldReader;
///Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: 000: lptim_ext0 001: lptim_ext1 010: lptim_ext2 011: lptim_ext3 100: lptim_ext4 101: lptim_ext5 110: lptim_ext6 111: lptim_ext7
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge: 00: software trigger (counting start is initiated by software) 01: rising edge is the active edge 10: falling edge is the active edge 11: both edges are active edges
pub type TrigenR = crate::FieldReader;
///Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge: 00: software trigger (counting start is initiated by software) 01: rising edge is the active edge 10: falling edge is the active edge 11: both edges are active edges
pub type TrigenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature 0: A trigger event arriving when the timer is already started will be ignored 1: A trigger event arriving when the timer is already started will reset and restart the LPTIM counter and the repetition counter
pub type TimoutR = crate::BitReader;
///Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature 0: A trigger event arriving when the timer is already started will be ignored 1: A trigger event arriving when the timer is already started will reset and restart the LPTIM counter and the repetition counter
pub type TimoutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape 0: Deactivate Set-once mode 1: Activate the Set-once mode
pub type WaveR = crate::BitReader;
///Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape 0: Deactivate Set-once mode 1: Activate the Set-once mode
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVPOL` reader - Waveform shape polarity The WAVEPOL bit controls the output polarity 0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers 1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
pub type WavpolR = crate::BitReader;
///Field `WAVPOL` writer - Waveform shape polarity The WAVEPOL bit controls the output polarity 0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers 1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
pub type WavpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTMODE` reader - counter mode in internal clock source mode (CKSEL=0). If CKSEL=1, this bit has no effect. 0: the counter is incremented following each internal clock pulse 1: the counter is incremented following each valid pulse on the external clock
pub type CountmodeR = crate::BitReader;
///Field `COUNTMODE` writer - counter mode in internal clock source mode (CKSEL=0). If CKSEL=1, this bit has no effect. 0: the counter is incremented following each internal clock pulse 1: the counter is incremented following each valid pulse on the external clock
pub type CountmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use: 0: LPTIM is clocked by internal clock source, according to INTCKSEL 1: LPTIM is clocked by external clock source, according to EXTCKSEL
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: 00: the rising edge is the active edge used for counting 01: the falling edge is the active edge used for counting 10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four time the external clock frequency. 11: not allowed
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any external clock signal level change is considered as a valid transition 01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition. 10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition. 11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition.
    #[inline(always)]
    pub fn ckflt(&self) -> CkfltR {
        CkfltR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Internal clock source selector 0: internal clock source is clk_lp 1: internal clock source is pclk2
    #[inline(always)]
    pub fn intcksel(&self) -> IntckselR {
        IntckselR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any trigger active level change is considered as a valid trigger 01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger. 10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger. 11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger.
    #[inline(always)]
    pub fn trgflt(&self) -> TrgfltR {
        TrgfltR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - External clock source selector 0: external clock source is from lptim_in 1: external clock source is from LPCOMP (if LPCOMP integrated)
    #[inline(always)]
    pub fn extcksel(&self) -> ExtckselR {
        ExtckselR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors: 000: /1 001: /2 010: /4 011: /8 100: /16 101: /32 110: /64 111: /128
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: 000: lptim_ext0 001: lptim_ext1 010: lptim_ext2 011: lptim_ext3 100: lptim_ext4 101: lptim_ext5 110: lptim_ext6 111: lptim_ext7
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge: 00: software trigger (counting start is initiated by software) 01: rising edge is the active edge 10: falling edge is the active edge 11: both edges are active edges
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature 0: A trigger event arriving when the timer is already started will be ignored 1: A trigger event arriving when the timer is already started will reset and restart the LPTIM counter and the repetition counter
    #[inline(always)]
    pub fn timout(&self) -> TimoutR {
        TimoutR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Waveform shape The WAVE bit controls the output shape 0: Deactivate Set-once mode 1: Activate the Set-once mode
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity 0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers 1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn wavpol(&self) -> WavpolR {
        WavpolR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - counter mode in internal clock source mode (CKSEL=0). If CKSEL=1, this bit has no effect. 0: the counter is incremented following each internal clock pulse 1: the counter is incremented following each valid pulse on the external clock
    #[inline(always)]
    pub fn countmode(&self) -> CountmodeR {
        CountmodeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("countmode", &self.countmode())
            .field("wavpol", &self.wavpol())
            .field("wave", &self.wave())
            .field("timout", &self.timout())
            .field("trigen", &self.trigen())
            .field("trigsel", &self.trigsel())
            .field("presc", &self.presc())
            .field("extcksel", &self.extcksel())
            .field("trgflt", &self.trgflt())
            .field("intcksel", &self.intcksel())
            .field("ckflt", &self.ckflt())
            .field("ckpol", &self.ckpol())
            .field("cksel", &self.cksel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use: 0: LPTIM is clocked by internal clock source, according to INTCKSEL 1: LPTIM is clocked by external clock source, according to EXTCKSEL
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<CFGRrs> {
        CkselW::new(self, 0)
    }
    ///Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: 00: the rising edge is the active edge used for counting 01: the falling edge is the active edge used for counting 10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four time the external clock frequency. 11: not allowed
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<CFGRrs> {
        CkpolW::new(self, 1)
    }
    ///Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any external clock signal level change is considered as a valid transition 01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition. 10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition. 11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition.
    #[inline(always)]
    pub fn ckflt(&mut self) -> CkfltW<CFGRrs> {
        CkfltW::new(self, 3)
    }
    ///Bit 5 - Internal clock source selector 0: internal clock source is clk_lp 1: internal clock source is pclk2
    #[inline(always)]
    pub fn intcksel(&mut self) -> IntckselW<CFGRrs> {
        IntckselW::new(self, 5)
    }
    ///Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any trigger active level change is considered as a valid trigger 01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger. 10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger. 11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger.
    #[inline(always)]
    pub fn trgflt(&mut self) -> TrgfltW<CFGRrs> {
        TrgfltW::new(self, 6)
    }
    ///Bit 8 - External clock source selector 0: external clock source is from lptim_in 1: external clock source is from LPCOMP (if LPCOMP integrated)
    #[inline(always)]
    pub fn extcksel(&mut self) -> ExtckselW<CFGRrs> {
        ExtckselW::new(self, 8)
    }
    ///Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors: 000: /1 001: /2 010: /4 011: /8 100: /16 101: /32 110: /64 111: /128
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<CFGRrs> {
        PrescW::new(self, 9)
    }
    ///Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: 000: lptim_ext0 001: lptim_ext1 010: lptim_ext2 011: lptim_ext3 100: lptim_ext4 101: lptim_ext5 110: lptim_ext6 111: lptim_ext7
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<CFGRrs> {
        TrigselW::new(self, 13)
    }
    ///Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge: 00: software trigger (counting start is initiated by software) 01: rising edge is the active edge 10: falling edge is the active edge 11: both edges are active edges
    #[inline(always)]
    pub fn trigen(&mut self) -> TrigenW<CFGRrs> {
        TrigenW::new(self, 17)
    }
    ///Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature 0: A trigger event arriving when the timer is already started will be ignored 1: A trigger event arriving when the timer is already started will reset and restart the LPTIM counter and the repetition counter
    #[inline(always)]
    pub fn timout(&mut self) -> TimoutW<CFGRrs> {
        TimoutW::new(self, 19)
    }
    ///Bit 20 - Waveform shape The WAVE bit controls the output shape 0: Deactivate Set-once mode 1: Activate the Set-once mode
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<CFGRrs> {
        WaveW::new(self, 20)
    }
    ///Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity 0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers 1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn wavpol(&mut self) -> WavpolW<CFGRrs> {
        WavpolW::new(self, 21)
    }
    ///Bit 23 - counter mode in internal clock source mode (CKSEL=0). If CKSEL=1, this bit has no effect. 0: the counter is incremented following each internal clock pulse 1: the counter is incremented following each valid pulse on the external clock
    #[inline(always)]
    pub fn countmode(&mut self) -> CountmodeW<CFGRrs> {
        CountmodeW::new(self, 23)
    }
}
///LPTIM configuration register
///
///You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
