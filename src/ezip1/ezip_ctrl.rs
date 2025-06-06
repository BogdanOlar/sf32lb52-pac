///Register `EZIP_CTRL` reader
pub type R = crate::R<EZIP_CTRLrs>;
///Register `EZIP_CTRL` writer
pub type W = crate::W<EZIP_CTRLrs>;
///Field `EZIP_CTRL` reader - 1:start or run 0:stop or end
pub type EzipCtrlR = crate::BitReader;
///Field `EZIP_CTRL` writer - 1:start or run 0:stop or end
pub type EzipCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1:start or run 0:stop or end
    #[inline(always)]
    pub fn ezip_ctrl(&self) -> EzipCtrlR {
        EzipCtrlR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EZIP_CTRL")
            .field("ezip_ctrl", &self.ezip_ctrl())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1:start or run 0:stop or end
    #[inline(always)]
    pub fn ezip_ctrl(&mut self) -> EzipCtrlW<EZIP_CTRLrs> {
        EzipCtrlW::new(self, 0)
    }
}
///ezip/aezip_frame decoder ctrl
///
///You can [`read`](crate::Reg::read) this register and get [`ezip_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ezip_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EZIP_CTRLrs;
impl crate::RegisterSpec for EZIP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ezip_ctrl::R`](R) reader structure
impl crate::Readable for EZIP_CTRLrs {}
///`write(|w| ..)` method takes [`ezip_ctrl::W`](W) writer structure
impl crate::Writable for EZIP_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EZIP_CTRL to value 0
impl crate::Resettable for EZIP_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
