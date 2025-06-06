///Register `L2_ROT_M_CFG3` reader
pub type R = crate::R<L2_ROT_M_CFG3rs>;
///Register `L2_ROT_M_CFG3` writer
pub type W = crate::W<L2_ROT_M_CFG3rs>;
///Field `M_XTL` reader - manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported
pub type MXtlR = crate::FieldReader<u16>;
///Field `M_XTL` writer - manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported
pub type MXtlW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `M_YTL` reader - manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported
pub type MYtlR = crate::FieldReader<u16>;
///Field `M_YTL` writer - manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported
pub type MYtlW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_xtl(&self) -> MXtlR {
        MXtlR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_ytl(&self) -> MYtlR {
        MYtlR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_ROT_M_CFG3")
            .field("m_ytl", &self.m_ytl())
            .field("m_xtl", &self.m_xtl())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_xtl(&mut self) -> MXtlW<L2_ROT_M_CFG3rs> {
        MXtlW::new(self, 0)
    }
    ///Bits 16:26 - manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_ytl(&mut self) -> MYtlW<L2_ROT_M_CFG3rs> {
        MYtlW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_m_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_m_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_ROT_M_CFG3rs;
impl crate::RegisterSpec for L2_ROT_M_CFG3rs {
    type Ux = u32;
}
///`read()` method returns [`l2_rot_m_cfg3::R`](R) reader structure
impl crate::Readable for L2_ROT_M_CFG3rs {}
///`write(|w| ..)` method takes [`l2_rot_m_cfg3::W`](W) writer structure
impl crate::Writable for L2_ROT_M_CFG3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_ROT_M_CFG3 to value 0
impl crate::Resettable for L2_ROT_M_CFG3rs {
    const RESET_VALUE: u32 = 0;
}
