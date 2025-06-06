///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `FRAC` reader - Fractional part of baud rate prescaler
pub type FracR = crate::FieldReader;
///Field `FRAC` writer - Fractional part of baud rate prescaler
pub type FracW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INT` reader - Integer part of baud rate prescaler If OVER8 = 0, Baud Rate = 48000000 / (INT + FRAC/16) / 16 If OVER8 = 1, Baud Rate = 48000000 / (INT + FRAC/16) / 8 For example: OVER=0, INT=3, FRAC=0, Baud Rate = 48000000/(3+0)/16 = 1Mbps OVER=0, INT=3, FRAC=4, Baud Rate = 48000000/(3+4/16)/16 = 923077 = 921600 + 1.6‰ OVER=1, INT=52, FRAC=1, Baud Rate = 48000000/(52+1/16)/8 = 115246 = 115200 + 0.4‰
pub type IntR = crate::FieldReader<u16>;
///Field `INT` writer - Integer part of baud rate prescaler If OVER8 = 0, Baud Rate = 48000000 / (INT + FRAC/16) / 16 If OVER8 = 1, Baud Rate = 48000000 / (INT + FRAC/16) / 8 For example: OVER=0, INT=3, FRAC=0, Baud Rate = 48000000/(3+0)/16 = 1Mbps OVER=0, INT=3, FRAC=4, Baud Rate = 48000000/(3+4/16)/16 = 923077 = 921600 + 1.6‰ OVER=1, INT=52, FRAC=1, Baud Rate = 48000000/(52+1/16)/8 = 115246 = 115200 + 0.4‰
pub type IntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:3 - Fractional part of baud rate prescaler
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:15 - Integer part of baud rate prescaler If OVER8 = 0, Baud Rate = 48000000 / (INT + FRAC/16) / 16 If OVER8 = 1, Baud Rate = 48000000 / (INT + FRAC/16) / 8 For example: OVER=0, INT=3, FRAC=0, Baud Rate = 48000000/(3+0)/16 = 1Mbps OVER=0, INT=3, FRAC=4, Baud Rate = 48000000/(3+4/16)/16 = 923077 = 921600 + 1.6‰ OVER=1, INT=52, FRAC=1, Baud Rate = 48000000/(52+1/16)/8 = 115246 = 115200 + 0.4‰
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR")
            .field("int", &self.int())
            .field("frac", &self.frac())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Fractional part of baud rate prescaler
    #[inline(always)]
    pub fn frac(&mut self) -> FracW<BRRrs> {
        FracW::new(self, 0)
    }
    ///Bits 4:15 - Integer part of baud rate prescaler If OVER8 = 0, Baud Rate = 48000000 / (INT + FRAC/16) / 16 If OVER8 = 1, Baud Rate = 48000000 / (INT + FRAC/16) / 8 For example: OVER=0, INT=3, FRAC=0, Baud Rate = 48000000/(3+0)/16 = 1Mbps OVER=0, INT=3, FRAC=4, Baud Rate = 48000000/(3+4/16)/16 = 923077 = 921600 + 1.6‰ OVER=1, INT=52, FRAC=1, Baud Rate = 48000000/(52+1/16)/8 = 115246 = 115200 + 0.4‰
    #[inline(always)]
    pub fn int(&mut self) -> IntW<BRRrs> {
        IntW::new(self, 4)
    }
}
///Baud Rate Register
///
///You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
