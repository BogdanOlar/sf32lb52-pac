///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
///Field `RBSIZE` reader - Row boundary size. 0: no row boundary 1: 2^(1+3) = 16 bytes 2: 2^(2+3) = 32 bytes … n: 2^(n+3) bytes
pub type RbsizeR = crate::FieldReader;
///Field `RBSIZE` writer - Row boundary size. 0: no row boundary 1: 2^(1+3) = 16 bytes 2: 2^(2+3) = 32 bytes … n: 2^(n+3) bytes
pub type RbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DQSE` reader - DQS enable. Setting to 1 indicates device provides DQS signal for Rx data latching
pub type DqseR = crate::BitReader;
///Field `DQSE` writer - DQS enable. Setting to 1 indicates device provides DQS signal for Rx data latching
pub type DqseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYPER` reader - HyperBus protocol. Set to 1 for HyperRAM.
pub type HyperR = crate::BitReader;
///Field `HYPER` writer - HyperBus protocol. Set to 1 for HyperRAM.
pub type HyperW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XLEGACY` reader - Xccela legacy protocol. Set to 1 for AP 32Mb PSRAM only, othersize always set to 0.
pub type XlegacyR = crate::BitReader;
///Field `XLEGACY` writer - Xccela legacy protocol. Set to 1 for AP 32Mb PSRAM only, othersize always set to 0.
pub type XlegacyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSLMAX` reader - Maximum CS low active time in MCLK cycles For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMAX = n, then CS Low time = (n+1) * 1000/240 ns which must meet the maximum tCEM requirement for PSRAM
pub type CslmaxR = crate::FieldReader<u16>;
///Field `CSLMAX` writer - Maximum CS low active time in MCLK cycles For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMAX = n, then CS Low time = (n+1) * 1000/240 ns which must meet the maximum tCEM requirement for PSRAM
pub type CslmaxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `CSLMIN` reader - Minimum CS low active time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMIN = n, then CS Low time = (n+1) * 1000/240 ns which must meet the minimum tCEM requirement for PSRAM
pub type CslminR = crate::FieldReader;
///Field `CSLMIN` writer - Minimum CS low active time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMIN = n, then CS Low time = (n+1) * 1000/240 ns which must meet the minimum tCEM requirement for PSRAM
pub type CslminW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CSHMIN` reader - Minimum CS high deselect time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSHMIN = n, then CS High time = (n+1) * 1000/240 ns which must meet minimum tCPH requirement for PSRAM
pub type CshminR = crate::FieldReader;
///Field `CSHMIN` writer - Minimum CS high deselect time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSHMIN = n, then CS High time = (n+1) * 1000/240 ns which must meet minimum tCPH requirement for PSRAM
pub type CshminW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRCMIN` reader - Write/Read cycle minimum time in internal MCLK cycles. Please see MCLK frequency in PSCLR description. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and TRCMIN = n, then tRC time = (n+1) * 1000/240 ns which must meet minimum tRC requirement for PSRAM
pub type TrcminR = crate::FieldReader;
///Field `TRCMIN` writer - Write/Read cycle minimum time in internal MCLK cycles. Please see MCLK frequency in PSCLR description. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and TRCMIN = n, then tRC time = (n+1) * 1000/240 ns which must meet minimum tRC requirement for PSRAM
pub type TrcminW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FIXLAT` reader - Indicate PSRAM is fixed latency or variable latency. It must be compatible to the configuration in PSRAM registers. Recommend always set to 1. 0: variable latency 1: fixed latency
pub type FixlatR = crate::BitReader;
///Field `FIXLAT` writer - Indicate PSRAM is fixed latency or variable latency. It must be compatible to the configuration in PSRAM registers. Recommend always set to 1. 0: variable latency 1: fixed latency
pub type FixlatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Row boundary size. 0: no row boundary 1: 2^(1+3) = 16 bytes 2: 2^(2+3) = 32 bytes … n: 2^(n+3) bytes
    #[inline(always)]
    pub fn rbsize(&self) -> RbsizeR {
        RbsizeR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - DQS enable. Setting to 1 indicates device provides DQS signal for Rx data latching
    #[inline(always)]
    pub fn dqse(&self) -> DqseR {
        DqseR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HyperBus protocol. Set to 1 for HyperRAM.
    #[inline(always)]
    pub fn hyper(&self) -> HyperR {
        HyperR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Xccela legacy protocol. Set to 1 for AP 32Mb PSRAM only, othersize always set to 0.
    #[inline(always)]
    pub fn xlegacy(&self) -> XlegacyR {
        XlegacyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:17 - Maximum CS low active time in MCLK cycles For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMAX = n, then CS Low time = (n+1) * 1000/240 ns which must meet the maximum tCEM requirement for PSRAM
    #[inline(always)]
    pub fn cslmax(&self) -> CslmaxR {
        CslmaxR::new(((self.bits >> 6) & 0x0fff) as u16)
    }
    ///Bits 18:21 - Minimum CS low active time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMIN = n, then CS Low time = (n+1) * 1000/240 ns which must meet the minimum tCEM requirement for PSRAM
    #[inline(always)]
    pub fn cslmin(&self) -> CslminR {
        CslminR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:25 - Minimum CS high deselect time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSHMIN = n, then CS High time = (n+1) * 1000/240 ns which must meet minimum tCPH requirement for PSRAM
    #[inline(always)]
    pub fn cshmin(&self) -> CshminR {
        CshminR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bits 26:30 - Write/Read cycle minimum time in internal MCLK cycles. Please see MCLK frequency in PSCLR description. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and TRCMIN = n, then tRC time = (n+1) * 1000/240 ns which must meet minimum tRC requirement for PSRAM
    #[inline(always)]
    pub fn trcmin(&self) -> TrcminR {
        TrcminR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Indicate PSRAM is fixed latency or variable latency. It must be compatible to the configuration in PSRAM registers. Recommend always set to 1. 0: variable latency 1: fixed latency
    #[inline(always)]
    pub fn fixlat(&self) -> FixlatR {
        FixlatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("fixlat", &self.fixlat())
            .field("trcmin", &self.trcmin())
            .field("cshmin", &self.cshmin())
            .field("cslmin", &self.cslmin())
            .field("cslmax", &self.cslmax())
            .field("xlegacy", &self.xlegacy())
            .field("hyper", &self.hyper())
            .field("dqse", &self.dqse())
            .field("rbsize", &self.rbsize())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Row boundary size. 0: no row boundary 1: 2^(1+3) = 16 bytes 2: 2^(2+3) = 32 bytes … n: 2^(n+3) bytes
    #[inline(always)]
    pub fn rbsize(&mut self) -> RbsizeW<DCRrs> {
        RbsizeW::new(self, 0)
    }
    ///Bit 3 - DQS enable. Setting to 1 indicates device provides DQS signal for Rx data latching
    #[inline(always)]
    pub fn dqse(&mut self) -> DqseW<DCRrs> {
        DqseW::new(self, 3)
    }
    ///Bit 4 - HyperBus protocol. Set to 1 for HyperRAM.
    #[inline(always)]
    pub fn hyper(&mut self) -> HyperW<DCRrs> {
        HyperW::new(self, 4)
    }
    ///Bit 5 - Xccela legacy protocol. Set to 1 for AP 32Mb PSRAM only, othersize always set to 0.
    #[inline(always)]
    pub fn xlegacy(&mut self) -> XlegacyW<DCRrs> {
        XlegacyW::new(self, 5)
    }
    ///Bits 6:17 - Maximum CS low active time in MCLK cycles For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMAX = n, then CS Low time = (n+1) * 1000/240 ns which must meet the maximum tCEM requirement for PSRAM
    #[inline(always)]
    pub fn cslmax(&mut self) -> CslmaxW<DCRrs> {
        CslmaxW::new(self, 6)
    }
    ///Bits 18:21 - Minimum CS low active time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMIN = n, then CS Low time = (n+1) * 1000/240 ns which must meet the minimum tCEM requirement for PSRAM
    #[inline(always)]
    pub fn cslmin(&mut self) -> CslminW<DCRrs> {
        CslminW::new(self, 18)
    }
    ///Bits 22:25 - Minimum CS high deselect time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSHMIN = n, then CS High time = (n+1) * 1000/240 ns which must meet minimum tCPH requirement for PSRAM
    #[inline(always)]
    pub fn cshmin(&mut self) -> CshminW<DCRrs> {
        CshminW::new(self, 22)
    }
    ///Bits 26:30 - Write/Read cycle minimum time in internal MCLK cycles. Please see MCLK frequency in PSCLR description. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and TRCMIN = n, then tRC time = (n+1) * 1000/240 ns which must meet minimum tRC requirement for PSRAM
    #[inline(always)]
    pub fn trcmin(&mut self) -> TrcminW<DCRrs> {
        TrcminW::new(self, 26)
    }
    ///Bit 31 - Indicate PSRAM is fixed latency or variable latency. It must be compatible to the configuration in PSRAM registers. Recommend always set to 1. 0: variable latency 1: fixed latency
    #[inline(always)]
    pub fn fixlat(&mut self) -> FixlatW<DCRrs> {
        FixlatW::new(self, 31)
    }
}
///Device Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DCRrs {
    const RESET_VALUE: u32 = 0;
}
