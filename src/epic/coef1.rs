///Register `COEF1` reader
pub type R = crate::R<COEF1rs>;
///Register `COEF1` writer
pub type W = crate::W<COEF1rs>;
///Field `FVG` reader - YUV Fvg coef
pub type FvgR = crate::FieldReader<u16>;
///Field `FVG` writer - YUV Fvg coef
pub type FvgW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `FVR` reader - YUV Fvr coef
pub type FvrR = crate::FieldReader<u16>;
///Field `FVR` writer - YUV Fvr coef
pub type FvrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - YUV Fvg coef
    #[inline(always)]
    pub fn fvg(&self) -> FvgR {
        FvgR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - YUV Fvr coef
    #[inline(always)]
    pub fn fvr(&self) -> FvrR {
        FvrR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEF1")
            .field("fvr", &self.fvr())
            .field("fvg", &self.fvg())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - YUV Fvg coef
    #[inline(always)]
    pub fn fvg(&mut self) -> FvgW<COEF1rs> {
        FvgW::new(self, 0)
    }
    ///Bits 10:19 - YUV Fvr coef
    #[inline(always)]
    pub fn fvr(&mut self) -> FvrW<COEF1rs> {
        FvrW::new(self, 10)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`coef1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coef1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COEF1rs;
impl crate::RegisterSpec for COEF1rs {
    type Ux = u32;
}
///`read()` method returns [`coef1::R`](R) reader structure
impl crate::Readable for COEF1rs {}
///`write(|w| ..)` method takes [`coef1::W`](W) writer structure
impl crate::Writable for COEF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COEF1 to value 0
impl crate::Resettable for COEF1rs {
    const RESET_VALUE: u32 = 0;
}
