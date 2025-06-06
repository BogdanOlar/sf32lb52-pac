///Register `JDI_PAR_EX_CTRL` reader
pub type R = crate::R<JDI_PAR_EX_CTRLrs>;
///Register `JDI_PAR_EX_CTRL` writer
pub type W = crate::W<JDI_PAR_EX_CTRLrs>;
///Field `MAX_CNT` reader - VCOM/FRP/XFRP max counter
pub type MaxCntR = crate::FieldReader<u32>;
///Field `MAX_CNT` writer - VCOM/FRP/XFRP max counter
pub type MaxCntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `CNT_EN` reader - VCOM/FRP/XFRP counter enable
pub type CntEnR = crate::BitReader;
///Field `CNT_EN` writer - VCOM/FRP/XFRP counter enable
pub type CntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XFRP` reader - XFRP value
pub type XfrpR = crate::BitReader;
///Field `XFRP` writer - XFRP value
pub type XfrpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRP` reader - FRP value
pub type FrpR = crate::BitReader;
///Field `FRP` writer - FRP value
pub type FrpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCOM` reader - VCOM value
pub type VcomR = crate::BitReader;
///Field `VCOM` writer - VCOM value
pub type VcomW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:23 - VCOM/FRP/XFRP max counter
    #[inline(always)]
    pub fn max_cnt(&self) -> MaxCntR {
        MaxCntR::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 28 - VCOM/FRP/XFRP counter enable
    #[inline(always)]
    pub fn cnt_en(&self) -> CntEnR {
        CntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - XFRP value
    #[inline(always)]
    pub fn xfrp(&self) -> XfrpR {
        XfrpR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - FRP value
    #[inline(always)]
    pub fn frp(&self) -> FrpR {
        FrpR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - VCOM value
    #[inline(always)]
    pub fn vcom(&self) -> VcomR {
        VcomR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_EX_CTRL")
            .field("vcom", &self.vcom())
            .field("frp", &self.frp())
            .field("xfrp", &self.xfrp())
            .field("cnt_en", &self.cnt_en())
            .field("max_cnt", &self.max_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - VCOM/FRP/XFRP max counter
    #[inline(always)]
    pub fn max_cnt(&mut self) -> MaxCntW<JDI_PAR_EX_CTRLrs> {
        MaxCntW::new(self, 0)
    }
    ///Bit 28 - VCOM/FRP/XFRP counter enable
    #[inline(always)]
    pub fn cnt_en(&mut self) -> CntEnW<JDI_PAR_EX_CTRLrs> {
        CntEnW::new(self, 28)
    }
    ///Bit 29 - XFRP value
    #[inline(always)]
    pub fn xfrp(&mut self) -> XfrpW<JDI_PAR_EX_CTRLrs> {
        XfrpW::new(self, 29)
    }
    ///Bit 30 - FRP value
    #[inline(always)]
    pub fn frp(&mut self) -> FrpW<JDI_PAR_EX_CTRLrs> {
        FrpW::new(self, 30)
    }
    ///Bit 31 - VCOM value
    #[inline(always)]
    pub fn vcom(&mut self) -> VcomW<JDI_PAR_EX_CTRLrs> {
        VcomW::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_ex_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_ex_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_EX_CTRLrs;
impl crate::RegisterSpec for JDI_PAR_EX_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_ex_ctrl::R`](R) reader structure
impl crate::Readable for JDI_PAR_EX_CTRLrs {}
///`write(|w| ..)` method takes [`jdi_par_ex_ctrl::W`](W) writer structure
impl crate::Writable for JDI_PAR_EX_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_EX_CTRL to value 0
impl crate::Resettable for JDI_PAR_EX_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
