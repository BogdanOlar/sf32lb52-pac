///Register `RESERVED_IN1` reader
pub type R = crate::R<RESERVED_IN1rs>;
///Register `RESERVED_IN1` writer
pub type W = crate::W<RESERVED_IN1rs>;
///Field `CTRL4` reader - reserved control 4
pub type Ctrl4R = crate::FieldReader;
///Field `CTRL4` writer - reserved control 4
pub type Ctrl4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTRL5` reader - reserved control 5
pub type Ctrl5R = crate::FieldReader;
///Field `CTRL5` writer - reserved control 5
pub type Ctrl5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - reserved control 4
    #[inline(always)]
    pub fn ctrl4(&self) -> Ctrl4R {
        Ctrl4R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - reserved control 5
    #[inline(always)]
    pub fn ctrl5(&self) -> Ctrl5R {
        Ctrl5R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED_IN1")
            .field("ctrl5", &self.ctrl5())
            .field("ctrl4", &self.ctrl4())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reserved control 4
    #[inline(always)]
    pub fn ctrl4(&mut self) -> Ctrl4W<RESERVED_IN1rs> {
        Ctrl4W::new(self, 0)
    }
    ///Bits 8:15 - reserved control 5
    #[inline(always)]
    pub fn ctrl5(&mut self) -> Ctrl5W<RESERVED_IN1rs> {
        Ctrl5W::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`reserved_in1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved_in1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RESERVED_IN1rs;
impl crate::RegisterSpec for RESERVED_IN1rs {
    type Ux = u32;
}
///`read()` method returns [`reserved_in1::R`](R) reader structure
impl crate::Readable for RESERVED_IN1rs {}
///`write(|w| ..)` method takes [`reserved_in1::W`](W) writer structure
impl crate::Writable for RESERVED_IN1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESERVED_IN1 to value 0
impl crate::Resettable for RESERVED_IN1rs {
    const RESET_VALUE: u32 = 0;
}
