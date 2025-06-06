///Register `CDR` reader
pub type R = crate::R<CDRrs>;
///Register `CDR` writer
pub type W = crate::W<CDRrs>;
///Field `SD_DATA3_CD` reader - Use sd_data\[3\]
///to do card detect 0: use special pin to do card detect / write protect. (Currently not supported) 1: use sd_data\[3\]
///to do card detect (default)
pub type SdData3CdR = crate::BitReader;
///Field `SD_DATA3_CD` writer - Use sd_data\[3\]
///to do card detect 0: use special pin to do card detect / write protect. (Currently not supported) 1: use sd_data\[3\]
///to do card detect (default)
pub type SdData3CdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITIMING_SEL` reader - select input sample timing (according to itiming config)
pub type ItimingSelR = crate::BitReader;
///Field `ITIMING_SEL` writer - select input sample timing (according to itiming config)
pub type ItimingSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTIMING_SEL` reader - select output timing (according to otiming config)
pub type OtimingSelR = crate::BitReader;
///Field `OTIMING_SEL` writer - select output timing (according to otiming config)
pub type OtimingSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_CD` reader - Enable card detect Only when the bit is valid, controller does card detect. If use sd_data\[3\]
///to do card detect, the bit should be cleared when transfer valid data.
pub type EnCdR = crate::BitReader;
///Field `EN_CD` writer - Enable card detect Only when the bit is valid, controller does card detect. If use sd_data\[3\]
///to do card detect, the bit should be cleared when transfer valid data.
pub type EnCdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CD_HVALID` reader - Card detect high level valid 0: detect low level means card exist 1: detect high level means card exist (default)
pub type CdHvalidR = crate::BitReader;
///Field `CD_HVALID` writer - Card detect high level valid 0: detect low level means card exist 1: detect high level means card exist (default)
pub type CdHvalidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_OD` reader - Open Drain mode for cmd line (for eMMC identification mode) 0: cmd line is push-pull 1: cmd line is open-drain
pub type CmdOdR = crate::BitReader;
///Field `CMD_OD` writer - Open Drain mode for cmd line (for eMMC identification mode) 0: cmd line is push-pull 1: cmd line is open-drain
pub type CmdOdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITIMING` reader - define input timing
pub type ItimingR = crate::FieldReader<u16>;
///Field `ITIMING` writer - define input timing
pub type ItimingW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `OTIMING` reader - define output timing
pub type OtimingR = crate::FieldReader<u16>;
///Field `OTIMING` writer - define output timing
pub type OtimingW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bit 0 - Use sd_data\[3\]
    ///to do card detect 0: use special pin to do card detect / write protect. (Currently not supported) 1: use sd_data\[3\]
    ///to do card detect (default)
    #[inline(always)]
    pub fn sd_data3_cd(&self) -> SdData3CdR {
        SdData3CdR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - select input sample timing (according to itiming config)
    #[inline(always)]
    pub fn itiming_sel(&self) -> ItimingSelR {
        ItimingSelR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - select output timing (according to otiming config)
    #[inline(always)]
    pub fn otiming_sel(&self) -> OtimingSelR {
        OtimingSelR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable card detect Only when the bit is valid, controller does card detect. If use sd_data\[3\]
    ///to do card detect, the bit should be cleared when transfer valid data.
    #[inline(always)]
    pub fn en_cd(&self) -> EnCdR {
        EnCdR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Card detect high level valid 0: detect low level means card exist 1: detect high level means card exist (default)
    #[inline(always)]
    pub fn cd_hvalid(&self) -> CdHvalidR {
        CdHvalidR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Open Drain mode for cmd line (for eMMC identification mode) 0: cmd line is push-pull 1: cmd line is open-drain
    #[inline(always)]
    pub fn cmd_od(&self) -> CmdOdR {
        CmdOdR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:18 - define input timing
    #[inline(always)]
    pub fn itiming(&self) -> ItimingR {
        ItimingR::new(((self.bits >> 6) & 0x1fff) as u16)
    }
    ///Bits 19:31 - define output timing
    #[inline(always)]
    pub fn otiming(&self) -> OtimingR {
        OtimingR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR")
            .field("otiming", &self.otiming())
            .field("itiming", &self.itiming())
            .field("cmd_od", &self.cmd_od())
            .field("cd_hvalid", &self.cd_hvalid())
            .field("en_cd", &self.en_cd())
            .field("otiming_sel", &self.otiming_sel())
            .field("itiming_sel", &self.itiming_sel())
            .field("sd_data3_cd", &self.sd_data3_cd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Use sd_data\[3\]
    ///to do card detect 0: use special pin to do card detect / write protect. (Currently not supported) 1: use sd_data\[3\]
    ///to do card detect (default)
    #[inline(always)]
    pub fn sd_data3_cd(&mut self) -> SdData3CdW<CDRrs> {
        SdData3CdW::new(self, 0)
    }
    ///Bit 1 - select input sample timing (according to itiming config)
    #[inline(always)]
    pub fn itiming_sel(&mut self) -> ItimingSelW<CDRrs> {
        ItimingSelW::new(self, 1)
    }
    ///Bit 2 - select output timing (according to otiming config)
    #[inline(always)]
    pub fn otiming_sel(&mut self) -> OtimingSelW<CDRrs> {
        OtimingSelW::new(self, 2)
    }
    ///Bit 3 - Enable card detect Only when the bit is valid, controller does card detect. If use sd_data\[3\]
    ///to do card detect, the bit should be cleared when transfer valid data.
    #[inline(always)]
    pub fn en_cd(&mut self) -> EnCdW<CDRrs> {
        EnCdW::new(self, 3)
    }
    ///Bit 4 - Card detect high level valid 0: detect low level means card exist 1: detect high level means card exist (default)
    #[inline(always)]
    pub fn cd_hvalid(&mut self) -> CdHvalidW<CDRrs> {
        CdHvalidW::new(self, 4)
    }
    ///Bit 5 - Open Drain mode for cmd line (for eMMC identification mode) 0: cmd line is push-pull 1: cmd line is open-drain
    #[inline(always)]
    pub fn cmd_od(&mut self) -> CmdOdW<CDRrs> {
        CmdOdW::new(self, 5)
    }
    ///Bits 6:18 - define input timing
    #[inline(always)]
    pub fn itiming(&mut self) -> ItimingW<CDRrs> {
        ItimingW::new(self, 6)
    }
    ///Bits 19:31 - define output timing
    #[inline(always)]
    pub fn otiming(&mut self) -> OtimingW<CDRrs> {
        OtimingW::new(self, 19)
    }
}
///card interface control and card detect register
///
///You can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
///`read()` method returns [`cdr::R`](R) reader structure
impl crate::Readable for CDRrs {}
///`write(|w| ..)` method takes [`cdr::W`](W) writer structure
impl crate::Writable for CDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDRrs {
    const RESET_VALUE: u32 = 0;
}
