///Register `JDI_SER_CONF1` reader
pub type R = crate::R<JDI_SER_CONF1rs>;
///Register `JDI_SER_CONF1` writer
pub type W = crate::W<JDI_SER_CONF1rs>;
///Field `WR_LEN` reader - jdi single write bit length
pub type WrLenR = crate::FieldReader;
///Field `WR_LEN` writer - jdi single write bit length
pub type WrLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLK_DIV` reader - jdi serial clock divider
pub type ClkDivR = crate::FieldReader;
///Field `CLK_DIV` writer - jdi serial clock divider
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:4 - jdi single write bit length
    #[inline(always)]
    pub fn wr_len(&self) -> WrLenR {
        WrLenR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:15 - jdi serial clock divider
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_SER_CONF1")
            .field("rsvd", &self.rsvd())
            .field("clk_div", &self.clk_div())
            .field("rsvd2", &self.rsvd2())
            .field("wr_len", &self.wr_len())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - jdi single write bit length
    #[inline(always)]
    pub fn wr_len(&mut self) -> WrLenW<JDI_SER_CONF1rs> {
        WrLenW::new(self, 0)
    }
    ///Bits 5:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<JDI_SER_CONF1rs> {
        Rsvd2W::new(self, 5)
    }
    ///Bits 8:15 - jdi serial clock divider
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<JDI_SER_CONF1rs> {
        ClkDivW::new(self, 8)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<JDI_SER_CONF1rs> {
        RsvdW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_ser_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_ser_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_SER_CONF1rs;
impl crate::RegisterSpec for JDI_SER_CONF1rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_ser_conf1::R`](R) reader structure
impl crate::Readable for JDI_SER_CONF1rs {}
///`write(|w| ..)` method takes [`jdi_ser_conf1::W`](W) writer structure
impl crate::Writable for JDI_SER_CONF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_SER_CONF1 to value 0
impl crate::Resettable for JDI_SER_CONF1rs {
    const RESET_VALUE: u32 = 0;
}
