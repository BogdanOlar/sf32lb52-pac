///Register `HASH_SETTING` reader
pub type R = crate::R<HASH_SETTINGrs>;
///Register `HASH_SETTING` writer
pub type W = crate::W<HASH_SETTINGrs>;
///Field `HASH_MODE` reader - HASH Mode: 3'h0: SHA-1 3'h1: SHA-224 3'h2: SHA-256 3'h3: SM3 Others: Reserved
pub type HashModeR = crate::FieldReader;
///Field `HASH_MODE` writer - HASH Mode: 3'h0: SHA-1 3'h1: SHA-224 3'h2: SHA-256 3'h3: SM3 Others: Reserved
pub type HashModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DO_PADDING` reader - HASH padding enable. Set 1 to do padding after data transfer.
pub type DoPaddingR = crate::BitReader;
///Field `DO_PADDING` writer - HASH padding enable. Set 1 to do padding after data transfer.
pub type DoPaddingW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTE_SWAP` reader - HASH byte swap option. Set 1 to swap byte order when read data from memory.
pub type ByteSwapR = crate::BitReader;
///Field `BYTE_SWAP` writer - HASH byte swap option. Set 1 to swap byte order when read data from memory.
pub type ByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFT_IV_SEL` reader - HASH default iv select. 1'h0: default iv according to hash mode 1'h1: default iv from HASH_IV_H* registers
pub type DftIvSelR = crate::BitReader;
///Field `DFT_IV_SEL` writer - HASH default iv select. 1'h0: default iv according to hash mode 1'h1: default iv from HASH_IV_H* registers
pub type DftIvSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESULT_ENDIAN` reader - hash result endian setting: 1'h0: little endian 1'h1: big endian
pub type ResultEndianR = crate::BitReader;
///Field `RESULT_ENDIAN` writer - hash result endian setting: 1'h0: little endian 1'h1: big endian
pub type ResultEndianW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_IV_LOAD` reader - write 1 to load hash iv
pub type HashIvLoadR = crate::BitReader;
///Field `HASH_IV_LOAD` writer - write 1 to load hash iv
pub type HashIvLoadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_LEN_LOAD` reader - write 1 to load hash length
pub type HashLenLoadR = crate::BitReader;
///Field `HASH_LEN_LOAD` writer - write 1 to load hash length
pub type HashLenLoadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:2 - HASH Mode: 3'h0: SHA-1 3'h1: SHA-224 3'h2: SHA-256 3'h3: SM3 Others: Reserved
    #[inline(always)]
    pub fn hash_mode(&self) -> HashModeR {
        HashModeR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - HASH padding enable. Set 1 to do padding after data transfer.
    #[inline(always)]
    pub fn do_padding(&self) -> DoPaddingR {
        DoPaddingR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HASH byte swap option. Set 1 to swap byte order when read data from memory.
    #[inline(always)]
    pub fn byte_swap(&self) -> ByteSwapR {
        ByteSwapR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH default iv select. 1'h0: default iv according to hash mode 1'h1: default iv from HASH_IV_H* registers
    #[inline(always)]
    pub fn dft_iv_sel(&self) -> DftIvSelR {
        DftIvSelR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - hash result endian setting: 1'h0: little endian 1'h1: big endian
    #[inline(always)]
    pub fn result_endian(&self) -> ResultEndianR {
        ResultEndianR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - write 1 to load hash iv
    #[inline(always)]
    pub fn hash_iv_load(&self) -> HashIvLoadR {
        HashIvLoadR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - write 1 to load hash length
    #[inline(always)]
    pub fn hash_len_load(&self) -> HashLenLoadR {
        HashLenLoadR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_SETTING")
            .field("rsvd", &self.rsvd())
            .field("hash_len_load", &self.hash_len_load())
            .field("hash_iv_load", &self.hash_iv_load())
            .field("result_endian", &self.result_endian())
            .field("dft_iv_sel", &self.dft_iv_sel())
            .field("byte_swap", &self.byte_swap())
            .field("do_padding", &self.do_padding())
            .field("hash_mode", &self.hash_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HASH Mode: 3'h0: SHA-1 3'h1: SHA-224 3'h2: SHA-256 3'h3: SM3 Others: Reserved
    #[inline(always)]
    pub fn hash_mode(&mut self) -> HashModeW<HASH_SETTINGrs> {
        HashModeW::new(self, 0)
    }
    ///Bit 3 - HASH padding enable. Set 1 to do padding after data transfer.
    #[inline(always)]
    pub fn do_padding(&mut self) -> DoPaddingW<HASH_SETTINGrs> {
        DoPaddingW::new(self, 3)
    }
    ///Bit 4 - HASH byte swap option. Set 1 to swap byte order when read data from memory.
    #[inline(always)]
    pub fn byte_swap(&mut self) -> ByteSwapW<HASH_SETTINGrs> {
        ByteSwapW::new(self, 4)
    }
    ///Bit 5 - HASH default iv select. 1'h0: default iv according to hash mode 1'h1: default iv from HASH_IV_H* registers
    #[inline(always)]
    pub fn dft_iv_sel(&mut self) -> DftIvSelW<HASH_SETTINGrs> {
        DftIvSelW::new(self, 5)
    }
    ///Bit 6 - hash result endian setting: 1'h0: little endian 1'h1: big endian
    #[inline(always)]
    pub fn result_endian(&mut self) -> ResultEndianW<HASH_SETTINGrs> {
        ResultEndianW::new(self, 6)
    }
    ///Bit 7 - write 1 to load hash iv
    #[inline(always)]
    pub fn hash_iv_load(&mut self) -> HashIvLoadW<HASH_SETTINGrs> {
        HashIvLoadW::new(self, 7)
    }
    ///Bit 8 - write 1 to load hash length
    #[inline(always)]
    pub fn hash_len_load(&mut self) -> HashLenLoadW<HASH_SETTINGrs> {
        HashLenLoadW::new(self, 8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<HASH_SETTINGrs> {
        RsvdW::new(self, 9)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hash_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HASH_SETTINGrs;
impl crate::RegisterSpec for HASH_SETTINGrs {
    type Ux = u32;
}
///`read()` method returns [`hash_setting::R`](R) reader structure
impl crate::Readable for HASH_SETTINGrs {}
///`write(|w| ..)` method takes [`hash_setting::W`](W) writer structure
impl crate::Writable for HASH_SETTINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HASH_SETTING to value 0
impl crate::Resettable for HASH_SETTINGrs {
    const RESET_VALUE: u32 = 0;
}
