///Register `PTA_PINR` reader
pub type R = crate::R<PTA_PINRrs>;
///Register `PTA_PINR` writer
pub type W = crate::W<PTA_PINRrs>;
///Field `BT_ACTIVE` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type BtActiveR = crate::FieldReader;
///Field `BT_ACTIVE` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type BtActiveW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BT_COLLISION` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type BtCollisionR = crate::FieldReader;
///Field `BT_COLLISION` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type BtCollisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BT_PRIORITY` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type BtPriorityR = crate::FieldReader;
///Field `BT_PRIORITY` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type BtPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WLAN_ACTIVE` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type WlanActiveR = crate::FieldReader;
///Field `WLAN_ACTIVE` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type WlanActiveW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn bt_active(&self) -> BtActiveR {
        BtActiveR::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn bt_collision(&self) -> BtCollisionR {
        BtCollisionR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:18 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn bt_priority(&self) -> BtPriorityR {
        BtPriorityR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn wlan_active(&self) -> WlanActiveR {
        WlanActiveR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTA_PINR")
            .field("wlan_active", &self.wlan_active())
            .field("bt_priority", &self.bt_priority())
            .field("bt_collision", &self.bt_collision())
            .field("bt_active", &self.bt_active())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn bt_active(&mut self) -> BtActiveW<PTA_PINRrs> {
        BtActiveW::new(self, 0)
    }
    ///Bits 8:10 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn bt_collision(&mut self) -> BtCollisionW<PTA_PINRrs> {
        BtCollisionW::new(self, 8)
    }
    ///Bits 16:18 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn bt_priority(&mut self) -> BtPriorityW<PTA_PINRrs> {
        BtPriorityW::new(self, 16)
    }
    ///Bits 24:26 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn wlan_active(&mut self) -> WlanActiveW<PTA_PINRrs> {
        WlanActiveW::new(self, 24)
    }
}
///PTA Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`pta_pinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pta_pinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PTA_PINRrs;
impl crate::RegisterSpec for PTA_PINRrs {
    type Ux = u32;
}
///`read()` method returns [`pta_pinr::R`](R) reader structure
impl crate::Readable for PTA_PINRrs {}
///`write(|w| ..)` method takes [`pta_pinr::W`](W) writer structure
impl crate::Writable for PTA_PINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PTA_PINR to value 0
impl crate::Resettable for PTA_PINRrs {
    const RESET_VALUE: u32 = 0;
}
