///Register `DOECR0` reader
pub type R = crate::R<DOECR0rs>;
///Register `DOECR0` writer
pub type W = crate::W<DOECR0rs>;
///Field `DOEC` reader - set 1 to disable output of corresponding GPIO\[31:0\]
pub type DoecR = crate::FieldReader<u32>;
///Field `DOEC` writer - set 1 to disable output of corresponding GPIO\[31:0\]
pub type DoecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 to disable output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn doec(&self) -> DoecR {
        DoecR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOECR0")
            .field("doec", &self.doec())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 to disable output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn doec(&mut self) -> DoecW<DOECR0rs> {
        DoecW::new(self, 0)
    }
}
///Data Output Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`doecr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doecr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOECR0rs;
impl crate::RegisterSpec for DOECR0rs {
    type Ux = u32;
}
///`read()` method returns [`doecr0::R`](R) reader structure
impl crate::Readable for DOECR0rs {}
///`write(|w| ..)` method takes [`doecr0::W`](W) writer structure
impl crate::Writable for DOECR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOECR0 to value 0
impl crate::Resettable for DOECR0rs {
    const RESET_VALUE: u32 = 0;
}
