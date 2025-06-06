///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
pub type UieR = crate::BitReader;
///Field `UIE` writer - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
pub type UdeR = crate::BitReader;
///Field `UDE` writer - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("ude", &self.ude())
            .field("uie", &self.uie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<DIERrs> {
        UieW::new(self, 0)
    }
    ///Bit 8 - Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<DIERrs> {
        UdeW::new(self, 8)
    }
}
///TIM DMA/Interrupt enable register
///
///You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
