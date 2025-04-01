#[doc = "LPUART Baud Rate Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc = "Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub const fn sbr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub const fn set_sbr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Stop Bit Number Select"]
    #[inline(always)]
    pub const fn sbns(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Bit Number Select"]
    #[inline(always)]
    pub const fn set_sbns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub const fn rxedgie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxedgie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn lbkdie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lbkdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Resynchronization Disable"]
    #[inline(always)]
    pub const fn resyncdis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Resynchronization Disable"]
    #[inline(always)]
    pub const fn set_resyncdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Both Edge Sampling"]
    #[inline(always)]
    pub const fn bothedge(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Both Edge Sampling"]
    #[inline(always)]
    pub const fn set_bothedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn matcfg(&self) -> super::vals::Matcfg {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: super::vals::Matcfg) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Receiver Idle DMA Enable"]
    #[inline(always)]
    pub const fn ridmae(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Idle DMA Enable"]
    #[inline(always)]
    pub const fn set_ridmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receiver Full DMA Enable"]
    #[inline(always)]
    pub const fn rdmae(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Full DMA Enable"]
    #[inline(always)]
    pub const fn set_rdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmitter DMA Enable"]
    #[inline(always)]
    pub const fn tdmae(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter DMA Enable"]
    #[inline(always)]
    pub const fn set_tdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Oversampling Ratio"]
    #[inline(always)]
    pub const fn osr(&self) -> super::vals::Osr {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Osr::from_bits(val as u8)
    }
    #[doc = "Oversampling Ratio"]
    #[inline(always)]
    pub const fn set_osr(&mut self, val: super::vals::Osr) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
    #[doc = "10-bit Mode select"]
    #[inline(always)]
    pub const fn m10(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "10-bit Mode select"]
    #[inline(always)]
    pub const fn set_m10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Match Address Mode Enable 2"]
    #[inline(always)]
    pub const fn maen2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Match Address Mode Enable 2"]
    #[inline(always)]
    pub const fn set_maen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Match Address Mode Enable 1"]
    #[inline(always)]
    pub const fn maen1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Match Address Mode Enable 1"]
    #[inline(always)]
    pub const fn set_maen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Baud {
    #[inline(always)]
    fn default() -> Baud {
        Baud(0)
    }
}
#[doc = "LPUART Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Parity Type"]
    #[inline(always)]
    pub const fn pt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Type"]
    #[inline(always)]
    pub const fn set_pt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Idle Line Type Select"]
    #[inline(always)]
    pub const fn ilt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Type Select"]
    #[inline(always)]
    pub const fn set_ilt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver Wakeup Method Select"]
    #[inline(always)]
    pub const fn wake(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Wakeup Method Select"]
    #[inline(always)]
    pub const fn set_wake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receiver Source Select"]
    #[inline(always)]
    pub const fn rsrc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Source Select"]
    #[inline(always)]
    pub const fn set_rsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn dozeen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Loop Mode Select"]
    #[inline(always)]
    pub const fn loops(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop Mode Select"]
    #[inline(always)]
    pub const fn set_loops(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Idle Configuration"]
    #[inline(always)]
    pub const fn idlecfg(&self) -> super::vals::Idlecfg {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Idlecfg::from_bits(val as u8)
    }
    #[doc = "Idle Configuration"]
    #[inline(always)]
    pub const fn set_idlecfg(&mut self, val: super::vals::Idlecfg) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "7-Bit Mode Select"]
    #[inline(always)]
    pub const fn m7(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "7-Bit Mode Select"]
    #[inline(always)]
    pub const fn set_m7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Match 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn ma2ie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Match 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ma2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Match 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn ma1ie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Match 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ma1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Send Break"]
    #[inline(always)]
    pub const fn sbk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Send Break"]
    #[inline(always)]
    pub const fn set_sbk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receiver Wakeup Control"]
    #[inline(always)]
    pub const fn rwu(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Wakeup Control"]
    #[inline(always)]
    pub const fn set_rwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Idle Line Interrupt Enable"]
    #[inline(always)]
    pub const fn ilie(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ilie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receiver Interrupt Enable"]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Parity Error Interrupt Enable"]
    #[inline(always)]
    pub const fn peie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_peie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Framing Error Interrupt Enable"]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Noise Error Interrupt Enable"]
    #[inline(always)]
    pub const fn neie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_neie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Overrun Interrupt Enable"]
    #[inline(always)]
    pub const fn orie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Interrupt Enable"]
    #[inline(always)]
    pub const fn set_orie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Data Inversion"]
    #[inline(always)]
    pub const fn txinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Inversion"]
    #[inline(always)]
    pub const fn set_txinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub const fn txdir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub const fn set_txdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub const fn r9t8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub const fn set_r9t8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub const fn r8t9(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub const fn set_r8t9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "LPUART Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "R0T0"]
    #[inline(always)]
    pub const fn r0t0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "R0T0"]
    #[inline(always)]
    pub const fn set_r0t0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "R1T1"]
    #[inline(always)]
    pub const fn r1t1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "R1T1"]
    #[inline(always)]
    pub const fn set_r1t1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "R2T2"]
    #[inline(always)]
    pub const fn r2t2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "R2T2"]
    #[inline(always)]
    pub const fn set_r2t2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "R3T3"]
    #[inline(always)]
    pub const fn r3t3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "R3T3"]
    #[inline(always)]
    pub const fn set_r3t3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "R4T4"]
    #[inline(always)]
    pub const fn r4t4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "R4T4"]
    #[inline(always)]
    pub const fn set_r4t4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "R5T5"]
    #[inline(always)]
    pub const fn r5t5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "R5T5"]
    #[inline(always)]
    pub const fn set_r5t5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "R6T6"]
    #[inline(always)]
    pub const fn r6t6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "R6T6"]
    #[inline(always)]
    pub const fn set_r6t6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "R7T7"]
    #[inline(always)]
    pub const fn r7t7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "R7T7"]
    #[inline(always)]
    pub const fn set_r7t7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "R8T8"]
    #[inline(always)]
    pub const fn r8t8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "R8T8"]
    #[inline(always)]
    pub const fn set_r8t8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "R9T9"]
    #[inline(always)]
    pub const fn r9t9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "R9T9"]
    #[inline(always)]
    pub const fn set_r9t9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Idle Line"]
    #[inline(always)]
    pub const fn idline(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line"]
    #[inline(always)]
    pub const fn set_idline(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Buffer Empty"]
    #[inline(always)]
    pub const fn rxempt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Empty"]
    #[inline(always)]
    pub const fn set_rxempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub const fn fretsc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub const fn set_fretsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error"]
    #[inline(always)]
    pub const fn paritye(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error"]
    #[inline(always)]
    pub const fn set_paritye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Noisy Data Received"]
    #[inline(always)]
    pub const fn noisy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Noisy Data Received"]
    #[inline(always)]
    pub const fn set_noisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
#[doc = "LPUART FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "Receive FIFO Buffer Depth"]
    #[inline(always)]
    pub const fn rxfifosize(&self) -> super::vals::Rxfifosize {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Rxfifosize::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Buffer Depth"]
    #[inline(always)]
    pub const fn set_rxfifosize(&mut self, val: super::vals::Rxfifosize) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Enable"]
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Enable"]
    #[inline(always)]
    pub const fn set_rxfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO Buffer Depth"]
    #[inline(always)]
    pub const fn txfifosize(&self) -> super::vals::Txfifosize {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Txfifosize::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Buffer Depth"]
    #[inline(always)]
    pub const fn set_txfifosize(&mut self, val: super::vals::Txfifosize) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn set_txfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn rxufe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxufe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn txofe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_txofe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver Idle Empty Enable"]
    #[inline(always)]
    pub const fn rxiden(&self) -> super::vals::Rxiden {
        let val = (self.0 >> 10usize) & 0x07;
        super::vals::Rxiden::from_bits(val as u8)
    }
    #[doc = "Receiver Idle Empty Enable"]
    #[inline(always)]
    pub const fn set_rxiden(&mut self, val: super::vals::Rxiden) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
    }
    #[doc = "Receive FIFO Flush"]
    #[inline(always)]
    pub const fn rxflush(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Flush"]
    #[inline(always)]
    pub const fn set_rxflush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmit FIFO Flush"]
    #[inline(always)]
    pub const fn txflush(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Flush"]
    #[inline(always)]
    pub const fn set_txflush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Receive FIFO/Buffer Empty"]
    #[inline(always)]
    pub const fn rxempt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO/Buffer Empty"]
    #[inline(always)]
    pub const fn set_rxempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit FIFO/Buffer Empty"]
    #[inline(always)]
    pub const fn txempt(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO/Buffer Empty"]
    #[inline(always)]
    pub const fn set_txempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
#[doc = "LPUART Global Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Global(pub u32);
impl Global {
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Global {
    #[inline(always)]
    fn default() -> Global {
        Global(0)
    }
}
#[doc = "LPUART Match Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Match Address 1"]
    #[inline(always)]
    pub const fn ma1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 1"]
    #[inline(always)]
    pub const fn set_ma1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Match Address 2"]
    #[inline(always)]
    pub const fn ma2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 2"]
    #[inline(always)]
    pub const fn set_ma2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
#[doc = "LPUART Modem IrDA Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Modir(pub u32);
impl Modir {
    #[doc = "Transmitter clear-to-send enable"]
    #[inline(always)]
    pub const fn txctse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter clear-to-send enable"]
    #[inline(always)]
    pub const fn set_txctse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmitter request-to-send enable"]
    #[inline(always)]
    pub const fn txrtse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter request-to-send enable"]
    #[inline(always)]
    pub const fn set_txrtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter request-to-send polarity"]
    #[inline(always)]
    pub const fn txrtspol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter request-to-send polarity"]
    #[inline(always)]
    pub const fn set_txrtspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver request-to-send enable"]
    #[inline(always)]
    pub const fn rxrtse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver request-to-send enable"]
    #[inline(always)]
    pub const fn set_rxrtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit CTS Configuration"]
    #[inline(always)]
    pub const fn txctsc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit CTS Configuration"]
    #[inline(always)]
    pub const fn set_txctsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit CTS Source"]
    #[inline(always)]
    pub const fn txctssrc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit CTS Source"]
    #[inline(always)]
    pub const fn set_txctssrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive RTS Configuration"]
    #[inline(always)]
    pub const fn rtswater(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Receive RTS Configuration"]
    #[inline(always)]
    pub const fn set_rtswater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Transmitter narrow pulse"]
    #[inline(always)]
    pub const fn tnp(&self) -> super::vals::Tnp {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Tnp::from_bits(val as u8)
    }
    #[doc = "Transmitter narrow pulse"]
    #[inline(always)]
    pub const fn set_tnp(&mut self, val: super::vals::Tnp) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Infrared enable"]
    #[inline(always)]
    pub const fn iren(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Infrared enable"]
    #[inline(always)]
    pub const fn set_iren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Modir {
    #[inline(always)]
    fn default() -> Modir {
        Modir(0)
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Transmit FIFO Size"]
    #[inline(always)]
    pub const fn txfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_txfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive FIFO Size"]
    #[inline(always)]
    pub const fn rxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Size"]
    #[inline(always)]
    pub const fn set_rxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
#[doc = "LPUART Pin Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pincfg(pub u32);
impl Pincfg {
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn trgsel(&self) -> super::vals::Trgsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Trgsel::from_bits(val as u8)
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: super::vals::Trgsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Pincfg {
    #[inline(always)]
    fn default() -> Pincfg {
        Pincfg(0)
    }
}
#[doc = "LPUART Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Match 2 Flag"]
    #[inline(always)]
    pub const fn ma2f(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Match 2 Flag"]
    #[inline(always)]
    pub const fn set_ma2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Match 1 Flag"]
    #[inline(always)]
    pub const fn ma1f(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Match 1 Flag"]
    #[inline(always)]
    pub const fn set_ma1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn pf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_pf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub const fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Noise Flag"]
    #[inline(always)]
    pub const fn nf(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Flag"]
    #[inline(always)]
    pub const fn set_nf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Receiver Overrun Flag"]
    #[inline(always)]
    pub const fn or(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Overrun Flag"]
    #[inline(always)]
    pub const fn set_or(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Idle Line Flag"]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Flag"]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receive Data Register Full Flag"]
    #[inline(always)]
    pub const fn rdrf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Register Full Flag"]
    #[inline(always)]
    pub const fn set_rdrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmission Complete Flag"]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Complete Flag"]
    #[inline(always)]
    pub const fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub const fn tdre(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Receiver Active Flag"]
    #[inline(always)]
    pub const fn raf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Active Flag"]
    #[inline(always)]
    pub const fn set_raf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LIN Break Detection Enable"]
    #[inline(always)]
    pub const fn lbkde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detection Enable"]
    #[inline(always)]
    pub const fn set_lbkde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Break Character Generation Length"]
    #[inline(always)]
    pub const fn brk13(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Break Character Generation Length"]
    #[inline(always)]
    pub const fn set_brk13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub const fn rwuid(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub const fn set_rwuid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Receive Data Inversion"]
    #[inline(always)]
    pub const fn rxinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Inversion"]
    #[inline(always)]
    pub const fn set_rxinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn msbf(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn set_msbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub const fn rxedgif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub const fn set_rxedgif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub const fn lbkdif(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub const fn set_lbkdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Identification Number"]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Identification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
#[doc = "LPUART Watermark Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Water(pub u32);
impl Water {
    #[doc = "Transmit Watermark"]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit Watermark"]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Transmit Counter"]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Counter"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Receive Watermark"]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Watermark"]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Receive Counter"]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Counter"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Water {
    #[inline(always)]
    fn default() -> Water {
        Water(0)
    }
}
