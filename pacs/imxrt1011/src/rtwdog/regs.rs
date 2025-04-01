#[doc = "Watchdog Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub const fn cntlow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub const fn set_cntlow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High byte of the Watchdog Counter"]
    #[inline(always)]
    pub const fn cnthigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of the Watchdog Counter"]
    #[inline(always)]
    pub const fn set_cnthigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
#[doc = "Watchdog Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn wait(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Watchdog Test"]
    #[inline(always)]
    pub const fn tst(&self) -> super::vals::Tst {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Tst::from_bits(val as u8)
    }
    #[doc = "Watchdog Test"]
    #[inline(always)]
    pub const fn set_tst(&mut self, val: super::vals::Tst) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Allow updates"]
    #[inline(always)]
    pub const fn update(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Allow updates"]
    #[inline(always)]
    pub const fn set_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Watchdog Interrupt"]
    #[inline(always)]
    pub const fn int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt"]
    #[inline(always)]
    pub const fn set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Watchdog Enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Watchdog Clock"]
    #[inline(always)]
    pub const fn clk(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Watchdog Clock"]
    #[inline(always)]
    pub const fn set_clk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reconfiguration Success"]
    #[inline(always)]
    pub const fn rcs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reconfiguration Success"]
    #[inline(always)]
    pub const fn set_rcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Unlock status"]
    #[inline(always)]
    pub const fn ulk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock status"]
    #[inline(always)]
    pub const fn set_ulk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog prescaler"]
    #[inline(always)]
    pub const fn pres(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog prescaler"]
    #[inline(always)]
    pub const fn set_pres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    pub const fn cmd32en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    pub const fn set_cmd32en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Watchdog Interrupt Flag"]
    #[inline(always)]
    pub const fn flg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt Flag"]
    #[inline(always)]
    pub const fn set_flg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Watchdog Window"]
    #[inline(always)]
    pub const fn win(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Window"]
    #[inline(always)]
    pub const fn set_win(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(0)
    }
}
#[doc = "Watchdog Timeout Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Toval(pub u32);
impl Toval {
    #[doc = "Low byte of the timeout value"]
    #[inline(always)]
    pub const fn tovallow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of the timeout value"]
    #[inline(always)]
    pub const fn set_tovallow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High byte of the timeout value"]
    #[inline(always)]
    pub const fn tovalhigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of the timeout value"]
    #[inline(always)]
    pub const fn set_tovalhigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Toval {
    #[inline(always)]
    fn default() -> Toval {
        Toval(0)
    }
}
#[doc = "Watchdog Window Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Win(pub u32);
impl Win {
    #[doc = "Low byte of Watchdog Window"]
    #[inline(always)]
    pub const fn winlow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of Watchdog Window"]
    #[inline(always)]
    pub const fn set_winlow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High byte of Watchdog Window"]
    #[inline(always)]
    pub const fn winhigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of Watchdog Window"]
    #[inline(always)]
    pub const fn set_winhigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Win {
    #[inline(always)]
    fn default() -> Win {
        Win(0)
    }
}
