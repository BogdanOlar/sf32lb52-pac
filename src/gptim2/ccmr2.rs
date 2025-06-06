///Register `CCMR2` reader
pub type R = crate::R<CCMR2rs>;
///Register `CCMR2` writer
pub type W = crate::W<CCMR2rs>;
///Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
pub type Cc3sR = crate::FieldReader;
///Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3PSC` reader - Input capture 3 prescaler
pub type Ic3pscR = crate::FieldReader;
///Field `IC3PSC` writer - Input capture 3 prescaler
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3F` reader - Input capture 3 filter
pub type Ic3fR = crate::FieldReader;
///Field `IC3F` writer - Input capture 3 filter
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
pub type Cc4sR = crate::FieldReader;
///Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4PSC` reader - Input capture 4 prescaler
pub type Ic4pscR = crate::FieldReader;
///Field `IC4PSC` writer - Input capture 4 prescaler
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4F` reader - Input capture 4 filter
pub type Ic4fR = crate::FieldReader;
///Field `IC4F` writer - Input capture 4 filter
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OC3CE` reader - Output compare 3 clear enable
pub type Oc3ceR = crate::BitReader;
///Field `OC3CE` writer - Output compare 3 clear enable
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC3PE` reader - Output compare 3 preload enable
pub type Oc3peR = crate::BitReader;
///Field `OC3PE` writer - Output compare 3 preload enable
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M` reader - Output compare 3 mode
pub type Oc3mR = crate::FieldReader;
///Field `OC3M` writer - Output compare 3 mode
pub type Oc3mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OC4CE` reader - Output compare 4 clear enable
pub type Oc4ceR = crate::BitReader;
///Field `OC4CE` writer - Output compare 4 clear enable
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC4PE` reader - Output compare 4 preload enable
pub type Oc4peR = crate::BitReader;
///Field `OC4PE` writer - Output compare 4 preload enable
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M` reader - Output compare 4 mode
pub type Oc4mR = crate::FieldReader;
///Field `OC4M` writer - Output compare 4 mode
pub type Oc4mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&self) -> Oc3mR {
        Oc3mR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&self) -> Oc4mR {
        Oc4mR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2")
            .field("oc4m", &self.oc4m())
            .field("oc4pe", &self.oc4pe())
            .field("rsvd", &self.rsvd())
            .field("oc4ce", &self.oc4ce())
            .field("oc3m", &self.oc3m())
            .field("oc3pe", &self.oc3pe())
            .field("rsvd2", &self.rsvd2())
            .field("oc3ce", &self.oc3ce())
            .field("ic4f", &self.ic4f())
            .field("ic4psc", &self.ic4psc())
            .field("cc4s", &self.cc4s())
            .field("ic3f", &self.ic3f())
            .field("ic3psc", &self.ic3psc())
            .field("cc3s", &self.cc3s())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<CCMR2rs> {
        Cc3sW::new(self, 0)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> Ic3pscW<CCMR2rs> {
        Ic3pscW::new(self, 2)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> Ic3fW<CCMR2rs> {
        Ic3fW::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<CCMR2rs> {
        Cc4sW::new(self, 8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> Ic4pscW<CCMR2rs> {
        Ic4pscW::new(self, 10)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> Ic4fW<CCMR2rs> {
        Ic4fW::new(self, 12)
    }
    ///Bit 16 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<CCMR2rs> {
        Oc3ceW::new(self, 16)
    }
    ///Bits 17:18
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CCMR2rs> {
        Rsvd2W::new(self, 17)
    }
    ///Bit 19 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<CCMR2rs> {
        Oc3peW::new(self, 19)
    }
    ///Bits 20:23 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&mut self) -> Oc3mW<CCMR2rs> {
        Oc3mW::new(self, 20)
    }
    ///Bit 24 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> Oc4ceW<CCMR2rs> {
        Oc4ceW::new(self, 24)
    }
    ///Bits 25:26
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CCMR2rs> {
        RsvdW::new(self, 25)
    }
    ///Bit 27 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&mut self) -> Oc4peW<CCMR2rs> {
        Oc4peW::new(self, 27)
    }
    ///Bits 28:31 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&mut self) -> Oc4mW<CCMR2rs> {
        Oc4mW::new(self, 28)
    }
}
///TIM capture/compare mode register 2
///
///You can [`read`](crate::Reg::read) this register and get [`ccmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCMR2rs;
impl crate::RegisterSpec for CCMR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2::R`](R) reader structure
impl crate::Readable for CCMR2rs {}
///`write(|w| ..)` method takes [`ccmr2::W`](W) writer structure
impl crate::Writable for CCMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR2 to value 0
impl crate::Resettable for CCMR2rs {
    const RESET_VALUE: u32 = 0;
}
