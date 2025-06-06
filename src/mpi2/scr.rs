///Register `SCR` reader
pub type R = crate::R<SCRrs>;
///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `TCFC` reader - Write 1 to clear TCF
pub type TcfcR = crate::BitReader;
///Field `TCFC` writer - Write 1 to clear TCF
pub type TcfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMFC` reader - Write 1 to clear SMF
pub type SmfcR = crate::BitReader;
///Field `SMFC` writer - Write 1 to clear SMF
pub type SmfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSVFC` reader - Write 1 to clear CSVF
pub type CsvfcR = crate::BitReader;
///Field `CSVFC` writer - Write 1 to clear CSVF
pub type CsvfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBXFC` reader - Write 1 to clear RBXF
pub type RbxfcR = crate::BitReader;
///Field `RBXFC` writer - Write 1 to clear RBXF
pub type RbxfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Write 1 to clear TCF
    #[inline(always)]
    pub fn tcfc(&self) -> TcfcR {
        TcfcR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Write 1 to clear SMF
    #[inline(always)]
    pub fn smfc(&self) -> SmfcR {
        SmfcR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Write 1 to clear CSVF
    #[inline(always)]
    pub fn csvfc(&self) -> CsvfcR {
        CsvfcR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Write 1 to clear RBXF
    #[inline(always)]
    pub fn rbxfc(&self) -> RbxfcR {
        RbxfcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("rbxfc", &self.rbxfc())
            .field("csvfc", &self.csvfc())
            .field("smfc", &self.smfc())
            .field("tcfc", &self.tcfc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Write 1 to clear TCF
    #[inline(always)]
    pub fn tcfc(&mut self) -> TcfcW<SCRrs> {
        TcfcW::new(self, 0)
    }
    ///Bit 3 - Write 1 to clear SMF
    #[inline(always)]
    pub fn smfc(&mut self) -> SmfcW<SCRrs> {
        SmfcW::new(self, 3)
    }
    ///Bit 4 - Write 1 to clear CSVF
    #[inline(always)]
    pub fn csvfc(&mut self) -> CsvfcW<SCRrs> {
        CsvfcW::new(self, 4)
    }
    ///Bit 5 - Write 1 to clear RBXF
    #[inline(always)]
    pub fn rbxfc(&mut self) -> RbxfcW<SCRrs> {
        RbxfcW::new(self, 5)
    }
}
///Status Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCRrs {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
