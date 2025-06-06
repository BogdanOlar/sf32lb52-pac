///Register `SETTING` reader
pub type R = crate::R<SETTINGrs>;
///Register `SETTING` writer
pub type W = crate::W<SETTINGrs>;
///Field `DONE_IRQ_MASK` reader - AES_ACC done interrupt mask, 0: mask the interrupt
pub type DoneIrqMaskR = crate::BitReader;
///Field `DONE_IRQ_MASK` writer - AES_ACC done interrupt mask, 0: mask the interrupt
pub type DoneIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_ERR_IRQ_MASK` reader - AES_ACC bus error interrupt mask, 0: mask the interrupt
pub type BusErrIrqMaskR = crate::BitReader;
///Field `BUS_ERR_IRQ_MASK` writer - AES_ACC bus error interrupt mask, 0: mask the interrupt
pub type BusErrIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SETUP_ERR_IRQ_MASK` reader - AES_ACC setup error interrupt mask, 0: mask the interrupt
pub type SetupErrIrqMaskR = crate::BitReader;
///Field `SETUP_ERR_IRQ_MASK` writer - AES_ACC setup error interrupt mask, 0: mask the interrupt
pub type SetupErrIrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_DONE_MASK` reader - HASH_ACC done interrupt mask, 0: mask the interrupt
pub type HashDoneMaskR = crate::BitReader;
///Field `HASH_DONE_MASK` writer - HASH_ACC done interrupt mask, 0: mask the interrupt
pub type HashDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_BUS_ERR_MASK` reader - HASH_ACC bus error interrpt mask, 0: mask the interrupt
pub type HashBusErrMaskR = crate::BitReader;
///Field `HASH_BUS_ERR_MASK` writer - HASH_ACC bus error interrpt mask, 0: mask the interrupt
pub type HashBusErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH_PAD_ERR_MASK` reader - HASH_ACC padding error interrupt mask, 0: mask the interrupt
pub type HashPadErrMaskR = crate::BitReader;
///Field `HASH_PAD_ERR_MASK` writer - HASH_ACC padding error interrupt mask, 0: mask the interrupt
pub type HashPadErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES_ACC done interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn done_irq_mask(&self) -> DoneIrqMaskR {
        DoneIrqMaskR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES_ACC bus error interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn bus_err_irq_mask(&self) -> BusErrIrqMaskR {
        BusErrIrqMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AES_ACC setup error interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn setup_err_irq_mask(&self) -> SetupErrIrqMaskR {
        SetupErrIrqMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HASH_ACC done interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn hash_done_mask(&self) -> HashDoneMaskR {
        HashDoneMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HASH_ACC bus error interrpt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn hash_bus_err_mask(&self) -> HashBusErrMaskR {
        HashBusErrMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH_ACC padding error interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn hash_pad_err_mask(&self) -> HashPadErrMaskR {
        HashPadErrMaskR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETTING")
            .field("hash_pad_err_mask", &self.hash_pad_err_mask())
            .field("hash_bus_err_mask", &self.hash_bus_err_mask())
            .field("hash_done_mask", &self.hash_done_mask())
            .field("setup_err_irq_mask", &self.setup_err_irq_mask())
            .field("bus_err_irq_mask", &self.bus_err_irq_mask())
            .field("done_irq_mask", &self.done_irq_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES_ACC done interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn done_irq_mask(&mut self) -> DoneIrqMaskW<SETTINGrs> {
        DoneIrqMaskW::new(self, 0)
    }
    ///Bit 1 - AES_ACC bus error interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn bus_err_irq_mask(&mut self) -> BusErrIrqMaskW<SETTINGrs> {
        BusErrIrqMaskW::new(self, 1)
    }
    ///Bit 2 - AES_ACC setup error interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn setup_err_irq_mask(&mut self) -> SetupErrIrqMaskW<SETTINGrs> {
        SetupErrIrqMaskW::new(self, 2)
    }
    ///Bit 3 - HASH_ACC done interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn hash_done_mask(&mut self) -> HashDoneMaskW<SETTINGrs> {
        HashDoneMaskW::new(self, 3)
    }
    ///Bit 4 - HASH_ACC bus error interrpt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn hash_bus_err_mask(&mut self) -> HashBusErrMaskW<SETTINGrs> {
        HashBusErrMaskW::new(self, 4)
    }
    ///Bit 5 - HASH_ACC padding error interrupt mask, 0: mask the interrupt
    #[inline(always)]
    pub fn hash_pad_err_mask(&mut self) -> HashPadErrMaskW<SETTINGrs> {
        HashPadErrMaskW::new(self, 5)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SETTINGrs;
impl crate::RegisterSpec for SETTINGrs {
    type Ux = u32;
}
///`read()` method returns [`setting::R`](R) reader structure
impl crate::Readable for SETTINGrs {}
///`write(|w| ..)` method takes [`setting::W`](W) writer structure
impl crate::Writable for SETTINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SETTING to value 0
impl crate::Resettable for SETTINGrs {
    const RESET_VALUE: u32 = 0;
}
