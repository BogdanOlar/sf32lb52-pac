///Register `RESERVED_IN0` reader
pub type R = crate::R<RESERVED_IN0rs>;
///Register `RESERVED_IN0` writer
pub type W = crate::W<RESERVED_IN0rs>;
///Field `CTRL0` reader - reserved control 0
pub type Ctrl0R = crate::FieldReader;
///Field `CTRL0` writer - reserved control 0
pub type Ctrl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTRL1` reader - reserved control 1
pub type Ctrl1R = crate::FieldReader;
///Field `CTRL1` writer - reserved control 1
pub type Ctrl1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTRL2` reader - reserved control 2
pub type Ctrl2R = crate::FieldReader;
///Field `CTRL2` writer - reserved control 2
pub type Ctrl2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTRL3` reader - reserved control 3
pub type Ctrl3R = crate::FieldReader;
///Field `CTRL3` writer - reserved control 3
pub type Ctrl3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - reserved control 0
    #[inline(always)]
    pub fn ctrl0(&self) -> Ctrl0R {
        Ctrl0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - reserved control 1
    #[inline(always)]
    pub fn ctrl1(&self) -> Ctrl1R {
        Ctrl1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - reserved control 2
    #[inline(always)]
    pub fn ctrl2(&self) -> Ctrl2R {
        Ctrl2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - reserved control 3
    #[inline(always)]
    pub fn ctrl3(&self) -> Ctrl3R {
        Ctrl3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED_IN0")
            .field("ctrl3", &self.ctrl3())
            .field("ctrl2", &self.ctrl2())
            .field("ctrl1", &self.ctrl1())
            .field("ctrl0", &self.ctrl0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reserved control 0
    #[inline(always)]
    pub fn ctrl0(&mut self) -> Ctrl0W<RESERVED_IN0rs> {
        Ctrl0W::new(self, 0)
    }
    ///Bits 8:15 - reserved control 1
    #[inline(always)]
    pub fn ctrl1(&mut self) -> Ctrl1W<RESERVED_IN0rs> {
        Ctrl1W::new(self, 8)
    }
    ///Bits 16:23 - reserved control 2
    #[inline(always)]
    pub fn ctrl2(&mut self) -> Ctrl2W<RESERVED_IN0rs> {
        Ctrl2W::new(self, 16)
    }
    ///Bits 24:31 - reserved control 3
    #[inline(always)]
    pub fn ctrl3(&mut self) -> Ctrl3W<RESERVED_IN0rs> {
        Ctrl3W::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_in0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_in0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVED_IN0rs;
impl crate::RegisterSpec for RESERVED_IN0rs {
    type Ux = u32;
}
///`read()` method returns [`reserved_in0::R`](R) reader structure
impl crate::Readable for RESERVED_IN0rs {}
///`write(|w| ..)` method takes [`reserved_in0::W`](W) writer structure
impl crate::Writable for RESERVED_IN0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVED_IN0 to value 0
impl crate::Resettable for RESERVED_IN0rs {
    const RESET_VALUE: u32 = 0;
}
