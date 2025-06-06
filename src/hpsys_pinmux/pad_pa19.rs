///Register `PAD_PA19` reader
pub type R = crate::R<PAD_PA19rs>;
///Register `PAD_PA19` writer
pub type W = crate::W<PAD_PA19rs>;
///Field `FSEL` reader - Function Select
pub type FselR = crate::FieldReader;
///Field `FSEL` writer - Function Select
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PE` reader - Pull Enable. Logic HIGH enables week pull device
pub type PeR = crate::BitReader;
///Field `PE` writer - Pull Enable. Logic HIGH enables week pull device
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down
pub type PsR = crate::BitReader;
///Field `PS` writer - Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE` reader - Input Enable. Logic HIGH enables the input buffer
pub type IeR = crate::BitReader;
///Field `IE` writer - Input Enable. Logic HIGH enables the input buffer
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IS` reader - Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input
pub type IsR = crate::BitReader;
///Field `IS` writer - Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input
pub type IsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SR` reader - Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate
pub type SrR = crate::BitReader;
///Field `SR` writer - Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DS0` reader - Drive Select 0. Used to select output drive strength
pub type Ds0R = crate::BitReader;
///Field `DS0` writer - Drive Select 0. Used to select output drive strength
pub type Ds0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DS1` reader - Drive Select 1. Used to select output drive strength
pub type Ds1R = crate::BitReader;
///Field `DS1` writer - Drive Select 1. Used to select output drive strength
pub type Ds1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POE` reader - Reserved. Always set to logic LOW
pub type PoeR = crate::BitReader;
///Field `POE` writer - Reserved. Always set to logic LOW
pub type PoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Function Select
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Pull Enable. Logic HIGH enables week pull device
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Input Enable. Logic HIGH enables the input buffer
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Drive Select 0. Used to select output drive strength
    #[inline(always)]
    pub fn ds0(&self) -> Ds0R {
        Ds0R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Drive Select 1. Used to select output drive strength
    #[inline(always)]
    pub fn ds1(&self) -> Ds1R {
        Ds1R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reserved. Always set to logic LOW
    #[inline(always)]
    pub fn poe(&self) -> PoeR {
        PoeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_PA19")
            .field("poe", &self.poe())
            .field("ds1", &self.ds1())
            .field("ds0", &self.ds0())
            .field("sr", &self.sr())
            .field("is", &self.is())
            .field("ie", &self.ie())
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("fsel", &self.fsel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Function Select
    #[inline(always)]
    pub fn fsel(&mut self) -> FselW<PAD_PA19rs> {
        FselW::new(self, 0)
    }
    ///Bit 4 - Pull Enable. Logic HIGH enables week pull device
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<PAD_PA19rs> {
        PeW::new(self, 4)
    }
    ///Bit 5 - Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<PAD_PA19rs> {
        PsW::new(self, 5)
    }
    ///Bit 6 - Input Enable. Logic HIGH enables the input buffer
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<PAD_PA19rs> {
        IeW::new(self, 6)
    }
    ///Bit 7 - Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input
    #[inline(always)]
    pub fn is(&mut self) -> IsW<PAD_PA19rs> {
        IsW::new(self, 7)
    }
    ///Bit 8 - Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<PAD_PA19rs> {
        SrW::new(self, 8)
    }
    ///Bit 9 - Drive Select 0. Used to select output drive strength
    #[inline(always)]
    pub fn ds0(&mut self) -> Ds0W<PAD_PA19rs> {
        Ds0W::new(self, 9)
    }
    ///Bit 10 - Drive Select 1. Used to select output drive strength
    #[inline(always)]
    pub fn ds1(&mut self) -> Ds1W<PAD_PA19rs> {
        Ds1W::new(self, 10)
    }
    ///Bit 11 - Reserved. Always set to logic LOW
    #[inline(always)]
    pub fn poe(&mut self) -> PoeW<PAD_PA19rs> {
        PoeW::new(self, 11)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PAD_PA19rs;
impl crate::RegisterSpec for PAD_PA19rs {
    type Ux = u32;
}
///`read()` method returns [`pad_pa19::R`](R) reader structure
impl crate::Readable for PAD_PA19rs {}
///`write(|w| ..)` method takes [`pad_pa19::W`](W) writer structure
impl crate::Writable for PAD_PA19rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAD_PA19 to value 0
impl crate::Resettable for PAD_PA19rs {
    const RESET_VALUE: u32 = 0;
}
