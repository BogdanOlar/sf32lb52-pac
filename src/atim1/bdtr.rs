///Register `BDTR` reader
pub type R = crate::R<BDTRrs>;
///Register `BDTR` writer
pub type W = crate::W<BDTRrs>;
///Field `DTG` reader - Dead-time generator setup This bit-field, together with DTPSC, defines the duration of the dead-time inserted between the complementary outputs. If DTG=0, dead-time is disabled. Example if tCLK=8.33ns (120MHz), dead-time possible values are: 16.67ns to 8533.33 ns by 8.33 ns steps if DTPSC=0, 266.67ns to 136.53 us by 133.33 ns steps if DTPSC=1 This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type DtgR = crate::FieldReader<u16>;
///Field `DTG` writer - Dead-time generator setup This bit-field, together with DTPSC, defines the duration of the dead-time inserted between the complementary outputs. If DTG=0, dead-time is disabled. Example if tCLK=8.33ns (120MHz), dead-time possible values are: 16.67ns to 8533.33 ns by 8.33 ns steps if DTPSC=0, 266.67ns to 136.53 us by 133.33 ns steps if DTPSC=1 This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type DtgW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DTPSC` reader - Dead-time prescaler This bit-field enables dead-time prescaler. 0: dead-time is tCLK*(DTG+1) if DTG is not zero 1: dead-time is tCLK*(DTG+1)*16 if DTG is not zero This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type DtpscR = crate::BitReader;
///Field `DTPSC` writer - Dead-time prescaler This bit-field enables dead-time prescaler. 0: dead-time is tCLK*(DTG+1) if DTG is not zero 1: dead-time is tCLK*(DTG+1)*16 if DTG is not zero This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type DtpscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKE` reader - Break enable This bit enables the complete break protection. 0: Break function disabled 1: Break function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type BkeR = crate::BitReader;
///Field `BKE` writer - Break enable This bit enables the complete break protection. 0: Break function disabled 1: Break function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - Break polarity 0: Break input BRK is active low 1: Break input BRK is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type BkpR = crate::BitReader;
///Field `BKP` writer - Break polarity 0: Break input BRK is active low 1: Break input BRK is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AOE` reader - Automatic output enable 0: MOE can be set only by software 1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active) This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type AoeR = crate::BitReader;
///Field `AOE` writer - Automatic output enable 0: MOE can be set only by software 1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active) This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOE` reader - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: In response to a break 2 event. OC and OCN outputs are disabled In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in CCER register).
pub type MoeR = crate::BitReader;
///Field `MOE` writer - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: In response to a break 2 event. OC and OCN outputs are disabled In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in CCER register).
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKF` reader - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type BkfR = crate::FieldReader;
///Field `BKF` writer - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type BkfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BK2F` reader - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK2 acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2fR = crate::FieldReader;
///Field `BK2F` writer - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK2 acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BK2E` reader - Break 2 enable This bit enables the complete break 2 protection. 0: Break2 function disabled 1: Break2 function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2eR = crate::BitReader;
///Field `BK2E` writer - Break 2 enable This bit enables the complete break 2 protection. 0: Break2 function disabled 1: Break2 function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2P` reader - BK2P: Break 2 polarity 0: Break input BRK2 is active low 1: Break input BRK2 is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2pR = crate::BitReader;
///Field `BK2P` writer - BK2P: Break 2 polarity 0: Break input BRK2 is active low 1: Break input BRK2 is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2pW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKDSRM` reader - Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared.
pub type BkdsrmR = crate::BitReader;
///Field `BKDSRM` writer - Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared.
pub type BkdsrmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2DSRM` reader - Break2 Disarm
pub type Bk2dsrmR = crate::BitReader;
///Field `BK2DSRM` writer - Break2 Disarm
pub type Bk2dsrmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKBID` reader - Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in BDTR register).
pub type BkbidR = crate::BitReader;
///Field `BKBID` writer - Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in BDTR register).
pub type BkbidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2BID` reader - Break2 bidirectional
pub type Bk2bidR = crate::BitReader;
///Field `BK2BID` writer - Break2 bidirectional
pub type Bk2bidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSSI` reader - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, imposes a Hi-Z state). 1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output. This bit can not be modified as soon as the LOCK level 2 has been programmed.
pub type OssiR = crate::BitReader;
///Field `OSSI` writer - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, imposes a Hi-Z state). 1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output. This bit can not be modified as soon as the LOCK level 2 has been programmed.
pub type OssiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSSR` reader - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, forces a Hi-Z state). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). This bit can not be modified as soon as the LOCK level 2 has been programmed.
pub type OssrR = crate::BitReader;
///Field `OSSR` writer - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, forces a Hi-Z state). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). This bit can not be modified as soon as the LOCK level 2 has been programmed.
pub type OssrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - Dead-time generator setup This bit-field, together with DTPSC, defines the duration of the dead-time inserted between the complementary outputs. If DTG=0, dead-time is disabled. Example if tCLK=8.33ns (120MHz), dead-time possible values are: 16.67ns to 8533.33 ns by 8.33 ns steps if DTPSC=0, 266.67ns to 136.53 us by 133.33 ns steps if DTPSC=1 This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn dtg(&self) -> DtgR {
        DtgR::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 11 - Dead-time prescaler This bit-field enables dead-time prescaler. 0: dead-time is tCLK*(DTG+1) if DTG is not zero 1: dead-time is tCLK*(DTG+1)*16 if DTG is not zero This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn dtpsc(&self) -> DtpscR {
        DtpscR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable This bit enables the complete break protection. 0: Break function disabled 1: Break function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity 0: Break input BRK is active low 1: Break input BRK is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable 0: MOE can be set only by software 1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active) This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: In response to a break 2 event. OC and OCN outputs are disabled In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in CCER register).
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bkf(&self) -> BkfR {
        BkfR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK2 acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2f(&self) -> Bk2fR {
        Bk2fR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Break 2 enable This bit enables the complete break 2 protection. 0: Break2 function disabled 1: Break2 function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2e(&self) -> Bk2eR {
        Bk2eR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - BK2P: Break 2 polarity 0: Break input BRK2 is active low 1: Break input BRK2 is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2p(&self) -> Bk2pR {
        Bk2pR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared.
    #[inline(always)]
    pub fn bkdsrm(&self) -> BkdsrmR {
        BkdsrmR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    pub fn bk2dsrm(&self) -> Bk2dsrmR {
        Bk2dsrmR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in BDTR register).
    #[inline(always)]
    pub fn bkbid(&self) -> BkbidR {
        BkbidR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    pub fn bk2bid(&self) -> Bk2bidR {
        Bk2bidR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, imposes a Hi-Z state). 1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output. This bit can not be modified as soon as the LOCK level 2 has been programmed.
    #[inline(always)]
    pub fn ossi(&self) -> OssiR {
        OssiR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, forces a Hi-Z state). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). This bit can not be modified as soon as the LOCK level 2 has been programmed.
    #[inline(always)]
    pub fn ossr(&self) -> OssrR {
        OssrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTR")
            .field("ossr", &self.ossr())
            .field("ossi", &self.ossi())
            .field("bk2bid", &self.bk2bid())
            .field("bkbid", &self.bkbid())
            .field("bk2dsrm", &self.bk2dsrm())
            .field("bkdsrm", &self.bkdsrm())
            .field("bk2p", &self.bk2p())
            .field("bk2e", &self.bk2e())
            .field("bk2f", &self.bk2f())
            .field("bkf", &self.bkf())
            .field("moe", &self.moe())
            .field("aoe", &self.aoe())
            .field("bkp", &self.bkp())
            .field("bke", &self.bke())
            .field("dtpsc", &self.dtpsc())
            .field("dtg", &self.dtg())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Dead-time generator setup This bit-field, together with DTPSC, defines the duration of the dead-time inserted between the complementary outputs. If DTG=0, dead-time is disabled. Example if tCLK=8.33ns (120MHz), dead-time possible values are: 16.67ns to 8533.33 ns by 8.33 ns steps if DTPSC=0, 266.67ns to 136.53 us by 133.33 ns steps if DTPSC=1 This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn dtg(&mut self) -> DtgW<BDTRrs> {
        DtgW::new(self, 0)
    }
    ///Bit 11 - Dead-time prescaler This bit-field enables dead-time prescaler. 0: dead-time is tCLK*(DTG+1) if DTG is not zero 1: dead-time is tCLK*(DTG+1)*16 if DTG is not zero This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn dtpsc(&mut self) -> DtpscW<BDTRrs> {
        DtpscW::new(self, 11)
    }
    ///Bit 12 - Break enable This bit enables the complete break protection. 0: Break function disabled 1: Break function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bke(&mut self) -> BkeW<BDTRrs> {
        BkeW::new(self, 12)
    }
    ///Bit 13 - Break polarity 0: Break input BRK is active low 1: Break input BRK is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<BDTRrs> {
        BkpW::new(self, 13)
    }
    ///Bit 14 - Automatic output enable 0: MOE can be set only by software 1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active) This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<BDTRrs> {
        AoeW::new(self, 14)
    }
    ///Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: In response to a break 2 event. OC and OCN outputs are disabled In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in CCER register).
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<BDTRrs> {
        MoeW::new(self, 15)
    }
    ///Bits 16:19 - Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bkf(&mut self) -> BkfW<BDTRrs> {
        BkfW::new(self, 16)
    }
    ///Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK2 acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2f(&mut self) -> Bk2fW<BDTRrs> {
        Bk2fW::new(self, 20)
    }
    ///Bit 24 - Break 2 enable This bit enables the complete break 2 protection. 0: Break2 function disabled 1: Break2 function enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2e(&mut self) -> Bk2eW<BDTRrs> {
        Bk2eW::new(self, 24)
    }
    ///Bit 25 - BK2P: Break 2 polarity 0: Break input BRK2 is active low 1: Break input BRK2 is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2p(&mut self) -> Bk2pW<BDTRrs> {
        Bk2pW::new(self, 25)
    }
    ///Bit 26 - Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared.
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BkdsrmW<BDTRrs> {
        BkdsrmW::new(self, 26)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    pub fn bk2dsrm(&mut self) -> Bk2dsrmW<BDTRrs> {
        Bk2dsrmW::new(self, 27)
    }
    ///Bit 28 - Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in BDTR register).
    #[inline(always)]
    pub fn bkbid(&mut self) -> BkbidW<BDTRrs> {
        BkbidW::new(self, 28)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    pub fn bk2bid(&mut self) -> Bk2bidW<BDTRrs> {
        Bk2bidW::new(self, 29)
    }
    ///Bit 30 - Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, imposes a Hi-Z state). 1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output. This bit can not be modified as soon as the LOCK level 2 has been programmed.
    #[inline(always)]
    pub fn ossi(&mut self) -> OssiW<BDTRrs> {
        OssiW::new(self, 30)
    }
    ///Bit 31 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, forces a Hi-Z state). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). This bit can not be modified as soon as the LOCK level 2 has been programmed.
    #[inline(always)]
    pub fn ossr(&mut self) -> OssrW<BDTRrs> {
        OssrW::new(self, 31)
    }
}
///TIM break and dead-time register
///
///You can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtr::R`](R) reader structure
impl crate::Readable for BDTRrs {}
///`write(|w| ..)` method takes [`bdtr::W`](W) writer structure
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTRrs {
    const RESET_VALUE: u32 = 0;
}
