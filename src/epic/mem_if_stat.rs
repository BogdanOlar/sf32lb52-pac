///Register `MEM_IF_STAT` reader
pub type R = crate::R<MEM_IF_STATrs>;
///Register `MEM_IF_STAT` writer
pub type W = crate::W<MEM_IF_STATrs>;
///Field `AHB0` reader -
pub type Ahb0R = crate::FieldReader;
///Field `AHB0` writer -
pub type Ahb0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ARB_READ_PORT0` reader -
pub type ArbReadPort0R = crate::FieldReader;
///Field `ARB_READ_PORT0` writer -
pub type ArbReadPort0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ARB_MAIN0` reader -
pub type ArbMain0R = crate::FieldReader;
///Field `ARB_MAIN0` writer -
pub type ArbMain0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB1` reader -
pub type Ahb1R = crate::FieldReader;
///Field `AHB1` writer -
pub type Ahb1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ARB_READ_PORT1` reader -
pub type ArbReadPort1R = crate::FieldReader;
///Field `ARB_READ_PORT1` writer -
pub type ArbReadPort1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ARB_MAIN1` reader -
pub type ArbMain1R = crate::FieldReader;
///Field `ARB_MAIN1` writer -
pub type ArbMain1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB_CTRL` reader -
pub type AhbCtrlR = crate::FieldReader;
///Field `AHB_CTRL` writer -
pub type AhbCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB_CTRL_EOL` reader -
pub type AhbCtrlEolR = crate::BitReader;
///Field `AHB_CTRL_EOL` writer -
pub type AhbCtrlEolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB_CTRL_FIFO_CNT` reader -
pub type AhbCtrlFifoCntR = crate::FieldReader;
///Field `AHB_CTRL_FIFO_CNT` writer -
pub type AhbCtrlFifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn ahb0(&self) -> Ahb0R {
        Ahb0R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6
    #[inline(always)]
    pub fn arb_read_port0(&self) -> ArbReadPort0R {
        ArbReadPort0R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn arb_main0(&self) -> ArbMain0R {
        ArbMain0R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn ahb1(&self) -> Ahb1R {
        Ahb1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bits 14:16
    #[inline(always)]
    pub fn arb_read_port1(&self) -> ArbReadPort1R {
        ArbReadPort1R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn arb_main1(&self) -> ArbMain1R {
        ArbMain1R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22
    #[inline(always)]
    pub fn ahb_ctrl(&self) -> AhbCtrlR {
        AhbCtrlR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23
    #[inline(always)]
    pub fn ahb_ctrl_eol(&self) -> AhbCtrlEolR {
        AhbCtrlEolR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn ahb_ctrl_fifo_cnt(&self) -> AhbCtrlFifoCntR {
        AhbCtrlFifoCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_IF_STAT")
            .field("ahb_ctrl_fifo_cnt", &self.ahb_ctrl_fifo_cnt())
            .field("ahb_ctrl_eol", &self.ahb_ctrl_eol())
            .field("ahb_ctrl", &self.ahb_ctrl())
            .field("arb_main1", &self.arb_main1())
            .field("arb_read_port1", &self.arb_read_port1())
            .field("ahb1", &self.ahb1())
            .field("arb_main0", &self.arb_main0())
            .field("arb_read_port0", &self.arb_read_port0())
            .field("ahb0", &self.ahb0())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn ahb0(&mut self) -> Ahb0W<MEM_IF_STATrs> {
        Ahb0W::new(self, 0)
    }
    ///Bits 4:6
    #[inline(always)]
    pub fn arb_read_port0(&mut self) -> ArbReadPort0W<MEM_IF_STATrs> {
        ArbReadPort0W::new(self, 4)
    }
    ///Bits 7:9
    #[inline(always)]
    pub fn arb_main0(&mut self) -> ArbMain0W<MEM_IF_STATrs> {
        ArbMain0W::new(self, 7)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn ahb1(&mut self) -> Ahb1W<MEM_IF_STATrs> {
        Ahb1W::new(self, 10)
    }
    ///Bits 14:16
    #[inline(always)]
    pub fn arb_read_port1(&mut self) -> ArbReadPort1W<MEM_IF_STATrs> {
        ArbReadPort1W::new(self, 14)
    }
    ///Bits 17:19
    #[inline(always)]
    pub fn arb_main1(&mut self) -> ArbMain1W<MEM_IF_STATrs> {
        ArbMain1W::new(self, 17)
    }
    ///Bits 20:22
    #[inline(always)]
    pub fn ahb_ctrl(&mut self) -> AhbCtrlW<MEM_IF_STATrs> {
        AhbCtrlW::new(self, 20)
    }
    ///Bit 23
    #[inline(always)]
    pub fn ahb_ctrl_eol(&mut self) -> AhbCtrlEolW<MEM_IF_STATrs> {
        AhbCtrlEolW::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn ahb_ctrl_fifo_cnt(&mut self) -> AhbCtrlFifoCntW<MEM_IF_STATrs> {
        AhbCtrlFifoCntW::new(self, 24)
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
