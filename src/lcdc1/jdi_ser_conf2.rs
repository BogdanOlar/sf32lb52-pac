///Register `JDI_SER_CONF2` reader
pub type R = crate::R<JDI_SER_CONF2rs>;
///Register `JDI_SER_CONF2` writer
pub type W = crate::W<JDI_SER_CONF2rs>;
///Field `WR_CMD` reader - jdi serial data transfer write command
pub type WrCmdR = crate::FieldReader<u16>;
///Field `WR_CMD` writer - jdi serial data transfer write command
pub type WrCmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INIT_LINE_CNT` reader - jdi serial init line counter
pub type InitLineCntR = crate::FieldReader<u16>;
///Field `INIT_LINE_CNT` writer - jdi serial init line counter
pub type InitLineCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi serial data transfer write command
    #[inline(always)]
    pub fn wr_cmd(&self) -> WrCmdR {
        WrCmdR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi serial init line counter
    #[inline(always)]
    pub fn init_line_cnt(&self) -> InitLineCntR {
        InitLineCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_SER_CONF2")
            .field("init_line_cnt", &self.init_line_cnt())
            .field("wr_cmd", &self.wr_cmd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi serial data transfer write command
    #[inline(always)]
    pub fn wr_cmd(&mut self) -> WrCmdW<JDI_SER_CONF2rs> {
        WrCmdW::new(self, 0)
    }
    ///Bits 16:31 - jdi serial init line counter
    #[inline(always)]
    pub fn init_line_cnt(&mut self) -> InitLineCntW<JDI_SER_CONF2rs> {
        InitLineCntW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_ser_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_ser_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_SER_CONF2rs;
impl crate::RegisterSpec for JDI_SER_CONF2rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_ser_conf2::R`](R) reader structure
impl crate::Readable for JDI_SER_CONF2rs {}
///`write(|w| ..)` method takes [`jdi_ser_conf2::W`](W) writer structure
impl crate::Writable for JDI_SER_CONF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_SER_CONF2 to value 0
impl crate::Resettable for JDI_SER_CONF2rs {
    const RESET_VALUE: u32 = 0;
}
