///Register `MISCR` reader
pub type R = crate::R<MISCRrs>;
///Register `MISCR` writer
pub type W = crate::W<MISCRrs>;
///Field `RXCLKDLY` reader - Add delay on internal Rx clock to fine tune the sampling position. Note: effective 5-bit
pub type RxclkdlyR = crate::FieldReader;
///Field `RXCLKDLY` writer - Add delay on internal Rx clock to fine tune the sampling position. Note: effective 5-bit
pub type RxclkdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCKDLY` reader - Add delay on output clock to fine tune the clock position. Note: effective 7-bit
pub type SckdlyR = crate::FieldReader;
///Field `SCKDLY` writer - Add delay on output clock to fine tune the clock position. Note: effective 7-bit
pub type SckdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DQSDLY` reader - Delay the input DQS signal to the appropriate sampling position. For device w/ DQS signal only. Note: effective 7-bit
pub type DqsdlyR = crate::FieldReader;
///Field `DQSDLY` writer - Delay the input DQS signal to the appropriate sampling position. For device w/ DQS signal only. Note: effective 7-bit
pub type DqsdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RXCLKINV` reader - Invert internal Rx clock to add half-cycle delay (coarse tune) when sampling data. It is usually used for FLASH device w/ higher frequency.
pub type RxclkinvR = crate::BitReader;
///Field `RXCLKINV` writer - Invert internal Rx clock to add half-cycle delay (coarse tune) when sampling data. It is usually used for FLASH device w/ higher frequency.
pub type RxclkinvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCKINV` reader - Invert output clock. This bit is used to align (coarse tune) the output clock to the center of output data.
pub type SckinvR = crate::BitReader;
///Field `SCKINV` writer - Invert output clock. This bit is used to align (coarse tune) the output clock to the center of output data.
pub type SckinvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTRPRE` reader - Enable pre-sampling for DTRReserved-Do not modify
pub type DtrpreR = crate::BitReader;
///Field `DTRPRE` writer - Enable pre-sampling for DTRReserved-Do not modify
pub type DtrpreW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSEL` reader -
pub type DbgselR = crate::FieldReader;
///Field `DBGSEL` writer -
pub type DbgselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Add delay on internal Rx clock to fine tune the sampling position. Note: effective 5-bit
    #[inline(always)]
    pub fn rxclkdly(&self) -> RxclkdlyR {
        RxclkdlyR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Add delay on output clock to fine tune the clock position. Note: effective 7-bit
    #[inline(always)]
    pub fn sckdly(&self) -> SckdlyR {
        SckdlyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Delay the input DQS signal to the appropriate sampling position. For device w/ DQS signal only. Note: effective 7-bit
    #[inline(always)]
    pub fn dqsdly(&self) -> DqsdlyR {
        DqsdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Invert internal Rx clock to add half-cycle delay (coarse tune) when sampling data. It is usually used for FLASH device w/ higher frequency.
    #[inline(always)]
    pub fn rxclkinv(&self) -> RxclkinvR {
        RxclkinvR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Invert output clock. This bit is used to align (coarse tune) the output clock to the center of output data.
    #[inline(always)]
    pub fn sckinv(&self) -> SckinvR {
        SckinvR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Enable pre-sampling for DTRReserved-Do not modify
    #[inline(always)]
    pub fn dtrpre(&self) -> DtrpreR {
        DtrpreR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn dbgsel(&self) -> DbgselR {
        DbgselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISCR")
            .field("dbgsel", &self.dbgsel())
            .field("dtrpre", &self.dtrpre())
            .field("sckinv", &self.sckinv())
            .field("rxclkinv", &self.rxclkinv())
            .field("dqsdly", &self.dqsdly())
            .field("sckdly", &self.sckdly())
            .field("rxclkdly", &self.rxclkdly())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Add delay on internal Rx clock to fine tune the sampling position. Note: effective 5-bit
    #[inline(always)]
    pub fn rxclkdly(&mut self) -> RxclkdlyW<MISCRrs> {
        RxclkdlyW::new(self, 0)
    }
    ///Bits 8:15 - Add delay on output clock to fine tune the clock position. Note: effective 7-bit
    #[inline(always)]
    pub fn sckdly(&mut self) -> SckdlyW<MISCRrs> {
        SckdlyW::new(self, 8)
    }
    ///Bits 16:23 - Delay the input DQS signal to the appropriate sampling position. For device w/ DQS signal only. Note: effective 7-bit
    #[inline(always)]
    pub fn dqsdly(&mut self) -> DqsdlyW<MISCRrs> {
        DqsdlyW::new(self, 16)
    }
    ///Bit 24 - Invert internal Rx clock to add half-cycle delay (coarse tune) when sampling data. It is usually used for FLASH device w/ higher frequency.
    #[inline(always)]
    pub fn rxclkinv(&mut self) -> RxclkinvW<MISCRrs> {
        RxclkinvW::new(self, 24)
    }
    ///Bit 25 - Invert output clock. This bit is used to align (coarse tune) the output clock to the center of output data.
    #[inline(always)]
    pub fn sckinv(&mut self) -> SckinvW<MISCRrs> {
        SckinvW::new(self, 25)
    }
    ///Bit 26 - Enable pre-sampling for DTRReserved-Do not modify
    #[inline(always)]
    pub fn dtrpre(&mut self) -> DtrpreW<MISCRrs> {
        DtrpreW::new(self, 26)
    }
    ///Bits 28:31
    #[inline(always)]
    pub fn dbgsel(&mut self) -> DbgselW<MISCRrs> {
        DbgselW::new(self, 28)
    }
}
///Miscelaneous Register
///
///You can [`read`](crate::Reg::read) this register and get [`miscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MISCRrs;
impl crate::RegisterSpec for MISCRrs {
    type Ux = u32;
}
///`read()` method returns [`miscr::R`](R) reader structure
impl crate::Readable for MISCRrs {}
///`write(|w| ..)` method takes [`miscr::W`](W) writer structure
impl crate::Writable for MISCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MISCR to value 0
impl crate::Resettable for MISCRrs {
    const RESET_VALUE: u32 = 0;
}
