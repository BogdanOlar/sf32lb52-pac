///Register `ITR1` reader
pub type R = crate::R<ITR1rs>;
///Register `ITR1` writer
pub type W = crate::W<ITR1rs>;
///Field `ITR` reader - GPIO\[44:32\]
///interrupt type
pub type ItrR = crate::FieldReader<u16>;
///Field `ITR` writer - GPIO\[44:32\]
///interrupt type
pub type ItrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - GPIO\[44:32\]
    ///interrupt type
    #[inline(always)]
    pub fn itr(&self) -> ItrR {
        ItrR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITR1")
            .field("rsvd", &self.rsvd())
            .field("itr", &self.itr())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - GPIO\[44:32\]
    ///interrupt type
    #[inline(always)]
    pub fn itr(&mut self) -> ItrW<ITR1rs> {
        ItrW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ITR1rs> {
        RsvdW::new(self, 13)
    }
}
///Interrupt Type Register
///
///You can [`read`](crate::Reg::read) this register and get [`itr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ITR1rs;
impl crate::RegisterSpec for ITR1rs {
    type Ux = u32;
}
///`read()` method returns [`itr1::R`](R) reader structure
impl crate::Readable for ITR1rs {}
///`write(|w| ..)` method takes [`itr1::W`](W) writer structure
impl crate::Writable for ITR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITR1 to value 0
impl crate::Resettable for ITR1rs {
    const RESET_VALUE: u32 = 0;
}
