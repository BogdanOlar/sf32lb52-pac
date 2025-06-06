///Register `TE_CONF` reader
pub type R = crate::R<TE_CONFrs>;
///Register `TE_CONF` writer
pub type W = crate::W<TE_CONFrs>;
///Field `ENABLE` reader - TE enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - TE enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMARK_POL` reader - TE signal polarity
pub type FmarkPolR = crate::BitReader;
///Field `FMARK_POL` writer - TE signal polarity
pub type FmarkPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - 0: vsync only TE mode 1: vsync+hsync TE mode
pub type ModeR = crate::BitReader;
///Field `MODE` writer - 0: vsync only TE mode 1: vsync+hsync TE mode
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSYNC_DET_CNT` reader - vsync signal detect counter, used for mode 1 to detect vsync signal
pub type VsyncDetCntR = crate::FieldReader<u16>;
///Field `VSYNC_DET_CNT` writer - vsync signal detect counter, used for mode 1 to detect vsync signal
pub type VsyncDetCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `FMARK_MODE` reader - TE signal trigger mode 1: edge trigger 0: pulse trigger
pub type FmarkModeR = crate::BitReader;
///Field `FMARK_MODE` writer - TE signal trigger mode 1: edge trigger 0: pulse trigger
pub type FmarkModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMARK_SOURCE` reader - TE signal source 1: use TE signal from DSI 0: use TE signal from external pin
pub type FmarkSourceR = crate::BitReader;
///Field `FMARK_SOURCE` writer - TE signal source 1: use TE signal from DSI 0: use TE signal from external pin
pub type FmarkSourceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TE enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TE signal polarity
    #[inline(always)]
    pub fn fmark_pol(&self) -> FmarkPolR {
        FmarkPolR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 0: vsync only TE mode 1: vsync+hsync TE mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:18 - vsync signal detect counter, used for mode 1 to detect vsync signal
    #[inline(always)]
    pub fn vsync_det_cnt(&self) -> VsyncDetCntR {
        VsyncDetCntR::new(((self.bits >> 3) & 0xffff) as u16)
    }
    ///Bit 19 - TE signal trigger mode 1: edge trigger 0: pulse trigger
    #[inline(always)]
    pub fn fmark_mode(&self) -> FmarkModeR {
        FmarkModeR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TE signal source 1: use TE signal from DSI 0: use TE signal from external pin
    #[inline(always)]
    pub fn fmark_source(&self) -> FmarkSourceR {
        FmarkSourceR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TE_CONF")
            .field("fmark_source", &self.fmark_source())
            .field("fmark_mode", &self.fmark_mode())
            .field("vsync_det_cnt", &self.vsync_det_cnt())
            .field("mode", &self.mode())
            .field("fmark_pol", &self.fmark_pol())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - TE enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<TE_CONFrs> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - TE signal polarity
    #[inline(always)]
    pub fn fmark_pol(&mut self) -> FmarkPolW<TE_CONFrs> {
        FmarkPolW::new(self, 1)
    }
    ///Bit 2 - 0: vsync only TE mode 1: vsync+hsync TE mode
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<TE_CONFrs> {
        ModeW::new(self, 2)
    }
    ///Bits 3:18 - vsync signal detect counter, used for mode 1 to detect vsync signal
    #[inline(always)]
    pub fn vsync_det_cnt(&mut self) -> VsyncDetCntW<TE_CONFrs> {
        VsyncDetCntW::new(self, 3)
    }
    ///Bit 19 - TE signal trigger mode 1: edge trigger 0: pulse trigger
    #[inline(always)]
    pub fn fmark_mode(&mut self) -> FmarkModeW<TE_CONFrs> {
        FmarkModeW::new(self, 19)
    }
    ///Bit 20 - TE signal source 1: use TE signal from DSI 0: use TE signal from external pin
    #[inline(always)]
    pub fn fmark_source(&mut self) -> FmarkSourceW<TE_CONFrs> {
        FmarkSourceW::new(self, 20)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`te_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`te_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TE_CONFrs;
impl crate::RegisterSpec for TE_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`te_conf::R`](R) reader structure
impl crate::Readable for TE_CONFrs {}
///`write(|w| ..)` method takes [`te_conf::W`](W) writer structure
impl crate::Writable for TE_CONFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TE_CONF to value 0
impl crate::Resettable for TE_CONFrs {
    const RESET_VALUE: u32 = 0;
}
