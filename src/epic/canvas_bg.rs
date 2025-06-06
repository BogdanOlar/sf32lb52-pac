///Register `CANVAS_BG` reader
pub type R = crate::R<CANVAS_BGrs>;
///Register `CANVAS_BG` writer
pub type W = crate::W<CANVAS_BGrs>;
///Field `BLUE` reader - blue color
pub type BlueR = crate::FieldReader;
///Field `BLUE` writer - blue color
pub type BlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` reader - green color
pub type GreenR = crate::FieldReader;
///Field `GREEN` writer - green color
pub type GreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` reader - Red color
pub type RedR = crate::FieldReader;
///Field `RED` writer - Red color
pub type RedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BG_BLENDING_BYPASS` reader - if this bit is set, the layer is not blending with background. The alpha value will be reserved to output.
pub type BgBlendingBypassR = crate::BitReader;
///Field `BG_BLENDING_BYPASS` writer - if this bit is set, the layer is not blending with background. The alpha value will be reserved to output.
pub type BgBlendingBypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALL_BLENDING_BYPASS` reader - if this bit is set, epic is in pure dma mode. No blending operation.
pub type AllBlendingBypassR = crate::BitReader;
///Field `ALL_BLENDING_BYPASS` writer - if this bit is set, epic is in pure dma mode. No blending operation.
pub type AllBlendingBypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:7 - blue color
    #[inline(always)]
    pub fn blue(&self) -> BlueR {
        BlueR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - green color
    #[inline(always)]
    pub fn green(&self) -> GreenR {
        GreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red color
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - if this bit is set, the layer is not blending with background. The alpha value will be reserved to output.
    #[inline(always)]
    pub fn bg_blending_bypass(&self) -> BgBlendingBypassR {
        BgBlendingBypassR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - if this bit is set, epic is in pure dma mode. No blending operation.
    #[inline(always)]
    pub fn all_blending_bypass(&self) -> AllBlendingBypassR {
        AllBlendingBypassR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANVAS_BG")
            .field("rsvd", &self.rsvd())
            .field("all_blending_bypass", &self.all_blending_bypass())
            .field("bg_blending_bypass", &self.bg_blending_bypass())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - blue color
    #[inline(always)]
    pub fn blue(&mut self) -> BlueW<CANVAS_BGrs> {
        BlueW::new(self, 0)
    }
    ///Bits 8:15 - green color
    #[inline(always)]
    pub fn green(&mut self) -> GreenW<CANVAS_BGrs> {
        GreenW::new(self, 8)
    }
    ///Bits 16:23 - Red color
    #[inline(always)]
    pub fn red(&mut self) -> RedW<CANVAS_BGrs> {
        RedW::new(self, 16)
    }
    ///Bit 24 - if this bit is set, the layer is not blending with background. The alpha value will be reserved to output.
    #[inline(always)]
    pub fn bg_blending_bypass(&mut self) -> BgBlendingBypassW<CANVAS_BGrs> {
        BgBlendingBypassW::new(self, 24)
    }
    ///Bit 25 - if this bit is set, epic is in pure dma mode. No blending operation.
    #[inline(always)]
    pub fn all_blending_bypass(&mut self) -> AllBlendingBypassW<CANVAS_BGrs> {
        AllBlendingBypassW::new(self, 25)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CANVAS_BGrs> {
        RsvdW::new(self, 26)
    }
}
///Background color
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_bg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_bg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CANVAS_BGrs;
impl crate::RegisterSpec for CANVAS_BGrs {
    type Ux = u32;
}
///`read()` method returns [`canvas_bg::R`](R) reader structure
impl crate::Readable for CANVAS_BGrs {}
///`write(|w| ..)` method takes [`canvas_bg::W`](W) writer structure
impl crate::Writable for CANVAS_BGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CANVAS_BG to value 0
impl crate::Resettable for CANVAS_BGrs {
    const RESET_VALUE: u32 = 0;
}
