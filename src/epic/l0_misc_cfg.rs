///Register `L0_MISC_CFG` reader
pub type R = crate::R<L0_MISC_CFGrs>;
///Register `L0_MISC_CFG` writer
pub type W = crate::W<L0_MISC_CFGrs>;
///Field `CLUT_SEL` reader - L0 CLUT select: 1'h1: select pallette1 1'h0: select pallette0
pub type ClutSelR = crate::BitReader;
///Field `CLUT_SEL` writer - L0 CLUT select: 1'h1: select pallette1 1'h0: select pallette0
pub type ClutSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `V_MIRROR` reader - vertical mirror enable
pub type VMirrorR = crate::BitReader;
///Field `V_MIRROR` writer - vertical mirror enable
pub type VMirrorW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - L0 CLUT select: 1'h1: select pallette1 1'h0: select pallette0
    #[inline(always)]
    pub fn clut_sel(&self) -> ClutSelR {
        ClutSelR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - vertical mirror enable
    #[inline(always)]
    pub fn v_mirror(&self) -> VMirrorR {
        VMirrorR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0_MISC_CFG")
            .field("rsvd", &self.rsvd())
            .field("v_mirror", &self.v_mirror())
            .field("clut_sel", &self.clut_sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - L0 CLUT select: 1'h1: select pallette1 1'h0: select pallette0
    #[inline(always)]
    pub fn clut_sel(&mut self) -> ClutSelW<L0_MISC_CFGrs> {
        ClutSelW::new(self, 0)
    }
    ///Bit 1 - vertical mirror enable
    #[inline(always)]
    pub fn v_mirror(&mut self) -> VMirrorW<L0_MISC_CFGrs> {
        VMirrorW::new(self, 1)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<L0_MISC_CFGrs> {
        RsvdW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l0_misc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_misc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L0_MISC_CFGrs;
impl crate::RegisterSpec for L0_MISC_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`l0_misc_cfg::R`](R) reader structure
impl crate::Readable for L0_MISC_CFGrs {}
///`write(|w| ..)` method takes [`l0_misc_cfg::W`](W) writer structure
impl crate::Writable for L0_MISC_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L0_MISC_CFG to value 0
impl crate::Resettable for L0_MISC_CFGrs {
    const RESET_VALUE: u32 = 0;
}
