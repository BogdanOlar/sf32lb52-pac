///Register `PWRKEY_CNT` reader
pub type R = crate::R<PWRKEY_CNTrs>;
///Register `PWRKEY_CNT` writer
pub type W = crate::W<PWRKEY_CNTrs>;
///Field `RST_CNT` reader - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
pub type RstCntR = crate::FieldReader<u16>;
///Field `RST_CNT` writer - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
pub type RstCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 4:19 - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
    #[inline(always)]
    pub fn rst_cnt(&self) -> RstCntR {
        RstCntR::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRKEY_CNT")
            .field("rst_cnt", &self.rst_cnt())
            .finish()
    }
}
impl W {
    ///Bits 4:19 - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
    #[inline(always)]
    pub fn rst_cnt(&mut self) -> RstCntW<PWRKEY_CNTrs> {
        RstCntW::new(self, 4)
    }
}
///PowerKey Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrkey_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrkey_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PWRKEY_CNTrs;
impl crate::RegisterSpec for PWRKEY_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`pwrkey_cnt::R`](R) reader structure
impl crate::Readable for PWRKEY_CNTrs {}
///`write(|w| ..)` method takes [`pwrkey_cnt::W`](W) writer structure
impl crate::Writable for PWRKEY_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWRKEY_CNT to value 0
impl crate::Resettable for PWRKEY_CNTrs {
    const RESET_VALUE: u32 = 0;
}
