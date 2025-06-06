///Register `RESERVED_IN` reader
pub type R = crate::R<RESERVED_INrs>;
///Register `RESERVED_IN` writer
pub type W = crate::W<RESERVED_INrs>;
///Field `CTRL_0` reader - reserved control 0
pub type Ctrl0R = crate::FieldReader;
///Field `CTRL_0` writer - reserved control 0
pub type Ctrl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTRL_1` reader - reserved control 1
pub type Ctrl1R = crate::FieldReader;
///Field `CTRL_1` writer - reserved control 1
pub type Ctrl1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTRL_2` reader - reserved control 2
pub type Ctrl2R = crate::FieldReader;
///Field `CTRL_2` writer - reserved control 2
pub type Ctrl2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - reserved control 0
    #[inline(always)]
    pub fn ctrl_0(&self) -> Ctrl0R {
        Ctrl0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - reserved control 1
    #[inline(always)]
    pub fn ctrl_1(&self) -> Ctrl1R {
        Ctrl1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - reserved control 2
    #[inline(always)]
    pub fn ctrl_2(&self) -> Ctrl2R {
        Ctrl2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED_IN")
            .field("rsvd", &self.rsvd())
            .field("ctrl_2", &self.ctrl_2())
            .field("ctrl_1", &self.ctrl_1())
            .field("ctrl_0", &self.ctrl_0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reserved control 0
    #[inline(always)]
    pub fn ctrl_0(&mut self) -> Ctrl0W<RESERVED_INrs> {
        Ctrl0W::new(self, 0)
    }
    ///Bits 8:15 - reserved control 1
    #[inline(always)]
    pub fn ctrl_1(&mut self) -> Ctrl1W<RESERVED_INrs> {
        Ctrl1W::new(self, 8)
    }
    ///Bits 16:23 - reserved control 2
    #[inline(always)]
    pub fn ctrl_2(&mut self) -> Ctrl2W<RESERVED_INrs> {
        Ctrl2W::new(self, 16)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RESERVED_INrs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVED_INrs;
impl crate::RegisterSpec for RESERVED_INrs {
    type Ux = u32;
}
///`read()` method returns [`reserved_in::R`](R) reader structure
impl crate::Readable for RESERVED_INrs {}
///`write(|w| ..)` method takes [`reserved_in::W`](W) writer structure
impl crate::Writable for RESERVED_INrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVED_IN to value 0
impl crate::Resettable for RESERVED_INrs {
    const RESET_VALUE: u32 = 0;
}
