#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of Datalines"]
    #[inline(always)]
    pub const fn dataline(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Datalines"]
    #[inline(always)]
    pub const fn set_dataline(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Size"]
    #[inline(always)]
    pub const fn fifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Size"]
    #[inline(always)]
    pub const fn set_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn frame(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_frame(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
#[doc = "Receive Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcr1(pub u32);
impl Rcr1 {
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn rfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn set_rfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        Rcr1(0)
    }
}
#[doc = "Receive Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcr2(pub u32);
impl Rcr2 {
    #[doc = "Bit Clock Divide"]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit Clock Divide"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit Clock Direction"]
    #[inline(always)]
    pub const fn bcd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Direction"]
    #[inline(always)]
    pub const fn set_bcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Clock Polarity"]
    #[inline(always)]
    pub const fn bcp(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Polarity"]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "MCLK Select"]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Rcr2msel {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Rcr2msel::from_bits(val as u8)
    }
    #[doc = "MCLK Select"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Rcr2msel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Bit Clock Input"]
    #[inline(always)]
    pub const fn bci(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Input"]
    #[inline(always)]
    pub const fn set_bci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit Clock Swap"]
    #[inline(always)]
    pub const fn bcs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Swap"]
    #[inline(always)]
    pub const fn set_bcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        Rcr2(0)
    }
}
#[doc = "Receive Configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcr3(pub u32);
impl Rcr3 {
    #[doc = "Word Flag Configuration"]
    #[inline(always)]
    pub const fn wdfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Word Flag Configuration"]
    #[inline(always)]
    pub const fn set_wdfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Receive Channel Enable"]
    #[inline(always)]
    pub const fn rce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Channel Enable"]
    #[inline(always)]
    pub const fn set_rce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Channel FIFO Reset"]
    #[inline(always)]
    pub const fn cfr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Channel FIFO Reset"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Rcr3 {
    #[inline(always)]
    fn default() -> Rcr3 {
        Rcr3(0)
    }
}
#[doc = "Receive Configuration 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcr4(pub u32);
impl Rcr4 {
    #[doc = "Frame Sync Direction"]
    #[inline(always)]
    pub const fn fsd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Direction"]
    #[inline(always)]
    pub const fn set_fsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Sync Polarity"]
    #[inline(always)]
    pub const fn fsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Polarity"]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "On Demand Mode"]
    #[inline(always)]
    pub const fn ondem(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "On Demand Mode"]
    #[inline(always)]
    pub const fn set_ondem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Sync Early"]
    #[inline(always)]
    pub const fn fse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Early"]
    #[inline(always)]
    pub const fn set_fse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn mf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Sync Width"]
    #[inline(always)]
    pub const fn sywd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sync Width"]
    #[inline(always)]
    pub const fn set_sywd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn frsz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_frsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIFO Packing Mode"]
    #[inline(always)]
    pub const fn fpack(&self) -> super::vals::Rcr4fpack {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Rcr4fpack::from_bits(val as u8)
    }
    #[doc = "FIFO Packing Mode"]
    #[inline(always)]
    pub const fn set_fpack(&mut self, val: super::vals::Rcr4fpack) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FIFO Combine Mode"]
    #[inline(always)]
    pub const fn fcomb(&self) -> super::vals::Rcr4fcomb {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Rcr4fcomb::from_bits(val as u8)
    }
    #[doc = "FIFO Combine Mode"]
    #[inline(always)]
    pub const fn set_fcomb(&mut self, val: super::vals::Rcr4fcomb) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Continue on Error"]
    #[inline(always)]
    pub const fn fcont(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Continue on Error"]
    #[inline(always)]
    pub const fn set_fcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        Rcr4(0)
    }
}
#[doc = "Receive Configuration 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcr5(pub u32);
impl Rcr5 {
    #[doc = "First Bit Shifted"]
    #[inline(always)]
    pub const fn fbt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "First Bit Shifted"]
    #[inline(always)]
    pub const fn set_fbt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Word 0 Width"]
    #[inline(always)]
    pub const fn w0w(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Word 0 Width"]
    #[inline(always)]
    pub const fn set_w0w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Word N Width"]
    #[inline(always)]
    pub const fn wnw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Word N Width"]
    #[inline(always)]
    pub const fn set_wnw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Rcr5 {
    #[inline(always)]
    fn default() -> Rcr5 {
        Rcr5(0)
    }
}
#[doc = "Receive Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcsr(pub u32);
impl Rcsr {
    #[doc = "FIFO Request DMA Enable"]
    #[inline(always)]
    pub const fn frde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request DMA Enable"]
    #[inline(always)]
    pub const fn set_frde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[inline(always)]
    pub const fn fwde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[inline(always)]
    pub const fn set_fwde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub const fn frie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub const fn set_frie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn fwie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word Start Interrupt Enable"]
    #[inline(always)]
    pub const fn wsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FIFO Request Flag"]
    #[inline(always)]
    pub const fn frf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Flag"]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Warning Flag"]
    #[inline(always)]
    pub const fn fwf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Flag"]
    #[inline(always)]
    pub const fn set_fwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Sync Error Flag"]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Flag"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Word Start Flag"]
    #[inline(always)]
    pub const fn wsf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Flag"]
    #[inline(always)]
    pub const fn set_wsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn fr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit Clock Enable"]
    #[inline(always)]
    pub const fn bce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Enable"]
    #[inline(always)]
    pub const fn set_bce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn stope(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rcsr {
    #[inline(always)]
    fn default() -> Rcsr {
        Rcsr(0)
    }
}
#[doc = "Receive FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rfr(pub u32);
impl Rfr {
    #[doc = "Read FIFO Pointer"]
    #[inline(always)]
    pub const fn rfp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Read FIFO Pointer"]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Receive Channel Pointer"]
    #[inline(always)]
    pub const fn rcp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Channel Pointer"]
    #[inline(always)]
    pub const fn set_rcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write FIFO Pointer"]
    #[inline(always)]
    pub const fn wfp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Write FIFO Pointer"]
    #[inline(always)]
    pub const fn set_wfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Rfr {
    #[inline(always)]
    fn default() -> Rfr {
        Rfr(0)
    }
}
#[doc = "Receive Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rmr(pub u32);
impl Rmr {
    #[doc = "Receive Word Mask"]
    #[inline(always)]
    pub const fn rwm(&self) -> super::vals::Rwm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Rwm::from_bits(val as u32)
    }
    #[doc = "Receive Word Mask"]
    #[inline(always)]
    pub const fn set_rwm(&mut self, val: super::vals::Rwm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rmr {
    #[inline(always)]
    fn default() -> Rmr {
        Rmr(0)
    }
}
#[doc = "Transmit Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tcr1(pub u32);
impl Tcr1 {
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn tfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_tfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Tcr1 {
    #[inline(always)]
    fn default() -> Tcr1 {
        Tcr1(0)
    }
}
#[doc = "Transmit Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tcr2(pub u32);
impl Tcr2 {
    #[doc = "Bit Clock Divide"]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit Clock Divide"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit Clock Direction"]
    #[inline(always)]
    pub const fn bcd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Direction"]
    #[inline(always)]
    pub const fn set_bcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Clock Polarity"]
    #[inline(always)]
    pub const fn bcp(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Polarity"]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "MCLK Select"]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Tcr2msel {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Tcr2msel::from_bits(val as u8)
    }
    #[doc = "MCLK Select"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Tcr2msel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Bit Clock Input"]
    #[inline(always)]
    pub const fn bci(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Input"]
    #[inline(always)]
    pub const fn set_bci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit Clock Swap"]
    #[inline(always)]
    pub const fn bcs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Swap"]
    #[inline(always)]
    pub const fn set_bcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Tcr2 {
    #[inline(always)]
    fn default() -> Tcr2 {
        Tcr2(0)
    }
}
#[doc = "Transmit Configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tcr3(pub u32);
impl Tcr3 {
    #[doc = "Word Flag Configuration"]
    #[inline(always)]
    pub const fn wdfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Word Flag Configuration"]
    #[inline(always)]
    pub const fn set_wdfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Transmit Channel Enable"]
    #[inline(always)]
    pub const fn tce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit Channel Enable"]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Channel FIFO Reset"]
    #[inline(always)]
    pub const fn cfr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Channel FIFO Reset"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Tcr3 {
    #[inline(always)]
    fn default() -> Tcr3 {
        Tcr3(0)
    }
}
#[doc = "Transmit Configuration 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tcr4(pub u32);
impl Tcr4 {
    #[doc = "Frame Sync Direction"]
    #[inline(always)]
    pub const fn fsd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Direction"]
    #[inline(always)]
    pub const fn set_fsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Sync Polarity"]
    #[inline(always)]
    pub const fn fsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Polarity"]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "On Demand Mode"]
    #[inline(always)]
    pub const fn ondem(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "On Demand Mode"]
    #[inline(always)]
    pub const fn set_ondem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Sync Early"]
    #[inline(always)]
    pub const fn fse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Early"]
    #[inline(always)]
    pub const fn set_fse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn mf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel Mode"]
    #[inline(always)]
    pub const fn chmod(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Mode"]
    #[inline(always)]
    pub const fn set_chmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sync Width"]
    #[inline(always)]
    pub const fn sywd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sync Width"]
    #[inline(always)]
    pub const fn set_sywd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Frame size"]
    #[inline(always)]
    pub const fn frsz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Frame size"]
    #[inline(always)]
    pub const fn set_frsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIFO Packing Mode"]
    #[inline(always)]
    pub const fn fpack(&self) -> super::vals::Tcr4fpack {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Tcr4fpack::from_bits(val as u8)
    }
    #[doc = "FIFO Packing Mode"]
    #[inline(always)]
    pub const fn set_fpack(&mut self, val: super::vals::Tcr4fpack) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FIFO Combine Mode"]
    #[inline(always)]
    pub const fn fcomb(&self) -> super::vals::Tcr4fcomb {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Tcr4fcomb::from_bits(val as u8)
    }
    #[doc = "FIFO Combine Mode"]
    #[inline(always)]
    pub const fn set_fcomb(&mut self, val: super::vals::Tcr4fcomb) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Continue on Error"]
    #[inline(always)]
    pub const fn fcont(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Continue on Error"]
    #[inline(always)]
    pub const fn set_fcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Tcr4 {
    #[inline(always)]
    fn default() -> Tcr4 {
        Tcr4(0)
    }
}
#[doc = "Transmit Configuration 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tcr5(pub u32);
impl Tcr5 {
    #[doc = "First Bit Shifted"]
    #[inline(always)]
    pub const fn fbt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "First Bit Shifted"]
    #[inline(always)]
    pub const fn set_fbt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Word 0 Width"]
    #[inline(always)]
    pub const fn w0w(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Word 0 Width"]
    #[inline(always)]
    pub const fn set_w0w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Word N Width"]
    #[inline(always)]
    pub const fn wnw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Word N Width"]
    #[inline(always)]
    pub const fn set_wnw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Tcr5 {
    #[inline(always)]
    fn default() -> Tcr5 {
        Tcr5(0)
    }
}
#[doc = "Transmit Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tcsr(pub u32);
impl Tcsr {
    #[doc = "FIFO Request DMA Enable"]
    #[inline(always)]
    pub const fn frde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request DMA Enable"]
    #[inline(always)]
    pub const fn set_frde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[inline(always)]
    pub const fn fwde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[inline(always)]
    pub const fn set_fwde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub const fn frie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub const fn set_frie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn fwie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word Start Interrupt Enable"]
    #[inline(always)]
    pub const fn wsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FIFO Request Flag"]
    #[inline(always)]
    pub const fn frf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Flag"]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Warning Flag"]
    #[inline(always)]
    pub const fn fwf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Flag"]
    #[inline(always)]
    pub const fn set_fwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Sync Error Flag"]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Flag"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Word Start Flag"]
    #[inline(always)]
    pub const fn wsf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Flag"]
    #[inline(always)]
    pub const fn set_wsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn fr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit Clock Enable"]
    #[inline(always)]
    pub const fn bce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Enable"]
    #[inline(always)]
    pub const fn set_bce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn stope(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcsr {
    #[inline(always)]
    fn default() -> Tcsr {
        Tcsr(0)
    }
}
#[doc = "Transmit FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tfr(pub u32);
impl Tfr {
    #[doc = "Read FIFO Pointer"]
    #[inline(always)]
    pub const fn rfp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Read FIFO Pointer"]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Write FIFO Pointer"]
    #[inline(always)]
    pub const fn wfp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Write FIFO Pointer"]
    #[inline(always)]
    pub const fn set_wfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Write Channel Pointer"]
    #[inline(always)]
    pub const fn wcp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write Channel Pointer"]
    #[inline(always)]
    pub const fn set_wcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tfr {
    #[inline(always)]
    fn default() -> Tfr {
        Tfr(0)
    }
}
#[doc = "Transmit Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tmr(pub u32);
impl Tmr {
    #[doc = "Transmit Word Mask"]
    #[inline(always)]
    pub const fn twm(&self) -> super::vals::Twm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Twm::from_bits(val as u32)
    }
    #[doc = "Transmit Word Mask"]
    #[inline(always)]
    pub const fn set_twm(&mut self, val: super::vals::Twm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tmr {
    #[inline(always)]
    fn default() -> Tmr {
        Tmr(0)
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
