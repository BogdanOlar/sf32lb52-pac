///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `PECF` reader - Parity error clear flag. Wriring 1 to this bit clears the PE flag in the ISR register.
pub type PecfR = crate::BitReader;
///Field `PECF` writer - Parity error clear flag. Wriring 1 to this bit clears the PE flag in the ISR register.
pub type PecfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECF` reader - Framing error clear flag. Writing 1 to this bit clears the FE flag in the ISR register.
pub type FecfR = crate::BitReader;
///Field `FECF` writer - Framing error clear flag. Writing 1 to this bit clears the FE flag in the ISR register.
pub type FecfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NCF` reader - Noise detected clear flag. Writing 1 to this bit clears the NF flag in the ISR register.
pub type NcfR = crate::BitReader;
///Field `NCF` writer - Noise detected clear flag. Writing 1 to this bit clears the NF flag in the ISR register.
pub type NcfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORECF` reader - Overrun error clear flag. Writing 1 to this bit clears the ORE flag in the ISR register.
pub type OrecfR = crate::BitReader;
///Field `ORECF` writer - Overrun error clear flag. Writing 1 to this bit clears the ORE flag in the ISR register.
pub type OrecfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLECF` reader - Idle line detected clear flag. Writing 1 to this bit clears the IDLECF flag in the ISR register.
pub type IdlecfR = crate::BitReader;
///Field `IDLECF` writer - Idle line detected clear flag. Writing 1 to this bit clears the IDLECF flag in the ISR register.
pub type IdlecfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` reader - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
pub type TccfR = crate::BitReader;
///Field `TCCF` writer - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` reader - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
pub type CtscfR = crate::BitReader;
///Field `CTSCF` writer - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Parity error clear flag. Wriring 1 to this bit clears the PE flag in the ISR register.
    #[inline(always)]
    pub fn pecf(&self) -> PecfR {
        PecfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error clear flag. Writing 1 to this bit clears the FE flag in the ISR register.
    #[inline(always)]
    pub fn fecf(&self) -> FecfR {
        FecfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detected clear flag. Writing 1 to this bit clears the NF flag in the ISR register.
    #[inline(always)]
    pub fn ncf(&self) -> NcfR {
        NcfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error clear flag. Writing 1 to this bit clears the ORE flag in the ISR register.
    #[inline(always)]
    pub fn orecf(&self) -> OrecfR {
        OrecfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected clear flag. Writing 1 to this bit clears the IDLECF flag in the ISR register.
    #[inline(always)]
    pub fn idlecf(&self) -> IdlecfR {
        IdlecfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
    #[inline(always)]
    pub fn tccf(&self) -> TccfR {
        TccfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
    #[inline(always)]
    pub fn ctscf(&self) -> CtscfR {
        CtscfR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("ctscf", &self.ctscf())
            .field("tccf", &self.tccf())
            .field("idlecf", &self.idlecf())
            .field("orecf", &self.orecf())
            .field("ncf", &self.ncf())
            .field("fecf", &self.fecf())
            .field("pecf", &self.pecf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Parity error clear flag. Wriring 1 to this bit clears the PE flag in the ISR register.
    #[inline(always)]
    pub fn pecf(&mut self) -> PecfW<ICRrs> {
        PecfW::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag. Writing 1 to this bit clears the FE flag in the ISR register.
    #[inline(always)]
    pub fn fecf(&mut self) -> FecfW<ICRrs> {
        FecfW::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag. Writing 1 to this bit clears the NF flag in the ISR register.
    #[inline(always)]
    pub fn ncf(&mut self) -> NcfW<ICRrs> {
        NcfW::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag. Writing 1 to this bit clears the ORE flag in the ISR register.
    #[inline(always)]
    pub fn orecf(&mut self) -> OrecfW<ICRrs> {
        OrecfW::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag. Writing 1 to this bit clears the IDLECF flag in the ISR register.
    #[inline(always)]
    pub fn idlecf(&mut self) -> IdlecfW<ICRrs> {
        IdlecfW::new(self, 4)
    }
    ///Bit 6 - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
    #[inline(always)]
    pub fn tccf(&mut self) -> TccfW<ICRrs> {
        TccfW::new(self, 6)
    }
    ///Bit 9 - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
    #[inline(always)]
    pub fn ctscf(&mut self) -> CtscfW<ICRrs> {
        CtscfW::new(self, 9)
    }
}
///Interrupt flag Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
