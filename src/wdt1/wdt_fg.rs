///Register `WDT_FG` reader
pub type R = crate::R<WDT_FGrs>;
///Register `WDT_FG` writer
pub type W = crate::W<WDT_FGrs>;
///Field `RST_FG_CLR` reader - SinglePulse/A pulse to clear reset flag
pub type RstFgClrR = crate::BitReader;
///Field `RST_FG_CLR` writer - SinglePulse/A pulse to clear reset flag
pub type RstFgClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_FG` reader - 1 indicates wdt has already reset system
pub type RstFgR = crate::BitReader;
///Field `RST_FG` writer - 1 indicates wdt has already reset system
pub type RstFgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_FG_CLR` reader - SinglePulse/A pulse to clear sync flag
pub type SyncFgClrR = crate::BitReader;
///Field `SYNC_FG_CLR` writer - SinglePulse/A pulse to clear sync flag
pub type SyncFgClrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_FG` reader - 1 indicates one transition from system clk to wdt clk has complicated
pub type SyncFgR = crate::BitReader;
///Field `SYNC_FG` writer - 1 indicates one transition from system clk to wdt clk has complicated
pub type SyncFgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SinglePulse/A pulse to clear reset flag
    #[inline(always)]
    pub fn rst_fg_clr(&self) -> RstFgClrR {
        RstFgClrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1 indicates wdt has already reset system
    #[inline(always)]
    pub fn rst_fg(&self) -> RstFgR {
        RstFgR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SinglePulse/A pulse to clear sync flag
    #[inline(always)]
    pub fn sync_fg_clr(&self) -> SyncFgClrR {
        SyncFgClrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1 indicates one transition from system clk to wdt clk has complicated
    #[inline(always)]
    pub fn sync_fg(&self) -> SyncFgR {
        SyncFgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_FG")
            .field("sync_fg", &self.sync_fg())
            .field("sync_fg_clr", &self.sync_fg_clr())
            .field("rst_fg", &self.rst_fg())
            .field("rst_fg_clr", &self.rst_fg_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - SinglePulse/A pulse to clear reset flag
    #[inline(always)]
    pub fn rst_fg_clr(&mut self) -> RstFgClrW<WDT_FGrs> {
        RstFgClrW::new(self, 0)
    }
    ///Bit 1 - 1 indicates wdt has already reset system
    #[inline(always)]
    pub fn rst_fg(&mut self) -> RstFgW<WDT_FGrs> {
        RstFgW::new(self, 1)
    }
    ///Bit 2 - SinglePulse/A pulse to clear sync flag
    #[inline(always)]
    pub fn sync_fg_clr(&mut self) -> SyncFgClrW<WDT_FGrs> {
        SyncFgClrW::new(self, 2)
    }
    ///Bit 3 - 1 indicates one transition from system clk to wdt clk has complicated
    #[inline(always)]
    pub fn sync_fg(&mut self) -> SyncFgW<WDT_FGrs> {
        SyncFgW::new(self, 3)
    }
}
///WatchDog Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_fg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_fg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_FGrs;
impl crate::RegisterSpec for WDT_FGrs {
    type Ux = u32;
}
///`read()` method returns [`wdt_fg::R`](R) reader structure
impl crate::Readable for WDT_FGrs {}
///`write(|w| ..)` method takes [`wdt_fg::W`](W) writer structure
impl crate::Writable for WDT_FGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_FG to value 0
impl crate::Resettable for WDT_FGrs {
    const RESET_VALUE: u32 = 0;
}
