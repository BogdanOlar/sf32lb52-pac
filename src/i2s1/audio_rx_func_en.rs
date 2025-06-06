///Register `AUDIO_RX_FUNC_EN` reader
pub type R = crate::R<AUDIO_RX_FUNC_ENrs>;
///Register `AUDIO_RX_FUNC_EN` writer
pub type W = crate::W<AUDIO_RX_FUNC_ENrs>;
///Field `RX_EN` reader - 1: enable 0: disable
pub type RxEnR = crate::BitReader;
///Field `RX_EN` writer - 1: enable 0: disable
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_INTF_SEL` reader - 1: select external rx interface 0: select internal apb rx interface
pub type RxIntfSelR = crate::BitReader;
///Field `RX_INTF_SEL` writer - 1: select external rx interface 0: select internal apb rx interface
pub type RxIntfSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1: enable 0: disable
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: select external rx interface 0: select internal apb rx interface
    #[inline(always)]
    pub fn rx_intf_sel(&self) -> RxIntfSelR {
        RxIntfSelR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIO_RX_FUNC_EN")
            .field("rx_intf_sel", &self.rx_intf_sel())
            .field("rx_en", &self.rx_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1: enable 0: disable
    #[inline(always)]
    pub fn rx_en(&mut self) -> RxEnW<AUDIO_RX_FUNC_ENrs> {
        RxEnW::new(self, 0)
    }
    ///Bit 1 - 1: select external rx interface 0: select internal apb rx interface
    #[inline(always)]
    pub fn rx_intf_sel(&mut self) -> RxIntfSelW<AUDIO_RX_FUNC_ENrs> {
        RxIntfSelW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`audio_rx_func_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_rx_func_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AUDIO_RX_FUNC_ENrs;
impl crate::RegisterSpec for AUDIO_RX_FUNC_ENrs {
    type Ux = u32;
}
///`read()` method returns [`audio_rx_func_en::R`](R) reader structure
impl crate::Readable for AUDIO_RX_FUNC_ENrs {}
///`write(|w| ..)` method takes [`audio_rx_func_en::W`](W) writer structure
impl crate::Writable for AUDIO_RX_FUNC_ENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUDIO_RX_FUNC_EN to value 0
impl crate::Resettable for AUDIO_RX_FUNC_ENrs {
    const RESET_VALUE: u32 = 0;
}
