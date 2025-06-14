///Register `USART4_PINR` reader
pub type R = crate::R<USART4_PINRrs>;
///Register `USART4_PINR` writer
pub type W = crate::W<USART4_PINRrs>;
///Field `TXD_PIN` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type TxdPinR = crate::FieldReader;
///Field `TXD_PIN` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type TxdPinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXD_PIN` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type RxdPinR = crate::FieldReader;
///Field `RXD_PIN` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type RxdPinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RTS_PIN` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type RtsPinR = crate::FieldReader;
///Field `RTS_PIN` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type RtsPinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CTS_PIN` reader - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type CtsPinR = crate::FieldReader;
///Field `CTS_PIN` writer - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
pub type CtsPinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn txd_pin(&self) -> TxdPinR {
        TxdPinR::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn rxd_pin(&self) -> RxdPinR {
        RxdPinR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:18 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn rts_pin(&self) -> RtsPinR {
        RtsPinR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn cts_pin(&self) -> CtsPinR {
        CtsPinR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART4_PINR")
            .field("cts_pin", &self.cts_pin())
            .field("rts_pin", &self.rts_pin())
            .field("rxd_pin", &self.rxd_pin())
            .field("txd_pin", &self.txd_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn txd_pin(&mut self) -> TxdPinW<USART4_PINRrs> {
        TxdPinW::new(self, 0)
    }
    ///Bits 8:10 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn rxd_pin(&mut self) -> RxdPinW<USART4_PINRrs> {
        RxdPinW::new(self, 8)
    }
    ///Bits 16:18 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn rts_pin(&mut self) -> RtsPinW<USART4_PINRrs> {
        RtsPinW::new(self, 16)
    }
    ///Bits 24:26 - Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating.
    #[inline(always)]
    pub fn cts_pin(&mut self) -> CtsPinW<USART4_PINRrs> {
        CtsPinW::new(self, 24)
    }
}
///USART4 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart4_pinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart4_pinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct USART4_PINRrs;
impl crate::RegisterSpec for USART4_PINRrs {
    type Ux = u32;
}
///`read()` method returns [`usart4_pinr::R`](R) reader structure
impl crate::Readable for USART4_PINRrs {}
///`write(|w| ..)` method takes [`usart4_pinr::W`](W) writer structure
impl crate::Writable for USART4_PINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USART4_PINR to value 0
impl crate::Resettable for USART4_PINRrs {
    const RESET_VALUE: u32 = 0;
}
