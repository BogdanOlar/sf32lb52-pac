///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ENABLE` reader - LPTIM enable The ENABLE bit is set and cleared by software. 0:LPTIM is disabled 1:LPTIM is enabled
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - LPTIM enable The ENABLE bit is set and cleared by software. 0:LPTIM is disabled 1:LPTIM is enabled
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNGSTRT` reader - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
///= 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
///different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between ARR and CNT registers. If this bit is set simultaneously with CNTSTRT, then LPTIM will be in continuous counting mode.
pub type SngstrtR = crate::BitReader;
///Field `SNGSTRT` writer - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
///= 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
///different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between ARR and CNT registers. If this bit is set simultaneously with CNTSTRT, then LPTIM will be in continuous counting mode.
pub type SngstrtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTSTRT` reader - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
///= 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
///different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between ARR and CNT registers and the LPTIM counter keeps counting in Continuous mode.
pub type CntstrtR = crate::BitReader;
///Field `CNTSTRT` writer - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
///= 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
///different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between ARR and CNT registers and the LPTIM counter keeps counting in Continuous mode.
pub type CntstrtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTRST` reader - Counter reset This bit is set by software and cleared by hardware. When set to 1 this bit will trigger a synchronous reset of the CNT register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay. COUNTRST must never be set to 1 by software before it is already cleared to 0 by hardware. Software should consequently check that COUNTRST bit is already cleared to 0 before attempting to set it to 1.
pub type CountrstR = crate::BitReader;
///Field `COUNTRST` writer - Counter reset This bit is set by software and cleared by hardware. When set to 1 this bit will trigger a synchronous reset of the CNT register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay. COUNTRST must never be set to 1 by software before it is already cleared to 0 by hardware. Software should consequently check that COUNTRST bit is already cleared to 0 before attempting to set it to 1.
pub type CountrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software. 0:LPTIM is disabled 1:LPTIM is enabled
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    ///= 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
    ///different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between ARR and CNT registers. If this bit is set simultaneously with CNTSTRT, then LPTIM will be in continuous counting mode.
    #[inline(always)]
    pub fn sngstrt(&self) -> SngstrtR {
        SngstrtR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    ///= 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
    ///different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between ARR and CNT registers and the LPTIM counter keeps counting in Continuous mode.
    #[inline(always)]
    pub fn cntstrt(&self) -> CntstrtR {
        CntstrtR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to 1 this bit will trigger a synchronous reset of the CNT register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay. COUNTRST must never be set to 1 by software before it is already cleared to 0 by hardware. Software should consequently check that COUNTRST bit is already cleared to 0 before attempting to set it to 1.
    #[inline(always)]
    pub fn countrst(&self) -> CountrstR {
        CountrstR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("countrst", &self.countrst())
            .field("cntstrt", &self.cntstrt())
            .field("sngstrt", &self.sngstrt())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software. 0:LPTIM is disabled 1:LPTIM is enabled
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CRrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    ///= 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\]
    ///different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between ARR and CNT registers. If this bit is set simultaneously with CNTSTRT, then LPTIM will be in continuous counting mode.
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SngstrtW<CRrs> {
        SngstrtW::new(self, 1)
    }
    ///Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\]
    ///= 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\]
    ///different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between ARR and CNT registers and the LPTIM counter keeps counting in Continuous mode.
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CntstrtW<CRrs> {
        CntstrtW::new(self, 2)
    }
    ///Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to 1 this bit will trigger a synchronous reset of the CNT register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay. COUNTRST must never be set to 1 by software before it is already cleared to 0 by hardware. Software should consequently check that COUNTRST bit is already cleared to 0 before attempting to set it to 1.
    #[inline(always)]
    pub fn countrst(&mut self) -> CountrstW<CRrs> {
        CountrstW::new(self, 3)
    }
}
///LPTIM control register
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
