///Register `COMMAND` reader
pub type R = crate::R<COMMANDrs>;
///Register `COMMAND` writer
pub type W = crate::W<COMMANDrs>;
///Field `START` reader - write 1 to trigger the AES_ACC block
pub type StartR = crate::BitReader;
///Field `START` writer - write 1 to trigger the AES_ACC block
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES_ACC_RESET` reader - AES_ACC soft reset, 1'h1: reset the AES_ACC block
pub type AesAccResetR = crate::BitReader;
///Field `AES_ACC_RESET` writer - AES_ACC soft reset, 1'h1: reset the AES_ACC block
pub type AesAccResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_START` reader - write 1 to trigger the HASH_ACC block
pub type HashStartR = crate::BitReader;
///Field `HASH_START` writer - write 1 to trigger the HASH_ACC block
pub type HashStartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_RESET` reader - HASH_ACC soft reset, 1'h1: reset the HASH_ACC block
pub type HashResetR = crate::BitReader;
///Field `HASH_RESET` writer - HASH_ACC soft reset, 1'h1: reset the HASH_ACC block
pub type HashResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTO_GATE` reader - auto clock gating
pub type AutoGateR = crate::BitReader;
///Field `AUTO_GATE` writer - auto clock gating
pub type AutoGateW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bit 0 - write 1 to trigger the AES_ACC block
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES_ACC soft reset, 1'h1: reset the AES_ACC block
    #[inline(always)]
    pub fn aes_acc_reset(&self) -> AesAccResetR {
        AesAccResetR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - write 1 to trigger the HASH_ACC block
    #[inline(always)]
    pub fn hash_start(&self) -> HashStartR {
        HashStartR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HASH_ACC soft reset, 1'h1: reset the HASH_ACC block
    #[inline(always)]
    pub fn hash_reset(&self) -> HashResetR {
        HashResetR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - auto clock gating
    #[inline(always)]
    pub fn auto_gate(&self) -> AutoGateR {
        AutoGateR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND")
            .field("rsvd", &self.rsvd())
            .field("auto_gate", &self.auto_gate())
            .field("hash_reset", &self.hash_reset())
            .field("hash_start", &self.hash_start())
            .field("aes_acc_reset", &self.aes_acc_reset())
            .field("start", &self.start())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to trigger the AES_ACC block
    #[inline(always)]
    pub fn start(&mut self) -> StartW<COMMANDrs> {
        StartW::new(self, 0)
    }
    ///Bit 1 - AES_ACC soft reset, 1'h1: reset the AES_ACC block
    #[inline(always)]
    pub fn aes_acc_reset(&mut self) -> AesAccResetW<COMMANDrs> {
        AesAccResetW::new(self, 1)
    }
    ///Bit 2 - write 1 to trigger the HASH_ACC block
    #[inline(always)]
    pub fn hash_start(&mut self) -> HashStartW<COMMANDrs> {
        HashStartW::new(self, 2)
    }
    ///Bit 3 - HASH_ACC soft reset, 1'h1: reset the HASH_ACC block
    #[inline(always)]
    pub fn hash_reset(&mut self) -> HashResetW<COMMANDrs> {
        HashResetW::new(self, 3)
    }
    ///Bit 4 - auto clock gating
    #[inline(always)]
    pub fn auto_gate(&mut self) -> AutoGateW<COMMANDrs> {
        AutoGateW::new(self, 4)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<COMMANDrs> {
        RsvdW::new(self, 5)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COMMANDrs;
impl crate::RegisterSpec for COMMANDrs {
    type Ux = u32;
}
///`read()` method returns [`command::R`](R) reader structure
impl crate::Readable for COMMANDrs {}
///`write(|w| ..)` method takes [`command::W`](W) writer structure
impl crate::Writable for COMMANDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMMAND to value 0
impl crate::Resettable for COMMANDrs {
    const RESET_VALUE: u32 = 0;
}
