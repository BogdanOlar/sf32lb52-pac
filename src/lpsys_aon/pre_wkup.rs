///Register `PRE_WKUP` reader
pub type R = crate::R<PRE_WKUPrs>;
///Register `PRE_WKUP` writer
pub type W = crate::W<PRE_WKUPrs>;
///Field `XTAL_TIME` reader - cycles of clk_rtc for hxt48 ready before bt awake.
pub type XtalTimeR = crate::FieldReader<u16>;
///Field `XTAL_TIME` writer - cycles of clk_rtc for hxt48 ready before bt awake.
pub type XtalTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `WKUP_TIME` reader - cycles of clk_rtc for LPSYS ready before bt awake.
pub type WkupTimeR = crate::FieldReader<u16>;
///Field `WKUP_TIME` writer - cycles of clk_rtc for LPSYS ready before bt awake.
pub type WkupTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - cycles of clk_rtc for hxt48 ready before bt awake.
    #[inline(always)]
    pub fn xtal_time(&self) -> XtalTimeR {
        XtalTimeR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - cycles of clk_rtc for LPSYS ready before bt awake.
    #[inline(always)]
    pub fn wkup_time(&self) -> WkupTimeR {
        WkupTimeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRE_WKUP")
            .field("wkup_time", &self.wkup_time())
            .field("xtal_time", &self.xtal_time())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - cycles of clk_rtc for hxt48 ready before bt awake.
    #[inline(always)]
    pub fn xtal_time(&mut self) -> XtalTimeW<PRE_WKUPrs> {
        XtalTimeW::new(self, 0)
    }
    ///Bits 16:25 - cycles of clk_rtc for LPSYS ready before bt awake.
    #[inline(always)]
    pub fn wkup_time(&mut self) -> WkupTimeW<PRE_WKUPrs> {
        WkupTimeW::new(self, 16)
    }
}
///time before bt awake
///
///You can [`read`](crate::Reg::read) this register and get [`pre_wkup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_wkup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PRE_WKUPrs;
impl crate::RegisterSpec for PRE_WKUPrs {
    type Ux = u32;
}
///`read()` method returns [`pre_wkup::R`](R) reader structure
impl crate::Readable for PRE_WKUPrs {}
///`write(|w| ..)` method takes [`pre_wkup::W`](W) writer structure
impl crate::Writable for PRE_WKUPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRE_WKUP to value 0
impl crate::Resettable for PRE_WKUPrs {
    const RESET_VALUE: u32 = 0;
}
