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
///Field `RSVD11` reader -
pub type Rsvd11R = crate::BitReader;
///Field `RSVD11` writer -
pub type Rsvd11W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` reader - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
pub type TccfR = crate::BitReader;
///Field `TCCF` writer - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD10` reader -
pub type Rsvd10R = crate::BitReader;
///Field `RSVD10` writer -
pub type Rsvd10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD9` reader -
pub type Rsvd9R = crate::BitReader;
///Field `RSVD9` writer -
pub type Rsvd9W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` reader - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
pub type CtscfR = crate::BitReader;
///Field `CTSCF` writer - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD8` reader -
pub type Rsvd8R = crate::BitReader;
///Field `RSVD8` writer -
pub type Rsvd8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD7` reader -
pub type Rsvd7R = crate::BitReader;
///Field `RSVD7` writer -
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD6` reader -
pub type Rsvd6R = crate::BitReader;
///Field `RSVD6` writer -
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::FieldReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
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
    ///Bit 5
    #[inline(always)]
    pub fn rsvd11(&self) -> Rsvd11R {
        Rsvd11R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
    #[inline(always)]
    pub fn tccf(&self) -> TccfR {
        TccfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd10(&self) -> Rsvd10R {
        Rsvd10R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd9(&self) -> Rsvd9R {
        Rsvd9R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
    #[inline(always)]
    pub fn ctscf(&self) -> CtscfR {
        CtscfR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn rsvd8(&self) -> Rsvd8R {
        Rsvd8R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn rsvd6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("rsvd", &self.rsvd())
            .field("rsvd2", &self.rsvd2())
            .field("rsvd3", &self.rsvd3())
            .field("rsvd4", &self.rsvd4())
            .field("rsvd5", &self.rsvd5())
            .field("rsvd6", &self.rsvd6())
            .field("rsvd7", &self.rsvd7())
            .field("rsvd8", &self.rsvd8())
            .field("ctscf", &self.ctscf())
            .field("rsvd9", &self.rsvd9())
            .field("rsvd10", &self.rsvd10())
            .field("tccf", &self.tccf())
            .field("rsvd11", &self.rsvd11())
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
    ///Bit 5
    #[inline(always)]
    pub fn rsvd11(&mut self) -> Rsvd11W<ICRrs> {
        Rsvd11W::new(self, 5)
    }
    ///Bit 6 - Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register.
    #[inline(always)]
    pub fn tccf(&mut self) -> TccfW<ICRrs> {
        TccfW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd10(&mut self) -> Rsvd10W<ICRrs> {
        Rsvd10W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd9(&mut self) -> Rsvd9W<ICRrs> {
        Rsvd9W::new(self, 8)
    }
    ///Bit 9 - CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register.
    #[inline(always)]
    pub fn ctscf(&mut self) -> CtscfW<ICRrs> {
        CtscfW::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    pub fn rsvd8(&mut self) -> Rsvd8W<ICRrs> {
        Rsvd8W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd7(&mut self) -> Rsvd7W<ICRrs> {
        Rsvd7W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    pub fn rsvd6(&mut self) -> Rsvd6W<ICRrs> {
        Rsvd6W::new(self, 12)
    }
    ///Bits 13:16
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<ICRrs> {
        Rsvd5W::new(self, 13)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<ICRrs> {
        Rsvd4W::new(self, 17)
    }
    ///Bits 18:19
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<ICRrs> {
        Rsvd3W::new(self, 18)
    }
    ///Bit 20
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ICRrs> {
        Rsvd2W::new(self, 20)
    }
    ///Bits 21:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ICRrs> {
        RsvdW::new(self, 21)
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
