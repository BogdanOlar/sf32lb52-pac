///Register `INT_MSK` reader
pub type R = crate::R<INT_MSKrs>;
///Register `INT_MSK` writer
pub type W = crate::W<INT_MSKrs>;
///Field `INT_MASK_R` reader - 1:disable right channel irq to system; 0: enable right channel irq to system
pub type IntMaskRR = crate::BitReader;
///Field `INT_MASK_R` writer - 1:disable right channel irq to system; 0: enable right channel irq to system
pub type IntMaskRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_MASK_L` reader - 1:disable left channel irq to system; 0: enable left channel irq to system
pub type IntMaskLR = crate::BitReader;
///Field `INT_MASK_L` writer - 1:disable left channel irq to system; 0: enable left channel irq to system
pub type IntMaskLW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1:disable right channel irq to system; 0: enable right channel irq to system
    #[inline(always)]
    pub fn int_mask_r(&self) -> IntMaskRR {
        IntMaskRR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1:disable left channel irq to system; 0: enable left channel irq to system
    #[inline(always)]
    pub fn int_mask_l(&self) -> IntMaskLR {
        IntMaskLR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MSK")
            .field("int_mask_l", &self.int_mask_l())
            .field("int_mask_r", &self.int_mask_r())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1:disable right channel irq to system; 0: enable right channel irq to system
    #[inline(always)]
    pub fn int_mask_r(&mut self) -> IntMaskRW<INT_MSKrs> {
        IntMaskRW::new(self, 0)
    }
    ///Bit 1 - 1:disable left channel irq to system; 0: enable left channel irq to system
    #[inline(always)]
    pub fn int_mask_l(&mut self) -> IntMaskLW<INT_MSKrs> {
        IntMaskLW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`int_msk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_MSKrs;
impl crate::RegisterSpec for INT_MSKrs {
    type Ux = u32;
}
///`read()` method returns [`int_msk::R`](R) reader structure
impl crate::Readable for INT_MSKrs {}
///`write(|w| ..)` method takes [`int_msk::W`](W) writer structure
impl crate::Writable for INT_MSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_MSK to value 0
impl crate::Resettable for INT_MSKrs {
    const RESET_VALUE: u32 = 0;
}
