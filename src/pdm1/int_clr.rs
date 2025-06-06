///Register `INT_CLR` reader
pub type R = crate::R<INT_CLRrs>;
///Register `INT_CLR` writer
pub type W = crate::W<INT_CLRrs>;
///Field `INT_CLR_R` reader - clear right channel irq
pub type IntClrRR = crate::BitReader;
///Field `INT_CLR_R` writer - clear right channel irq
pub type IntClrRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_CLR_L` reader - clear left channel irq
pub type IntClrLR = crate::BitReader;
///Field `INT_CLR_L` writer - clear left channel irq
pub type IntClrLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - clear right channel irq
    #[inline(always)]
    pub fn int_clr_r(&self) -> IntClrRR {
        IntClrRR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - clear left channel irq
    #[inline(always)]
    pub fn int_clr_l(&self) -> IntClrLR {
        IntClrLR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR")
            .field("rsvd", &self.rsvd())
            .field("int_clr_l", &self.int_clr_l())
            .field("int_clr_r", &self.int_clr_r())
            .finish()
    }
}
impl W {
    ///Bit 0 - clear right channel irq
    #[inline(always)]
    pub fn int_clr_r(&mut self) -> IntClrRW<INT_CLRrs> {
        IntClrRW::new(self, 0)
    }
    ///Bit 1 - clear left channel irq
    #[inline(always)]
    pub fn int_clr_l(&mut self) -> IntClrLW<INT_CLRrs> {
        IntClrLW::new(self, 1)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<INT_CLRrs> {
        RsvdW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_CLRrs;
impl crate::RegisterSpec for INT_CLRrs {
    type Ux = u32;
}
///`read()` method returns [`int_clr::R`](R) reader structure
impl crate::Readable for INT_CLRrs {}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLRrs {
    const RESET_VALUE: u32 = 0;
}
