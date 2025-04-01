#[doc = "Master Clock Configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mccr0(pub u32);
impl Mccr0 {
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn clklo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_clklo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn clkhi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn set_clkhi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn sethold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn set_sethold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Mccr0 {
    #[inline(always)]
    fn default() -> Mccr0 {
        Mccr0(0)
    }
}
#[doc = "Master Clock Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mccr1(pub u32);
impl Mccr1 {
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn clklo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_clklo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn clkhi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn set_clkhi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn sethold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn set_sethold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Mccr1 {
    #[inline(always)]
    fn default() -> Mccr1 {
        Mccr1(0)
    }
}
#[doc = "Master Configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mcfgr0(pub u32);
impl Mcfgr0 {
    #[doc = "Host Request Enable"]
    #[inline(always)]
    pub const fn hren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Enable"]
    #[inline(always)]
    pub const fn set_hren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Request Polarity"]
    #[inline(always)]
    pub const fn hrpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Polarity"]
    #[inline(always)]
    pub const fn set_hrpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host Request Select"]
    #[inline(always)]
    pub const fn hrsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Select"]
    #[inline(always)]
    pub const fn set_hrsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Circular FIFO Enable"]
    #[inline(always)]
    pub const fn cirfifo(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Circular FIFO Enable"]
    #[inline(always)]
    pub const fn set_cirfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Data Match Only"]
    #[inline(always)]
    pub const fn rdmo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Match Only"]
    #[inline(always)]
    pub const fn set_rdmo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Mcfgr0 {
    #[inline(always)]
    fn default() -> Mcfgr0 {
        Mcfgr0(0)
    }
}
#[doc = "Master Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mcfgr1(pub u32);
impl Mcfgr1 {
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Automatic STOP Generation"]
    #[inline(always)]
    pub const fn autostop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic STOP Generation"]
    #[inline(always)]
    pub const fn set_autostop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IGNACK"]
    #[inline(always)]
    pub const fn ignack(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IGNACK"]
    #[inline(always)]
    pub const fn set_ignack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn timecfg(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn set_timecfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn matcfg(&self) -> super::vals::Matcfg {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: super::vals::Matcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pin Configuration"]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::Pincfg {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pincfg::from_bits(val as u8)
    }
    #[doc = "Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::Pincfg) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Mcfgr1 {
    #[inline(always)]
    fn default() -> Mcfgr1 {
        Mcfgr1(0)
    }
}
#[doc = "Master Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mcfgr2(pub u32);
impl Mcfgr2 {
    #[doc = "Bus Idle Timeout"]
    #[inline(always)]
    pub const fn busidle(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus Idle Timeout"]
    #[inline(always)]
    pub const fn set_busidle(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn filtscl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn set_filtscl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SDA"]
    #[inline(always)]
    pub const fn filtsda(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SDA"]
    #[inline(always)]
    pub const fn set_filtsda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Mcfgr2 {
    #[inline(always)]
    fn default() -> Mcfgr2 {
        Mcfgr2(0)
    }
}
#[doc = "Master Configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mcfgr3(pub u32);
impl Mcfgr3 {
    #[doc = "Pin Low Timeout"]
    #[inline(always)]
    pub const fn pinlow(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Pin Low Timeout"]
    #[inline(always)]
    pub const fn set_pinlow(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Mcfgr3 {
    #[inline(always)]
    fn default() -> Mcfgr3 {
        Mcfgr3(0)
    }
}
#[doc = "Master Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Master Enable"]
    #[inline(always)]
    pub const fn men(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Enable"]
    #[inline(always)]
    pub const fn set_men(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
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
    #[doc = "Doze mode enable"]
    #[inline(always)]
    pub const fn dozen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Doze mode enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn rtf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn rrf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
#[doc = "Master DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mder(pub u32);
impl Mder {
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Mder {
    #[inline(always)]
    fn default() -> Mder {
        Mder(0)
    }
}
#[doc = "Master Data Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mdmr(pub u32);
impl Mdmr {
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn match0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn set_match0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Match 1 Value"]
    #[inline(always)]
    pub const fn match1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Match 1 Value"]
    #[inline(always)]
    pub const fn set_match1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Mdmr {
    #[inline(always)]
    fn default() -> Mdmr {
        Mdmr(0)
    }
}
#[doc = "Master FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mfcr(pub u32);
impl Mfcr {
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Mfcr {
    #[inline(always)]
    fn default() -> Mfcr {
        Mfcr(0)
    }
}
#[doc = "Master FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mfsr(pub u32);
impl Mfsr {
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Count"]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Receive FIFO Count"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Mfsr {
    #[inline(always)]
    fn default() -> Mfsr {
        Mfsr(0)
    }
}
#[doc = "Master Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Packet Interrupt Enable"]
    #[inline(always)]
    pub const fn epie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End Packet Interrupt Enable"]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn sdie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn ndie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ndie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub const fn alie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub const fn set_alie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn pltie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pltie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Match Interrupt Enable"]
    #[inline(always)]
    pub const fn dmie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mier {
    #[inline(always)]
    fn default() -> Mier {
        Mier(0)
    }
}
#[doc = "Master Receive Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mrdr(pub u32);
impl Mrdr {
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Empty"]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RX Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mrdr {
    #[inline(always)]
    fn default() -> Mrdr {
        Mrdr(0)
    }
}
#[doc = "Master Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn tdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Packet Flag"]
    #[inline(always)]
    pub const fn epf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End Packet Flag"]
    #[inline(always)]
    pub const fn set_epf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Flag"]
    #[inline(always)]
    pub const fn sdf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Flag"]
    #[inline(always)]
    pub const fn set_sdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "NACK Detect Flag"]
    #[inline(always)]
    pub const fn ndf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NACK Detect Flag"]
    #[inline(always)]
    pub const fn set_ndf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Arbitration Lost Flag"]
    #[inline(always)]
    pub const fn alf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Lost Flag"]
    #[inline(always)]
    pub const fn set_alf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pin Low Timeout Flag"]
    #[inline(always)]
    pub const fn pltf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Low Timeout Flag"]
    #[inline(always)]
    pub const fn set_pltf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Match Flag"]
    #[inline(always)]
    pub const fn dmf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Flag"]
    #[inline(always)]
    pub const fn set_dmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Master Busy Flag"]
    #[inline(always)]
    pub const fn mbf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Master Busy Flag"]
    #[inline(always)]
    pub const fn set_mbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Busy Flag"]
    #[inline(always)]
    pub const fn bbf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Busy Flag"]
    #[inline(always)]
    pub const fn set_bbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0)
    }
}
#[doc = "Master Transmit Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mtdr(pub u32);
impl Mtdr {
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Command Data"]
    #[inline(always)]
    pub const fn cmd(&self) -> super::vals::Cmd {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmd::from_bits(val as u8)
    }
    #[doc = "Command Data"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: super::vals::Cmd) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Mtdr {
    #[inline(always)]
    fn default() -> Mtdr {
        Mtdr(0)
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Master Transmit FIFO Size"]
    #[inline(always)]
    pub const fn mtxfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Master Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_mtxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Master Receive FIFO Size"]
    #[inline(always)]
    pub const fn mrxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Master Receive FIFO Size"]
    #[inline(always)]
    pub const fn set_mrxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
#[doc = "Slave Address Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Samr(pub u32);
impl Samr {
    #[doc = "Address 0 Value"]
    #[inline(always)]
    pub const fn addr0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 0 Value"]
    #[inline(always)]
    pub const fn set_addr0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Address 1 Value"]
    #[inline(always)]
    pub const fn addr1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 1 Value"]
    #[inline(always)]
    pub const fn set_addr1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 17usize)) | (((val as u32) & 0x03ff) << 17usize);
    }
}
impl Default for Samr {
    #[inline(always)]
    fn default() -> Samr {
        Samr(0)
    }
}
#[doc = "Slave Address Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sasr(pub u32);
impl Sasr {
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn raddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn set_raddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Address Not Valid"]
    #[inline(always)]
    pub const fn anv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Address Not Valid"]
    #[inline(always)]
    pub const fn set_anv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Sasr {
    #[inline(always)]
    fn default() -> Sasr {
        Sasr(0)
    }
}
#[doc = "Slave Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Scfgr1(pub u32);
impl Scfgr1 {
    #[doc = "Address SCL Stall"]
    #[inline(always)]
    pub const fn adrstall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Address SCL Stall"]
    #[inline(always)]
    pub const fn set_adrstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX SCL Stall"]
    #[inline(always)]
    pub const fn rxstall(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX SCL Stall"]
    #[inline(always)]
    pub const fn set_rxstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TX Data SCL Stall"]
    #[inline(always)]
    pub const fn txdstall(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data SCL Stall"]
    #[inline(always)]
    pub const fn set_txdstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ACK SCL Stall"]
    #[inline(always)]
    pub const fn ackstall(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ACK SCL Stall"]
    #[inline(always)]
    pub const fn set_ackstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "General Call Enable"]
    #[inline(always)]
    pub const fn gcen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Enable"]
    #[inline(always)]
    pub const fn set_gcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SMBus Alert Enable"]
    #[inline(always)]
    pub const fn saen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Enable"]
    #[inline(always)]
    pub const fn set_saen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit Flag Configuration"]
    #[inline(always)]
    pub const fn txcfg(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Flag Configuration"]
    #[inline(always)]
    pub const fn set_txcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Data Configuration"]
    #[inline(always)]
    pub const fn rxcfg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Configuration"]
    #[inline(always)]
    pub const fn set_rxcfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Ignore NACK"]
    #[inline(always)]
    pub const fn ignack(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore NACK"]
    #[inline(always)]
    pub const fn set_ignack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub const fn hsmen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub const fn set_hsmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Address Configuration"]
    #[inline(always)]
    pub const fn addrcfg(&self) -> super::vals::Addrcfg {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Addrcfg::from_bits(val as u8)
    }
    #[doc = "Address Configuration"]
    #[inline(always)]
    pub const fn set_addrcfg(&mut self, val: super::vals::Addrcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
}
impl Default for Scfgr1 {
    #[inline(always)]
    fn default() -> Scfgr1 {
        Scfgr1(0)
    }
}
#[doc = "Slave Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Scfgr2(pub u32);
impl Scfgr2 {
    #[doc = "Clock Hold Time"]
    #[inline(always)]
    pub const fn clkhold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Hold Time"]
    #[inline(always)]
    pub const fn set_clkhold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn datavd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_datavd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn filtscl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn set_filtscl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SDA"]
    #[inline(always)]
    pub const fn filtsda(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SDA"]
    #[inline(always)]
    pub const fn set_filtsda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Scfgr2 {
    #[inline(always)]
    fn default() -> Scfgr2 {
        Scfgr2(0)
    }
}
#[doc = "Slave Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Slave Enable"]
    #[inline(always)]
    pub const fn sen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Enable"]
    #[inline(always)]
    pub const fn set_sen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
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
    #[doc = "Filter Enable"]
    #[inline(always)]
    pub const fn filten(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Filter Enable"]
    #[inline(always)]
    pub const fn set_filten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Filter Doze Enable"]
    #[inline(always)]
    pub const fn filtdz(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Filter Doze Enable"]
    #[inline(always)]
    pub const fn set_filtdz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn rtf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn rrf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
#[doc = "Slave DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sder(pub u32);
impl Sder {
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid DMA Enable"]
    #[inline(always)]
    pub const fn avde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid DMA Enable"]
    #[inline(always)]
    pub const fn set_avde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Sder {
    #[inline(always)]
    fn default() -> Sder {
        Sder(0)
    }
}
#[doc = "Slave Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sier(pub u32);
impl Sier {
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn avie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_avie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit ACK Interrupt Enable"]
    #[inline(always)]
    pub const fn taie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit ACK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_taie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeated Start Interrupt Enable"]
    #[inline(always)]
    pub const fn rsie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn sdie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn beie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_beie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Address Match 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn am0ie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_am0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Address Match 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn am1ie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_am1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Call Interrupt Enable"]
    #[inline(always)]
    pub const fn gcie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    pub const fn sarie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sarie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Sier {
    #[inline(always)]
    fn default() -> Sier {
        Sier(0)
    }
}
#[doc = "Slave Receive Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Srdr(pub u32);
impl Srdr {
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Empty"]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RX Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Start Of Frame"]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Start Of Frame"]
    #[inline(always)]
    pub const fn set_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Srdr {
    #[inline(always)]
    fn default() -> Srdr {
        Srdr(0)
    }
}
#[doc = "Slave Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ssr(pub u32);
impl Ssr {
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn tdf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Address Valid Flag"]
    #[inline(always)]
    pub const fn avf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Address Valid Flag"]
    #[inline(always)]
    pub const fn set_avf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit ACK Flag"]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit ACK Flag"]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Repeated Start Flag"]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repeated Start Flag"]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "STOP Detect Flag"]
    #[inline(always)]
    pub const fn sdf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Detect Flag"]
    #[inline(always)]
    pub const fn set_sdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit Error Flag"]
    #[inline(always)]
    pub const fn bef(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Flag"]
    #[inline(always)]
    pub const fn set_bef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Address Match 0 Flag"]
    #[inline(always)]
    pub const fn am0f(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 0 Flag"]
    #[inline(always)]
    pub const fn set_am0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Address Match 1 Flag"]
    #[inline(always)]
    pub const fn am1f(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Address Match 1 Flag"]
    #[inline(always)]
    pub const fn set_am1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Call Flag"]
    #[inline(always)]
    pub const fn gcf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Call Flag"]
    #[inline(always)]
    pub const fn set_gcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SMBus Alert Response Flag"]
    #[inline(always)]
    pub const fn sarf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus Alert Response Flag"]
    #[inline(always)]
    pub const fn set_sarf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave Busy Flag"]
    #[inline(always)]
    pub const fn sbf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Busy Flag"]
    #[inline(always)]
    pub const fn set_sbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus Busy Flag"]
    #[inline(always)]
    pub const fn bbf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Busy Flag"]
    #[inline(always)]
    pub const fn set_bbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        Ssr(0)
    }
}
#[doc = "Slave Transmit ACK"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Star(pub u32);
impl Star {
    #[doc = "Transmit NACK"]
    #[inline(always)]
    pub const fn txnack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit NACK"]
    #[inline(always)]
    pub const fn set_txnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Star {
    #[inline(always)]
    fn default() -> Star {
        Star(0)
    }
}
#[doc = "Slave Transmit Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Stdr(pub u32);
impl Stdr {
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Stdr {
    #[inline(always)]
    fn default() -> Stdr {
        Stdr(0)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
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
