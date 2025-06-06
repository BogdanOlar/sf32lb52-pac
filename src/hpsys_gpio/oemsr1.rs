///Register `OEMSR1` reader
pub type R = crate::R<OEMSR1rs>;
///Register `OEMSR1` writer
pub type W = crate::W<OEMSR1rs>;
///Field `OEMS` reader - output mode Set of corresponding GPIO\[44:32\]
pub type OemsR = crate::FieldReader<u16>;
///Field `OEMS` writer - output mode Set of corresponding GPIO\[44:32\]
pub type OemsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - output mode Set of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn oems(&self) -> OemsR {
        OemsR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEMSR1")
            .field("rsvd", &self.rsvd())
            .field("oems", &self.oems())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - output mode Set of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn oems(&mut self) -> OemsW<OEMSR1rs> {
        OemsW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<OEMSR1rs> {
        RsvdW::new(self, 13)
    }
}
///output mode Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OEMSR1rs;
impl crate::RegisterSpec for OEMSR1rs {
    type Ux = u32;
}
///`read()` method returns [`oemsr1::R`](R) reader structure
impl crate::Readable for OEMSR1rs {}
///`write(|w| ..)` method takes [`oemsr1::W`](W) writer structure
impl crate::Writable for OEMSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OEMSR1 to value 0
impl crate::Resettable for OEMSR1rs {
    const RESET_VALUE: u32 = 0;
}
