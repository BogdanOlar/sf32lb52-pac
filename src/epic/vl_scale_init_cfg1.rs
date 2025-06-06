///Register `VL_SCALE_INIT_CFG1` reader
pub type R = crate::R<VL_SCALE_INIT_CFG1rs>;
///Register `VL_SCALE_INIT_CFG1` writer
pub type W = crate::W<VL_SCALE_INIT_CFG1rs>;
///Field `X_VAL` reader - x-axis scale initial value, 10.16 format
pub type XValR = crate::FieldReader<u32>;
///Field `X_VAL` writer - x-axis scale initial value, 10.16 format
pub type XValW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:25 - x-axis scale initial value, 10.16 format
    #[inline(always)]
    pub fn x_val(&self) -> XValR {
        XValR::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_SCALE_INIT_CFG1")
            .field("rsvd", &self.rsvd())
            .field("x_val", &self.x_val())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - x-axis scale initial value, 10.16 format
    #[inline(always)]
    pub fn x_val(&mut self) -> XValW<VL_SCALE_INIT_CFG1rs> {
        XValW::new(self, 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<VL_SCALE_INIT_CFG1rs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_init_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_init_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_SCALE_INIT_CFG1rs;
impl crate::RegisterSpec for VL_SCALE_INIT_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`vl_scale_init_cfg1::R`](R) reader structure
impl crate::Readable for VL_SCALE_INIT_CFG1rs {}
///`write(|w| ..)` method takes [`vl_scale_init_cfg1::W`](W) writer structure
impl crate::Writable for VL_SCALE_INIT_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_SCALE_INIT_CFG1 to value 0
impl crate::Resettable for VL_SCALE_INIT_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
