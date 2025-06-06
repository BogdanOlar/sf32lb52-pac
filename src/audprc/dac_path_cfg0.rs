///Register `DAC_PATH_CFG0` reader
pub type R = crate::R<DAC_PATH_CFG0rs>;
///Register `DAC_PATH_CFG0` writer
pub type W = crate::W<DAC_PATH_CFG0rs>;
///Field `ROUGH_VOL_L` reader - dac mixer left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolLR = crate::FieldReader;
///Field `ROUGH_VOL_L` writer - dac mixer left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FINE_VOL_L` reader - dac mixer left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolLR = crate::FieldReader;
///Field `FINE_VOL_L` writer - dac mixer left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ROUGH_VOL_R` reader - dac mixer right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolRR = crate::FieldReader;
///Field `ROUGH_VOL_R` writer - dac mixer right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
pub type RoughVolRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FINE_VOL_R` reader - dac mixer right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolRR = crate::FieldReader;
///Field `FINE_VOL_R` writer - dac mixer right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
pub type FineVolRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MIXLSRC0` reader - dac mixer left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixlsrc0R = crate::FieldReader;
///Field `MIXLSRC0` writer - dac mixer left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixlsrc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MIXLSRC1` reader - dac mixer left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixlsrc1R = crate::FieldReader;
///Field `MIXLSRC1` writer - dac mixer left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixlsrc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MIXRSRC0` reader - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixrsrc0R = crate::FieldReader;
///Field `MIXRSRC0` writer - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixrsrc0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MIXRSRC1` reader - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixrsrc1R = crate::FieldReader;
///Field `MIXRSRC1` writer - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
pub type Mixrsrc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DST_SEL` reader - dac path destination select 2'h0: select audio codec 2'h1: select external interface 2'h2: select apb interface 2'h3: reserved
pub type DstSelR = crate::FieldReader;
///Field `DST_SEL` writer - dac path destination select 2'h0: select audio codec 2'h1: select external interface 2'h2: select apb interface 2'h3: reserved
pub type DstSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - dac mixer left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_l(&self) -> RoughVolLR {
        RoughVolLR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - dac mixer left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_l(&self) -> FineVolLR {
        FineVolLR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - dac mixer right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_r(&self) -> RoughVolRR {
        RoughVolRR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - dac mixer right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_r(&self) -> FineVolRR {
        FineVolRR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:18 - dac mixer left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixlsrc0(&self) -> Mixlsrc0R {
        Mixlsrc0R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - dac mixer left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixlsrc1(&self) -> Mixlsrc1R {
        Mixlsrc1R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:24 - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixrsrc0(&self) -> Mixrsrc0R {
        Mixrsrc0R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:27 - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixrsrc1(&self) -> Mixrsrc1R {
        Mixrsrc1R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:29 - dac path destination select 2'h0: select audio codec 2'h1: select external interface 2'h2: select apb interface 2'h3: reserved
    #[inline(always)]
    pub fn dst_sel(&self) -> DstSelR {
        DstSelR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_PATH_CFG0")
            .field("dst_sel", &self.dst_sel())
            .field("mixrsrc1", &self.mixrsrc1())
            .field("mixrsrc0", &self.mixrsrc0())
            .field("mixlsrc1", &self.mixlsrc1())
            .field("mixlsrc0", &self.mixlsrc0())
            .field("fine_vol_r", &self.fine_vol_r())
            .field("rough_vol_r", &self.rough_vol_r())
            .field("fine_vol_l", &self.fine_vol_l())
            .field("rough_vol_l", &self.rough_vol_l())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - dac mixer left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_l(&mut self) -> RoughVolLW<DAC_PATH_CFG0rs> {
        RoughVolLW::new(self, 0)
    }
    ///Bits 4:7 - dac mixer left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_l(&mut self) -> FineVolLW<DAC_PATH_CFG0rs> {
        FineVolLW::new(self, 4)
    }
    ///Bits 8:11 - dac mixer right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB
    #[inline(always)]
    pub fn rough_vol_r(&mut self) -> RoughVolRW<DAC_PATH_CFG0rs> {
        RoughVolRW::new(self, 8)
    }
    ///Bits 12:15 - dac mixer right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute
    #[inline(always)]
    pub fn fine_vol_r(&mut self) -> FineVolRW<DAC_PATH_CFG0rs> {
        FineVolRW::new(self, 12)
    }
    ///Bits 16:18 - dac mixer left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixlsrc0(&mut self) -> Mixlsrc0W<DAC_PATH_CFG0rs> {
        Mixlsrc0W::new(self, 16)
    }
    ///Bits 19:21 - dac mixer left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixlsrc1(&mut self) -> Mixlsrc1W<DAC_PATH_CFG0rs> {
        Mixlsrc1W::new(self, 19)
    }
    ///Bits 22:24 - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixrsrc0(&mut self) -> Mixrsrc0W<DAC_PATH_CFG0rs> {
        Mixrsrc0W::new(self, 22)
    }
    ///Bits 25:27 - dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute
    #[inline(always)]
    pub fn mixrsrc1(&mut self) -> Mixrsrc1W<DAC_PATH_CFG0rs> {
        Mixrsrc1W::new(self, 25)
    }
    ///Bits 28:29 - dac path destination select 2'h0: select audio codec 2'h1: select external interface 2'h2: select apb interface 2'h3: reserved
    #[inline(always)]
    pub fn dst_sel(&mut self) -> DstSelW<DAC_PATH_CFG0rs> {
        DstSelW::new(self, 28)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_PATH_CFG0rs;
impl crate::RegisterSpec for DAC_PATH_CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`dac_path_cfg0::R`](R) reader structure
impl crate::Readable for DAC_PATH_CFG0rs {}
///`write(|w| ..)` method takes [`dac_path_cfg0::W`](W) writer structure
impl crate::Writable for DAC_PATH_CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_PATH_CFG0 to value 0
impl crate::Resettable for DAC_PATH_CFG0rs {
    const RESET_VALUE: u32 = 0;
}
