///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Write 1 to enable PGM/READ. Self clear
pub type EnR = crate::BitReader;
///Field `EN` writer - Write 1 to enable PGM/READ. Self clear
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - 0 - READ, 1 - PGM
pub type ModeR = crate::BitReader;
///Field `MODE` writer - 0 - READ, 1 - PGM
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BANKSEL` reader - Bank select
pub type BankselR = crate::FieldReader;
///Field `BANKSEL` writer - Bank select
pub type BankselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IE` reader - Interrupt enable
pub type IeR = crate::BitReader;
///Field `IE` writer - Interrupt enable
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bit 0 - Write 1 to enable PGM/READ. Self clear
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 0 - READ, 1 - PGM
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Bank select
    #[inline(always)]
    pub fn banksel(&self) -> BankselR {
        BankselR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Interrupt enable
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rsvd", &self.rsvd())
            .field("ie", &self.ie())
            .field("banksel", &self.banksel())
            .field("mode", &self.mode())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Write 1 to enable PGM/READ. Self clear
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - 0 - READ, 1 - PGM
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CRrs> {
        ModeW::new(self, 1)
    }
    ///Bits 2:3 - Bank select
    #[inline(always)]
    pub fn banksel(&mut self) -> BankselW<CRrs> {
        BankselW::new(self, 2)
    }
    ///Bit 4 - Interrupt enable
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<CRrs> {
        IeW::new(self, 4)
    }
    ///Bits 5:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CRrs> {
        RsvdW::new(self, 5)
    }
}
///Control Register
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
