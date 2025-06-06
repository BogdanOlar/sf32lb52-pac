///Register `DITHER_CONF` reader
pub type R = crate::R<DITHER_CONFrs>;
///Register `DITHER_CONF` writer
pub type W = crate::W<DITHER_CONFrs>;
///Field `EN` reader - dither enable
pub type EnR = crate::BitReader;
///Field `EN` writer - dither enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `W_B` reader - blue dither width
pub type WBR = crate::FieldReader;
///Field `W_B` writer - blue dither width
pub type WBW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `W_G` reader - green dither width
pub type WGR = crate::FieldReader;
///Field `W_G` writer - green dither width
pub type WGW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `W_R` reader - red dither width
pub type WRR = crate::FieldReader;
///Field `W_R` writer - red dither width
pub type WRW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LFSR_LOAD_SEL` reader - select lfsr 0: none 1: red 2: green 3: blue
pub type LfsrLoadSelR = crate::FieldReader;
///Field `LFSR_LOAD_SEL` writer - select lfsr 0: none 1: red 2: green 3: blue
pub type LfsrLoadSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LFSR_LOAD` reader - load lfsr init value
pub type LfsrLoadR = crate::BitReader;
///Field `LFSR_LOAD` writer - load lfsr init value
pub type LfsrLoadW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bit 0 - dither enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - blue dither width
    #[inline(always)]
    pub fn w_b(&self) -> WBR {
        WBR::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:6 - green dither width
    #[inline(always)]
    pub fn w_g(&self) -> WGR {
        WGR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - red dither width
    #[inline(always)]
    pub fn w_r(&self) -> WRR {
        WRR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:11 - select lfsr 0: none 1: red 2: green 3: blue
    #[inline(always)]
    pub fn lfsr_load_sel(&self) -> LfsrLoadSelR {
        LfsrLoadSelR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - load lfsr init value
    #[inline(always)]
    pub fn lfsr_load(&self) -> LfsrLoadR {
        LfsrLoadR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DITHER_CONF")
            .field("rsvd", &self.rsvd())
            .field("lfsr_load", &self.lfsr_load())
            .field("lfsr_load_sel", &self.lfsr_load_sel())
            .field("w_r", &self.w_r())
            .field("w_g", &self.w_g())
            .field("w_b", &self.w_b())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - dither enable
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DITHER_CONFrs> {
        EnW::new(self, 0)
    }
    ///Bits 1:3 - blue dither width
    #[inline(always)]
    pub fn w_b(&mut self) -> WBW<DITHER_CONFrs> {
        WBW::new(self, 1)
    }
    ///Bits 4:6 - green dither width
    #[inline(always)]
    pub fn w_g(&mut self) -> WGW<DITHER_CONFrs> {
        WGW::new(self, 4)
    }
    ///Bits 7:9 - red dither width
    #[inline(always)]
    pub fn w_r(&mut self) -> WRW<DITHER_CONFrs> {
        WRW::new(self, 7)
    }
    ///Bits 10:11 - select lfsr 0: none 1: red 2: green 3: blue
    #[inline(always)]
    pub fn lfsr_load_sel(&mut self) -> LfsrLoadSelW<DITHER_CONFrs> {
        LfsrLoadSelW::new(self, 10)
    }
    ///Bit 12 - load lfsr init value
    #[inline(always)]
    pub fn lfsr_load(&mut self) -> LfsrLoadW<DITHER_CONFrs> {
        LfsrLoadW::new(self, 12)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DITHER_CONFrs> {
        RsvdW::new(self, 13)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dither_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dither_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DITHER_CONFrs;
impl crate::RegisterSpec for DITHER_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`dither_conf::R`](R) reader structure
impl crate::Readable for DITHER_CONFrs {}
///`write(|w| ..)` method takes [`dither_conf::W`](W) writer structure
impl crate::Writable for DITHER_CONFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DITHER_CONF to value 0
impl crate::Resettable for DITHER_CONFrs {
    const RESET_VALUE: u32 = 0;
}
