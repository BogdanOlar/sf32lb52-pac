///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Register `STATUS` writer
pub type W = crate::W<STATUSrs>;
///Field `BUSY` reader - AES_ACC block is busy
pub type BusyR = crate::BitReader;
///Field `BUSY` writer - AES_ACC block is busy
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_KEY_VALID` reader - flash key valid indicator
pub type FlashKeyValidR = crate::BitReader;
///Field `FLASH_KEY_VALID` writer - flash key valid indicator
pub type FlashKeyValidW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_BUSY` reader - HASH_ACC block is busy
pub type HashBusyR = crate::BitReader;
///Field `HASH_BUSY` writer - HASH_ACC block is busy
pub type HashBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES_ACC block is busy
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - flash key valid indicator
    #[inline(always)]
    pub fn flash_key_valid(&self) -> FlashKeyValidR {
        FlashKeyValidR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HASH_ACC block is busy
    #[inline(always)]
    pub fn hash_busy(&self) -> HashBusyR {
        HashBusyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("hash_busy", &self.hash_busy())
            .field("flash_key_valid", &self.flash_key_valid())
            .field("busy", &self.busy())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES_ACC block is busy
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<STATUSrs> {
        BusyW::new(self, 0)
    }
    ///Bit 1 - flash key valid indicator
    #[inline(always)]
    pub fn flash_key_valid(&mut self) -> FlashKeyValidW<STATUSrs> {
        FlashKeyValidW::new(self, 1)
    }
    ///Bit 2 - HASH_ACC block is busy
    #[inline(always)]
    pub fn hash_busy(&mut self) -> HashBusyW<STATUSrs> {
        HashBusyW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
