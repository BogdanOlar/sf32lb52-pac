///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RESET` reader - This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type ResetR = crate::BitReader;
///Field `RESET` writer - This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATASIZE` reader - Valid input data size These bits control the valid size of the input data. 00: lower 8-bit 01: lower 16-bit 10: lower 24-bit 11: all 32-bit
pub type DatasizeR = crate::FieldReader;
///Field `DATASIZE` writer - Valid input data size These bits control the valid size of the input data. 00: lower 8-bit 01: lower 16-bit 10: lower 24-bit 11: all 32-bit
pub type DatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial. 00: 32 bit polynomial 01: 16 bit polynomial 10: 8 bit polynomial 11: 7 bit polynomial
pub type PolysizeR = crate::FieldReader;
///Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial. 00: 32 bit polynomial 01: 16 bit polynomial 10: 8 bit polynomial 11: 7 bit polynomial
pub type PolysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REV_IN` reader - Reverse input data These bits control the reversal of the bit order of the input data 00: Bit order not affected 01: Bit reversal done by byte 10: Bit reversal done by half-word 11: Bit reversal done by word
pub type RevInR = crate::FieldReader;
///Field `REV_IN` writer - Reverse input data These bits control the reversal of the bit order of the input data 00: Bit order not affected 01: Bit reversal done by byte 10: Bit reversal done by half-word 11: Bit reversal done by word
pub type RevInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data. 0: Bit order not affected 1: Bit-reversed output format
pub type RevOutR = crate::BitReader;
///Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data. 0: Bit order not affected 1: Bit-reversed output format
pub type RevOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Valid input data size These bits control the valid size of the input data. 00: lower 8-bit 01: lower 16-bit 10: lower 24-bit 11: all 32-bit
    #[inline(always)]
    pub fn datasize(&self) -> DatasizeR {
        DatasizeR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial. 00: 32 bit polynomial 01: 16 bit polynomial 10: 8 bit polynomial 11: 7 bit polynomial
    #[inline(always)]
    pub fn polysize(&self) -> PolysizeR {
        PolysizeR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data 00: Bit order not affected 01: Bit reversal done by byte 10: Bit reversal done by half-word 11: Bit reversal done by word
    #[inline(always)]
    pub fn rev_in(&self) -> RevInR {
        RevInR::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data. 0: Bit order not affected 1: Bit-reversed output format
    #[inline(always)]
    pub fn rev_out(&self) -> RevOutR {
        RevOutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rev_out", &self.rev_out())
            .field("rev_in", &self.rev_in())
            .field("polysize", &self.polysize())
            .field("datasize", &self.datasize())
            .field("reset", &self.reset())
            .finish()
    }
}
impl W {
    ///Bit 0 - This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<CRrs> {
        ResetW::new(self, 0)
    }
    ///Bits 1:2 - Valid input data size These bits control the valid size of the input data. 00: lower 8-bit 01: lower 16-bit 10: lower 24-bit 11: all 32-bit
    #[inline(always)]
    pub fn datasize(&mut self) -> DatasizeW<CRrs> {
        DatasizeW::new(self, 1)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial. 00: 32 bit polynomial 01: 16 bit polynomial 10: 8 bit polynomial 11: 7 bit polynomial
    #[inline(always)]
    pub fn polysize(&mut self) -> PolysizeW<CRrs> {
        PolysizeW::new(self, 3)
    }
    ///Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data 00: Bit order not affected 01: Bit reversal done by byte 10: Bit reversal done by half-word 11: Bit reversal done by word
    #[inline(always)]
    pub fn rev_in(&mut self) -> RevInW<CRrs> {
        RevInW::new(self, 5)
    }
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data. 0: Bit order not affected 1: Bit-reversed output format
    #[inline(always)]
    pub fn rev_out(&mut self) -> RevOutW<CRrs> {
        RevOutW::new(self, 7)
    }
}
///Control register
///
///You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
