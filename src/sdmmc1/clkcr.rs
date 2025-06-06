///Register `CLKCR` reader
pub type R = crate::R<CLKCRrs>;
///Register `CLKCR` writer
pub type W = crate::W<CLKCRrs>;
///Field `STOP_CLK` reader - Disable SD card clock 1: stop SD card clock 0: SD card clock generated
pub type StopClkR = crate::BitReader;
///Field `STOP_CLK` writer - Disable SD card clock 1: stop SD card clock 0: SD card clock generated
pub type StopClkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOID_FIFO_ERROR` reader - Void FIFO error 0: close the function 1: open the function If open it, when FIFO will be overrun or underrun soon, the SD_CLK and the clock enable of this module will be closed, and wait to host to read or write FIFO. Note: this function needs to be supported by card.
pub type VoidFifoErrorR = crate::BitReader;
///Field `VOID_FIFO_ERROR` writer - Void FIFO error 0: close the function 1: open the function If open it, when FIFO will be overrun or underrun soon, the SD_CLK and the clock enable of this module will be closed, and wait to host to read or write FIFO. Note: this function needs to be supported by card.
pub type VoidFifoErrorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_TUNE_SEL` reader - select clock delay for rx sample 0: no delay 1: delay level 1 (~1.5ns typical) 2: delay level 2 (~3ns typical) 3: delay level 3 (~5ns typical)
pub type ClkTuneSelR = crate::FieldReader;
///Field `CLK_TUNE_SEL` writer - select clock delay for rx sample 0: no delay 1: delay level 1 (~1.5ns typical) 2: delay level 2 (~3ns typical) 3: delay level 3 (~5ns typical)
pub type ClkTuneSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DIV` reader - Divide card clock counter. 0 is illegal. sd_clock = hclk/(div + 1) If hclk is 240M and div is 599, 400KHz SD clock will be generated.
pub type DivR = crate::FieldReader<u16>;
///Field `DIV` writer - Divide card clock counter. 0 is illegal. sd_clock = hclk/(div + 1) If hclk is 240M and div is 599, 400KHz SD clock will be generated.
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bit 0 - Disable SD card clock 1: stop SD card clock 0: SD card clock generated
    #[inline(always)]
    pub fn stop_clk(&self) -> StopClkR {
        StopClkR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Void FIFO error 0: close the function 1: open the function If open it, when FIFO will be overrun or underrun soon, the SD_CLK and the clock enable of this module will be closed, and wait to host to read or write FIFO. Note: this function needs to be supported by card.
    #[inline(always)]
    pub fn void_fifo_error(&self) -> VoidFifoErrorR {
        VoidFifoErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - select clock delay for rx sample 0: no delay 1: delay level 1 (~1.5ns typical) 2: delay level 2 (~3ns typical) 3: delay level 3 (~5ns typical)
    #[inline(always)]
    pub fn clk_tune_sel(&self) -> ClkTuneSelR {
        ClkTuneSelR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:20 - Divide card clock counter. 0 is illegal. sd_clock = hclk/(div + 1) If hclk is 240M and div is 599, 400KHz SD clock will be generated.
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 8) & 0x1fff) as u16)
    }
    ///Bits 21:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCR")
            .field("rsvd", &self.rsvd())
            .field("div", &self.div())
            .field("rsvd2", &self.rsvd2())
            .field("clk_tune_sel", &self.clk_tune_sel())
            .field("void_fifo_error", &self.void_fifo_error())
            .field("stop_clk", &self.stop_clk())
            .finish()
    }
}
impl W {
    ///Bit 0 - Disable SD card clock 1: stop SD card clock 0: SD card clock generated
    #[inline(always)]
    pub fn stop_clk(&mut self) -> StopClkW<CLKCRrs> {
        StopClkW::new(self, 0)
    }
    ///Bit 1 - Void FIFO error 0: close the function 1: open the function If open it, when FIFO will be overrun or underrun soon, the SD_CLK and the clock enable of this module will be closed, and wait to host to read or write FIFO. Note: this function needs to be supported by card.
    #[inline(always)]
    pub fn void_fifo_error(&mut self) -> VoidFifoErrorW<CLKCRrs> {
        VoidFifoErrorW::new(self, 1)
    }
    ///Bits 2:3 - select clock delay for rx sample 0: no delay 1: delay level 1 (~1.5ns typical) 2: delay level 2 (~3ns typical) 3: delay level 3 (~5ns typical)
    #[inline(always)]
    pub fn clk_tune_sel(&mut self) -> ClkTuneSelW<CLKCRrs> {
        ClkTuneSelW::new(self, 2)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CLKCRrs> {
        Rsvd2W::new(self, 4)
    }
    ///Bits 8:20 - Divide card clock counter. 0 is illegal. sd_clock = hclk/(div + 1) If hclk is 240M and div is 599, 400KHz SD clock will be generated.
    #[inline(always)]
    pub fn div(&mut self) -> DivW<CLKCRrs> {
        DivW::new(self, 8)
    }
    ///Bits 21:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CLKCRrs> {
        RsvdW::new(self, 21)
    }
}
///clock control register
///
///You can [`read`](crate::Reg::read) this register and get [`clkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CLKCRrs;
impl crate::RegisterSpec for CLKCRrs {
    type Ux = u32;
}
///`read()` method returns [`clkcr::R`](R) reader structure
impl crate::Readable for CLKCRrs {}
///`write(|w| ..)` method takes [`clkcr::W`](W) writer structure
impl crate::Writable for CLKCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLKCR to value 0
impl crate::Resettable for CLKCRrs {
    const RESET_VALUE: u32 = 0;
}
