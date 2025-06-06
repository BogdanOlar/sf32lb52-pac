///Register `PRE_WKUP` reader
pub type R = crate::R<PRE_WKUPrs>;
///Register `PRE_WKUP` writer
pub type W = crate::W<PRE_WKUPrs>;
///Field `XTAL_TIME` reader - cycles of clk_rtc for hxt48 ready before bt awake.
pub type XtalTimeR = crate::FieldReader<u16>;
///Field `XTAL_TIME` writer - cycles of clk_rtc for hxt48 ready before bt awake.
pub type XtalTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `WKUP_TIME` reader - cycles of clk_rtc for LPSYS ready before bt awake.
pub type WkupTimeR = crate::FieldReader<u16>;
///Field `WKUP_TIME` writer - cycles of clk_rtc for LPSYS ready before bt awake.
pub type WkupTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:9 - cycles of clk_rtc for hxt48 ready before bt awake.
    #[inline(always)]
    pub fn xtal_time(&self) -> XtalTimeR {
        XtalTimeR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 16:25 - cycles of clk_rtc for LPSYS ready before bt awake.
    #[inline(always)]
    pub fn wkup_time(&self) -> WkupTimeR {
        WkupTimeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRE_WKUP")
            .field("rsvd", &self.rsvd())
            .field("wkup_time", &self.wkup_time())
            .field("rsvd2", &self.rsvd2())
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
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<PRE_WKUPrs> {
        Rsvd2W::new(self, 10)
    }
    ///Bits 16:25 - cycles of clk_rtc for LPSYS ready before bt awake.
    #[inline(always)]
    pub fn wkup_time(&mut self) -> WkupTimeW<PRE_WKUPrs> {
        WkupTimeW::new(self, 16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PRE_WKUPrs> {
        RsvdW::new(self, 26)
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
