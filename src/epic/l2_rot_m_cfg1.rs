///Register `L2_ROT_M_CFG1` reader
pub type R = crate::R<L2_ROT_M_CFG1rs>;
///Register `L2_ROT_M_CFG1` writer
pub type W = crate::W<L2_ROT_M_CFG1rs>;
///Field `M_ROT_MAX_LINE` reader - manual mode rotation max line, unsigned value
pub type MRotMaxLineR = crate::FieldReader<u16>;
///Field `M_ROT_MAX_LINE` writer - manual mode rotation max line, unsigned value
pub type MRotMaxLineW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `M_ROT_MAX_COL` reader - manual mode rotation max column, unsigned value
pub type MRotMaxColR = crate::FieldReader<u16>;
///Field `M_ROT_MAX_COL` writer - manual mode rotation max column, unsigned value
pub type MRotMaxColW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `M_MODE` reader - rotation mode setting 1'b0: auto mode 1'b1: manual mode
pub type MModeR = crate::BitReader;
///Field `M_MODE` writer - rotation mode setting 1'b0: auto mode 1'b1: manual mode
pub type MModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - manual mode rotation max line, unsigned value
    #[inline(always)]
    pub fn m_rot_max_line(&self) -> MRotMaxLineR {
        MRotMaxLineR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 16:26 - manual mode rotation max column, unsigned value
    #[inline(always)]
    pub fn m_rot_max_col(&self) -> MRotMaxColR {
        MRotMaxColR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bits 27:30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    ///Bit 31 - rotation mode setting 1'b0: auto mode 1'b1: manual mode
    #[inline(always)]
    pub fn m_mode(&self) -> MModeR {
        MModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_ROT_M_CFG1")
            .field("m_mode", &self.m_mode())
            .field("rsvd", &self.rsvd())
            .field("m_rot_max_col", &self.m_rot_max_col())
            .field("rsvd2", &self.rsvd2())
            .field("m_rot_max_line", &self.m_rot_max_line())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - manual mode rotation max line, unsigned value
    #[inline(always)]
    pub fn m_rot_max_line(&mut self) -> MRotMaxLineW<L2_ROT_M_CFG1rs> {
        MRotMaxLineW::new(self, 0)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<L2_ROT_M_CFG1rs> {
        Rsvd2W::new(self, 11)
    }
    ///Bits 16:26 - manual mode rotation max column, unsigned value
    #[inline(always)]
    pub fn m_rot_max_col(&mut self) -> MRotMaxColW<L2_ROT_M_CFG1rs> {
        MRotMaxColW::new(self, 16)
    }
    ///Bits 27:30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<L2_ROT_M_CFG1rs> {
        RsvdW::new(self, 27)
    }
    ///Bit 31 - rotation mode setting 1'b0: auto mode 1'b1: manual mode
    #[inline(always)]
    pub fn m_mode(&mut self) -> MModeW<L2_ROT_M_CFG1rs> {
        MModeW::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_m_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_m_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_ROT_M_CFG1rs;
impl crate::RegisterSpec for L2_ROT_M_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`l2_rot_m_cfg1::R`](R) reader structure
impl crate::Readable for L2_ROT_M_CFG1rs {}
///`write(|w| ..)` method takes [`l2_rot_m_cfg1::W`](W) writer structure
impl crate::Writable for L2_ROT_M_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_ROT_M_CFG1 to value 0
impl crate::Resettable for L2_ROT_M_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
