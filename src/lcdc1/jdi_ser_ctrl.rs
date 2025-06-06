///Register `JDI_SER_CTRL` reader
pub type R = crate::R<JDI_SER_CTRLrs>;
///Register `JDI_SER_CTRL` writer
pub type W = crate::W<JDI_SER_CTRLrs>;
///Field `DISP` reader - jdi serial interface disp control
pub type DispR = crate::BitReader;
///Field `DISP` writer - jdi serial interface disp control
pub type DispW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTCOMIN` reader - jdi serial interface extcomin control
pub type ExtcominR = crate::BitReader;
///Field `EXTCOMIN` writer - jdi serial interface extcomin control
pub type ExtcominW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - jdi serial interface disp control
    #[inline(always)]
    pub fn disp(&self) -> DispR {
        DispR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - jdi serial interface extcomin control
    #[inline(always)]
    pub fn extcomin(&self) -> ExtcominR {
        ExtcominR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_SER_CTRL")
            .field("rsvd", &self.rsvd())
            .field("extcomin", &self.extcomin())
            .field("disp", &self.disp())
            .finish()
    }
}
impl W {
    ///Bit 0 - jdi serial interface disp control
    #[inline(always)]
    pub fn disp(&mut self) -> DispW<JDI_SER_CTRLrs> {
        DispW::new(self, 0)
    }
    ///Bit 1 - jdi serial interface extcomin control
    #[inline(always)]
    pub fn extcomin(&mut self) -> ExtcominW<JDI_SER_CTRLrs> {
        ExtcominW::new(self, 1)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<JDI_SER_CTRLrs> {
        RsvdW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_ser_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_ser_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_SER_CTRLrs;
impl crate::RegisterSpec for JDI_SER_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`jdi_ser_ctrl::R`](R) reader structure
impl crate::Readable for JDI_SER_CTRLrs {}
///`write(|w| ..)` method takes [`jdi_ser_ctrl::W`](W) writer structure
impl crate::Writable for JDI_SER_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_SER_CTRL to value 0
impl crate::Resettable for JDI_SER_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
