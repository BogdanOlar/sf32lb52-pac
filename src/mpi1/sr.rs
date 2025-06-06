///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `TCF` reader - Transfer complete flag
pub type TcfR = crate::BitReader;
///Field `TCF` writer - Transfer complete flag
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMF` reader - Status match flag in Polling Mode
pub type SmfR = crate::BitReader;
///Field `SMF` writer - Status match flag in Polling Mode
pub type SmfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSVF` reader - CS max violation flag
pub type CsvfR = crate::BitReader;
///Field `CSVF` writer - CS max violation flag
pub type CsvfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBXF` reader - Row boundary crossing flag
pub type RbxfR = crate::BitReader;
///Field `RBXF` writer - Row boundary crossing flag
pub type RbxfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSY` reader - For debug purpose only
pub type BusyR = crate::BitReader;
///Field `BUSY` writer - For debug purpose only
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Status match flag in Polling Mode
    #[inline(always)]
    pub fn smf(&self) -> SmfR {
        SmfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CS max violation flag
    #[inline(always)]
    pub fn csvf(&self) -> CsvfR {
        CsvfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Row boundary crossing flag
    #[inline(always)]
    pub fn rbxf(&self) -> RbxfR {
        RbxfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 31 - For debug purpose only
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("rbxf", &self.rbxf())
            .field("csvf", &self.csvf())
            .field("smf", &self.smf())
            .field("tcf", &self.tcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&mut self) -> TcfW<SRrs> {
        TcfW::new(self, 0)
    }
    ///Bit 3 - Status match flag in Polling Mode
    #[inline(always)]
    pub fn smf(&mut self) -> SmfW<SRrs> {
        SmfW::new(self, 3)
    }
    ///Bit 4 - CS max violation flag
    #[inline(always)]
    pub fn csvf(&mut self) -> CsvfW<SRrs> {
        CsvfW::new(self, 4)
    }
    ///Bit 5 - Row boundary crossing flag
    #[inline(always)]
    pub fn rbxf(&mut self) -> RbxfW<SRrs> {
        RbxfW::new(self, 5)
    }
    ///Bit 31 - For debug purpose only
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<SRrs> {
        BusyW::new(self, 31)
    }
}
///Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0x06
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x06;
}
