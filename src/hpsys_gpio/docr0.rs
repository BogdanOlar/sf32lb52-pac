///Register `DOCR0` reader
pub type R = crate::R<DOCR0rs>;
///Register `DOCR0` writer
pub type W = crate::W<DOCR0rs>;
///Field `DOC` reader - set 1 to pull down output of corresponding GPIO\[31:0\]
pub type DocR = crate::FieldReader<u32>;
///Field `DOC` writer - set 1 to pull down output of corresponding GPIO\[31:0\]
pub type DocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 to pull down output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn doc(&self) -> DocR {
        DocR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOCR0").field("doc", &self.doc()).finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 to pull down output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn doc(&mut self) -> DocW<DOCR0rs> {
        DocW::new(self, 0)
    }
}
///Data Output Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`docr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOCR0rs;
impl crate::RegisterSpec for DOCR0rs {
    type Ux = u32;
}
///`read()` method returns [`docr0::R`](R) reader structure
impl crate::Readable for DOCR0rs {}
///`write(|w| ..)` method takes [`docr0::W`](W) writer structure
impl crate::Writable for DOCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOCR0 to value 0
impl crate::Resettable for DOCR0rs {
    const RESET_VALUE: u32 = 0;
}
