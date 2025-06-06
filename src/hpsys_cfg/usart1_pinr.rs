///Register `USART1_PINR` reader
pub type R = crate::R<USART1_PINRrs>;
///Register `USART1_PINR` writer
pub type W = crate::W<USART1_PINRrs>;
///Field `TXD_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type TxdPinR = crate::FieldReader;
///Field `TXD_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type TxdPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::FieldReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXD_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type RxdPinR = crate::FieldReader;
///Field `RXD_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type RxdPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTS_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type RtsPinR = crate::FieldReader;
///Field `RTS_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type RtsPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CTS_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type CtsPinR = crate::FieldReader;
///Field `CTS_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type CtsPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn txd_pin(&self) -> TxdPinR {
        TxdPinR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn rxd_pin(&self) -> RxdPinR {
        RxdPinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn rts_pin(&self) -> RtsPinR {
        RtsPinR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 22:23
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:29 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn cts_pin(&self) -> CtsPinR {
        CtsPinR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1_PINR")
            .field("rsvd", &self.rsvd())
            .field("cts_pin", &self.cts_pin())
            .field("rsvd2", &self.rsvd2())
            .field("rts_pin", &self.rts_pin())
            .field("rsvd3", &self.rsvd3())
            .field("rxd_pin", &self.rxd_pin())
            .field("rsvd4", &self.rsvd4())
            .field("txd_pin", &self.txd_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn txd_pin(&mut self) -> TxdPinW<USART1_PINRrs> {
        TxdPinW::new(self, 0)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<USART1_PINRrs> {
        Rsvd4W::new(self, 6)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn rxd_pin(&mut self) -> RxdPinW<USART1_PINRrs> {
        RxdPinW::new(self, 8)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<USART1_PINRrs> {
        Rsvd3W::new(self, 14)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn rts_pin(&mut self) -> RtsPinW<USART1_PINRrs> {
        RtsPinW::new(self, 16)
    }
    ///Bits 22:23
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<USART1_PINRrs> {
        Rsvd2W::new(self, 22)
    }
    ///Bits 24:29 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn cts_pin(&mut self) -> CtsPinW<USART1_PINRrs> {
        CtsPinW::new(self, 24)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<USART1_PINRrs> {
        RsvdW::new(self, 30)
    }
}
///USART1 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`usart1_pinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_pinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct USART1_PINRrs;
impl crate::RegisterSpec for USART1_PINRrs {
    type Ux = u32;
}
///`read()` method returns [`usart1_pinr::R`](R) reader structure
impl crate::Readable for USART1_PINRrs {}
///`write(|w| ..)` method takes [`usart1_pinr::W`](W) writer structure
impl crate::Writable for USART1_PINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USART1_PINR to value 0
impl crate::Resettable for USART1_PINRrs {
    const RESET_VALUE: u32 = 0;
}
