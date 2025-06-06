///Register `GREY_PARA` reader
pub type R = crate::R<GREY_PARArs>;
///Register `GREY_PARA` writer
pub type W = crate::W<GREY_PARArs>;
///Field `GREY_PARA` reader - fill color parameter, when send grey data to epic . \[23:16\]-R,\[15:8\]-G,\[7:0\]-B
pub type GreyParaR = crate::FieldReader<u32>;
///Field `GREY_PARA` writer - fill color parameter, when send grey data to epic . \[23:16\]-R,\[15:8\]-G,\[7:0\]-B
pub type GreyParaW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - fill color parameter, when send grey data to epic . \[23:16\]-R,\[15:8\]-G,\[7:0\]-B
    #[inline(always)]
    pub fn grey_para(&self) -> GreyParaR {
        GreyParaR::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GREY_PARA")
            .field("rsvd", &self.rsvd())
            .field("grey_para", &self.grey_para())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - fill color parameter, when send grey data to epic . \[23:16\]-R,\[15:8\]-G,\[7:0\]-B
    #[inline(always)]
    pub fn grey_para(&mut self) -> GreyParaW<GREY_PARArs> {
        GreyParaW::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<GREY_PARArs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`grey_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grey_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GREY_PARArs;
impl crate::RegisterSpec for GREY_PARArs {
    type Ux = u32;
}
///`read()` method returns [`grey_para::R`](R) reader structure
impl crate::Readable for GREY_PARArs {}
///`write(|w| ..)` method takes [`grey_para::W`](W) writer structure
impl crate::Writable for GREY_PARArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GREY_PARA to value 0
impl crate::Resettable for GREY_PARArs {
    const RESET_VALUE: u32 = 0;
}
