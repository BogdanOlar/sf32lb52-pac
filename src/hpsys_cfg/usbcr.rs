///Register `USBCR` reader
pub type R = crate::R<USBCRrs>;
///Register `USBCR` writer
pub type W = crate::W<USBCRrs>;
///Field `USB_EN` reader - USB PHY enable, turn on power swith, power up LDO and bias
pub type UsbEnR = crate::BitReader;
///Field `USB_EN` writer - USB PHY enable, turn on power swith, power up LDO and bias
pub type UsbEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO_VSEL` reader - 2.5V LDO output voltage setting 0 = 2.40 V, 1 = 2.47 V, 2 = 2.53 V, 3 = 2.60 V, 4 = 2.60 V, 5 = 2.67 V, 6 = 2.73 V, 7 = 2.8 V
pub type LdoVselR = crate::FieldReader;
///Field `LDO_VSEL` writer - 2.5V LDO output voltage setting 0 = 2.40 V, 1 = 2.47 V, 2 = 2.53 V, 3 = 2.60 V, 4 = 2.60 V, 5 = 2.67 V, 6 = 2.73 V, 7 = 2.8 V
pub type LdoVselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LDO_LP_EN` reader - 2.5V LDO low power mode enable. 0 = 240 uA, 1 = 50 uA
pub type LdoLpEnR = crate::BitReader;
///Field `LDO_LP_EN` writer - 2.5V LDO low power mode enable. 0 = 240 uA, 1 = 50 uA
pub type LdoLpEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DM_PD` reader - enable DM 15k Ohm pull down resistor
pub type DmPdR = crate::BitReader;
///Field `DM_PD` writer - enable DM 15k Ohm pull down resistor
pub type DmPdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DP_EN` reader - 0:disable dp pull up or pull down 1:enable dp pull or pull down
pub type DpEnR = crate::BitReader;
///Field `DP_EN` writer - 0:disable dp pull up or pull down 1:enable dp pull or pull down
pub type DpEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_RTUNE` reader - TX outp impedance tuning 0 = 50 Ohm, 1 = 46 Ohm, 2 = 43 Ohm, 3 = 40 Ohm, 4 = 37.5 Ohm, 5 = 35 Ohm, 6 = 33 Ohm, 7 = 31.5 Ohm
pub type TxRtuneR = crate::FieldReader;
///Field `TX_RTUNE` writer - TX outp impedance tuning 0 = 50 Ohm, 1 = 46 Ohm, 2 = 43 Ohm, 3 = 40 Ohm, 4 = 37.5 Ohm, 5 = 35 Ohm, 6 = 33 Ohm, 7 = 31.5 Ohm
pub type TxRtuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DC_TE` reader - reserved for debug
pub type DcTeR = crate::BitReader;
///Field `DC_TE` writer - reserved for debug
pub type DcTeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_TR` reader - reserved for debug
pub type DcTrR = crate::FieldReader;
///Field `DC_TR` writer - reserved for debug
pub type DcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - USB PHY enable, turn on power swith, power up LDO and bias
    #[inline(always)]
    pub fn usb_en(&self) -> UsbEnR {
        UsbEnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - 2.5V LDO output voltage setting 0 = 2.40 V, 1 = 2.47 V, 2 = 2.53 V, 3 = 2.60 V, 4 = 2.60 V, 5 = 2.67 V, 6 = 2.73 V, 7 = 2.8 V
    #[inline(always)]
    pub fn ldo_vsel(&self) -> LdoVselR {
        LdoVselR::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - 2.5V LDO low power mode enable. 0 = 240 uA, 1 = 50 uA
    #[inline(always)]
    pub fn ldo_lp_en(&self) -> LdoLpEnR {
        LdoLpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable DM 15k Ohm pull down resistor
    #[inline(always)]
    pub fn dm_pd(&self) -> DmPdR {
        DmPdR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 0:disable dp pull up or pull down 1:enable dp pull or pull down
    #[inline(always)]
    pub fn dp_en(&self) -> DpEnR {
        DpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:10 - TX outp impedance tuning 0 = 50 Ohm, 1 = 46 Ohm, 2 = 43 Ohm, 3 = 40 Ohm, 4 = 37.5 Ohm, 5 = 35 Ohm, 6 = 33 Ohm, 7 = 31.5 Ohm
    #[inline(always)]
    pub fn tx_rtune(&self) -> TxRtuneR {
        TxRtuneR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - reserved for debug
    #[inline(always)]
    pub fn dc_te(&self) -> DcTeR {
        DcTeR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - reserved for debug
    #[inline(always)]
    pub fn dc_tr(&self) -> DcTrR {
        DcTrR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCR")
            .field("dc_tr", &self.dc_tr())
            .field("dc_te", &self.dc_te())
            .field("tx_rtune", &self.tx_rtune())
            .field("dp_en", &self.dp_en())
            .field("dm_pd", &self.dm_pd())
            .field("ldo_lp_en", &self.ldo_lp_en())
            .field("ldo_vsel", &self.ldo_vsel())
            .field("usb_en", &self.usb_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB PHY enable, turn on power swith, power up LDO and bias
    #[inline(always)]
    pub fn usb_en(&mut self) -> UsbEnW<USBCRrs> {
        UsbEnW::new(self, 0)
    }
    ///Bits 1:3 - 2.5V LDO output voltage setting 0 = 2.40 V, 1 = 2.47 V, 2 = 2.53 V, 3 = 2.60 V, 4 = 2.60 V, 5 = 2.67 V, 6 = 2.73 V, 7 = 2.8 V
    #[inline(always)]
    pub fn ldo_vsel(&mut self) -> LdoVselW<USBCRrs> {
        LdoVselW::new(self, 1)
    }
    ///Bit 4 - 2.5V LDO low power mode enable. 0 = 240 uA, 1 = 50 uA
    #[inline(always)]
    pub fn ldo_lp_en(&mut self) -> LdoLpEnW<USBCRrs> {
        LdoLpEnW::new(self, 4)
    }
    ///Bit 5 - enable DM 15k Ohm pull down resistor
    #[inline(always)]
    pub fn dm_pd(&mut self) -> DmPdW<USBCRrs> {
        DmPdW::new(self, 5)
    }
    ///Bit 6 - 0:disable dp pull up or pull down 1:enable dp pull or pull down
    #[inline(always)]
    pub fn dp_en(&mut self) -> DpEnW<USBCRrs> {
        DpEnW::new(self, 6)
    }
    ///Bits 8:10 - TX outp impedance tuning 0 = 50 Ohm, 1 = 46 Ohm, 2 = 43 Ohm, 3 = 40 Ohm, 4 = 37.5 Ohm, 5 = 35 Ohm, 6 = 33 Ohm, 7 = 31.5 Ohm
    #[inline(always)]
    pub fn tx_rtune(&mut self) -> TxRtuneW<USBCRrs> {
        TxRtuneW::new(self, 8)
    }
    ///Bit 12 - reserved for debug
    #[inline(always)]
    pub fn dc_te(&mut self) -> DcTeW<USBCRrs> {
        DcTeW::new(self, 12)
    }
    ///Bits 13:15 - reserved for debug
    #[inline(always)]
    pub fn dc_tr(&mut self) -> DcTrW<USBCRrs> {
        DcTrW::new(self, 13)
    }
}
///USB Control register
///
///You can [`read`](crate::Reg::read) this register and get [`usbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct USBCRrs;
impl crate::RegisterSpec for USBCRrs {
    type Ux = u32;
}
///`read()` method returns [`usbcr::R`](R) reader structure
impl crate::Readable for USBCRrs {}
///`write(|w| ..)` method takes [`usbcr::W`](W) writer structure
impl crate::Writable for USBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USBCR to value 0
impl crate::Resettable for USBCRrs {
    const RESET_VALUE: u32 = 0;
}
