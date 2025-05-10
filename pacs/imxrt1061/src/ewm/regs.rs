#[doc = "Clock Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkctrl(pub u8);
impl Clkctrl {
    #[doc = "CLKSEL"]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CLKSEL"]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
    }
}
impl Default for Clkctrl {
    #[inline(always)]
    fn default() -> Clkctrl {
        Clkctrl(0)
    }
}
impl core::fmt::Debug for Clkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkctrl")
            .field("clksel", &self.clksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkctrl {{ clksel: {=u8:?} }}", self.clksel())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u8);
impl Ctrl {
    #[doc = "EWM enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ewmen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EWM enable."]
    #[inline(always)]
    pub const fn set_ewmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "EWM_in's Assertion State Select."]
    #[must_use]
    #[inline(always)]
    pub const fn assin(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EWM_in's Assertion State Select."]
    #[inline(always)]
    pub const fn set_assin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn inen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable."]
    #[inline(always)]
    pub const fn set_inen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn inten(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn set_inten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("ewmen", &self.ewmen())
            .field("assin", &self.assin())
            .field("inen", &self.inen())
            .field("inten", &self.inten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ ewmen: {=bool:?}, assin: {=bool:?}, inen: {=bool:?}, inten: {=bool:?} }}",
            self.ewmen(),
            self.assin(),
            self.inen(),
            self.inten()
        )
    }
}
