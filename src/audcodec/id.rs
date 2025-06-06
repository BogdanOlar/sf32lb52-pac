///Register `ID` reader
pub type R = crate::R<IDrs>;
///Register `ID` writer
pub type W = crate::W<IDrs>;
///Field `FUNC` reader - function id
pub type FuncR = crate::FieldReader<u32>;
///Field `FUNC` writer - function id
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - function id
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID").field("func", &self.func()).finish()
    }
}
impl W {
    ///Bits 0:31 - function id
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<IDrs> {
        FuncW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IDrs;
impl crate::RegisterSpec for IDrs {
    type Ux = u32;
}
///`read()` method returns [`id::R`](R) reader structure
impl crate::Readable for IDrs {}
///`write(|w| ..)` method takes [`id::W`](W) writer structure
impl crate::Writable for IDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ID to value 0xc0de_c000
impl crate::Resettable for IDrs {
    const RESET_VALUE: u32 = 0xc0de_c000;
}
