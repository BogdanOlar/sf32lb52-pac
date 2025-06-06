///Register `HCMDR` reader
pub type R = crate::R<HCMDRrs>;
///Register `HCMDR` writer
pub type W = crate::W<HCMDRrs>;
///Field `RCMD` reader - AHB read command. During XIP, the AHB read transaction will be translated into this Read Command on memory interface
pub type RcmdR = crate::FieldReader;
///Field `RCMD` writer - AHB read command. During XIP, the AHB read transaction will be translated into this Read Command on memory interface
pub type RcmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WCMD` reader - AHB write command. During XIP, the AHB write transaction will be translated into this Write Command on memory interface
pub type WcmdR = crate::FieldReader;
///Field `WCMD` writer - AHB write command. During XIP, the AHB write transaction will be translated into this Write Command on memory interface
pub type WcmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:7 - AHB read command. During XIP, the AHB read transaction will be translated into this Read Command on memory interface
    #[inline(always)]
    pub fn rcmd(&self) -> RcmdR {
        RcmdR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - AHB write command. During XIP, the AHB write transaction will be translated into this Write Command on memory interface
    #[inline(always)]
    pub fn wcmd(&self) -> WcmdR {
        WcmdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCMDR")
            .field("rsvd", &self.rsvd())
            .field("wcmd", &self.wcmd())
            .field("rcmd", &self.rcmd())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - AHB read command. During XIP, the AHB read transaction will be translated into this Read Command on memory interface
    #[inline(always)]
    pub fn rcmd(&mut self) -> RcmdW<HCMDRrs> {
        RcmdW::new(self, 0)
    }
    ///Bits 8:15 - AHB write command. During XIP, the AHB write transaction will be translated into this Write Command on memory interface
    #[inline(always)]
    pub fn wcmd(&mut self) -> WcmdW<HCMDRrs> {
        WcmdW::new(self, 8)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<HCMDRrs> {
        RsvdW::new(self, 16)
    }
}
///AHB Command Register
///
///You can [`read`](crate::Reg::read) this register and get [`hcmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HCMDRrs;
impl crate::RegisterSpec for HCMDRrs {
    type Ux = u32;
}
///`read()` method returns [`hcmdr::R`](R) reader structure
impl crate::Readable for HCMDRrs {}
///`write(|w| ..)` method takes [`hcmdr::W`](W) writer structure
impl crate::Writable for HCMDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HCMDR to value 0
impl crate::Resettable for HCMDRrs {
    const RESET_VALUE: u32 = 0;
}
