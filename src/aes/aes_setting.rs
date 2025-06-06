///Register `AES_SETTING` reader
pub type R = crate::R<AES_SETTINGrs>;
///Register `AES_SETTING` writer
pub type W = crate::W<AES_SETTINGrs>;
///Field `AES_MODE` reader - AES Mode: 3'h0: ECB 3'h1: CTR 3'h2: CBC Others: Reserved
pub type AesModeR = crate::FieldReader;
///Field `AES_MODE` writer - AES Mode: 3'h0: ECB 3'h1: CTR 3'h2: CBC Others: Reserved
pub type AesModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AES_LENGTH` reader - AES Length: 2'h0: 128-bit 2'h1: 192-bit 2'h2: 256-bit 2'h3: Reserved
pub type AesLengthR = crate::FieldReader;
///Field `AES_LENGTH` writer - AES Length: 2'h0: 128-bit 2'h1: 192-bit 2'h2: 256-bit 2'h3: Reserved
pub type AesLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEY_SEL` reader - 1'h0: select key from AES_ACC key registers 1'h1: use internal root key
pub type KeySelR = crate::BitReader;
///Field `KEY_SEL` writer - 1'h0: select key from AES_ACC key registers 1'h1: use internal root key
pub type KeySelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGO_STANDARD` reader - 1'h0: AES 1'h1: SM4
pub type AlgoStandardR = crate::BitReader;
///Field `ALGO_STANDARD` writer - 1'h0: AES 1'h1: SM4
pub type AlgoStandardW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES_OP_MODE` reader - 1'h0: decryption 1'h1: encryption
pub type AesOpModeR = crate::BitReader;
///Field `AES_OP_MODE` writer - 1'h0: decryption 1'h1: encryption
pub type AesOpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES_BYPASS` reader - 1'h0: normal operation 1'h1: bypass
pub type AesBypassR = crate::BitReader;
///Field `AES_BYPASS` writer - 1'h0: normal operation 1'h1: bypass
pub type AesBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - AES Mode: 3'h0: ECB 3'h1: CTR 3'h2: CBC Others: Reserved
    #[inline(always)]
    pub fn aes_mode(&self) -> AesModeR {
        AesModeR::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - AES Length: 2'h0: 128-bit 2'h1: 192-bit 2'h2: 256-bit 2'h3: Reserved
    #[inline(always)]
    pub fn aes_length(&self) -> AesLengthR {
        AesLengthR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - 1'h0: select key from AES_ACC key registers 1'h1: use internal root key
    #[inline(always)]
    pub fn key_sel(&self) -> KeySelR {
        KeySelR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1'h0: AES 1'h1: SM4
    #[inline(always)]
    pub fn algo_standard(&self) -> AlgoStandardR {
        AlgoStandardR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1'h0: decryption 1'h1: encryption
    #[inline(always)]
    pub fn aes_op_mode(&self) -> AesOpModeR {
        AesOpModeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1'h0: normal operation 1'h1: bypass
    #[inline(always)]
    pub fn aes_bypass(&self) -> AesBypassR {
        AesBypassR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_SETTING")
            .field("aes_bypass", &self.aes_bypass())
            .field("aes_op_mode", &self.aes_op_mode())
            .field("algo_standard", &self.algo_standard())
            .field("key_sel", &self.key_sel())
            .field("aes_length", &self.aes_length())
            .field("aes_mode", &self.aes_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AES Mode: 3'h0: ECB 3'h1: CTR 3'h2: CBC Others: Reserved
    #[inline(always)]
    pub fn aes_mode(&mut self) -> AesModeW<AES_SETTINGrs> {
        AesModeW::new(self, 0)
    }
    ///Bits 3:4 - AES Length: 2'h0: 128-bit 2'h1: 192-bit 2'h2: 256-bit 2'h3: Reserved
    #[inline(always)]
    pub fn aes_length(&mut self) -> AesLengthW<AES_SETTINGrs> {
        AesLengthW::new(self, 3)
    }
    ///Bit 5 - 1'h0: select key from AES_ACC key registers 1'h1: use internal root key
    #[inline(always)]
    pub fn key_sel(&mut self) -> KeySelW<AES_SETTINGrs> {
        KeySelW::new(self, 5)
    }
    ///Bit 6 - 1'h0: AES 1'h1: SM4
    #[inline(always)]
    pub fn algo_standard(&mut self) -> AlgoStandardW<AES_SETTINGrs> {
        AlgoStandardW::new(self, 6)
    }
    ///Bit 7 - 1'h0: decryption 1'h1: encryption
    #[inline(always)]
    pub fn aes_op_mode(&mut self) -> AesOpModeW<AES_SETTINGrs> {
        AesOpModeW::new(self, 7)
    }
    ///Bit 8 - 1'h0: normal operation 1'h1: bypass
    #[inline(always)]
    pub fn aes_bypass(&mut self) -> AesBypassW<AES_SETTINGrs> {
        AesBypassW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`aes_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AES_SETTINGrs;
impl crate::RegisterSpec for AES_SETTINGrs {
    type Ux = u32;
}
///`read()` method returns [`aes_setting::R`](R) reader structure
impl crate::Readable for AES_SETTINGrs {}
///`write(|w| ..)` method takes [`aes_setting::W`](W) writer structure
impl crate::Writable for AES_SETTINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_SETTING to value 0
impl crate::Resettable for AES_SETTINGrs {
    const RESET_VALUE: u32 = 0;
}
