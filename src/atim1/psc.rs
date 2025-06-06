///Register `PSC` reader
pub type R = crate::R<PSCrs>;
///Register `PSC` writer
pub type W = crate::W<PSCrs>;
///Field `PSC` reader - Prescaler value The counter clock frequency is fCLK/(PSC+1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in "reset mode").
pub type PscR = crate::FieldReader<u16>;
///Field `PSC` writer - Prescaler value The counter clock frequency is fCLK/(PSC+1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in "reset mode").
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Prescaler value The counter clock frequency is fCLK/(PSC+1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in "reset mode").
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSC")
            .field("rsvd", &self.rsvd())
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value The counter clock frequency is fCLK/(PSC+1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in "reset mode").
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<PSCrs> {
        PscW::new(self, 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PSCrs> {
        RsvdW::new(self, 16)
    }
}
///Prescaler
///
///You can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PSCrs;
impl crate::RegisterSpec for PSCrs {
    type Ux = u32;
}
///`read()` method returns [`psc::R`](R) reader structure
impl crate::Readable for PSCrs {}
///`write(|w| ..)` method takes [`psc::W`](W) writer structure
impl crate::Writable for PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSC to value 0
impl crate::Resettable for PSCrs {
    const RESET_VALUE: u32 = 0;
}
