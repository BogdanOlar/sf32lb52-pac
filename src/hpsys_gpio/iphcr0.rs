///Register `IPHCR0` reader
pub type R = crate::R<IPHCR0rs>;
///Register `IPHCR0` writer
pub type W = crate::W<IPHCR0rs>;
///Field `IPHC` reader - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
pub type IphcR = crate::FieldReader<u32>;
///Field `IPHC` writer - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
pub type IphcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iphc(&self) -> IphcR {
        IphcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPHCR0")
            .field("iphc", &self.iphc())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iphc(&mut self) -> IphcW<IPHCR0rs> {
        IphcW::new(self, 0)
    }
}
///Interrupt Polarity High Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPHCR0rs;
impl crate::RegisterSpec for IPHCR0rs {
    type Ux = u32;
}
///`read()` method returns [`iphcr0::R`](R) reader structure
impl crate::Readable for IPHCR0rs {}
///`write(|w| ..)` method takes [`iphcr0::W`](W) writer structure
impl crate::Writable for IPHCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPHCR0 to value 0
impl crate::Resettable for IPHCR0rs {
    const RESET_VALUE: u32 = 0;
}
