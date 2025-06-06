///Register `AEZIP_CTRL` reader
pub type R = crate::R<AEZIP_CTRLrs>;
///Register `AEZIP_CTRL` writer
pub type W = crate::W<AEZIP_CTRLrs>;
///Field `AEZIP_CTRL` reader - AEZIP ctrl
pub type AezipCtrlR = crate::BitReader;
///Field `AEZIP_CTRL` writer - AEZIP ctrl
pub type AezipCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - AEZIP ctrl
    #[inline(always)]
    pub fn aezip_ctrl(&self) -> AezipCtrlR {
        AezipCtrlR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AEZIP_CTRL")
            .field("rsvd", &self.rsvd())
            .field("aezip_ctrl", &self.aezip_ctrl())
            .finish()
    }
}
impl W {
    ///Bit 0 - AEZIP ctrl
    #[inline(always)]
    pub fn aezip_ctrl(&mut self) -> AezipCtrlW<AEZIP_CTRLrs> {
        AezipCtrlW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AEZIP_CTRLrs> {
        RsvdW::new(self, 1)
    }
}
///AEZIP ctrl
///
///You can [`read`](crate::Reg::read) this register and get [`aezip_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aezip_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AEZIP_CTRLrs;
impl crate::RegisterSpec for AEZIP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`aezip_ctrl::R`](R) reader structure
impl crate::Readable for AEZIP_CTRLrs {}
///`write(|w| ..)` method takes [`aezip_ctrl::W`](W) writer structure
impl crate::Writable for AEZIP_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AEZIP_CTRL to value 0
impl crate::Resettable for AEZIP_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
