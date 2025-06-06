///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Enable MPI
pub type EnR = crate::BitReader;
///Field `EN` writer - Enable MPI
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WPE` reader - Enable WP function on IO2. Use this only in SPI or Dual SPI mode
pub type WpeR = crate::BitReader;
///Field `WPE` writer - Enable WP function on IO2. Use this only in SPI or Dual SPI mode
pub type WpeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP` reader - The value of WP when WPE is set
pub type WpR = crate::BitReader;
///Field `WP` writer - The value of WP when WPE is set
pub type WpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLDE` reader - Enable HOLD function on IO3. Use this only in SPI or Dual SPI mode
pub type HoldeR = crate::BitReader;
///Field `HOLDE` writer - Enable HOLD function on IO3. Use this only in SPI or Dual SPI mode
pub type HoldeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOLD` reader - The value of HOLD when HOLDE is set
pub type HoldR = crate::BitReader;
///Field `HOLD` writer - The value of HOLD when HOLDE is set
pub type HoldW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAE` reader - DMA enable 0: disabled 1: enable DMA to read or write DR register
pub type DmaeR = crate::BitReader;
///Field `DMAE` writer - DMA enable 0: disabled 1: enable DMA to read or write DR register
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRE` reader - AES-CTR on-the-fly decryption enable 0: disabled 1: enabled, data read from memory will be decrypted on the fly by MPI controller
pub type CtreR = crate::BitReader;
///Field `CTRE` writer - AES-CTR on-the-fly decryption enable 0: disabled 1: enabled, data read from memory will be decrypted on the fly by MPI controller
pub type CtreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRM` reader - AES-CTR mode 0: AES-128 1: AES-256
pub type CtrmR = crate::BitReader;
///Field `CTRM` writer - AES-CTR mode 0: AES-128 1: AES-256
pub type CtrmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TcieR = crate::BitReader;
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMIE` reader - Status match interrupt enable
pub type SmieR = crate::BitReader;
///Field `SMIE` writer - Status match interrupt enable
pub type SmieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSVIE` reader - CS max violation interrupt enable
pub type CsvieR = crate::BitReader;
///Field `CSVIE` writer - CS max violation interrupt enable
pub type CsvieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBXIE` reader - Row boundary crossing interrupt enable
pub type RbxieR = crate::BitReader;
///Field `RBXIE` writer - Row boundary crossing interrupt enable
pub type RbxieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD2E` reader - Enable CMD2 0: disabled 1: CMD2 is enabled and will be issued after CMD1 with an interval of TI2
pub type Cmd2eR = crate::BitReader;
///Field `CMD2E` writer - Enable CMD2 0: disabled 1: CMD2 is enabled and will be issued after CMD1 with an interval of TI2
pub type Cmd2eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SME1` reader - Status match enable. If enabled, CMD1 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled (either SME1 or SME2 can be enabled, and SME1 has high priority)
pub type Sme1R = crate::BitReader;
///Field `SME1` writer - Status match enable. If enabled, CMD1 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled (either SME1 or SME2 can be enabled, and SME1 has high priority)
pub type Sme1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SME2` reader - Status match enable. If enabled, CMD2 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled
pub type Sme2R = crate::BitReader;
///Field `SME2` writer - Status match enable. If enabled, CMD2 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled
pub type Sme2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMM` reader - Status match mode 0: AND mode 1: OR mode
pub type SmmR = crate::BitReader;
///Field `SMM` writer - Status match mode 0: AND mode 1: OR mode
pub type SmmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HWIFE` reader - Hardware interface enableReserved-Do not modify
pub type HwifeR = crate::BitReader;
///Field `HWIFE` writer - Hardware interface enableReserved-Do not modify
pub type HwifeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPIE` reader - OPI interface enable 0: x8 mode disabled 1: x8 mode enabled
pub type OpieR = crate::BitReader;
///Field `OPIE` writer - OPI interface enable 0: x8 mode disabled 1: x8 mode enabled
pub type OpieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFE` reader - Prefetch enable. If enabled, MPI will prefetch at consequtive address following a read transaction. Recommend to use when reading large data in a burst manner. 0: prefetch disabled 1: prefetch enabled
pub type PrefeR = crate::BitReader;
///Field `PREFE` writer - Prefetch enable. If enabled, MPI will prefetch at consequtive address following a read transaction. Recommend to use when reading large data in a burst manner. 0: prefetch disabled 1: prefetch enabled
pub type PrefeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MX16` reader - Mode X16Reserved-Do not modify
pub type Mx16R = crate::BitReader;
///Field `MX16` writer - Mode X16Reserved-Do not modify
pub type Mx16W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFM` reader - Dual Flash ModeReserved-Do not modify
pub type DfmR = crate::BitReader;
///Field `DFM` writer - Dual Flash ModeReserved-Do not modify
pub type DfmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBDIS` reader - Hold hreadyout low if AHB access
pub type AhbdisR = crate::BitReader;
///Field `AHBDIS` writer - Hold hreadyout low if AHB access
pub type AhbdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` reader - Write 1 to abort internal state machine. For debug purpose only
pub type AbortR = crate::BitReader;
///Field `ABORT` writer - Write 1 to abort internal state machine. For debug purpose only
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable MPI
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable WP function on IO2. Use this only in SPI or Dual SPI mode
    #[inline(always)]
    pub fn wpe(&self) -> WpeR {
        WpeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The value of WP when WPE is set
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable HOLD function on IO3. Use this only in SPI or Dual SPI mode
    #[inline(always)]
    pub fn holde(&self) -> HoldeR {
        HoldeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The value of HOLD when HOLDE is set
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA enable 0: disabled 1: enable DMA to read or write DR register
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AES-CTR on-the-fly decryption enable 0: disabled 1: enabled, data read from memory will be decrypted on the fly by MPI controller
    #[inline(always)]
    pub fn ctre(&self) -> CtreR {
        CtreR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AES-CTR mode 0: AES-128 1: AES-256
    #[inline(always)]
    pub fn ctrm(&self) -> CtrmR {
        CtrmR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&self) -> SmieR {
        SmieR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CS max violation interrupt enable
    #[inline(always)]
    pub fn csvie(&self) -> CsvieR {
        CsvieR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Row boundary crossing interrupt enable
    #[inline(always)]
    pub fn rbxie(&self) -> RbxieR {
        RbxieR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Enable CMD2 0: disabled 1: CMD2 is enabled and will be issued after CMD1 with an interval of TI2
    #[inline(always)]
    pub fn cmd2e(&self) -> Cmd2eR {
        Cmd2eR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Status match enable. If enabled, CMD1 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled (either SME1 or SME2 can be enabled, and SME1 has high priority)
    #[inline(always)]
    pub fn sme1(&self) -> Sme1R {
        Sme1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Status match enable. If enabled, CMD2 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled
    #[inline(always)]
    pub fn sme2(&self) -> Sme2R {
        Sme2R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Status match mode 0: AND mode 1: OR mode
    #[inline(always)]
    pub fn smm(&self) -> SmmR {
        SmmR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Hardware interface enableReserved-Do not modify
    #[inline(always)]
    pub fn hwife(&self) -> HwifeR {
        HwifeR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OPI interface enable 0: x8 mode disabled 1: x8 mode enabled
    #[inline(always)]
    pub fn opie(&self) -> OpieR {
        OpieR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Prefetch enable. If enabled, MPI will prefetch at consequtive address following a read transaction. Recommend to use when reading large data in a burst manner. 0: prefetch disabled 1: prefetch enabled
    #[inline(always)]
    pub fn prefe(&self) -> PrefeR {
        PrefeR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Mode X16Reserved-Do not modify
    #[inline(always)]
    pub fn mx16(&self) -> Mx16R {
        Mx16R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Dual Flash ModeReserved-Do not modify
    #[inline(always)]
    pub fn dfm(&self) -> DfmR {
        DfmR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Hold hreadyout low if AHB access
    #[inline(always)]
    pub fn ahbdis(&self) -> AhbdisR {
        AhbdisR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - Write 1 to abort internal state machine. For debug purpose only
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("abort", &self.abort())
            .field("ahbdis", &self.ahbdis())
            .field("dfm", &self.dfm())
            .field("mx16", &self.mx16())
            .field("prefe", &self.prefe())
            .field("opie", &self.opie())
            .field("hwife", &self.hwife())
            .field("smm", &self.smm())
            .field("sme2", &self.sme2())
            .field("sme1", &self.sme1())
            .field("cmd2e", &self.cmd2e())
            .field("rbxie", &self.rbxie())
            .field("csvie", &self.csvie())
            .field("smie", &self.smie())
            .field("tcie", &self.tcie())
            .field("ctrm", &self.ctrm())
            .field("ctre", &self.ctre())
            .field("dmae", &self.dmae())
            .field("hold", &self.hold())
            .field("holde", &self.holde())
            .field("wp", &self.wp())
            .field("wpe", &self.wpe())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable MPI
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - Enable WP function on IO2. Use this only in SPI or Dual SPI mode
    #[inline(always)]
    pub fn wpe(&mut self) -> WpeW<CRrs> {
        WpeW::new(self, 1)
    }
    ///Bit 2 - The value of WP when WPE is set
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<CRrs> {
        WpW::new(self, 2)
    }
    ///Bit 3 - Enable HOLD function on IO3. Use this only in SPI or Dual SPI mode
    #[inline(always)]
    pub fn holde(&mut self) -> HoldeW<CRrs> {
        HoldeW::new(self, 3)
    }
    ///Bit 4 - The value of HOLD when HOLDE is set
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<CRrs> {
        HoldW::new(self, 4)
    }
    ///Bit 5 - DMA enable 0: disabled 1: enable DMA to read or write DR register
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<CRrs> {
        DmaeW::new(self, 5)
    }
    ///Bit 6 - AES-CTR on-the-fly decryption enable 0: disabled 1: enabled, data read from memory will be decrypted on the fly by MPI controller
    #[inline(always)]
    pub fn ctre(&mut self) -> CtreW<CRrs> {
        CtreW::new(self, 6)
    }
    ///Bit 7 - AES-CTR mode 0: AES-128 1: AES-256
    #[inline(always)]
    pub fn ctrm(&mut self) -> CtrmW<CRrs> {
        CtrmW::new(self, 7)
    }
    ///Bit 8 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<CRrs> {
        TcieW::new(self, 8)
    }
    ///Bit 11 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&mut self) -> SmieW<CRrs> {
        SmieW::new(self, 11)
    }
    ///Bit 12 - CS max violation interrupt enable
    #[inline(always)]
    pub fn csvie(&mut self) -> CsvieW<CRrs> {
        CsvieW::new(self, 12)
    }
    ///Bit 13 - Row boundary crossing interrupt enable
    #[inline(always)]
    pub fn rbxie(&mut self) -> RbxieW<CRrs> {
        RbxieW::new(self, 13)
    }
    ///Bit 16 - Enable CMD2 0: disabled 1: CMD2 is enabled and will be issued after CMD1 with an interval of TI2
    #[inline(always)]
    pub fn cmd2e(&mut self) -> Cmd2eW<CRrs> {
        Cmd2eW::new(self, 16)
    }
    ///Bit 17 - Status match enable. If enabled, CMD1 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled (either SME1 or SME2 can be enabled, and SME1 has high priority)
    #[inline(always)]
    pub fn sme1(&mut self) -> Sme1W<CRrs> {
        Sme1W::new(self, 17)
    }
    ///Bit 18 - Status match enable. If enabled, CMD2 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled
    #[inline(always)]
    pub fn sme2(&mut self) -> Sme2W<CRrs> {
        Sme2W::new(self, 18)
    }
    ///Bit 19 - Status match mode 0: AND mode 1: OR mode
    #[inline(always)]
    pub fn smm(&mut self) -> SmmW<CRrs> {
        SmmW::new(self, 19)
    }
    ///Bit 20 - Hardware interface enableReserved-Do not modify
    #[inline(always)]
    pub fn hwife(&mut self) -> HwifeW<CRrs> {
        HwifeW::new(self, 20)
    }
    ///Bit 21 - OPI interface enable 0: x8 mode disabled 1: x8 mode enabled
    #[inline(always)]
    pub fn opie(&mut self) -> OpieW<CRrs> {
        OpieW::new(self, 21)
    }
    ///Bit 22 - Prefetch enable. If enabled, MPI will prefetch at consequtive address following a read transaction. Recommend to use when reading large data in a burst manner. 0: prefetch disabled 1: prefetch enabled
    #[inline(always)]
    pub fn prefe(&mut self) -> PrefeW<CRrs> {
        PrefeW::new(self, 22)
    }
    ///Bit 23 - Mode X16Reserved-Do not modify
    #[inline(always)]
    pub fn mx16(&mut self) -> Mx16W<CRrs> {
        Mx16W::new(self, 23)
    }
    ///Bit 24 - Dual Flash ModeReserved-Do not modify
    #[inline(always)]
    pub fn dfm(&mut self) -> DfmW<CRrs> {
        DfmW::new(self, 24)
    }
    ///Bit 25 - Hold hreadyout low if AHB access
    #[inline(always)]
    pub fn ahbdis(&mut self) -> AhbdisW<CRrs> {
        AhbdisW::new(self, 25)
    }
    ///Bit 31 - Write 1 to abort internal state machine. For debug purpose only
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<CRrs> {
        AbortW::new(self, 31)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
