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
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WKUPCLR` reader - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
pub type WkupclrR = crate::BitReader;
///Field `WKUPCLR` writer - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
pub type WkupclrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
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
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
    #[inline(always)]
    pub fn wkupclr(&self) -> WkupclrR {
        WkupclrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("rsvd", &self.rsvd())
            .field("wkupclr", &self.wkupclr())
            .field("rsvd2", &self.rsvd2())
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
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ICRrs> {
        Rsvd2W::new(self, 4)
    }
    ///Bit 8 - wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register.
    #[inline(always)]
    pub fn wkupclr(&mut self) -> WkupclrW<ICRrs> {
        WkupclrW::new(self, 8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ICRrs> {
        RsvdW::new(self, 9)
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
