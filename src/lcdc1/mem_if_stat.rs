///Register `MEM_IF_STAT` reader
pub type R = crate::R<MEM_IF_STATrs>;
///Register `MEM_IF_STAT` writer
pub type W = crate::W<MEM_IF_STATrs>;
///Field `AHB` reader -
pub type AhbR = crate::FieldReader;
///Field `AHB` writer -
pub type AhbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ARB_READ_PORT` reader -
pub type ArbReadPortR = crate::FieldReader;
///Field `ARB_READ_PORT` writer -
pub type ArbReadPortW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ARB_MAIN` reader -
pub type ArbMainR = crate::FieldReader;
///Field `ARB_MAIN` writer -
pub type ArbMainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn ahb(&self) -> AhbR {
        AhbR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6
    #[inline(always)]
    pub fn arb_read_port(&self) -> ArbReadPortR {
        ArbReadPortR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn arb_main(&self) -> ArbMainR {
        ArbMainR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_IF_STAT")
            .field("rsvd", &self.rsvd())
            .field("arb_main", &self.arb_main())
            .field("arb_read_port", &self.arb_read_port())
            .field("ahb", &self.ahb())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn ahb(&mut self) -> AhbW<MEM_IF_STATrs> {
        AhbW::new(self, 0)
    }
    ///Bits 4:6
    #[inline(always)]
    pub fn arb_read_port(&mut self) -> ArbReadPortW<MEM_IF_STATrs> {
        ArbReadPortW::new(self, 4)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn arb_main(&mut self) -> ArbMainW<MEM_IF_STATrs> {
        ArbMainW::new(self, 7)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<MEM_IF_STATrs> {
        RsvdW::new(self, 10)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`mem_if_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_if_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MEM_IF_STATrs;
impl crate::RegisterSpec for MEM_IF_STATrs {
    type Ux = u32;
}
///`read()` method returns [`mem_if_stat::R`](R) reader structure
impl crate::Readable for MEM_IF_STATrs {}
///`write(|w| ..)` method takes [`mem_if_stat::W`](W) writer structure
impl crate::Writable for MEM_IF_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_IF_STAT to value 0
impl crate::Resettable for MEM_IF_STATrs {
    const RESET_VALUE: u32 = 0;
}
