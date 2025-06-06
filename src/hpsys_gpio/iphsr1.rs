///Register `IPHSR1` reader
pub type R = crate::R<IPHSR1rs>;
///Register `IPHSR1` writer
pub type W = crate::W<IPHSR1rs>;
///Field `IPHS` reader - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
pub type IphsR = crate::FieldReader<u16>;
///Field `IPHS` writer - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
pub type IphsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iphs(&self) -> IphsR {
        IphsR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPHSR1")
            .field("iphs", &self.iphs())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iphs(&mut self) -> IphsW<IPHSR1rs> {
        IphsW::new(self, 0)
    }
}
///Interrupt Polarity High Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPHSR1rs;
impl crate::RegisterSpec for IPHSR1rs {
    type Ux = u32;
}
///`read()` method returns [`iphsr1::R`](R) reader structure
impl crate::Readable for IPHSR1rs {}
///`write(|w| ..)` method takes [`iphsr1::W`](W) writer structure
impl crate::Writable for IPHSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPHSR1 to value 0
impl crate::Resettable for IPHSR1rs {
    const RESET_VALUE: u32 = 0;
}
