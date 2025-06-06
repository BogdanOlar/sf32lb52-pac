///Register `CEATA` reader
pub type R = crate::R<CEATArs>;
///Register `CEATA` writer
pub type W = crate::W<CEATArs>;
///Field `ATA_MODE` reader - Select the card type, default is sd card 0: sd card mode 1: CE-ATA device mode
pub type AtaModeR = crate::BitReader;
///Field `ATA_MODE` writer - Select the card type, default is sd card 0: sd card mode 1: CE-ATA device mode
pub type AtaModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_SDIO_IRQ` reader - Select the sdio card mode, default is sd card 0: sd card mode , no sdio card interrupt 1: sdio card mode , enable sdio card interrupt
pub type EnableSdioIrqR = crate::BitReader;
///Field `ENABLE_SDIO_IRQ` writer - Select the sdio card mode, default is sd card 0: sd card mode , no sdio card interrupt 1: sdio card mode , enable sdio card interrupt
pub type EnableSdioIrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_4WIRES_IRQ` reader - Select the sdio host 4 wires interrupt support 0: host not support 4 wires interrupt on single-block data transfers 1: host support 4 wires interrupt on single-block data transfers
pub type Sdio4wiresIrqR = crate::BitReader;
///Field `SDIO_4WIRES_IRQ` writer - Select the sdio host 4 wires interrupt support 0: host not support 4 wires interrupt on single-block data transfers 1: host support 4 wires interrupt on single-block data transfers
pub type Sdio4wiresIrqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_4WIRES_MULTI_IRQ` reader - Select the sdio host 4 wires interrupt on multi-block support 0: host not support 4 wires interrupt on multi-block data transfers 1: host support 4 wires interrupt on multi-block data transfers
pub type Sdio4wiresMultiIrqR = crate::BitReader;
///Field `SDIO_4WIRES_MULTI_IRQ` writer - Select the sdio host 4 wires interrupt on multi-block support 0: host not support 4 wires interrupt on multi-block data transfers 1: host support 4 wires interrupt on multi-block data transfers
pub type Sdio4wiresMultiIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Select the card type, default is sd card 0: sd card mode 1: CE-ATA device mode
    #[inline(always)]
    pub fn ata_mode(&self) -> AtaModeR {
        AtaModeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Select the sdio card mode, default is sd card 0: sd card mode , no sdio card interrupt 1: sdio card mode , enable sdio card interrupt
    #[inline(always)]
    pub fn enable_sdio_irq(&self) -> EnableSdioIrqR {
        EnableSdioIrqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Select the sdio host 4 wires interrupt support 0: host not support 4 wires interrupt on single-block data transfers 1: host support 4 wires interrupt on single-block data transfers
    #[inline(always)]
    pub fn sdio_4wires_irq(&self) -> Sdio4wiresIrqR {
        Sdio4wiresIrqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Select the sdio host 4 wires interrupt on multi-block support 0: host not support 4 wires interrupt on multi-block data transfers 1: host support 4 wires interrupt on multi-block data transfers
    #[inline(always)]
    pub fn sdio_4wires_multi_irq(&self) -> Sdio4wiresMultiIrqR {
        Sdio4wiresMultiIrqR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEATA")
            .field("sdio_4wires_multi_irq", &self.sdio_4wires_multi_irq())
            .field("sdio_4wires_irq", &self.sdio_4wires_irq())
            .field("enable_sdio_irq", &self.enable_sdio_irq())
            .field("ata_mode", &self.ata_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Select the card type, default is sd card 0: sd card mode 1: CE-ATA device mode
    #[inline(always)]
    pub fn ata_mode(&mut self) -> AtaModeW<CEATArs> {
        AtaModeW::new(self, 0)
    }
    ///Bit 1 - Select the sdio card mode, default is sd card 0: sd card mode , no sdio card interrupt 1: sdio card mode , enable sdio card interrupt
    #[inline(always)]
    pub fn enable_sdio_irq(&mut self) -> EnableSdioIrqW<CEATArs> {
        EnableSdioIrqW::new(self, 1)
    }
    ///Bit 2 - Select the sdio host 4 wires interrupt support 0: host not support 4 wires interrupt on single-block data transfers 1: host support 4 wires interrupt on single-block data transfers
    #[inline(always)]
    pub fn sdio_4wires_irq(&mut self) -> Sdio4wiresIrqW<CEATArs> {
        Sdio4wiresIrqW::new(self, 2)
    }
    ///Bit 3 - Select the sdio host 4 wires interrupt on multi-block support 0: host not support 4 wires interrupt on multi-block data transfers 1: host support 4 wires interrupt on multi-block data transfers
    #[inline(always)]
    pub fn sdio_4wires_multi_irq(&mut self) -> Sdio4wiresMultiIrqW<CEATArs> {
        Sdio4wiresMultiIrqW::new(self, 3)
    }
}
///CE-ATA/SDIO mode register
///
///You can [`read`](crate::Reg::read) this register and get [`ceata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CEATArs;
impl crate::RegisterSpec for CEATArs {
    type Ux = u32;
}
///`read()` method returns [`ceata::R`](R) reader structure
impl crate::Readable for CEATArs {}
///`write(|w| ..)` method takes [`ceata::W`](W) writer structure
impl crate::Writable for CEATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CEATA to value 0
impl crate::Resettable for CEATArs {
    const RESET_VALUE: u32 = 0;
}
