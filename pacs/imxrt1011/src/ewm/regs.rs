#[doc = "Clock Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clkctrl(pub u32);
impl Clkctrl {
    #[doc = "CLKSEL"]
    #[inline(always)]
    pub const fn clksel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CLKSEL"]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Clkctrl {
    #[inline(always)]
    fn default() -> Clkctrl {
        Clkctrl(0)
    }
}
#[doc = "Clock Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clkprescaler(pub u32);
impl Clkprescaler {
    #[doc = "CLK_DIV"]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CLK_DIV"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Clkprescaler {
    #[inline(always)]
    fn default() -> Clkprescaler {
        Clkprescaler(0)
    }
}
#[doc = "Compare High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cmph(pub u32);
impl Cmph {
    #[doc = "COMPAREH"]
    #[inline(always)]
    pub const fn compareh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COMPAREH"]
    #[inline(always)]
    pub const fn set_compareh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmph {
    #[inline(always)]
    fn default() -> Cmph {
        Cmph(0)
    }
}
#[doc = "Compare Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cmpl(pub u32);
impl Cmpl {
    #[doc = "COMPAREL"]
    #[inline(always)]
    pub const fn comparel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "COMPAREL"]
    #[inline(always)]
    pub const fn set_comparel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmpl {
    #[inline(always)]
    fn default() -> Cmpl {
        Cmpl(0)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "EWM enable."]
    #[inline(always)]
    pub const fn ewmen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EWM enable."]
    #[inline(always)]
    pub const fn set_ewmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EWM_in's Assertion State Select."]
    #[inline(always)]
    pub const fn assin(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EWM_in's Assertion State Select."]
    #[inline(always)]
    pub const fn set_assin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input Enable."]
    #[inline(always)]
    pub const fn inen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable."]
    #[inline(always)]
    pub const fn set_inen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn inten(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn set_inten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Service Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Serv(pub u32);
impl Serv {
    #[doc = "SERVICE"]
    #[inline(always)]
    pub const fn service(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SERVICE"]
    #[inline(always)]
    pub const fn set_service(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Serv {
    #[inline(always)]
    fn default() -> Serv {
        Serv(0)
    }
}
