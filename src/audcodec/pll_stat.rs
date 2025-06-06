///Register `PLL_STAT` reader
pub type R = crate::R<PLL_STATrs>;
///Register `PLL_STAT` writer
pub type W = crate::W<PLL_STATrs>;
///Field `UNLOCK` reader - 1:pll unlock
pub type UnlockR = crate::BitReader;
///Field `UNLOCK` writer - 1:pll unlock
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLIPPED_UP` reader - slip up
pub type SlippedUpR = crate::BitReader;
///Field `SLIPPED_UP` writer - slip up
pub type SlippedUpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLIPPED_DN` reader - slip dn
pub type SlippedDnR = crate::BitReader;
///Field `SLIPPED_DN` writer - slip dn
pub type SlippedDnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bit 0 - 1:pll unlock
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - slip up
    #[inline(always)]
    pub fn slipped_up(&self) -> SlippedUpR {
        SlippedUpR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - slip dn
    #[inline(always)]
    pub fn slipped_dn(&self) -> SlippedDnR {
        SlippedDnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_STAT")
            .field("rsvd", &self.rsvd())
            .field("slipped_dn", &self.slipped_dn())
            .field("slipped_up", &self.slipped_up())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1:pll unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<PLL_STATrs> {
        UnlockW::new(self, 0)
    }
    ///Bit 1 - slip up
    #[inline(always)]
    pub fn slipped_up(&mut self) -> SlippedUpW<PLL_STATrs> {
        SlippedUpW::new(self, 1)
    }
    ///Bit 2 - slip dn
    #[inline(always)]
    pub fn slipped_dn(&mut self) -> SlippedDnW<PLL_STATrs> {
        SlippedDnW::new(self, 2)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PLL_STATrs> {
        RsvdW::new(self, 3)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_STATrs;
impl crate::RegisterSpec for PLL_STATrs {
    type Ux = u32;
}
///`read()` method returns [`pll_stat::R`](R) reader structure
impl crate::Readable for PLL_STATrs {}
///`write(|w| ..)` method takes [`pll_stat::W`](W) writer structure
impl crate::Writable for PLL_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_STAT to value 0
impl crate::Resettable for PLL_STATrs {
    const RESET_VALUE: u32 = 0;
}
