///Register `VL_MISC_CFG` reader
pub type R = crate::R<VL_MISC_CFGrs>;
///Register `VL_MISC_CFG` writer
pub type W = crate::W<VL_MISC_CFGrs>;
///Field `CLUT_SEL` reader - VL CLUT select: 1'h1: select pallette1 1'h0: select pallette0
pub type ClutSelR = crate::BitReader;
///Field `CLUT_SEL` writer - VL CLUT select: 1'h1: select pallette1 1'h0: select pallette0
pub type ClutSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `V_MIRROR` reader - vertical mirror enable
pub type VMirrorR = crate::BitReader;
///Field `V_MIRROR` writer - vertical mirror enable
pub type VMirrorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `H_MIRROR` reader - horizontal mirror enable
pub type HMirrorR = crate::BitReader;
///Field `H_MIRROR` writer - horizontal mirror enable
pub type HMirrorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COS_FORCE_VALUE` reader - external absolute value of cos. U13.12 format.
pub type CosForceValueR = crate::FieldReader<u16>;
///Field `COS_FORCE_VALUE` writer - external absolute value of cos. U13.12 format.
pub type CosForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `SIN_FORCE_VALUE` reader - external absolute value of sin. U13.12 format.
pub type SinForceValueR = crate::FieldReader<u16>;
///Field `SIN_FORCE_VALUE` writer - external absolute value of sin. U13.12 format.
pub type SinForceValueW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `DEG_FORCE` reader - force epic use external sin and cos value, quadrant is still calculated from ROT_DEG.
pub type DegForceR = crate::BitReader;
///Field `DEG_FORCE` writer - force epic use external sin and cos value, quadrant is still calculated from ROT_DEG.
pub type DegForceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - VL CLUT select: 1'h1: select pallette1 1'h0: select pallette0
    #[inline(always)]
    pub fn clut_sel(&self) -> ClutSelR {
        ClutSelR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - vertical mirror enable
    #[inline(always)]
    pub fn v_mirror(&self) -> VMirrorR {
        VMirrorR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - horizontal mirror enable
    #[inline(always)]
    pub fn h_mirror(&self) -> HMirrorR {
        HMirrorR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:15 - external absolute value of cos. U13.12 format.
    #[inline(always)]
    pub fn cos_force_value(&self) -> CosForceValueR {
        CosForceValueR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    ///Bits 16:28 - external absolute value of sin. U13.12 format.
    #[inline(always)]
    pub fn sin_force_value(&self) -> SinForceValueR {
        SinForceValueR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    ///Bit 29 - force epic use external sin and cos value, quadrant is still calculated from ROT_DEG.
    #[inline(always)]
    pub fn deg_force(&self) -> DegForceR {
        DegForceR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_MISC_CFG")
            .field("rsvd", &self.rsvd())
            .field("deg_force", &self.deg_force())
            .field("sin_force_value", &self.sin_force_value())
            .field("cos_force_value", &self.cos_force_value())
            .field("h_mirror", &self.h_mirror())
            .field("v_mirror", &self.v_mirror())
            .field("clut_sel", &self.clut_sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - VL CLUT select: 1'h1: select pallette1 1'h0: select pallette0
    #[inline(always)]
    pub fn clut_sel(&mut self) -> ClutSelW<VL_MISC_CFGrs> {
        ClutSelW::new(self, 0)
    }
    ///Bit 1 - vertical mirror enable
    #[inline(always)]
    pub fn v_mirror(&mut self) -> VMirrorW<VL_MISC_CFGrs> {
        VMirrorW::new(self, 1)
    }
    ///Bit 2 - horizontal mirror enable
    #[inline(always)]
    pub fn h_mirror(&mut self) -> HMirrorW<VL_MISC_CFGrs> {
        HMirrorW::new(self, 2)
    }
    ///Bits 3:15 - external absolute value of cos. U13.12 format.
    #[inline(always)]
    pub fn cos_force_value(&mut self) -> CosForceValueW<VL_MISC_CFGrs> {
        CosForceValueW::new(self, 3)
    }
    ///Bits 16:28 - external absolute value of sin. U13.12 format.
    #[inline(always)]
    pub fn sin_force_value(&mut self) -> SinForceValueW<VL_MISC_CFGrs> {
        SinForceValueW::new(self, 16)
    }
    ///Bit 29 - force epic use external sin and cos value, quadrant is still calculated from ROT_DEG.
    #[inline(always)]
    pub fn deg_force(&mut self) -> DegForceW<VL_MISC_CFGrs> {
        DegForceW::new(self, 29)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<VL_MISC_CFGrs> {
        RsvdW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_misc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_misc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_MISC_CFGrs;
impl crate::RegisterSpec for VL_MISC_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`vl_misc_cfg::R`](R) reader structure
impl crate::Readable for VL_MISC_CFGrs {}
///`write(|w| ..)` method takes [`vl_misc_cfg::W`](W) writer structure
impl crate::Writable for VL_MISC_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_MISC_CFG to value 0
impl crate::Resettable for VL_MISC_CFGrs {
    const RESET_VALUE: u32 = 0;
}
