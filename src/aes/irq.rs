///Register `IRQ` reader
pub type R = crate::R<IRQrs>;
///Register `IRQ` writer
pub type W = crate::W<IRQrs>;
///Field `DONE_STAT` reader - AES_ACC done status
pub type DoneStatR = crate::BitReader;
///Field `DONE_STAT` writer - AES_ACC done status
pub type DoneStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_ERR_STAT` reader - AES_ACC bus error status
pub type BusErrStatR = crate::BitReader;
///Field `BUS_ERR_STAT` writer - AES_ACC bus error status
pub type BusErrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SETUP_ERR_STAT` reader - AES_ACC setup error status
pub type SetupErrStatR = crate::BitReader;
///Field `SETUP_ERR_STAT` writer - AES_ACC setup error status
pub type SetupErrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_DONE_STAT` reader - HASH_ACC done status
pub type HashDoneStatR = crate::BitReader;
///Field `HASH_DONE_STAT` writer - HASH_ACC done status
pub type HashDoneStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_BUS_ERR_STAT` reader - HASH_ACC bus error status
pub type HashBusErrStatR = crate::BitReader;
///Field `HASH_BUS_ERR_STAT` writer - HASH_ACC bus error status
pub type HashBusErrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_PAD_ERR_STAT` reader - HASH_ACC padding error status
pub type HashPadErrStatR = crate::BitReader;
///Field `HASH_PAD_ERR_STAT` writer - HASH_ACC padding error status
pub type HashPadErrStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DONE_RAW_STAT` reader - AES_ACC done raw status
pub type DoneRawStatR = crate::BitReader;
///Field `DONE_RAW_STAT` writer - AES_ACC done raw status
pub type DoneRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_ERR_RAW_STAT` reader - AES_ACC bus error raw status
pub type BusErrRawStatR = crate::BitReader;
///Field `BUS_ERR_RAW_STAT` writer - AES_ACC bus error raw status
pub type BusErrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SETUP_ERR_RAW_STAT` reader - AES_ACC setup error raw status
pub type SetupErrRawStatR = crate::BitReader;
///Field `SETUP_ERR_RAW_STAT` writer - AES_ACC setup error raw status
pub type SetupErrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_DONE_RAW_STAT` reader - HASH_ACC done raw status
pub type HashDoneRawStatR = crate::BitReader;
///Field `HASH_DONE_RAW_STAT` writer - HASH_ACC done raw status
pub type HashDoneRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_BUS_ERR_RAW_STAT` reader - HASH_ACC bus error raw status
pub type HashBusErrRawStatR = crate::BitReader;
///Field `HASH_BUS_ERR_RAW_STAT` writer - HASH_ACC bus error raw status
pub type HashBusErrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_PAD_ERR_RAW_STAT` reader - HASH_ACC padding error raw status
pub type HashPadErrRawStatR = crate::BitReader;
///Field `HASH_PAD_ERR_RAW_STAT` writer - HASH_ACC padding error raw status
pub type HashPadErrRawStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES_ACC done status
    #[inline(always)]
    pub fn done_stat(&self) -> DoneStatR {
        DoneStatR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES_ACC bus error status
    #[inline(always)]
    pub fn bus_err_stat(&self) -> BusErrStatR {
        BusErrStatR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AES_ACC setup error status
    #[inline(always)]
    pub fn setup_err_stat(&self) -> SetupErrStatR {
        SetupErrStatR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HASH_ACC done status
    #[inline(always)]
    pub fn hash_done_stat(&self) -> HashDoneStatR {
        HashDoneStatR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HASH_ACC bus error status
    #[inline(always)]
    pub fn hash_bus_err_stat(&self) -> HashBusErrStatR {
        HashBusErrStatR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH_ACC padding error status
    #[inline(always)]
    pub fn hash_pad_err_stat(&self) -> HashPadErrStatR {
        HashPadErrStatR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - AES_ACC done raw status
    #[inline(always)]
    pub fn done_raw_stat(&self) -> DoneRawStatR {
        DoneRawStatR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AES_ACC bus error raw status
    #[inline(always)]
    pub fn bus_err_raw_stat(&self) -> BusErrRawStatR {
        BusErrRawStatR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AES_ACC setup error raw status
    #[inline(always)]
    pub fn setup_err_raw_stat(&self) -> SetupErrRawStatR {
        SetupErrRawStatR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HASH_ACC done raw status
    #[inline(always)]
    pub fn hash_done_raw_stat(&self) -> HashDoneRawStatR {
        HashDoneRawStatR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HASH_ACC bus error raw status
    #[inline(always)]
    pub fn hash_bus_err_raw_stat(&self) -> HashBusErrRawStatR {
        HashBusErrRawStatR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - HASH_ACC padding error raw status
    #[inline(always)]
    pub fn hash_pad_err_raw_stat(&self) -> HashPadErrRawStatR {
        HashPadErrRawStatR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("hash_pad_err_raw_stat", &self.hash_pad_err_raw_stat())
            .field("hash_bus_err_raw_stat", &self.hash_bus_err_raw_stat())
            .field("hash_done_raw_stat", &self.hash_done_raw_stat())
            .field("setup_err_raw_stat", &self.setup_err_raw_stat())
            .field("bus_err_raw_stat", &self.bus_err_raw_stat())
            .field("done_raw_stat", &self.done_raw_stat())
            .field("hash_pad_err_stat", &self.hash_pad_err_stat())
            .field("hash_bus_err_stat", &self.hash_bus_err_stat())
            .field("hash_done_stat", &self.hash_done_stat())
            .field("setup_err_stat", &self.setup_err_stat())
            .field("bus_err_stat", &self.bus_err_stat())
            .field("done_stat", &self.done_stat())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES_ACC done status
    #[inline(always)]
    pub fn done_stat(&mut self) -> DoneStatW<IRQrs> {
        DoneStatW::new(self, 0)
    }
    ///Bit 1 - AES_ACC bus error status
    #[inline(always)]
    pub fn bus_err_stat(&mut self) -> BusErrStatW<IRQrs> {
        BusErrStatW::new(self, 1)
    }
    ///Bit 2 - AES_ACC setup error status
    #[inline(always)]
    pub fn setup_err_stat(&mut self) -> SetupErrStatW<IRQrs> {
        SetupErrStatW::new(self, 2)
    }
    ///Bit 3 - HASH_ACC done status
    #[inline(always)]
    pub fn hash_done_stat(&mut self) -> HashDoneStatW<IRQrs> {
        HashDoneStatW::new(self, 3)
    }
    ///Bit 4 - HASH_ACC bus error status
    #[inline(always)]
    pub fn hash_bus_err_stat(&mut self) -> HashBusErrStatW<IRQrs> {
        HashBusErrStatW::new(self, 4)
    }
    ///Bit 5 - HASH_ACC padding error status
    #[inline(always)]
    pub fn hash_pad_err_stat(&mut self) -> HashPadErrStatW<IRQrs> {
        HashPadErrStatW::new(self, 5)
    }
    ///Bit 16 - AES_ACC done raw status
    #[inline(always)]
    pub fn done_raw_stat(&mut self) -> DoneRawStatW<IRQrs> {
        DoneRawStatW::new(self, 16)
    }
    ///Bit 17 - AES_ACC bus error raw status
    #[inline(always)]
    pub fn bus_err_raw_stat(&mut self) -> BusErrRawStatW<IRQrs> {
        BusErrRawStatW::new(self, 17)
    }
    ///Bit 18 - AES_ACC setup error raw status
    #[inline(always)]
    pub fn setup_err_raw_stat(&mut self) -> SetupErrRawStatW<IRQrs> {
        SetupErrRawStatW::new(self, 18)
    }
    ///Bit 19 - HASH_ACC done raw status
    #[inline(always)]
    pub fn hash_done_raw_stat(&mut self) -> HashDoneRawStatW<IRQrs> {
        HashDoneRawStatW::new(self, 19)
    }
    ///Bit 20 - HASH_ACC bus error raw status
    #[inline(always)]
    pub fn hash_bus_err_raw_stat(&mut self) -> HashBusErrRawStatW<IRQrs> {
        HashBusErrRawStatW::new(self, 20)
    }
    ///Bit 21 - HASH_ACC padding error raw status
    #[inline(always)]
    pub fn hash_pad_err_raw_stat(&mut self) -> HashPadErrRawStatW<IRQrs> {
        HashPadErrRawStatW::new(self, 21)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IRQrs;
impl crate::RegisterSpec for IRQrs {
    type Ux = u32;
}
///`read()` method returns [`irq::R`](R) reader structure
impl crate::Readable for IRQrs {}
///`write(|w| ..)` method takes [`irq::W`](W) writer structure
impl crate::Writable for IRQrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ to value 0
impl crate::Resettable for IRQrs {
    const RESET_VALUE: u32 = 0;
}
