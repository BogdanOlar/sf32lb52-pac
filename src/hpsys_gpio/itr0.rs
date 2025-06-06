///Register `ITR0` reader
pub type R = crate::R<ITR0rs>;
///Register `ITR0` writer
pub type W = crate::W<ITR0rs>;
///Field `ITR` reader - GPIO\[31:0\]
///interrupt type
pub type ItrR = crate::FieldReader<u32>;
///Field `ITR` writer - GPIO\[31:0\]
///interrupt type
pub type ItrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO\[31:0\]
    ///interrupt type
    #[inline(always)]
    pub fn itr(&self) -> ItrR {
        ItrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITR0").field("itr", &self.itr()).finish()
    }
}
impl W {
    ///Bits 0:31 - GPIO\[31:0\]
    ///interrupt type
    #[inline(always)]
    pub fn itr(&mut self) -> ItrW<ITR0rs> {
        ItrW::new(self, 0)
    }
}
///Interrupt Type Register
///
///You can [`read`](crate::Reg::read) this register and get [`itr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ITR0rs;
impl crate::RegisterSpec for ITR0rs {
    type Ux = u32;
}
///`read()` method returns [`itr0::R`](R) reader structure
impl crate::Readable for ITR0rs {}
///`write(|w| ..)` method takes [`itr0::W`](W) writer structure
impl crate::Writable for ITR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITR0 to value 0
impl crate::Resettable for ITR0rs {
    const RESET_VALUE: u32 = 0;
}
