///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
pub type UieR = crate::BitReader;
///Field `UIE` writer - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
pub type Cc1ieR = crate::BitReader;
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled. 1: CC2 interrupt enabled.
pub type Cc2ieR = crate::BitReader;
///Field `CC2IE` writer - Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled. 1: CC2 interrupt enabled.
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IE` reader - Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled. 1: CC3 interrupt enabled
pub type Cc3ieR = crate::BitReader;
///Field `CC3IE` writer - Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled. 1: CC3 interrupt enabled
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IE` reader - Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled. 1: CC4 interrupt enabled
pub type Cc4ieR = crate::BitReader;
///Field `CC4IE` writer - Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled. 1: CC4 interrupt enabled
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
pub type TieR = crate::BitReader;
///Field `TIE` writer - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
pub type UdeR = crate::BitReader;
///Field `UDE` writer - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled.
pub type Cc1deR = crate::BitReader;
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled.
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled. 1: CC2 DMA request enabled.
pub type Cc2deR = crate::BitReader;
///Field `CC2DE` writer - Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled. 1: CC2 DMA request enabled.
pub type Cc2deW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3DE` reader - Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled. 1: CC3 DMA request enabled.
pub type Cc3deR = crate::BitReader;
///Field `CC3DE` writer - Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled. 1: CC3 DMA request enabled.
pub type Cc3deW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4DE` reader - Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled. 1: CC4 DMA request enabled
pub type Cc4deR = crate::BitReader;
///Field `CC4DE` writer - Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled. 1: CC4 DMA request enabled
pub type Cc4deW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDE` reader - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
pub type TdeR = crate::BitReader;
///Field `TDE` writer - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bit 0 - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled. 1: CC2 interrupt enabled.
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled. 1: CC3 interrupt enabled
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled. 1: CC4 interrupt enabled
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled.
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled. 1: CC2 DMA request enabled.
    #[inline(always)]
    pub fn cc2de(&self) -> Cc2deR {
        Cc2deR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled. 1: CC3 DMA request enabled.
    #[inline(always)]
    pub fn cc3de(&self) -> Cc3deR {
        Cc3deR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled. 1: CC4 DMA request enabled
    #[inline(always)]
    pub fn cc4de(&self) -> Cc4deR {
        Cc4deR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("rsvd", &self.rsvd())
            .field("tde", &self.tde())
            .field("rsvd2", &self.rsvd2())
            .field("cc4de", &self.cc4de())
            .field("cc3de", &self.cc3de())
            .field("cc2de", &self.cc2de())
            .field("cc1de", &self.cc1de())
            .field("ude", &self.ude())
            .field("rsvd3", &self.rsvd3())
            .field("tie", &self.tie())
            .field("rsvd4", &self.rsvd4())
            .field("cc4ie", &self.cc4ie())
            .field("cc3ie", &self.cc3ie())
            .field("cc2ie", &self.cc2ie())
            .field("cc1ie", &self.cc1ie())
            .field("uie", &self.uie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<DIERrs> {
        UieW::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<DIERrs> {
        Cc1ieW::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled. 1: CC2 interrupt enabled.
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<DIERrs> {
        Cc2ieW::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled. 1: CC3 interrupt enabled
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<DIERrs> {
        Cc3ieW::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled. 1: CC4 interrupt enabled
    #[inline(always)]
    pub fn cc4ie(&mut self) -> Cc4ieW<DIERrs> {
        Cc4ieW::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<DIERrs> {
        Rsvd4W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<DIERrs> {
        TieW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<DIERrs> {
        Rsvd3W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<DIERrs> {
        UdeW::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled.
    #[inline(always)]
    pub fn cc1de(&mut self) -> Cc1deW<DIERrs> {
        Cc1deW::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled. 1: CC2 DMA request enabled.
    #[inline(always)]
    pub fn cc2de(&mut self) -> Cc2deW<DIERrs> {
        Cc2deW::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled. 1: CC3 DMA request enabled.
    #[inline(always)]
    pub fn cc3de(&mut self) -> Cc3deW<DIERrs> {
        Cc3deW::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled. 1: CC4 DMA request enabled
    #[inline(always)]
    pub fn cc4de(&mut self) -> Cc4deW<DIERrs> {
        Cc4deW::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DIERrs> {
        Rsvd2W::new(self, 13)
    }
    ///Bit 14 - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<DIERrs> {
        TdeW::new(self, 14)
    }
    ///Bits 15:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DIERrs> {
        RsvdW::new(self, 15)
    }
}
///TIM DMA/Interrupt enable register
///
///You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
