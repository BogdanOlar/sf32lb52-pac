///Register `OEMSR0` reader
pub type R = crate::R<OEMSR0rs>;
///Register `OEMSR0` writer
pub type W = crate::W<OEMSR0rs>;
///Field `OEMS` reader - output mode Set of corresponding GPIO\[31:0\]
pub type OemsR = crate::FieldReader<u32>;
///Field `OEMS` writer - output mode Set of corresponding GPIO\[31:0\]
pub type OemsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - output mode Set of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn oems(&self) -> OemsR {
        OemsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEMSR0")
            .field("oems", &self.oems())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - output mode Set of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn oems(&mut self) -> OemsW<OEMSR0rs> {
        OemsW::new(self, 0)
    }
}
///output mode Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OEMSR0rs;
impl crate::RegisterSpec for OEMSR0rs {
    type Ux = u32;
}
///`read()` method returns [`oemsr0::R`](R) reader structure
impl crate::Readable for OEMSR0rs {}
///`write(|w| ..)` method takes [`oemsr0::W`](W) writer structure
impl crate::Writable for OEMSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OEMSR0 to value 0
impl crate::Resettable for OEMSR0rs {
    const RESET_VALUE: u32 = 0;
}
