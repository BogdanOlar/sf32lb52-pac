///Register `PSCLR` reader
pub type R = crate::R<PSCLRrs>;
///Register `PSCLR` writer
pub type W = crate::W<PSCLRrs>;
///Field `DIV` reader - Prescaler divider. 0: MCLK = FCLK/1 1: MCLK = FCLK/1 2: MCLK = FCLK/2 n: MCLK = FCLK/n Note: FLASH clock = MCLK. E.g. FCLK=192M and DIV=2, then FLASH clock = MCLK = 192/2 = 96MHz PSRAM clock = MCLK/2. E.g. FCLK=240M and DIV=1, then PSRAM clock = MCLK/2 = 240/2 = 120MHz
pub type DivR = crate::FieldReader;
///Field `DIV` writer - Prescaler divider. 0: MCLK = FCLK/1 1: MCLK = FCLK/1 2: MCLK = FCLK/2 n: MCLK = FCLK/n Note: FLASH clock = MCLK. E.g. FCLK=192M and DIV=2, then FLASH clock = MCLK = 192/2 = 96MHz PSRAM clock = MCLK/2. E.g. FCLK=240M and DIV=1, then PSRAM clock = MCLK/2 = 240/2 = 120MHz
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:7 - Prescaler divider. 0: MCLK = FCLK/1 1: MCLK = FCLK/1 2: MCLK = FCLK/2 n: MCLK = FCLK/n Note: FLASH clock = MCLK. E.g. FCLK=192M and DIV=2, then FLASH clock = MCLK = 192/2 = 96MHz PSRAM clock = MCLK/2. E.g. FCLK=240M and DIV=1, then PSRAM clock = MCLK/2 = 240/2 = 120MHz
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCLR")
            .field("rsvd", &self.rsvd())
            .field("div", &self.div())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Prescaler divider. 0: MCLK = FCLK/1 1: MCLK = FCLK/1 2: MCLK = FCLK/2 n: MCLK = FCLK/n Note: FLASH clock = MCLK. E.g. FCLK=192M and DIV=2, then FLASH clock = MCLK = 192/2 = 96MHz PSRAM clock = MCLK/2. E.g. FCLK=240M and DIV=1, then PSRAM clock = MCLK/2 = 240/2 = 120MHz
    #[inline(always)]
    pub fn div(&mut self) -> DivW<PSCLRrs> {
        DivW::new(self, 0)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PSCLRrs> {
        RsvdW::new(self, 8)
    }
}
///Prescaler Register
///
///You can [`read`](crate::Reg::read) this register and get [`psclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PSCLRrs;
impl crate::RegisterSpec for PSCLRrs {
    type Ux = u32;
}
///`read()` method returns [`psclr::R`](R) reader structure
impl crate::Readable for PSCLRrs {}
///`write(|w| ..)` method takes [`psclr::W`](W) writer structure
impl crate::Writable for PSCLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSCLR to value 0
impl crate::Resettable for PSCLRrs {
    const RESET_VALUE: u32 = 0;
}
