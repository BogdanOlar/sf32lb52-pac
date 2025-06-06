///Register `WDTR` reader
pub type R = crate::R<WDTRrs>;
///Register `WDTR` writer
pub type W = crate::W<WDTRrs>;
///Field `TIMEOUT` reader - Set timeout value in number of clk_wdt cycles
pub type TimeoutR = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - Set timeout value in number of clk_wdt cycles
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EN` reader - WDT enable. This watchdog is on AHB side such that bus access will not hang in exceptional cases
pub type EnR = crate::BitReader;
///Field `EN` writer - WDT enable. This watchdog is on AHB side such that bus access will not hang in exceptional cases
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `TOF` reader - Timeout flag. Self cleared when HREADYOUT becomes ready
pub type TofR = crate::BitReader;
///Field `TOF` writer - Timeout flag. Self cleared when HREADYOUT becomes ready
pub type TofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Set timeout value in number of clk_wdt cycles
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - WDT enable. This watchdog is on AHB side such that bus access will not hang in exceptional cases
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 17) & 0x3fff) as u16)
    }
    ///Bit 31 - Timeout flag. Self cleared when HREADYOUT becomes ready
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTR")
            .field("tof", &self.tof())
            .field("rsvd", &self.rsvd())
            .field("en", &self.en())
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Set timeout value in number of clk_wdt cycles
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<WDTRrs> {
        TimeoutW::new(self, 0)
    }
    ///Bit 16 - WDT enable. This watchdog is on AHB side such that bus access will not hang in exceptional cases
    #[inline(always)]
    pub fn en(&mut self) -> EnW<WDTRrs> {
        EnW::new(self, 16)
    }
    ///Bits 17:30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<WDTRrs> {
        RsvdW::new(self, 17)
    }
    ///Bit 31 - Timeout flag. Self cleared when HREADYOUT becomes ready
    #[inline(always)]
    pub fn tof(&mut self) -> TofW<WDTRrs> {
        TofW::new(self, 31)
    }
}
///WDT Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDTRrs;
impl crate::RegisterSpec for WDTRrs {
    type Ux = u32;
}
///`read()` method returns [`wdtr::R`](R) reader structure
impl crate::Readable for WDTRrs {}
///`write(|w| ..)` method takes [`wdtr::W`](W) writer structure
impl crate::Writable for WDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDTR to value 0
impl crate::Resettable for WDTRrs {
    const RESET_VALUE: u32 = 0;
}
