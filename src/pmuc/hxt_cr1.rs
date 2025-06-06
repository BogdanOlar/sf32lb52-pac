///Register `HXT_CR1` reader
pub type R = crate::R<HXT_CR1rs>;
///Register `HXT_CR1` writer
pub type W = crate::W<HXT_CR1rs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUF_EN` reader -
pub type BufEnR = crate::BitReader;
///Field `BUF_EN` writer -
pub type BufEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUF_DIG_EN` reader -
pub type BufDigEnR = crate::BitReader;
///Field `BUF_DIG_EN` writer -
pub type BufDigEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUF_DIG_STR` reader -
pub type BufDigStrR = crate::FieldReader;
///Field `BUF_DIG_STR` writer -
pub type BufDigStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUF_DLL_EN` reader -
pub type BufDllEnR = crate::BitReader;
///Field `BUF_DLL_EN` writer -
pub type BufDllEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUF_DLL_STR` reader -
pub type BufDllStrR = crate::FieldReader;
///Field `BUF_DLL_STR` writer -
pub type BufDllStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUF_AUD_EN` reader -
pub type BufAudEnR = crate::BitReader;
///Field `BUF_AUD_EN` writer -
pub type BufAudEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUF_AUD_STR` reader -
pub type BufAudStrR = crate::FieldReader;
///Field `BUF_AUD_STR` writer -
pub type BufAudStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUF_RF_STR` reader -
pub type BufRfStrR = crate::FieldReader;
///Field `BUF_RF_STR` writer -
pub type BufRfStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LDO_VREF` reader -
pub type LdoVrefR = crate::FieldReader;
///Field `LDO_VREF` writer -
pub type LdoVrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LDO_FLT_RSEL` reader -
pub type LdoFltRselR = crate::FieldReader;
///Field `LDO_FLT_RSEL` writer -
pub type LdoFltRselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GM_EN` reader -
pub type GmEnR = crate::BitReader;
///Field `GM_EN` writer -
pub type GmEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBANK_SEL` reader -
pub type CbankSelR = crate::FieldReader<u16>;
///Field `CBANK_SEL` writer -
pub type CbankSelW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn buf_en(&self) -> BufEnR {
        BufEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn buf_dig_en(&self) -> BufDigEnR {
        BufDigEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn buf_dig_str(&self) -> BufDigStrR {
        BufDigStrR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5
    #[inline(always)]
    pub fn buf_dll_en(&self) -> BufDllEnR {
        BufDllEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn buf_dll_str(&self) -> BufDllStrR {
        BufDllStrR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn buf_aud_en(&self) -> BufAudEnR {
        BufAudEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10
    #[inline(always)]
    pub fn buf_aud_str(&self) -> BufAudStrR {
        BufAudStrR::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn buf_rf_str(&self) -> BufRfStrR {
        BufRfStrR::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:16
    #[inline(always)]
    pub fn ldo_vref(&self) -> LdoVrefR {
        LdoVrefR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:18
    #[inline(always)]
    pub fn ldo_flt_rsel(&self) -> LdoFltRselR {
        LdoFltRselR::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19
    #[inline(always)]
    pub fn gm_en(&self) -> GmEnR {
        GmEnR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:29
    #[inline(always)]
    pub fn cbank_sel(&self) -> CbankSelR {
        CbankSelR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HXT_CR1")
            .field("cbank_sel", &self.cbank_sel())
            .field("gm_en", &self.gm_en())
            .field("ldo_flt_rsel", &self.ldo_flt_rsel())
            .field("ldo_vref", &self.ldo_vref())
            .field("buf_rf_str", &self.buf_rf_str())
            .field("buf_aud_str", &self.buf_aud_str())
            .field("buf_aud_en", &self.buf_aud_en())
            .field("buf_dll_str", &self.buf_dll_str())
            .field("buf_dll_en", &self.buf_dll_en())
            .field("buf_dig_str", &self.buf_dig_str())
            .field("buf_dig_en", &self.buf_dig_en())
            .field("buf_en", &self.buf_en())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<HXT_CR1rs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn buf_en(&mut self) -> BufEnW<HXT_CR1rs> {
        BufEnW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn buf_dig_en(&mut self) -> BufDigEnW<HXT_CR1rs> {
        BufDigEnW::new(self, 2)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn buf_dig_str(&mut self) -> BufDigStrW<HXT_CR1rs> {
        BufDigStrW::new(self, 3)
    }
    ///Bit 5
    #[inline(always)]
    pub fn buf_dll_en(&mut self) -> BufDllEnW<HXT_CR1rs> {
        BufDllEnW::new(self, 5)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn buf_dll_str(&mut self) -> BufDllStrW<HXT_CR1rs> {
        BufDllStrW::new(self, 6)
    }
    ///Bit 8
    #[inline(always)]
    pub fn buf_aud_en(&mut self) -> BufAudEnW<HXT_CR1rs> {
        BufAudEnW::new(self, 8)
    }
    ///Bits 9:10
    #[inline(always)]
    pub fn buf_aud_str(&mut self) -> BufAudStrW<HXT_CR1rs> {
        BufAudStrW::new(self, 9)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn buf_rf_str(&mut self) -> BufRfStrW<HXT_CR1rs> {
        BufRfStrW::new(self, 11)
    }
    ///Bits 13:16
    #[inline(always)]
    pub fn ldo_vref(&mut self) -> LdoVrefW<HXT_CR1rs> {
        LdoVrefW::new(self, 13)
    }
    ///Bits 17:18
    #[inline(always)]
    pub fn ldo_flt_rsel(&mut self) -> LdoFltRselW<HXT_CR1rs> {
        LdoFltRselW::new(self, 17)
    }
    ///Bit 19
    #[inline(always)]
    pub fn gm_en(&mut self) -> GmEnW<HXT_CR1rs> {
        GmEnW::new(self, 19)
    }
    ///Bits 20:29
    #[inline(always)]
    pub fn cbank_sel(&mut self) -> CbankSelW<HXT_CR1rs> {
        CbankSelW::new(self, 20)
    }
}
///HXT48 Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`hxt_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hxt_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HXT_CR1rs;
impl crate::RegisterSpec for HXT_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`hxt_cr1::R`](R) reader structure
impl crate::Readable for HXT_CR1rs {}
///`write(|w| ..)` method takes [`hxt_cr1::W`](W) writer structure
impl crate::Writable for HXT_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HXT_CR1 to value 0
impl crate::Resettable for HXT_CR1rs {
    const RESET_VALUE: u32 = 0;
}
