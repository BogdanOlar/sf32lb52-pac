///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Register `STATUS` writer
pub type W = crate::W<STATUSrs>;
///Field `IA_BUSY` reader - Graphics accelerator busy flag
pub type IaBusyR = crate::BitReader;
///Field `IA_BUSY` writer - Graphics accelerator busy flag
pub type IaBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_BUSY` reader - LCD controll busy flag
pub type LcdBusyR = crate::BitReader;
///Field `LCD_BUSY` writer - LCD controll busy flag
pub type LcdBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Graphics accelerator busy flag
    #[inline(always)]
    pub fn ia_busy(&self) -> IaBusyR {
        IaBusyR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - LCD controll busy flag
    #[inline(always)]
    pub fn lcd_busy(&self) -> LcdBusyR {
        LcdBusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("lcd_busy", &self.lcd_busy())
            .field("ia_busy", &self.ia_busy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Graphics accelerator busy flag
    #[inline(always)]
    pub fn ia_busy(&mut self) -> IaBusyW<STATUSrs> {
        IaBusyW::new(self, 0)
    }
    ///Bit 4 - LCD controll busy flag
    #[inline(always)]
    pub fn lcd_busy(&mut self) -> LcdBusyW<STATUSrs> {
        LcdBusyW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
