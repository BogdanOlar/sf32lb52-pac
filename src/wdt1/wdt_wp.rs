///Register `WDT_WP` reader
pub type R = crate::R<WDT_WPrs>;
///Register `WDT_WP` writer
pub type W = crate::W<WDT_WPrs>;
///Field `WRPT` reader - write 0x58ab99fc generate write_protect, write 0x51ff8621 to release
pub type WrptR = crate::FieldReader<u32>;
///Field `WRPT` writer - write 0x58ab99fc generate write_protect, write 0x51ff8621 to release
pub type WrptW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `WRPT_ST` reader - 1 indicates write protect is active
pub type WrptStR = crate::BitReader;
///Field `WRPT_ST` writer - 1 indicates write protect is active
pub type WrptStW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - write 0x58ab99fc generate write_protect, write 0x51ff8621 to release
    #[inline(always)]
    pub fn wrpt(&self) -> WrptR {
        WrptR::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - 1 indicates write protect is active
    #[inline(always)]
    pub fn wrpt_st(&self) -> WrptStR {
        WrptStR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_WP")
            .field("wrpt_st", &self.wrpt_st())
            .field("wrpt", &self.wrpt())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - write 0x58ab99fc generate write_protect, write 0x51ff8621 to release
    #[inline(always)]
    pub fn wrpt(&mut self) -> WrptW<WDT_WPrs> {
        WrptW::new(self, 0)
    }
    ///Bit 31 - 1 indicates write protect is active
    #[inline(always)]
    pub fn wrpt_st(&mut self) -> WrptStW<WDT_WPrs> {
        WrptStW::new(self, 31)
    }
}
///WatchDog Write Protect Register
///
///You can [`read`](crate::Reg::read) this register and get [`wdt_wp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_wp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WDT_WPrs;
impl crate::RegisterSpec for WDT_WPrs {
    type Ux = u32;
}
///`read()` method returns [`wdt_wp::R`](R) reader structure
impl crate::Readable for WDT_WPrs {}
///`write(|w| ..)` method takes [`wdt_wp::W`](W) writer structure
impl crate::Writable for WDT_WPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDT_WP to value 0
impl crate::Resettable for WDT_WPrs {
    const RESET_VALUE: u32 = 0;
}
