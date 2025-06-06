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
///Field `COMIE` reader - COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
pub type ComieR = crate::BitReader;
///Field `COMIE` writer - COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
pub type TieR = crate::BitReader;
///Field `TIE` writer - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIE` reader - Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
pub type BieR = crate::BitReader;
///Field `BIE` writer - Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `COMDE` reader - COM DMA request enable 0: COM DMA request disabled 1: COM DMA request enabled
pub type ComdeR = crate::BitReader;
///Field `COMDE` writer - COM DMA request enable 0: COM DMA request disabled 1: COM DMA request enabled
pub type ComdeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDE` reader - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
pub type TdeR = crate::BitReader;
///Field `TDE` writer - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5IE` reader - Capture/Compare 5 interrupt enable 0: CC5 interrupt disabled. 1: CC5 interrupt enabled
pub type Cc5ieR = crate::BitReader;
///Field `CC5IE` writer - Capture/Compare 5 interrupt enable 0: CC5 interrupt disabled. 1: CC5 interrupt enabled
pub type Cc5ieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6IE` reader - Capture/Compare 6 interrupt enable 0: CC6 interrupt disabled. 1: CC6 interrupt enabled
pub type Cc6ieR = crate::BitReader;
///Field `CC6IE` writer - Capture/Compare 6 interrupt enable 0: CC6 interrupt disabled. 1: CC6 interrupt enabled
pub type Cc6ieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
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
    ///Bit 5 - COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
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
    ///Bit 13 - COM DMA request enable 0: COM DMA request disabled 1: COM DMA request enabled
    #[inline(always)]
    pub fn comde(&self) -> ComdeR {
        ComdeR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Capture/Compare 5 interrupt enable 0: CC5 interrupt disabled. 1: CC5 interrupt enabled
    #[inline(always)]
    pub fn cc5ie(&self) -> Cc5ieR {
        Cc5ieR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Capture/Compare 6 interrupt enable 0: CC6 interrupt disabled. 1: CC6 interrupt enabled
    #[inline(always)]
    pub fn cc6ie(&self) -> Cc6ieR {
        Cc6ieR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("rsvd", &self.rsvd())
            .field("cc6ie", &self.cc6ie())
            .field("cc5ie", &self.cc5ie())
            .field("rsvd2", &self.rsvd2())
            .field("tde", &self.tde())
            .field("comde", &self.comde())
            .field("cc4de", &self.cc4de())
            .field("cc3de", &self.cc3de())
            .field("cc2de", &self.cc2de())
            .field("cc1de", &self.cc1de())
            .field("ude", &self.ude())
            .field("bie", &self.bie())
            .field("tie", &self.tie())
            .field("comie", &self.comie())
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
    ///Bit 5 - COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
    #[inline(always)]
    pub fn comie(&mut self) -> ComieW<DIERrs> {
        ComieW::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<DIERrs> {
        TieW::new(self, 6)
    }
    ///Bit 7 - Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<DIERrs> {
        BieW::new(self, 7)
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
    ///Bit 13 - COM DMA request enable 0: COM DMA request disabled 1: COM DMA request enabled
    #[inline(always)]
    pub fn comde(&mut self) -> ComdeW<DIERrs> {
        ComdeW::new(self, 13)
    }
    ///Bit 14 - Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled.
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<DIERrs> {
        TdeW::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DIERrs> {
        Rsvd2W::new(self, 15)
    }
    ///Bit 16 - Capture/Compare 5 interrupt enable 0: CC5 interrupt disabled. 1: CC5 interrupt enabled
    #[inline(always)]
    pub fn cc5ie(&mut self) -> Cc5ieW<DIERrs> {
        Cc5ieW::new(self, 16)
    }
    ///Bit 17 - Capture/Compare 6 interrupt enable 0: CC6 interrupt disabled. 1: CC6 interrupt enabled
    #[inline(always)]
    pub fn cc6ie(&mut self) -> Cc6ieW<DIERrs> {
        Cc6ieW::new(self, 17)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DIERrs> {
        RsvdW::new(self, 18)
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
