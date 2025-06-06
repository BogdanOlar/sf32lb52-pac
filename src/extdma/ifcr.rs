///Register `IFCR` reader
pub type R = crate::R<IFCRrs>;
///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `CGIF` reader - CGIF, global interrupt flag clear
pub type CgifR = crate::BitReader;
///Field `CGIF` writer - CGIF, global interrupt flag clear
pub type CgifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF` reader - CTCIF, transfer complete flag clear
pub type CtcifR = crate::BitReader;
///Field `CTCIF` writer - CTCIF, transfer complete flag clear
pub type CtcifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF` reader - CHTIF, half transfer flag clear
pub type ChtifR = crate::BitReader;
///Field `CHTIF` writer - CHTIF, half transfer flag clear
pub type ChtifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF` reader - CTEIF, transfer error flag clear
pub type CteifR = crate::BitReader;
///Field `CTEIF` writer - CTEIF, transfer error flag clear
pub type CteifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif(&self) -> CgifR {
        CgifR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif(&self) -> CtcifR {
        CtcifR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif(&self) -> ChtifR {
        ChtifR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif(&self) -> CteifR {
        CteifR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFCR")
            .field("cteif", &self.cteif())
            .field("chtif", &self.chtif())
            .field("ctcif", &self.ctcif())
            .field("cgif", &self.cgif())
            .finish()
    }
}
impl W {
    ///Bit 0 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif(&mut self) -> CgifW<IFCRrs> {
        CgifW::new(self, 0)
    }
    ///Bit 1 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif(&mut self) -> CtcifW<IFCRrs> {
        CtcifW::new(self, 1)
    }
    ///Bit 2 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif(&mut self) -> ChtifW<IFCRrs> {
        ChtifW::new(self, 2)
    }
    ///Bit 3 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif(&mut self) -> CteifW<IFCRrs> {
        CteifW::new(self, 3)
    }
}
///interrupt clear register
///
///You can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ifcr::R`](R) reader structure
impl crate::Readable for IFCRrs {}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
