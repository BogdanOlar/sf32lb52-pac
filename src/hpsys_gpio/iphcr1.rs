///Register `IPHCR1` reader
pub type R = crate::R<IPHCR1rs>;
///Register `IPHCR1` writer
pub type W = crate::W<IPHCR1rs>;
///Field `IPHC` reader - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
pub type IphcR = crate::FieldReader<u16>;
///Field `IPHC` writer - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
pub type IphcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iphc(&self) -> IphcR {
        IphcR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPHCR1")
            .field("iphc", &self.iphc())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iphc(&mut self) -> IphcW<IPHCR1rs> {
        IphcW::new(self, 0)
    }
}
///Interrupt Polarity High Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPHCR1rs;
impl crate::RegisterSpec for IPHCR1rs {
    type Ux = u32;
}
///`read()` method returns [`iphcr1::R`](R) reader structure
impl crate::Readable for IPHCR1rs {}
///`write(|w| ..)` method takes [`iphcr1::W`](W) writer structure
impl crate::Writable for IPHCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPHCR1 to value 0
impl crate::Resettable for IPHCR1rs {
    const RESET_VALUE: u32 = 0;
}
