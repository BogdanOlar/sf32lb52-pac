///Register `RESERVED_OUT` reader
pub type R = crate::R<RESERVED_OUTrs>;
///Register `RESERVED_OUT` writer
pub type W = crate::W<RESERVED_OUTrs>;
///Field `STAT0` reader - reserved status0
pub type Stat0R = crate::FieldReader;
///Field `STAT0` writer - reserved status0
pub type Stat0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `STAT1` reader - reserved status1
pub type Stat1R = crate::FieldReader;
///Field `STAT1` writer - reserved status1
pub type Stat1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - reserved status0
    #[inline(always)]
    pub fn stat0(&self) -> Stat0R {
        Stat0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - reserved status1
    #[inline(always)]
    pub fn stat1(&self) -> Stat1R {
        Stat1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED_OUT")
            .field("stat1", &self.stat1())
            .field("stat0", &self.stat0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reserved status0
    #[inline(always)]
    pub fn stat0(&mut self) -> Stat0W<RESERVED_OUTrs> {
        Stat0W::new(self, 0)
    }
    ///Bits 8:15 - reserved status1
    #[inline(always)]
    pub fn stat1(&mut self) -> Stat1W<RESERVED_OUTrs> {
        Stat1W::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVED_OUTrs;
impl crate::RegisterSpec for RESERVED_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`reserved_out::R`](R) reader structure
impl crate::Readable for RESERVED_OUTrs {}
///`write(|w| ..)` method takes [`reserved_out::W`](W) writer structure
impl crate::Writable for RESERVED_OUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVED_OUT to value 0
impl crate::Resettable for RESERVED_OUTrs {
    const RESET_VALUE: u32 = 0;
}
