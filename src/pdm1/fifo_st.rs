///Register `FIFO_ST` reader
pub type R = crate::R<FIFO_STrs>;
///Register `FIFO_ST` writer
pub type W = crate::W<FIFO_STrs>;
///Field `ALMOST_EMPTY_R` reader - 1 indicates right channel fifo is less than two datas left
pub type AlmostEmptyRR = crate::BitReader;
///Field `ALMOST_EMPTY_R` writer - 1 indicates right channel fifo is less than two datas left
pub type AlmostEmptyRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALMOST_FULL_R` reader - 1 indicates right channel fifo is less than two full
pub type AlmostFullRR = crate::BitReader;
///Field `ALMOST_FULL_R` writer - 1 indicates right channel fifo is less than two full
pub type AlmostFullRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMPTY_R` reader - 1 indicates right channel fifo is empty
pub type EmptyRR = crate::BitReader;
///Field `EMPTY_R` writer - 1 indicates right channel fifo is empty
pub type EmptyRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FULL_R` reader - 1 indicates right channel fifo is full
pub type FullRR = crate::BitReader;
///Field `FULL_R` writer - 1 indicates right channel fifo is full
pub type FullRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALMOST_EMPTY_L` reader - 1 indicates left channel fifo is less than two datas left
pub type AlmostEmptyLR = crate::BitReader;
///Field `ALMOST_EMPTY_L` writer - 1 indicates left channel fifo is less than two datas left
pub type AlmostEmptyLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALMOST_FULL_L` reader - 1 indicates left channel fifo is less than two full
pub type AlmostFullLR = crate::BitReader;
///Field `ALMOST_FULL_L` writer - 1 indicates left channel fifo is less than two full
pub type AlmostFullLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMPTY_L` reader - 1 indicates left channel fifo is empty
pub type EmptyLR = crate::BitReader;
///Field `EMPTY_L` writer - 1 indicates left channel fifo is empty
pub type EmptyLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FULL_L` reader - 1 indicates left channel fifo is full
pub type FullLR = crate::BitReader;
///Field `FULL_L` writer - 1 indicates left channel fifo is full
pub type FullLW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1 indicates right channel fifo is less than two datas left
    #[inline(always)]
    pub fn almost_empty_r(&self) -> AlmostEmptyRR {
        AlmostEmptyRR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1 indicates right channel fifo is less than two full
    #[inline(always)]
    pub fn almost_full_r(&self) -> AlmostFullRR {
        AlmostFullRR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 1 indicates right channel fifo is empty
    #[inline(always)]
    pub fn empty_r(&self) -> EmptyRR {
        EmptyRR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1 indicates right channel fifo is full
    #[inline(always)]
    pub fn full_r(&self) -> FullRR {
        FullRR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1 indicates left channel fifo is less than two datas left
    #[inline(always)]
    pub fn almost_empty_l(&self) -> AlmostEmptyLR {
        AlmostEmptyLR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 1 indicates left channel fifo is less than two full
    #[inline(always)]
    pub fn almost_full_l(&self) -> AlmostFullLR {
        AlmostFullLR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1 indicates left channel fifo is empty
    #[inline(always)]
    pub fn empty_l(&self) -> EmptyLR {
        EmptyLR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1 indicates left channel fifo is full
    #[inline(always)]
    pub fn full_l(&self) -> FullLR {
        FullLR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_ST")
            .field("full_l", &self.full_l())
            .field("empty_l", &self.empty_l())
            .field("almost_full_l", &self.almost_full_l())
            .field("almost_empty_l", &self.almost_empty_l())
            .field("full_r", &self.full_r())
            .field("empty_r", &self.empty_r())
            .field("almost_full_r", &self.almost_full_r())
            .field("almost_empty_r", &self.almost_empty_r())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1 indicates right channel fifo is less than two datas left
    #[inline(always)]
    pub fn almost_empty_r(&mut self) -> AlmostEmptyRW<FIFO_STrs> {
        AlmostEmptyRW::new(self, 0)
    }
    ///Bit 1 - 1 indicates right channel fifo is less than two full
    #[inline(always)]
    pub fn almost_full_r(&mut self) -> AlmostFullRW<FIFO_STrs> {
        AlmostFullRW::new(self, 1)
    }
    ///Bit 2 - 1 indicates right channel fifo is empty
    #[inline(always)]
    pub fn empty_r(&mut self) -> EmptyRW<FIFO_STrs> {
        EmptyRW::new(self, 2)
    }
    ///Bit 3 - 1 indicates right channel fifo is full
    #[inline(always)]
    pub fn full_r(&mut self) -> FullRW<FIFO_STrs> {
        FullRW::new(self, 3)
    }
    ///Bit 4 - 1 indicates left channel fifo is less than two datas left
    #[inline(always)]
    pub fn almost_empty_l(&mut self) -> AlmostEmptyLW<FIFO_STrs> {
        AlmostEmptyLW::new(self, 4)
    }
    ///Bit 5 - 1 indicates left channel fifo is less than two full
    #[inline(always)]
    pub fn almost_full_l(&mut self) -> AlmostFullLW<FIFO_STrs> {
        AlmostFullLW::new(self, 5)
    }
    ///Bit 6 - 1 indicates left channel fifo is empty
    #[inline(always)]
    pub fn empty_l(&mut self) -> EmptyLW<FIFO_STrs> {
        EmptyLW::new(self, 6)
    }
    ///Bit 7 - 1 indicates left channel fifo is full
    #[inline(always)]
    pub fn full_l(&mut self) -> FullLW<FIFO_STrs> {
        FullLW::new(self, 7)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FIFO_STrs;
impl crate::RegisterSpec for FIFO_STrs {
    type Ux = u32;
}
///`read()` method returns [`fifo_st::R`](R) reader structure
impl crate::Readable for FIFO_STrs {}
///`write(|w| ..)` method takes [`fifo_st::W`](W) writer structure
impl crate::Writable for FIFO_STrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_ST to value 0
impl crate::Resettable for FIFO_STrs {
    const RESET_VALUE: u32 = 0;
}
