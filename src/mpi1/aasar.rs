///Register `AASAR` reader
pub type R = crate::R<AASARrs>;
///Register `AASAR` writer
pub type W = crate::W<AASARrs>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `SA` reader - Starting address of the address aliasing area. Always 1KB aligned.Together with AAEAR, the aliasing area is \[AASAR, AAEAR). If the address falls into this area, an offset AAOAR is added and the aliased address will be used to access external memory
pub type SaR = crate::FieldReader<u32>;
///Field `SA` writer - Starting address of the address aliasing area. Always 1KB aligned.Together with AAEAR, the aliasing area is \[AASAR, AAEAR). If the address falls into this area, an offset AAOAR is added and the aliased address will be used to access external memory
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:31 - Starting address of the address aliasing area. Always 1KB aligned.Together with AAEAR, the aliasing area is \[AASAR, AAEAR). If the address falls into this area, an offset AAOAR is added and the aliased address will be used to access external memory
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AASAR")
            .field("sa", &self.sa())
            .field("rsvd", &self.rsvd())
            .finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AASARrs> {
        RsvdW::new(self, 0)
    }
    ///Bits 10:31 - Starting address of the address aliasing area. Always 1KB aligned.Together with AAEAR, the aliasing area is \[AASAR, AAEAR). If the address falls into this area, an offset AAOAR is added and the aliased address will be used to access external memory
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<AASARrs> {
        SaW::new(self, 10)
    }
}
///Address Aliasing Start Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`aasar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aasar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AASARrs;
impl crate::RegisterSpec for AASARrs {
    type Ux = u32;
}
///`read()` method returns [`aasar::R`](R) reader structure
impl crate::Readable for AASARrs {}
///`write(|w| ..)` method takes [`aasar::W`](W) writer structure
impl crate::Writable for AASARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AASAR to value 0
impl crate::Resettable for AASARrs {
    const RESET_VALUE: u32 = 0;
}
