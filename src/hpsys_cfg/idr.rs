///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Register `IDR` writer
pub type W = crate::W<IDRrs>;
///Field `REVID` reader - Revision ID
pub type RevidR = crate::FieldReader;
///Field `REVID` writer - Revision ID
pub type RevidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PID` reader - Package ID
pub type PidR = crate::FieldReader;
///Field `PID` writer - Package ID
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CID` reader - Chip ID
pub type CidR = crate::FieldReader;
///Field `CID` writer - Chip ID
pub type CidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SID` reader - Series ID
pub type SidR = crate::FieldReader;
///Field `SID` writer - Series ID
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Revision ID
    #[inline(always)]
    pub fn revid(&self) -> RevidR {
        RevidR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Package ID
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Chip ID
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Series ID
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("sid", &self.sid())
            .field("cid", &self.cid())
            .field("pid", &self.pid())
            .field("revid", &self.revid())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Revision ID
    #[inline(always)]
    pub fn revid(&mut self) -> RevidW<IDRrs> {
        RevidW::new(self, 0)
    }
    ///Bits 8:15 - Package ID
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<IDRrs> {
        PidW::new(self, 8)
    }
    ///Bits 16:23 - Chip ID
    #[inline(always)]
    pub fn cid(&mut self) -> CidW<IDRrs> {
        CidW::new(self, 16)
    }
    ///Bits 24:31 - Series ID
    #[inline(always)]
    pub fn sid(&mut self) -> SidW<IDRrs> {
        SidW::new(self, 24)
    }
}
///ID Register
///
///You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`write(|w| ..)` method takes [`idr::W`](W) writer structure
impl crate::Writable for IDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
