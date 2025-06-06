///Register `PAD_PA41` reader
pub type R = crate::R<PAD_PA41rs>;
///Register `PAD_PA41` writer
pub type W = crate::W<PAD_PA41rs>;
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
///Field `MODE` reader - Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode
pub type ModeR = crate::BitReader;
///Field `MODE` writer - Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DS` reader - Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive
pub type DsR = crate::BitReader;
///Field `DS` writer - Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive
pub type DsW<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reserved. Always set to logic LOW
    #[inline(always)]
    pub fn poe(&self) -> PoeR {
        PoeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_PA41")
            .field("poe", &self.poe())
            .field("ds", &self.ds())
            .field("mode", &self.mode())
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
    pub fn fsel(&mut self) -> FselW<PAD_PA41rs> {
        FselW::new(self, 0)
    }
    ///Bit 4 - Pull Enable. Logic HIGH enables week pull device
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<PAD_PA41rs> {
        PeW::new(self, 4)
    }
    ///Bit 5 - Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<PAD_PA41rs> {
        PsW::new(self, 5)
    }
    ///Bit 6 - Input Enable. Logic HIGH enables the input buffer
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<PAD_PA41rs> {
        IeW::new(self, 6)
    }
    ///Bit 7 - Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input
    #[inline(always)]
    pub fn is(&mut self) -> IsW<PAD_PA41rs> {
        IsW::new(self, 7)
    }
    ///Bit 8 - Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<PAD_PA41rs> {
        ModeW::new(self, 8)
    }
    ///Bit 10 - Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<PAD_PA41rs> {
        DsW::new(self, 10)
    }
    ///Bit 11 - Reserved. Always set to logic LOW
    #[inline(always)]
    pub fn poe(&mut self) -> PoeW<PAD_PA41rs> {
        PoeW::new(self, 11)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pad_pa41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pa41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PAD_PA41rs;
impl crate::RegisterSpec for PAD_PA41rs {
    type Ux = u32;
}
///`read()` method returns [`pad_pa41::R`](R) reader structure
impl crate::Readable for PAD_PA41rs {}
///`write(|w| ..)` method takes [`pad_pa41::W`](W) writer structure
impl crate::Writable for PAD_PA41rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAD_PA41 to value 0
impl crate::Resettable for PAD_PA41rs {
    const RESET_VALUE: u32 = 0;
}
