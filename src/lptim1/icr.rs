///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `UECLR` reader - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
pub type UeclrR = crate::BitReader;
///Field `UECLR` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
pub type UeclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFCLR` reader - Overflow clear flag Writing 1 to this bit clears the OF flag in the LPTIM_ISR register
pub type OfclrR = crate::BitReader;
///Field `OFCLR` writer - Overflow clear flag Writing 1 to this bit clears the OF flag in the LPTIM_ISR register
pub type OfclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCCLR` reader - Output compare clear flag Writing 1 to this bit clears the OC flag in the LPTIM_ISR register
pub type OcclrR = crate::BitReader;
///Field `OCCLR` writer - Output compare clear flag Writing 1 to this bit clears the OC flag in the LPTIM_ISR register
pub type OcclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETCLR` reader - External trigger valid edge clear flag Writing 1 to this bit clears the ET flag in the LPTIM_ISR register
pub type EtclrR = crate::BitReader;
///Field `ETCLR` writer - External trigger valid edge clear flag Writing 1 to this bit clears the ET flag in the LPTIM_ISR register
pub type EtclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPCLR` reader - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
pub type WkupclrR = crate::BitReader;
///Field `WKUPCLR` writer - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
pub type WkupclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn ueclr(&self) -> UeclrR {
        UeclrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overflow clear flag Writing 1 to this bit clears the OF flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn ofclr(&self) -> OfclrR {
        OfclrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output compare clear flag Writing 1 to this bit clears the OC flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn occlr(&self) -> OcclrR {
        OcclrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - External trigger valid edge clear flag Writing 1 to this bit clears the ET flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn etclr(&self) -> EtclrR {
        EtclrR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
    #[inline(always)]
    pub fn wkupclr(&self) -> WkupclrR {
        WkupclrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("wkupclr", &self.wkupclr())
            .field("etclr", &self.etclr())
            .field("occlr", &self.occlr())
            .field("ofclr", &self.ofclr())
            .field("ueclr", &self.ueclr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn ueclr(&mut self) -> UeclrW<ICRrs> {
        UeclrW::new(self, 0)
    }
    ///Bit 1 - Overflow clear flag Writing 1 to this bit clears the OF flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn ofclr(&mut self) -> OfclrW<ICRrs> {
        OfclrW::new(self, 1)
    }
    ///Bit 2 - Output compare clear flag Writing 1 to this bit clears the OC flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn occlr(&mut self) -> OcclrW<ICRrs> {
        OcclrW::new(self, 2)
    }
    ///Bit 3 - External trigger valid edge clear flag Writing 1 to this bit clears the ET flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn etclr(&mut self) -> EtclrW<ICRrs> {
        EtclrW::new(self, 3)
    }
    ///Bit 8 - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
    #[inline(always)]
    pub fn wkupclr(&mut self) -> WkupclrW<ICRrs> {
        WkupclrW::new(self, 8)
    }
}
///LPTIM interrupt and status clear register
///
///You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
