///Register `HXT_CR3` reader
pub type R = crate::R<HXT_CR3rs>;
///Register `HXT_CR3` writer
pub type W = crate::W<HXT_CR3rs>;
///Field `BUF_DAC_STR` reader -
pub type BufDacStrR = crate::FieldReader;
///Field `BUF_DAC_STR` writer -
pub type BufDacStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUF_OSLO_STR` reader -
pub type BufOsloStrR = crate::FieldReader;
///Field `BUF_OSLO_STR` writer -
pub type BufOsloStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DLY` reader -
pub type DlyR = crate::FieldReader;
///Field `DLY` writer -
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn buf_dac_str(&self) -> BufDacStrR {
        BufDacStrR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn buf_oslo_str(&self) -> BufOsloStrR {
        BufOsloStrR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:9
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HXT_CR3")
            .field("dly", &self.dly())
            .field("buf_oslo_str", &self.buf_oslo_str())
            .field("buf_dac_str", &self.buf_dac_str())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    pub fn buf_dac_str(&mut self) -> BufDacStrW<HXT_CR3rs> {
        BufDacStrW::new(self, 0)
    }
    ///Bits 2:3
    #[inline(always)]
    pub fn buf_oslo_str(&mut self) -> BufOsloStrW<HXT_CR3rs> {
        BufOsloStrW::new(self, 2)
    }
    ///Bits 4:9
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<HXT_CR3rs> {
        DlyW::new(self, 4)
    }
}
///HXT48 Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`hxt_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hxt_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HXT_CR3rs;
impl crate::RegisterSpec for HXT_CR3rs {
    type Ux = u32;
}
///`read()` method returns [`hxt_cr3::R`](R) reader structure
impl crate::Readable for HXT_CR3rs {}
///`write(|w| ..)` method takes [`hxt_cr3::W`](W) writer structure
impl crate::Writable for HXT_CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HXT_CR3 to value 0
impl crate::Resettable for HXT_CR3rs {
    const RESET_VALUE: u32 = 0;
}
