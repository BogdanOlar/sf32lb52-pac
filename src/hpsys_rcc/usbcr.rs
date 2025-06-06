///Register `USBCR` reader
pub type R = crate::R<USBCRrs>;
///Register `USBCR` writer
pub type W = crate::W<USBCRrs>;
///Field `DIV` reader - USB function clock is USB source clock divided by DIV. After divider, USB function clock must be 60MHz. For example, if USBC clock source is 240MHz clk_dll2, DIV should be 4.
pub type DivR = crate::FieldReader;
///Field `DIV` writer - USB function clock is USB source clock divided by DIV. After divider, USB function clock must be 60MHz. For example, if USBC clock source is 240MHz clk_dll2, DIV should be 4.
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - USB function clock is USB source clock divided by DIV. After divider, USB function clock must be 60MHz. For example, if USBC clock source is 240MHz clk_dll2, DIV should be 4.
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCR").field("div", &self.div()).finish()
    }
}
impl W {
    ///Bits 0:2 - USB function clock is USB source clock divided by DIV. After divider, USB function clock must be 60MHz. For example, if USBC clock source is 240MHz clk_dll2, DIV should be 4.
    #[inline(always)]
    pub fn div(&mut self) -> DivW<USBCRrs> {
        DivW::new(self, 0)
    }
}
///USBC Control Register
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
