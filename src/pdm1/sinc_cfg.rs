///Register `SINC_CFG` reader
pub type R = crate::R<SINC_CFGrs>;
///Register `SINC_CFG` writer
pub type W = crate::W<SINC_CFGrs>;
///Field `SINC_RATE` reader - dowmsampling rate of sinc filter
pub type SincRateR = crate::FieldReader;
///Field `SINC_RATE` writer - dowmsampling rate of sinc filter
pub type SincRateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SINC_ORDER_SEL` reader - 1:select four differentiators in sinc filter; 0:select three differentiators in sinc filter
pub type SincOrderSelR = crate::BitReader;
///Field `SINC_ORDER_SEL` writer - 1:select four differentiators in sinc filter; 0:select three differentiators in sinc filter
pub type SincOrderSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - dowmsampling rate of sinc filter
    #[inline(always)]
    pub fn sinc_rate(&self) -> SincRateR {
        SincRateR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - 1:select four differentiators in sinc filter; 0:select three differentiators in sinc filter
    #[inline(always)]
    pub fn sinc_order_sel(&self) -> SincOrderSelR {
        SincOrderSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINC_CFG")
            .field("sinc_order_sel", &self.sinc_order_sel())
            .field("sinc_rate", &self.sinc_rate())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - dowmsampling rate of sinc filter
    #[inline(always)]
    pub fn sinc_rate(&mut self) -> SincRateW<SINC_CFGrs> {
        SincRateW::new(self, 0)
    }
    ///Bit 8 - 1:select four differentiators in sinc filter; 0:select three differentiators in sinc filter
    #[inline(always)]
    pub fn sinc_order_sel(&mut self) -> SincOrderSelW<SINC_CFGrs> {
        SincOrderSelW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`sinc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sinc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SINC_CFGrs;
impl crate::RegisterSpec for SINC_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`sinc_cfg::R`](R) reader structure
impl crate::Readable for SINC_CFGrs {}
///`write(|w| ..)` method takes [`sinc_cfg::W`](W) writer structure
impl crate::Writable for SINC_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SINC_CFG to value 0
impl crate::Resettable for SINC_CFGrs {
    const RESET_VALUE: u32 = 0;
}
