///Register `CCMR3` reader
pub type R = crate::R<CCMR3rs>;
///Register `CCMR3` writer
pub type W = crate::W<CCMR3rs>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader<u16>;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `GC5C1` reader - Group Channel 5 and Channel 1 Distortion on Channel 1 output: 0: No effect of OC5REF on OC1REFC5 1: OC1REFC is the logical AND of OC1REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
pub type Gc5c1R = crate::BitReader;
///Field `GC5C1` writer - Group Channel 5 and Channel 1 Distortion on Channel 1 output: 0: No effect of OC5REF on OC1REFC5 1: OC1REFC is the logical AND of OC1REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
pub type Gc5c1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GC5C2` reader - Group Channel 5 and Channel 2 Distortion on Channel 2 output: 0: No effect of OC5REF on OC2REFC 1: OC2REFC is the logical AND of OC2REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
pub type Gc5c2R = crate::BitReader;
///Field `GC5C2` writer - Group Channel 5 and Channel 2 Distortion on Channel 2 output: 0: No effect of OC5REF on OC2REFC 1: OC2REFC is the logical AND of OC2REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
pub type Gc5c2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GC5C3` reader - Group Channel 5 and Channel 3 Distortion on Channel 3 output: 0: No effect of OC5REF on OC3REFC 1: OC3REFC is the logical AND of OC3REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2).
pub type Gc5c3R = crate::BitReader;
///Field `GC5C3` writer - Group Channel 5 and Channel 3 Distortion on Channel 3 output: 0: No effect of OC5REF on OC3REFC 1: OC3REFC is the logical AND of OC3REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2).
pub type Gc5c3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC5CE` reader - Output compare 5 clear enable
pub type Oc5ceR = crate::BitReader;
///Field `OC5CE` writer - Output compare 5 clear enable
pub type Oc5ceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC5PE` reader - Output compare 5 preload enable
pub type Oc5peR = crate::BitReader;
///Field `OC5PE` writer - Output compare 5 preload enable
pub type Oc5peW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC5M` reader - Output compare 5 mode
pub type Oc5mR = crate::FieldReader;
///Field `OC5M` writer - Output compare 5 mode
pub type Oc5mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OC6CE` reader - Output compare 6 clear enable
pub type Oc6ceR = crate::BitReader;
///Field `OC6CE` writer - Output compare 6 clear enable
pub type Oc6ceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC6PE` reader - Output compare 6 preload enable
pub type Oc6peR = crate::BitReader;
///Field `OC6PE` writer - Output compare 6 preload enable
pub type Oc6peW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC6M` reader - Output compare 6 mode
pub type Oc6mR = crate::FieldReader;
///Field `OC6M` writer - Output compare 6 mode
pub type Oc6mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:12
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new((self.bits & 0x1fff) as u16)
    }
    ///Bit 13 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: 0: No effect of OC5REF on OC1REFC5 1: OC1REFC is the logical AND of OC1REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
    #[inline(always)]
    pub fn gc5c1(&self) -> Gc5c1R {
        Gc5c1R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: 0: No effect of OC5REF on OC2REFC 1: OC2REFC is the logical AND of OC2REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
    #[inline(always)]
    pub fn gc5c2(&self) -> Gc5c2R {
        Gc5c2R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: 0: No effect of OC5REF on OC3REFC 1: OC3REFC is the logical AND of OC3REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2).
    #[inline(always)]
    pub fn gc5c3(&self) -> Gc5c3R {
        Gc5c3R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&self) -> Oc5ceR {
        Oc5ceR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&self) -> Oc5peR {
        Oc5peR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m(&self) -> Oc5mR {
        Oc5mR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&self) -> Oc6ceR {
        Oc6ceR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&self) -> Oc6peR {
        Oc6peR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m(&self) -> Oc6mR {
        Oc6mR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR3")
            .field("oc6m", &self.oc6m())
            .field("oc6pe", &self.oc6pe())
            .field("rsvd", &self.rsvd())
            .field("oc6ce", &self.oc6ce())
            .field("oc5m", &self.oc5m())
            .field("oc5pe", &self.oc5pe())
            .field("rsvd2", &self.rsvd2())
            .field("oc5ce", &self.oc5ce())
            .field("gc5c3", &self.gc5c3())
            .field("gc5c2", &self.gc5c2())
            .field("gc5c1", &self.gc5c1())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
impl W {
    ///Bits 0:12
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CCMR3rs> {
        Rsvd3W::new(self, 0)
    }
    ///Bit 13 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: 0: No effect of OC5REF on OC1REFC5 1: OC1REFC is the logical AND of OC1REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
    #[inline(always)]
    pub fn gc5c1(&mut self) -> Gc5c1W<CCMR3rs> {
        Gc5c1W::new(self, 13)
    }
    ///Bit 14 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: 0: No effect of OC5REF on OC2REFC 1: OC2REFC is the logical AND of OC2REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1).
    #[inline(always)]
    pub fn gc5c2(&mut self) -> Gc5c2W<CCMR3rs> {
        Gc5c2W::new(self, 14)
    }
    ///Bit 15 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: 0: No effect of OC5REF on OC3REFC 1: OC3REFC is the logical AND of OC3REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2).
    #[inline(always)]
    pub fn gc5c3(&mut self) -> Gc5c3W<CCMR3rs> {
        Gc5c3W::new(self, 15)
    }
    ///Bit 16 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&mut self) -> Oc5ceW<CCMR3rs> {
        Oc5ceW::new(self, 16)
    }
    ///Bits 17:18
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CCMR3rs> {
        Rsvd2W::new(self, 17)
    }
    ///Bit 19 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&mut self) -> Oc5peW<CCMR3rs> {
        Oc5peW::new(self, 19)
    }
    ///Bits 20:23 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m(&mut self) -> Oc5mW<CCMR3rs> {
        Oc5mW::new(self, 20)
    }
    ///Bit 24 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&mut self) -> Oc6ceW<CCMR3rs> {
        Oc6ceW::new(self, 24)
    }
    ///Bits 25:26
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CCMR3rs> {
        RsvdW::new(self, 25)
    }
    ///Bit 27 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&mut self) -> Oc6peW<CCMR3rs> {
        Oc6peW::new(self, 27)
    }
    ///Bits 28:31 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m(&mut self) -> Oc6mW<CCMR3rs> {
        Oc6mW::new(self, 28)
    }
}
///TIM capture/compare mode register 3
///
///You can [`read`](crate::Reg::read) this register and get [`ccmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCMR3rs;
impl crate::RegisterSpec for CCMR3rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr3::R`](R) reader structure
impl crate::Readable for CCMR3rs {}
///`write(|w| ..)` method takes [`ccmr3::W`](W) writer structure
impl crate::Writable for CCMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR3 to value 0
impl crate::Resettable for CCMR3rs {
    const RESET_VALUE: u32 = 0;
}
