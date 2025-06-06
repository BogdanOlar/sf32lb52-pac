///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `UEIE` reader - Update event interrupt enable 0: Update event interrupt disabled 1: Update event interrupt enabled
pub type UeieR = crate::BitReader;
///Field `UEIE` writer - Update event interrupt enable 0: Update event interrupt disabled 1: Update event interrupt enabled
pub type UeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFIE` reader - Overflow Interrupt Enable 0: Overflow interrupt disabled 1: Overflow interrupt enabled
pub type OfieR = crate::BitReader;
///Field `OFIE` writer - Overflow Interrupt Enable 0: Overflow interrupt disabled 1: Overflow interrupt enabled
pub type OfieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCIE` reader - Output compare Interrupt Enable 0: Output compare interrupt disabled 1: Output compare interrupt enabled
pub type OcieR = crate::BitReader;
///Field `OCIE` writer - Output compare Interrupt Enable 0: Output compare interrupt disabled 1: Output compare interrupt enabled
pub type OcieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETIE` reader - External trigger valid edge Interrupt Enable 0: External trigger interrupt disabled 1: External trigger interrupt enabled
pub type EtieR = crate::BitReader;
///Field `ETIE` writer - External trigger valid edge Interrupt Enable 0: External trigger interrupt disabled 1: External trigger interrupt enabled
pub type EtieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `UEWE` reader - Update event Wakeup enable 0: Update event Wakeup disabled 1: Update event Wakeup enabled
pub type UeweR = crate::BitReader;
///Field `UEWE` writer - Update event Wakeup enable 0: Update event Wakeup disabled 1: Update event Wakeup enabled
pub type UeweW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFWE` reader - Overflow Wakeup Enable 0: Overflow Wakeup disabled 1: Overflow Wakeup enabled
pub type OfweR = crate::BitReader;
///Field `OFWE` writer - Overflow Wakeup Enable 0: Overflow Wakeup disabled 1: Overflow Wakeup enabled
pub type OfweW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCWE` reader - Output compare Wakeup Enable 0: Output compare wakeup disabled 1: Output compare wakeup enabled
pub type OcweR = crate::BitReader;
///Field `OCWE` writer - Output compare Wakeup Enable 0: Output compare wakeup disabled 1: Output compare wakeup enabled
pub type OcweW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    ///Bit 0 - Update event interrupt enable 0: Update event interrupt disabled 1: Update event interrupt enabled
    #[inline(always)]
    pub fn ueie(&self) -> UeieR {
        UeieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overflow Interrupt Enable 0: Overflow interrupt disabled 1: Overflow interrupt enabled
    #[inline(always)]
    pub fn ofie(&self) -> OfieR {
        OfieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output compare Interrupt Enable 0: Output compare interrupt disabled 1: Output compare interrupt enabled
    #[inline(always)]
    pub fn ocie(&self) -> OcieR {
        OcieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - External trigger valid edge Interrupt Enable 0: External trigger interrupt disabled 1: External trigger interrupt enabled
    #[inline(always)]
    pub fn etie(&self) -> EtieR {
        EtieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Update event Wakeup enable 0: Update event Wakeup disabled 1: Update event Wakeup enabled
    #[inline(always)]
    pub fn uewe(&self) -> UeweR {
        UeweR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Overflow Wakeup Enable 0: Overflow Wakeup disabled 1: Overflow Wakeup enabled
    #[inline(always)]
    pub fn ofwe(&self) -> OfweR {
        OfweR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output compare Wakeup Enable 0: Output compare wakeup disabled 1: Output compare wakeup enabled
    #[inline(always)]
    pub fn ocwe(&self) -> OcweR {
        OcweR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rsvd", &self.rsvd())
            .field("ocwe", &self.ocwe())
            .field("ofwe", &self.ofwe())
            .field("uewe", &self.uewe())
            .field("rsvd2", &self.rsvd2())
            .field("etie", &self.etie())
            .field("ocie", &self.ocie())
            .field("ofie", &self.ofie())
            .field("ueie", &self.ueie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update event interrupt enable 0: Update event interrupt disabled 1: Update event interrupt enabled
    #[inline(always)]
    pub fn ueie(&mut self) -> UeieW<IERrs> {
        UeieW::new(self, 0)
    }
    ///Bit 1 - Overflow Interrupt Enable 0: Overflow interrupt disabled 1: Overflow interrupt enabled
    #[inline(always)]
    pub fn ofie(&mut self) -> OfieW<IERrs> {
        OfieW::new(self, 1)
    }
    ///Bit 2 - Output compare Interrupt Enable 0: Output compare interrupt disabled 1: Output compare interrupt enabled
    #[inline(always)]
    pub fn ocie(&mut self) -> OcieW<IERrs> {
        OcieW::new(self, 2)
    }
    ///Bit 3 - External trigger valid edge Interrupt Enable 0: External trigger interrupt disabled 1: External trigger interrupt enabled
    #[inline(always)]
    pub fn etie(&mut self) -> EtieW<IERrs> {
        EtieW::new(self, 3)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<IERrs> {
        Rsvd2W::new(self, 4)
    }
    ///Bit 8 - Update event Wakeup enable 0: Update event Wakeup disabled 1: Update event Wakeup enabled
    #[inline(always)]
    pub fn uewe(&mut self) -> UeweW<IERrs> {
        UeweW::new(self, 8)
    }
    ///Bit 9 - Overflow Wakeup Enable 0: Overflow Wakeup disabled 1: Overflow Wakeup enabled
    #[inline(always)]
    pub fn ofwe(&mut self) -> OfweW<IERrs> {
        OfweW::new(self, 9)
    }
    ///Bit 10 - Output compare Wakeup Enable 0: Output compare wakeup disabled 1: Output compare wakeup enabled
    #[inline(always)]
    pub fn ocwe(&mut self) -> OcweW<IERrs> {
        OcweW::new(self, 10)
    }
    ///Bits 11:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IERrs> {
        RsvdW::new(self, 11)
    }
}
///LPTIM interrupt and wakeup enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
