///Register `CCR8` reader
pub type R = crate::R<CCR8rs>;
///Register `CCR8` writer
pub type W = crate::W<CCR8rs>;
///Field `EN` reader - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the ISR register is cleared (by setting the CTEIFx bit of the IFCR register). 0: disabled 1: enabled
pub type EnR = crate::BitReader;
///Field `EN` writer - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the ISR register is cleared (by setting the CTEIFx bit of the IFCR register). 0: disabled 1: enabled
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - transfer complete interrupt enable 0: disabled 1: enabled
pub type TcieR = crate::BitReader;
///Field `TCIE` writer - transfer complete interrupt enable 0: disabled 1: enabled
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - half transfer interrupt enable 0: disabled 1: enabled
pub type HtieR = crate::BitReader;
///Field `HTIE` writer - half transfer interrupt enable 0: disabled 1: enabled
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - transfer error interrupt enable 0: disabled 1: enabled
pub type TeieR = crate::BitReader;
///Field `TEIE` writer - transfer error interrupt enable 0: disabled 1: enabled
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. 0: read from peripheral Source attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode. 1: read from memory Destination attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode.
pub type DirR = crate::BitReader;
///Field `DIR` writer - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. 0: read from peripheral Source attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode. 1: read from memory Destination attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode.
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIRC` reader - circular mode 0: disabled 1: enabled
pub type CircR = crate::BitReader;
///Field `CIRC` writer - circular mode 0: disabled 1: enabled
pub type CircW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINC` reader - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 0: disabled 1: enabled
pub type PincR = crate::BitReader;
///Field `PINC` writer - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 0: disabled 1: enabled
pub type PincW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINC` reader - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 0: disabled 1: enabled
pub type MincR = crate::BitReader;
///Field `MINC` writer - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 0: disabled 1: enabled
pub type MincW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
pub type PsizeR = crate::FieldReader;
///Field `PSIZE` writer - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSIZE` reader - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
pub type MsizeR = crate::FieldReader;
///Field `MSIZE` writer - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PL` reader - priority level 00: low 01: medium 10: high 11: very high
pub type PlR = crate::FieldReader;
///Field `PL` writer - priority level 00: low 01: medium 10: high 11: very high
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MEM2MEM` reader - memory-to-memory mode 0: disabled 1: enabled
pub type Mem2memR = crate::BitReader;
///Field `MEM2MEM` writer - memory-to-memory mode 0: disabled 1: enabled
pub type Mem2memW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the ISR register is cleared (by setting the CTEIFx bit of the IFCR register). 0: disabled 1: enabled
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - transfer complete interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - half transfer interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - transfer error interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. 0: read from peripheral Source attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode. 1: read from memory Destination attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode.
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - circular mode 0: disabled 1: enabled
    #[inline(always)]
    pub fn circ(&self) -> CircR {
        CircR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 0: disabled 1: enabled
    #[inline(always)]
    pub fn pinc(&self) -> PincR {
        PincR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 0: disabled 1: enabled
    #[inline(always)]
    pub fn minc(&self) -> MincR {
        MincR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - priority level 00: low 01: medium 10: high 11: very high
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - memory-to-memory mode 0: disabled 1: enabled
    #[inline(always)]
    pub fn mem2mem(&self) -> Mem2memR {
        Mem2memR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR8")
            .field("mem2mem", &self.mem2mem())
            .field("pl", &self.pl())
            .field("msize", &self.msize())
            .field("psize", &self.psize())
            .field("minc", &self.minc())
            .field("pinc", &self.pinc())
            .field("circ", &self.circ())
            .field("dir", &self.dir())
            .field("teie", &self.teie())
            .field("htie", &self.htie())
            .field("tcie", &self.tcie())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the ISR register is cleared (by setting the CTEIFx bit of the IFCR register). 0: disabled 1: enabled
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CCR8rs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - transfer complete interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<CCR8rs> {
        TcieW::new(self, 1)
    }
    ///Bit 2 - half transfer interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn htie(&mut self) -> HtieW<CCR8rs> {
        HtieW::new(self, 2)
    }
    ///Bit 3 - transfer error interrupt enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<CCR8rs> {
        TeieW::new(self, 3)
    }
    ///Bit 4 - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. 0: read from peripheral Source attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode. 1: read from memory Destination attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode.
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<CCR8rs> {
        DirW::new(self, 4)
    }
    ///Bit 5 - circular mode 0: disabled 1: enabled
    #[inline(always)]
    pub fn circ(&mut self) -> CircW<CCR8rs> {
        CircW::new(self, 5)
    }
    ///Bit 6 - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 0: disabled 1: enabled
    #[inline(always)]
    pub fn pinc(&mut self) -> PincW<CCR8rs> {
        PincW::new(self, 6)
    }
    ///Bit 7 - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 0: disabled 1: enabled
    #[inline(always)]
    pub fn minc(&mut self) -> MincW<CCR8rs> {
        MincW::new(self, 7)
    }
    ///Bits 8:9 - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
    #[inline(always)]
    pub fn psize(&mut self) -> PsizeW<CCR8rs> {
        PsizeW::new(self, 8)
    }
    ///Bits 10:11 - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved
    #[inline(always)]
    pub fn msize(&mut self) -> MsizeW<CCR8rs> {
        MsizeW::new(self, 10)
    }
    ///Bits 12:13 - priority level 00: low 01: medium 10: high 11: very high
    #[inline(always)]
    pub fn pl(&mut self) -> PlW<CCR8rs> {
        PlW::new(self, 12)
    }
    ///Bit 14 - memory-to-memory mode 0: disabled 1: enabled
    #[inline(always)]
    pub fn mem2mem(&mut self) -> Mem2memW<CCR8rs> {
        Mem2memW::new(self, 14)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ccr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCR8rs;
impl crate::RegisterSpec for CCR8rs {
    type Ux = u32;
}
///`read()` method returns [`ccr8::R`](R) reader structure
impl crate::Readable for CCR8rs {}
///`write(|w| ..)` method takes [`ccr8::W`](W) writer structure
impl crate::Writable for CCR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR8 to value 0
impl crate::Resettable for CCR8rs {
    const RESET_VALUE: u32 = 0;
}
