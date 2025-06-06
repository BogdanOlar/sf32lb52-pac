///Register `AAEAR` reader
pub type R = crate::R<AAEARrs>;
///Register `AAEAR` writer
pub type W = crate::W<AAEARrs>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `EA` reader - Ending address of the address aliasing area
pub type EaR = crate::FieldReader<u32>;
///Field `EA` writer - Ending address of the address aliasing area
pub type EaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:31 - Ending address of the address aliasing area
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AAEAR")
            .field("ea", &self.ea())
            .field("rsvd", &self.rsvd())
            .finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AAEARrs> {
        RsvdW::new(self, 0)
    }
    ///Bits 10:31 - Ending address of the address aliasing area
    #[inline(always)]
    pub fn ea(&mut self) -> EaW<AAEARrs> {
        EaW::new(self, 10)
    }
}
///Address Aliasing Ending Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`aaear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aaear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AAEARrs;
impl crate::RegisterSpec for AAEARrs {
    type Ux = u32;
}
///`read()` method returns [`aaear::R`](R) reader structure
impl crate::Readable for AAEARrs {}
///`write(|w| ..)` method takes [`aaear::W`](W) writer structure
impl crate::Writable for AAEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AAEAR to value 0
impl crate::Resettable for AAEARrs {
    const RESET_VALUE: u32 = 0;
}
