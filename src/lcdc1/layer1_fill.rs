///Register `LAYER1_FILL` reader
pub type R = crate::R<LAYER1_FILLrs>;
///Register `LAYER1_FILL` writer
pub type W = crate::W<LAYER1_FILLrs>;
///Field `BG_B` reader - background b color
pub type BgBR = crate::FieldReader;
///Field `BG_B` writer - background b color
pub type BgBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BG_G` reader - background g color
pub type BgGR = crate::FieldReader;
///Field `BG_G` writer - background g color
pub type BgGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BG_R` reader - background r color
pub type BgRR = crate::FieldReader;
///Field `BG_R` writer - background r color
pub type BgRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BG_MODE` reader - not used
pub type BgModeR = crate::BitReader;
///Field `BG_MODE` writer - not used
pub type BgModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENDIAN` reader - input 565 data format endian 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
pub type EndianR = crate::BitReader;
///Field `ENDIAN` writer - input 565 data format endian 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - background b color
    #[inline(always)]
    pub fn bg_b(&self) -> BgBR {
        BgBR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - background g color
    #[inline(always)]
    pub fn bg_g(&self) -> BgGR {
        BgGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - background r color
    #[inline(always)]
    pub fn bg_r(&self) -> BgRR {
        BgRR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - not used
    #[inline(always)]
    pub fn bg_mode(&self) -> BgModeR {
        BgModeR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - input 565 data format endian 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER1_FILL")
            .field("endian", &self.endian())
            .field("bg_mode", &self.bg_mode())
            .field("bg_r", &self.bg_r())
            .field("bg_g", &self.bg_g())
            .field("bg_b", &self.bg_b())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - background b color
    #[inline(always)]
    pub fn bg_b(&mut self) -> BgBW<LAYER1_FILLrs> {
        BgBW::new(self, 0)
    }
    ///Bits 8:15 - background g color
    #[inline(always)]
    pub fn bg_g(&mut self) -> BgGW<LAYER1_FILLrs> {
        BgGW::new(self, 8)
    }
    ///Bits 16:23 - background r color
    #[inline(always)]
    pub fn bg_r(&mut self) -> BgRW<LAYER1_FILLrs> {
        BgRW::new(self, 16)
    }
    ///Bit 24 - not used
    #[inline(always)]
    pub fn bg_mode(&mut self) -> BgModeW<LAYER1_FILLrs> {
        BgModeW::new(self, 24)
    }
    ///Bit 25 - input 565 data format endian 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
    #[inline(always)]
    pub fn endian(&mut self) -> EndianW<LAYER1_FILLrs> {
        EndianW::new(self, 25)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer1_fill::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1_fill::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER1_FILLrs;
impl crate::RegisterSpec for LAYER1_FILLrs {
    type Ux = u32;
}
///`read()` method returns [`layer1_fill::R`](R) reader structure
impl crate::Readable for LAYER1_FILLrs {}
///`write(|w| ..)` method takes [`layer1_fill::W`](W) writer structure
impl crate::Writable for LAYER1_FILLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER1_FILL to value 0
impl crate::Resettable for LAYER1_FILLrs {
    const RESET_VALUE: u32 = 0;
}
