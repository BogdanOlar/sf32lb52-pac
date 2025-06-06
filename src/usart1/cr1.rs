///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `UE` reader - USART enable 0: disabled 1: enabled
pub type UeR = crate::BitReader;
///Field `UE` writer - USART enable 0: disabled 1: enabled
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RE` reader - Receiver enable 0: receiver is disabled 1: receiver is enabled
pub type ReR = crate::BitReader;
///Field `RE` writer - Receiver enable 0: receiver is disabled 1: receiver is enabled
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - Transmitter enable 0: transmitter is disabled 1: transmitter is enabled
pub type TeR = crate::BitReader;
///Field `TE` writer - Transmitter enable 0: transmitter is disabled 1: transmitter is enabled
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEIE` reader - Idle line interrupt enable 0: interrupt disabled 1: interrupt is generated whenever IDLE=1 in the ISR register
pub type IdleieR = crate::BitReader;
///Field `IDLEIE` writer - Idle line interrupt enable 0: interrupt disabled 1: interrupt is generated whenever IDLE=1 in the ISR register
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEIE` reader - Rx not empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenever RXNE=1 in the ISR register
pub type RxneieR = crate::BitReader;
///Field `RXNEIE` writer - Rx not empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenever RXNE=1 in the ISR register
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer compelete interrupt enable 0: interrupt disabled 1: interrupt is generated whenever TC=1 in the ISR register
pub type TcieR = crate::BitReader;
///Field `TCIE` writer - Transfer compelete interrupt enable 0: interrupt disabled 1: interrupt is generated whenever TC=1 in the ISR register
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEIE` reader - Tx empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenver TXE=1 in the ISR register
pub type TxeieR = crate::BitReader;
///Field `TXEIE` writer - Tx empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenver TXE=1 in the ISR register
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIE` reader - Parity error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever PE=1 in the ISR register
pub type PeieR = crate::BitReader;
///Field `PEIE` writer - Parity error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever PE=1 in the ISR register
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - Parity select 0: even parity 1: odd parity
pub type PsR = crate::BitReader;
///Field `PS` writer - Parity select 0: even parity 1: odd parity
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCE` reader - Parity check enable. If enabled, parity bit is inserted at the MSB position 0: parity check disabled 1: parity check enabled
pub type PceR = crate::BitReader;
///Field `PCE` writer - Parity check enable. If enabled, parity bit is inserted at the MSB position 0: parity check disabled 1: parity check enabled
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVER8` reader - Oversampling mode 0: Oversampling by 16 1: Oversampling by 8
pub type Over8R = crate::BitReader;
///Field `OVER8` writer - Oversampling mode 0: Oversampling by 16 1: Oversampling by 8
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M` reader - Mode bit indicates the length of the packet, including data bits and parity. Stop bits not included. 0: 6 bits (e.g. 6 data bits + no parity bit) 1: 7 bits (e.g. 6 data bits + 1 parity bit) 2: 8 bits (e.g. 7 data bits + 1 parity bit, or 6 data bits + 2 parity bits) 3: 9 bits (e.g. 8 data bits + 1 parity bit, or 7 data bits + 2 parity bits)
pub type MR = crate::FieldReader;
///Field `M` writer - Mode bit indicates the length of the packet, including data bits and parity. Stop bits not included. 0: 6 bits (e.g. 6 data bits + no parity bit) 1: 7 bits (e.g. 6 data bits + 1 parity bit) 2: 8 bits (e.g. 7 data bits + 1 parity bit, or 6 data bits + 2 parity bits) 3: 9 bits (e.g. 8 data bits + 1 parity bit, or 7 data bits + 2 parity bits)
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - USART enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Receiver enable 0: receiver is disabled 1: receiver is enabled
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable 0: transmitter is disabled 1: transmitter is enabled
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line interrupt enable 0: interrupt disabled 1: interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx not empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenever RXNE=1 in the ISR register
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transfer compelete interrupt enable 0: interrupt disabled 1: interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tx empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenver TXE=1 in the ISR register
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Parity error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity select 0: even parity 1: odd parity
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity check enable. If enabled, parity bit is inserted at the MSB position 0: parity check disabled 1: parity check enabled
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Oversampling mode 0: Oversampling by 16 1: Oversampling by 8
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 27:28 - Mode bit indicates the length of the packet, including data bits and parity. Stop bits not included. 0: 6 bits (e.g. 6 data bits + no parity bit) 1: 7 bits (e.g. 6 data bits + 1 parity bit) 2: 8 bits (e.g. 7 data bits + 1 parity bit, or 6 data bits + 2 parity bits) 3: 9 bits (e.g. 8 data bits + 1 parity bit, or 7 data bits + 2 parity bits)
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("m", &self.m())
            .field("over8", &self.over8())
            .field("pce", &self.pce())
            .field("ps", &self.ps())
            .field("peie", &self.peie())
            .field("txeie", &self.txeie())
            .field("tcie", &self.tcie())
            .field("rxneie", &self.rxneie())
            .field("idleie", &self.idleie())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("ue", &self.ue())
            .finish()
    }
}
impl W {
    ///Bit 0 - USART enable 0: disabled 1: enabled
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<CR1rs> {
        UeW::new(self, 0)
    }
    ///Bit 2 - Receiver enable 0: receiver is disabled 1: receiver is enabled
    #[inline(always)]
    pub fn re(&mut self) -> ReW<CR1rs> {
        ReW::new(self, 2)
    }
    ///Bit 3 - Transmitter enable 0: transmitter is disabled 1: transmitter is enabled
    #[inline(always)]
    pub fn te(&mut self) -> TeW<CR1rs> {
        TeW::new(self, 3)
    }
    ///Bit 4 - Idle line interrupt enable 0: interrupt disabled 1: interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<CR1rs> {
        IdleieW::new(self, 4)
    }
    ///Bit 5 - Rx not empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenever RXNE=1 in the ISR register
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<CR1rs> {
        RxneieW::new(self, 5)
    }
    ///Bit 6 - Transfer compelete interrupt enable 0: interrupt disabled 1: interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<CR1rs> {
        TcieW::new(self, 6)
    }
    ///Bit 7 - Tx empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenver TXE=1 in the ISR register
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<CR1rs> {
        TxeieW::new(self, 7)
    }
    ///Bit 8 - Parity error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<CR1rs> {
        PeieW::new(self, 8)
    }
    ///Bit 9 - Parity select 0: even parity 1: odd parity
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<CR1rs> {
        PsW::new(self, 9)
    }
    ///Bit 10 - Parity check enable. If enabled, parity bit is inserted at the MSB position 0: parity check disabled 1: parity check enabled
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<CR1rs> {
        PceW::new(self, 10)
    }
    ///Bit 14 - Oversampling mode 0: Oversampling by 16 1: Oversampling by 8
    #[inline(always)]
    pub fn over8(&mut self) -> Over8W<CR1rs> {
        Over8W::new(self, 14)
    }
    ///Bits 27:28 - Mode bit indicates the length of the packet, including data bits and parity. Stop bits not included. 0: 6 bits (e.g. 6 data bits + no parity bit) 1: 7 bits (e.g. 6 data bits + 1 parity bit) 2: 8 bits (e.g. 7 data bits + 1 parity bit, or 6 data bits + 2 parity bits) 3: 9 bits (e.g. 8 data bits + 1 parity bit, or 7 data bits + 2 parity bits)
    #[inline(always)]
    pub fn m(&mut self) -> MW<CR1rs> {
        MW::new(self, 27)
    }
}
///Control Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
