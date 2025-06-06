///Register `INT_ST` reader
pub type R = crate::R<INT_STrs>;
///Register `INT_ST` writer
pub type W = crate::W<INT_STrs>;
///Field `OVERFLOW_R` reader - 1 indicates right channel fifo has already overflowed and as irq at same time
pub type OverflowRR = crate::BitReader;
///Field `OVERFLOW_R` writer - 1 indicates right channel fifo has already overflowed and as irq at same time
pub type OverflowRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVERFLOW_L` reader - 1 indicates left channel fifo has already overflowed and as irq at same time
pub type OverflowLR = crate::BitReader;
///Field `OVERFLOW_L` writer - 1 indicates left channel fifo has already overflowed and as irq at same time
pub type OverflowLW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1 indicates right channel fifo has already overflowed and as irq at same time
    #[inline(always)]
    pub fn overflow_r(&self) -> OverflowRR {
        OverflowRR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1 indicates left channel fifo has already overflowed and as irq at same time
    #[inline(always)]
    pub fn overflow_l(&self) -> OverflowLR {
        OverflowLR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("overflow_l", &self.overflow_l())
            .field("overflow_r", &self.overflow_r())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1 indicates right channel fifo has already overflowed and as irq at same time
    #[inline(always)]
    pub fn overflow_r(&mut self) -> OverflowRW<INT_STrs> {
        OverflowRW::new(self, 0)
    }
    ///Bit 1 - 1 indicates left channel fifo has already overflowed and as irq at same time
    #[inline(always)]
    pub fn overflow_l(&mut self) -> OverflowLW<INT_STrs> {
        OverflowLW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_STrs;
impl crate::RegisterSpec for INT_STrs {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_STrs {}
///`write(|w| ..)` method takes [`int_st::W`](W) writer structure
impl crate::Writable for INT_STrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_STrs {
    const RESET_VALUE: u32 = 0;
}
