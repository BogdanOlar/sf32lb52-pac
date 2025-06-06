///Register `CFG0` reader
pub type R = crate::R<CFG0rs>;
///Register `CFG0` writer
pub type W = crate::W<CFG0rs>;
///Field `PDMCOREEN` reader - 1:Enable pdm module; 0: Disable pdm module
pub type PdmcoreenR = crate::BitReader;
///Field `PDMCOREEN` writer - 1:Enable pdm module; 0: Disable pdm module
pub type PdmcoreenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_SEL` reader - 1:Clk select dll 3.072MHz; 0: Clk selct xtal 9.6MHz
pub type ClkSelR = crate::BitReader;
///Field `CLK_SEL` writer - 1:Clk select dll 3.072MHz; 0: Clk selct xtal 9.6MHz
pub type ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DIV` reader - Clock frequency division ratio of 3.072MHz or 9.6MHz according to register clk_sel
pub type ClkDivR = crate::FieldReader;
///Field `CLK_DIV` writer - Clock frequency division ratio of 3.072MHz or 9.6MHz according to register clk_sel
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LEFT_EN` reader - 1: Enable left channel pdm data sampling; 0: Disable left channel pdm data sampling
pub type LeftEnR = crate::BitReader;
///Field `LEFT_EN` writer - 1: Enable left channel pdm data sampling; 0: Disable left channel pdm data sampling
pub type LeftEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIGHT_EN` reader - 1: Enable right channel pdm data sampling; 0: Disable right channel pdm data sampling
pub type RightEnR = crate::BitReader;
///Field `RIGHT_EN` writer - 1: Enable right channel pdm data sampling; 0: Disable right channel pdm data sampling
pub type RightEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STEREO_EN` reader - 1:Enable double channels pdm data sampling; 0: Disable double channels pdm data sampling
pub type StereoEnR = crate::BitReader;
///Field `STEREO_EN` writer - 1:Enable double channels pdm data sampling; 0: Disable double channels pdm data sampling
pub type StereoEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_EN` reader - 1: Swap right channel and left channel pdm data; 0: Not swap right channel and left channel pdm data
pub type SwapEnR = crate::BitReader;
///Field `SWAP_EN` writer - 1: Swap right channel and left channel pdm data; 0: Not swap right channel and left channel pdm data
pub type SwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bit 0 - 1:Enable pdm module; 0: Disable pdm module
    #[inline(always)]
    pub fn pdmcoreen(&self) -> PdmcoreenR {
        PdmcoreenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1:Clk select dll 3.072MHz; 0: Clk selct xtal 9.6MHz
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Clock frequency division ratio of 3.072MHz or 9.6MHz according to register clk_sel
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - 1: Enable left channel pdm data sampling; 0: Disable left channel pdm data sampling
    #[inline(always)]
    pub fn left_en(&self) -> LeftEnR {
        LeftEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1: Enable right channel pdm data sampling; 0: Disable right channel pdm data sampling
    #[inline(always)]
    pub fn right_en(&self) -> RightEnR {
        RightEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1:Enable double channels pdm data sampling; 0: Disable double channels pdm data sampling
    #[inline(always)]
    pub fn stereo_en(&self) -> StereoEnR {
        StereoEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 1: Swap right channel and left channel pdm data; 0: Not swap right channel and left channel pdm data
    #[inline(always)]
    pub fn swap_en(&self) -> SwapEnR {
        SwapEnR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG0")
            .field("rsvd", &self.rsvd())
            .field("swap_en", &self.swap_en())
            .field("stereo_en", &self.stereo_en())
            .field("right_en", &self.right_en())
            .field("left_en", &self.left_en())
            .field("clk_div", &self.clk_div())
            .field("clk_sel", &self.clk_sel())
            .field("pdmcoreen", &self.pdmcoreen())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1:Enable pdm module; 0: Disable pdm module
    #[inline(always)]
    pub fn pdmcoreen(&mut self) -> PdmcoreenW<CFG0rs> {
        PdmcoreenW::new(self, 0)
    }
    ///Bit 1 - 1:Clk select dll 3.072MHz; 0: Clk selct xtal 9.6MHz
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<CFG0rs> {
        ClkSelW::new(self, 1)
    }
    ///Bits 2:5 - Clock frequency division ratio of 3.072MHz or 9.6MHz according to register clk_sel
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<CFG0rs> {
        ClkDivW::new(self, 2)
    }
    ///Bit 6 - 1: Enable left channel pdm data sampling; 0: Disable left channel pdm data sampling
    #[inline(always)]
    pub fn left_en(&mut self) -> LeftEnW<CFG0rs> {
        LeftEnW::new(self, 6)
    }
    ///Bit 7 - 1: Enable right channel pdm data sampling; 0: Disable right channel pdm data sampling
    #[inline(always)]
    pub fn right_en(&mut self) -> RightEnW<CFG0rs> {
        RightEnW::new(self, 7)
    }
    ///Bit 8 - 1:Enable double channels pdm data sampling; 0: Disable double channels pdm data sampling
    #[inline(always)]
    pub fn stereo_en(&mut self) -> StereoEnW<CFG0rs> {
        StereoEnW::new(self, 8)
    }
    ///Bit 9 - 1: Swap right channel and left channel pdm data; 0: Not swap right channel and left channel pdm data
    #[inline(always)]
    pub fn swap_en(&mut self) -> SwapEnW<CFG0rs> {
        SwapEnW::new(self, 9)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CFG0rs> {
        RsvdW::new(self, 10)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFG0rs;
impl crate::RegisterSpec for CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`cfg0::R`](R) reader structure
impl crate::Readable for CFG0rs {}
///`write(|w| ..)` method takes [`cfg0::W`](W) writer structure
impl crate::Writable for CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG0 to value 0
impl crate::Resettable for CFG0rs {
    const RESET_VALUE: u32 = 0;
}
