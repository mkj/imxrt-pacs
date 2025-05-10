#[doc = "Interrupt Priority Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip0(pub u8);
impl Nvicip0 {
    #[doc = "Priority of the INT_DMA0_DMA16 interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn pri0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA0_DMA16 interrupt 0"]
    #[inline(always)]
    pub const fn set_pri0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip0 {
    #[inline(always)]
    fn default() -> Nvicip0 {
        Nvicip0(0)
    }
}
impl core::fmt::Debug for Nvicip0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip0")
            .field("pri0", &self.pri0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip0 {{ pri0: {=u8:?} }}", self.pri0())
    }
}
#[doc = "Interrupt Priority Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip1(pub u8);
impl Nvicip1 {
    #[doc = "Priority of the INT_DMA1_DMA17 interrupt 1"]
    #[must_use]
    #[inline(always)]
    pub const fn pri1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA1_DMA17 interrupt 1"]
    #[inline(always)]
    pub const fn set_pri1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip1 {
    #[inline(always)]
    fn default() -> Nvicip1 {
        Nvicip1(0)
    }
}
impl core::fmt::Debug for Nvicip1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip1")
            .field("pri1", &self.pri1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip1 {{ pri1: {=u8:?} }}", self.pri1())
    }
}
#[doc = "Interrupt Priority Register 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip10(pub u8);
impl Nvicip10 {
    #[doc = "Priority of the INT_DMA10_DMA26 interrupt 10"]
    #[must_use]
    #[inline(always)]
    pub const fn pri10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA10_DMA26 interrupt 10"]
    #[inline(always)]
    pub const fn set_pri10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip10 {
    #[inline(always)]
    fn default() -> Nvicip10 {
        Nvicip10(0)
    }
}
impl core::fmt::Debug for Nvicip10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip10")
            .field("pri10", &self.pri10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip10 {{ pri10: {=u8:?} }}", self.pri10())
    }
}
#[doc = "Interrupt Priority Register 100"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip100(pub u8);
impl Nvicip100 {
    #[doc = "Priority of the INT_GPT1 interrupt 100"]
    #[must_use]
    #[inline(always)]
    pub const fn pri100(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPT1 interrupt 100"]
    #[inline(always)]
    pub const fn set_pri100(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip100 {
    #[inline(always)]
    fn default() -> Nvicip100 {
        Nvicip100(0)
    }
}
impl core::fmt::Debug for Nvicip100 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip100")
            .field("pri100", &self.pri100())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip100 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip100 {{ pri100: {=u8:?} }}", self.pri100())
    }
}
#[doc = "Interrupt Priority Register 101"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip101(pub u8);
impl Nvicip101 {
    #[doc = "Priority of the INT_GPT2 interrupt 101"]
    #[must_use]
    #[inline(always)]
    pub const fn pri101(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPT2 interrupt 101"]
    #[inline(always)]
    pub const fn set_pri101(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip101 {
    #[inline(always)]
    fn default() -> Nvicip101 {
        Nvicip101(0)
    }
}
impl core::fmt::Debug for Nvicip101 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip101")
            .field("pri101", &self.pri101())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip101 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip101 {{ pri101: {=u8:?} }}", self.pri101())
    }
}
#[doc = "Interrupt Priority Register 102"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip102(pub u8);
impl Nvicip102 {
    #[doc = "Priority of the INT_PWM1_0 interrupt 102"]
    #[must_use]
    #[inline(always)]
    pub const fn pri102(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_0 interrupt 102"]
    #[inline(always)]
    pub const fn set_pri102(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip102 {
    #[inline(always)]
    fn default() -> Nvicip102 {
        Nvicip102(0)
    }
}
impl core::fmt::Debug for Nvicip102 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip102")
            .field("pri102", &self.pri102())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip102 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip102 {{ pri102: {=u8:?} }}", self.pri102())
    }
}
#[doc = "Interrupt Priority Register 103"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip103(pub u8);
impl Nvicip103 {
    #[doc = "Priority of the INT_PWM1_1 interrupt 103"]
    #[must_use]
    #[inline(always)]
    pub const fn pri103(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_1 interrupt 103"]
    #[inline(always)]
    pub const fn set_pri103(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip103 {
    #[inline(always)]
    fn default() -> Nvicip103 {
        Nvicip103(0)
    }
}
impl core::fmt::Debug for Nvicip103 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip103")
            .field("pri103", &self.pri103())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip103 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip103 {{ pri103: {=u8:?} }}", self.pri103())
    }
}
#[doc = "Interrupt Priority Register 104"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip104(pub u8);
impl Nvicip104 {
    #[doc = "Priority of the INT_PWM1_2 interrupt 104"]
    #[must_use]
    #[inline(always)]
    pub const fn pri104(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_2 interrupt 104"]
    #[inline(always)]
    pub const fn set_pri104(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip104 {
    #[inline(always)]
    fn default() -> Nvicip104 {
        Nvicip104(0)
    }
}
impl core::fmt::Debug for Nvicip104 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip104")
            .field("pri104", &self.pri104())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip104 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip104 {{ pri104: {=u8:?} }}", self.pri104())
    }
}
#[doc = "Interrupt Priority Register 105"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip105(pub u8);
impl Nvicip105 {
    #[doc = "Priority of the INT_PWM1_3 interrupt 105"]
    #[must_use]
    #[inline(always)]
    pub const fn pri105(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_3 interrupt 105"]
    #[inline(always)]
    pub const fn set_pri105(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip105 {
    #[inline(always)]
    fn default() -> Nvicip105 {
        Nvicip105(0)
    }
}
impl core::fmt::Debug for Nvicip105 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip105")
            .field("pri105", &self.pri105())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip105 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip105 {{ pri105: {=u8:?} }}", self.pri105())
    }
}
#[doc = "Interrupt Priority Register 106"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip106(pub u8);
impl Nvicip106 {
    #[doc = "Priority of the INT_PWM1_FAULT interrupt 106"]
    #[must_use]
    #[inline(always)]
    pub const fn pri106(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_FAULT interrupt 106"]
    #[inline(always)]
    pub const fn set_pri106(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip106 {
    #[inline(always)]
    fn default() -> Nvicip106 {
        Nvicip106(0)
    }
}
impl core::fmt::Debug for Nvicip106 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip106")
            .field("pri106", &self.pri106())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip106 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip106 {{ pri106: {=u8:?} }}", self.pri106())
    }
}
#[doc = "Interrupt Priority Register 107"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip107(pub u8);
impl Nvicip107 {
    #[doc = "Priority of the INT_FLEXSPI2 interrupt 107"]
    #[must_use]
    #[inline(always)]
    pub const fn pri107(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXSPI2 interrupt 107"]
    #[inline(always)]
    pub const fn set_pri107(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip107 {
    #[inline(always)]
    fn default() -> Nvicip107 {
        Nvicip107(0)
    }
}
impl core::fmt::Debug for Nvicip107 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip107")
            .field("pri107", &self.pri107())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip107 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip107 {{ pri107: {=u8:?} }}", self.pri107())
    }
}
#[doc = "Interrupt Priority Register 108"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip108(pub u8);
impl Nvicip108 {
    #[doc = "Priority of interrupt 108"]
    #[must_use]
    #[inline(always)]
    pub const fn pri108(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of interrupt 108"]
    #[inline(always)]
    pub const fn set_pri108(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip108 {
    #[inline(always)]
    fn default() -> Nvicip108 {
        Nvicip108(0)
    }
}
impl core::fmt::Debug for Nvicip108 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip108")
            .field("pri108", &self.pri108())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip108 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip108 {{ pri108: {=u8:?} }}", self.pri108())
    }
}
#[doc = "Interrupt Priority Register 109"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip109(pub u8);
impl Nvicip109 {
    #[doc = "Priority of the INT_SEMC interrupt 109"]
    #[must_use]
    #[inline(always)]
    pub const fn pri109(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SEMC interrupt 109"]
    #[inline(always)]
    pub const fn set_pri109(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip109 {
    #[inline(always)]
    fn default() -> Nvicip109 {
        Nvicip109(0)
    }
}
impl core::fmt::Debug for Nvicip109 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip109")
            .field("pri109", &self.pri109())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip109 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip109 {{ pri109: {=u8:?} }}", self.pri109())
    }
}
#[doc = "Interrupt Priority Register 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip11(pub u8);
impl Nvicip11 {
    #[doc = "Priority of the INT_DMA11_DMA27 interrupt 11"]
    #[must_use]
    #[inline(always)]
    pub const fn pri11(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA11_DMA27 interrupt 11"]
    #[inline(always)]
    pub const fn set_pri11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip11 {
    #[inline(always)]
    fn default() -> Nvicip11 {
        Nvicip11(0)
    }
}
impl core::fmt::Debug for Nvicip11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip11")
            .field("pri11", &self.pri11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip11 {{ pri11: {=u8:?} }}", self.pri11())
    }
}
#[doc = "Interrupt Priority Register 110"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip110(pub u8);
impl Nvicip110 {
    #[doc = "Priority of the INT_USDHC1 interrupt 110"]
    #[must_use]
    #[inline(always)]
    pub const fn pri110(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USDHC1 interrupt 110"]
    #[inline(always)]
    pub const fn set_pri110(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip110 {
    #[inline(always)]
    fn default() -> Nvicip110 {
        Nvicip110(0)
    }
}
impl core::fmt::Debug for Nvicip110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip110")
            .field("pri110", &self.pri110())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip110 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip110 {{ pri110: {=u8:?} }}", self.pri110())
    }
}
#[doc = "Interrupt Priority Register 111"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip111(pub u8);
impl Nvicip111 {
    #[doc = "Priority of the INT_USDHC2 interrupt 111"]
    #[must_use]
    #[inline(always)]
    pub const fn pri111(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USDHC2 interrupt 111"]
    #[inline(always)]
    pub const fn set_pri111(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip111 {
    #[inline(always)]
    fn default() -> Nvicip111 {
        Nvicip111(0)
    }
}
impl core::fmt::Debug for Nvicip111 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip111")
            .field("pri111", &self.pri111())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip111 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip111 {{ pri111: {=u8:?} }}", self.pri111())
    }
}
#[doc = "Interrupt Priority Register 112"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip112(pub u8);
impl Nvicip112 {
    #[doc = "Priority of the INT_USB_OTG2 interrupt 112"]
    #[must_use]
    #[inline(always)]
    pub const fn pri112(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USB_OTG2 interrupt 112"]
    #[inline(always)]
    pub const fn set_pri112(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip112 {
    #[inline(always)]
    fn default() -> Nvicip112 {
        Nvicip112(0)
    }
}
impl core::fmt::Debug for Nvicip112 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip112")
            .field("pri112", &self.pri112())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip112 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip112 {{ pri112: {=u8:?} }}", self.pri112())
    }
}
#[doc = "Interrupt Priority Register 113"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip113(pub u8);
impl Nvicip113 {
    #[doc = "Priority of the INT_USB_OTG1 interrupt 113"]
    #[must_use]
    #[inline(always)]
    pub const fn pri113(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USB_OTG1 interrupt 113"]
    #[inline(always)]
    pub const fn set_pri113(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip113 {
    #[inline(always)]
    fn default() -> Nvicip113 {
        Nvicip113(0)
    }
}
impl core::fmt::Debug for Nvicip113 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip113")
            .field("pri113", &self.pri113())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip113 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip113 {{ pri113: {=u8:?} }}", self.pri113())
    }
}
#[doc = "Interrupt Priority Register 114"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip114(pub u8);
impl Nvicip114 {
    #[doc = "Priority of the INT_ENET interrupt 114"]
    #[must_use]
    #[inline(always)]
    pub const fn pri114(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENET interrupt 114"]
    #[inline(always)]
    pub const fn set_pri114(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip114 {
    #[inline(always)]
    fn default() -> Nvicip114 {
        Nvicip114(0)
    }
}
impl core::fmt::Debug for Nvicip114 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip114")
            .field("pri114", &self.pri114())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip114 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip114 {{ pri114: {=u8:?} }}", self.pri114())
    }
}
#[doc = "Interrupt Priority Register 115"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip115(pub u8);
impl Nvicip115 {
    #[doc = "Priority of the INT_ENET_1588_Timer interrupt 115"]
    #[must_use]
    #[inline(always)]
    pub const fn pri115(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENET_1588_Timer interrupt 115"]
    #[inline(always)]
    pub const fn set_pri115(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip115 {
    #[inline(always)]
    fn default() -> Nvicip115 {
        Nvicip115(0)
    }
}
impl core::fmt::Debug for Nvicip115 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip115")
            .field("pri115", &self.pri115())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip115 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip115 {{ pri115: {=u8:?} }}", self.pri115())
    }
}
#[doc = "Interrupt Priority Register 116"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip116(pub u8);
impl Nvicip116 {
    #[doc = "Priority of the INT_XBAR1_IRQ_0_1 interrupt 116"]
    #[must_use]
    #[inline(always)]
    pub const fn pri116(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_XBAR1_IRQ_0_1 interrupt 116"]
    #[inline(always)]
    pub const fn set_pri116(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip116 {
    #[inline(always)]
    fn default() -> Nvicip116 {
        Nvicip116(0)
    }
}
impl core::fmt::Debug for Nvicip116 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip116")
            .field("pri116", &self.pri116())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip116 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip116 {{ pri116: {=u8:?} }}", self.pri116())
    }
}
#[doc = "Interrupt Priority Register 117"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip117(pub u8);
impl Nvicip117 {
    #[doc = "Priority of the INT_XBAR1_IRQ_2_3 interrupt 117"]
    #[must_use]
    #[inline(always)]
    pub const fn pri117(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_XBAR1_IRQ_2_3 interrupt 117"]
    #[inline(always)]
    pub const fn set_pri117(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip117 {
    #[inline(always)]
    fn default() -> Nvicip117 {
        Nvicip117(0)
    }
}
impl core::fmt::Debug for Nvicip117 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip117")
            .field("pri117", &self.pri117())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip117 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip117 {{ pri117: {=u8:?} }}", self.pri117())
    }
}
#[doc = "Interrupt Priority Register 118"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip118(pub u8);
impl Nvicip118 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ0 interrupt 118"]
    #[must_use]
    #[inline(always)]
    pub const fn pri118(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ0 interrupt 118"]
    #[inline(always)]
    pub const fn set_pri118(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip118 {
    #[inline(always)]
    fn default() -> Nvicip118 {
        Nvicip118(0)
    }
}
impl core::fmt::Debug for Nvicip118 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip118")
            .field("pri118", &self.pri118())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip118 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip118 {{ pri118: {=u8:?} }}", self.pri118())
    }
}
#[doc = "Interrupt Priority Register 119"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip119(pub u8);
impl Nvicip119 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ1 interrupt 119"]
    #[must_use]
    #[inline(always)]
    pub const fn pri119(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ1 interrupt 119"]
    #[inline(always)]
    pub const fn set_pri119(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip119 {
    #[inline(always)]
    fn default() -> Nvicip119 {
        Nvicip119(0)
    }
}
impl core::fmt::Debug for Nvicip119 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip119")
            .field("pri119", &self.pri119())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip119 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip119 {{ pri119: {=u8:?} }}", self.pri119())
    }
}
#[doc = "Interrupt Priority Register 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip12(pub u8);
impl Nvicip12 {
    #[doc = "Priority of the INT_DMA12_DMA28 interrupt 12"]
    #[must_use]
    #[inline(always)]
    pub const fn pri12(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA12_DMA28 interrupt 12"]
    #[inline(always)]
    pub const fn set_pri12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip12 {
    #[inline(always)]
    fn default() -> Nvicip12 {
        Nvicip12(0)
    }
}
impl core::fmt::Debug for Nvicip12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip12")
            .field("pri12", &self.pri12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip12 {{ pri12: {=u8:?} }}", self.pri12())
    }
}
#[doc = "Interrupt Priority Register 120"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip120(pub u8);
impl Nvicip120 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ2 interrupt 120"]
    #[must_use]
    #[inline(always)]
    pub const fn pri120(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ2 interrupt 120"]
    #[inline(always)]
    pub const fn set_pri120(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip120 {
    #[inline(always)]
    fn default() -> Nvicip120 {
        Nvicip120(0)
    }
}
impl core::fmt::Debug for Nvicip120 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip120")
            .field("pri120", &self.pri120())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip120 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip120 {{ pri120: {=u8:?} }}", self.pri120())
    }
}
#[doc = "Interrupt Priority Register 121"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip121(pub u8);
impl Nvicip121 {
    #[doc = "Priority of the INT_ADC_ETC_ERROR_IRQ interrupt 121"]
    #[must_use]
    #[inline(always)]
    pub const fn pri121(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_ERROR_IRQ interrupt 121"]
    #[inline(always)]
    pub const fn set_pri121(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip121 {
    #[inline(always)]
    fn default() -> Nvicip121 {
        Nvicip121(0)
    }
}
impl core::fmt::Debug for Nvicip121 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip121")
            .field("pri121", &self.pri121())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip121 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip121 {{ pri121: {=u8:?} }}", self.pri121())
    }
}
#[doc = "Interrupt Priority Register 122"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip122(pub u8);
impl Nvicip122 {
    #[doc = "Priority of the INT_PIT interrupt 122"]
    #[must_use]
    #[inline(always)]
    pub const fn pri122(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PIT interrupt 122"]
    #[inline(always)]
    pub const fn set_pri122(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip122 {
    #[inline(always)]
    fn default() -> Nvicip122 {
        Nvicip122(0)
    }
}
impl core::fmt::Debug for Nvicip122 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip122")
            .field("pri122", &self.pri122())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip122 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip122 {{ pri122: {=u8:?} }}", self.pri122())
    }
}
#[doc = "Interrupt Priority Register 123"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip123(pub u8);
impl Nvicip123 {
    #[doc = "Priority of the INT_ACMP1 interrupt 123"]
    #[must_use]
    #[inline(always)]
    pub const fn pri123(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ACMP1 interrupt 123"]
    #[inline(always)]
    pub const fn set_pri123(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip123 {
    #[inline(always)]
    fn default() -> Nvicip123 {
        Nvicip123(0)
    }
}
impl core::fmt::Debug for Nvicip123 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip123")
            .field("pri123", &self.pri123())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip123 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip123 {{ pri123: {=u8:?} }}", self.pri123())
    }
}
#[doc = "Interrupt Priority Register 124"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip124(pub u8);
impl Nvicip124 {
    #[doc = "Priority of the INT_ACMP2 interrupt 124"]
    #[must_use]
    #[inline(always)]
    pub const fn pri124(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ACMP2 interrupt 124"]
    #[inline(always)]
    pub const fn set_pri124(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip124 {
    #[inline(always)]
    fn default() -> Nvicip124 {
        Nvicip124(0)
    }
}
impl core::fmt::Debug for Nvicip124 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip124")
            .field("pri124", &self.pri124())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip124 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip124 {{ pri124: {=u8:?} }}", self.pri124())
    }
}
#[doc = "Interrupt Priority Register 125"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip125(pub u8);
impl Nvicip125 {
    #[doc = "Priority of the INT_ACMP3 interrupt 125"]
    #[must_use]
    #[inline(always)]
    pub const fn pri125(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ACMP3 interrupt 125"]
    #[inline(always)]
    pub const fn set_pri125(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip125 {
    #[inline(always)]
    fn default() -> Nvicip125 {
        Nvicip125(0)
    }
}
impl core::fmt::Debug for Nvicip125 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip125")
            .field("pri125", &self.pri125())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip125 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip125 {{ pri125: {=u8:?} }}", self.pri125())
    }
}
#[doc = "Interrupt Priority Register 126"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip126(pub u8);
impl Nvicip126 {
    #[doc = "Priority of the INT_ACMP4 interrupt 126"]
    #[must_use]
    #[inline(always)]
    pub const fn pri126(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ACMP4 interrupt 126"]
    #[inline(always)]
    pub const fn set_pri126(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip126 {
    #[inline(always)]
    fn default() -> Nvicip126 {
        Nvicip126(0)
    }
}
impl core::fmt::Debug for Nvicip126 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip126")
            .field("pri126", &self.pri126())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip126 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip126 {{ pri126: {=u8:?} }}", self.pri126())
    }
}
#[doc = "Interrupt Priority Register 127"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip127(pub u8);
impl Nvicip127 {
    #[doc = "Priority of the INT_Reserved143 interrupt 127"]
    #[must_use]
    #[inline(always)]
    pub const fn pri127(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved143 interrupt 127"]
    #[inline(always)]
    pub const fn set_pri127(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip127 {
    #[inline(always)]
    fn default() -> Nvicip127 {
        Nvicip127(0)
    }
}
impl core::fmt::Debug for Nvicip127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip127")
            .field("pri127", &self.pri127())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip127 {{ pri127: {=u8:?} }}", self.pri127())
    }
}
#[doc = "Interrupt Priority Register 128"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip128(pub u8);
impl Nvicip128 {
    #[doc = "Priority of the INT_Reserved144 interrupt 128"]
    #[must_use]
    #[inline(always)]
    pub const fn pri128(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved144 interrupt 128"]
    #[inline(always)]
    pub const fn set_pri128(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip128 {
    #[inline(always)]
    fn default() -> Nvicip128 {
        Nvicip128(0)
    }
}
impl core::fmt::Debug for Nvicip128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip128")
            .field("pri128", &self.pri128())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip128 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip128 {{ pri128: {=u8:?} }}", self.pri128())
    }
}
#[doc = "Interrupt Priority Register 129"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip129(pub u8);
impl Nvicip129 {
    #[doc = "Priority of the INT_ENC1 interrupt 129"]
    #[must_use]
    #[inline(always)]
    pub const fn pri129(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENC1 interrupt 129"]
    #[inline(always)]
    pub const fn set_pri129(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip129 {
    #[inline(always)]
    fn default() -> Nvicip129 {
        Nvicip129(0)
    }
}
impl core::fmt::Debug for Nvicip129 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip129")
            .field("pri129", &self.pri129())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip129 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip129 {{ pri129: {=u8:?} }}", self.pri129())
    }
}
#[doc = "Interrupt Priority Register 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip13(pub u8);
impl Nvicip13 {
    #[doc = "Priority of the INT_DMA13_DMA29 interrupt 13"]
    #[must_use]
    #[inline(always)]
    pub const fn pri13(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA13_DMA29 interrupt 13"]
    #[inline(always)]
    pub const fn set_pri13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip13 {
    #[inline(always)]
    fn default() -> Nvicip13 {
        Nvicip13(0)
    }
}
impl core::fmt::Debug for Nvicip13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip13")
            .field("pri13", &self.pri13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip13 {{ pri13: {=u8:?} }}", self.pri13())
    }
}
#[doc = "Interrupt Priority Register 130"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip130(pub u8);
impl Nvicip130 {
    #[doc = "Priority of the INT_ENC2 interrupt 130"]
    #[must_use]
    #[inline(always)]
    pub const fn pri130(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENC2 interrupt 130"]
    #[inline(always)]
    pub const fn set_pri130(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip130 {
    #[inline(always)]
    fn default() -> Nvicip130 {
        Nvicip130(0)
    }
}
impl core::fmt::Debug for Nvicip130 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip130")
            .field("pri130", &self.pri130())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip130 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip130 {{ pri130: {=u8:?} }}", self.pri130())
    }
}
#[doc = "Interrupt Priority Register 131"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip131(pub u8);
impl Nvicip131 {
    #[doc = "Priority of the INT_ENC3 interrupt 131"]
    #[must_use]
    #[inline(always)]
    pub const fn pri131(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENC3 interrupt 131"]
    #[inline(always)]
    pub const fn set_pri131(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip131 {
    #[inline(always)]
    fn default() -> Nvicip131 {
        Nvicip131(0)
    }
}
impl core::fmt::Debug for Nvicip131 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip131")
            .field("pri131", &self.pri131())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip131 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip131 {{ pri131: {=u8:?} }}", self.pri131())
    }
}
#[doc = "Interrupt Priority Register 132"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip132(pub u8);
impl Nvicip132 {
    #[doc = "Priority of the INT_ENC4 interrupt 132"]
    #[must_use]
    #[inline(always)]
    pub const fn pri132(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENC4 interrupt 132"]
    #[inline(always)]
    pub const fn set_pri132(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip132 {
    #[inline(always)]
    fn default() -> Nvicip132 {
        Nvicip132(0)
    }
}
impl core::fmt::Debug for Nvicip132 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip132")
            .field("pri132", &self.pri132())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip132 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip132 {{ pri132: {=u8:?} }}", self.pri132())
    }
}
#[doc = "Interrupt Priority Register 133"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip133(pub u8);
impl Nvicip133 {
    #[doc = "Priority of the INT_TMR1 interrupt 133"]
    #[must_use]
    #[inline(always)]
    pub const fn pri133(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TMR1 interrupt 133"]
    #[inline(always)]
    pub const fn set_pri133(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip133 {
    #[inline(always)]
    fn default() -> Nvicip133 {
        Nvicip133(0)
    }
}
impl core::fmt::Debug for Nvicip133 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip133")
            .field("pri133", &self.pri133())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip133 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip133 {{ pri133: {=u8:?} }}", self.pri133())
    }
}
#[doc = "Interrupt Priority Register 134"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip134(pub u8);
impl Nvicip134 {
    #[doc = "Priority of the INT_TMR2 interrupt 134"]
    #[must_use]
    #[inline(always)]
    pub const fn pri134(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TMR2 interrupt 134"]
    #[inline(always)]
    pub const fn set_pri134(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip134 {
    #[inline(always)]
    fn default() -> Nvicip134 {
        Nvicip134(0)
    }
}
impl core::fmt::Debug for Nvicip134 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip134")
            .field("pri134", &self.pri134())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip134 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip134 {{ pri134: {=u8:?} }}", self.pri134())
    }
}
#[doc = "Interrupt Priority Register 135"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip135(pub u8);
impl Nvicip135 {
    #[doc = "Priority of the INT_TMR3 interrupt 135"]
    #[must_use]
    #[inline(always)]
    pub const fn pri135(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TMR3 interrupt 135"]
    #[inline(always)]
    pub const fn set_pri135(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip135 {
    #[inline(always)]
    fn default() -> Nvicip135 {
        Nvicip135(0)
    }
}
impl core::fmt::Debug for Nvicip135 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip135")
            .field("pri135", &self.pri135())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip135 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip135 {{ pri135: {=u8:?} }}", self.pri135())
    }
}
#[doc = "Interrupt Priority Register 136"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip136(pub u8);
impl Nvicip136 {
    #[doc = "Priority of the INT_TMR4 interrupt 136"]
    #[must_use]
    #[inline(always)]
    pub const fn pri136(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TMR4 interrupt 136"]
    #[inline(always)]
    pub const fn set_pri136(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip136 {
    #[inline(always)]
    fn default() -> Nvicip136 {
        Nvicip136(0)
    }
}
impl core::fmt::Debug for Nvicip136 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip136")
            .field("pri136", &self.pri136())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip136 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip136 {{ pri136: {=u8:?} }}", self.pri136())
    }
}
#[doc = "Interrupt Priority Register 137"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip137(pub u8);
impl Nvicip137 {
    #[doc = "Priority of the INT_PWM2_0 interrupt 137"]
    #[must_use]
    #[inline(always)]
    pub const fn pri137(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM2_0 interrupt 137"]
    #[inline(always)]
    pub const fn set_pri137(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip137 {
    #[inline(always)]
    fn default() -> Nvicip137 {
        Nvicip137(0)
    }
}
impl core::fmt::Debug for Nvicip137 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip137")
            .field("pri137", &self.pri137())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip137 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip137 {{ pri137: {=u8:?} }}", self.pri137())
    }
}
#[doc = "Interrupt Priority Register 138"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip138(pub u8);
impl Nvicip138 {
    #[doc = "Priority of the INT_PWM2_1 interrupt 138"]
    #[must_use]
    #[inline(always)]
    pub const fn pri138(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM2_1 interrupt 138"]
    #[inline(always)]
    pub const fn set_pri138(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip138 {
    #[inline(always)]
    fn default() -> Nvicip138 {
        Nvicip138(0)
    }
}
impl core::fmt::Debug for Nvicip138 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip138")
            .field("pri138", &self.pri138())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip138 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip138 {{ pri138: {=u8:?} }}", self.pri138())
    }
}
#[doc = "Interrupt Priority Register 139"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip139(pub u8);
impl Nvicip139 {
    #[doc = "Priority of the INT_PWM2_2 interrupt 139"]
    #[must_use]
    #[inline(always)]
    pub const fn pri139(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM2_2 interrupt 139"]
    #[inline(always)]
    pub const fn set_pri139(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip139 {
    #[inline(always)]
    fn default() -> Nvicip139 {
        Nvicip139(0)
    }
}
impl core::fmt::Debug for Nvicip139 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip139")
            .field("pri139", &self.pri139())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip139 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip139 {{ pri139: {=u8:?} }}", self.pri139())
    }
}
#[doc = "Interrupt Priority Register 14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip14(pub u8);
impl Nvicip14 {
    #[doc = "Priority of the INT_DMA14_DMA30 interrupt 14"]
    #[must_use]
    #[inline(always)]
    pub const fn pri14(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA14_DMA30 interrupt 14"]
    #[inline(always)]
    pub const fn set_pri14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip14 {
    #[inline(always)]
    fn default() -> Nvicip14 {
        Nvicip14(0)
    }
}
impl core::fmt::Debug for Nvicip14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip14")
            .field("pri14", &self.pri14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip14 {{ pri14: {=u8:?} }}", self.pri14())
    }
}
#[doc = "Interrupt Priority Register 140"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip140(pub u8);
impl Nvicip140 {
    #[doc = "Priority of the INT_PWM2_3 interrupt 140"]
    #[must_use]
    #[inline(always)]
    pub const fn pri140(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM2_3 interrupt 140"]
    #[inline(always)]
    pub const fn set_pri140(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip140 {
    #[inline(always)]
    fn default() -> Nvicip140 {
        Nvicip140(0)
    }
}
impl core::fmt::Debug for Nvicip140 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip140")
            .field("pri140", &self.pri140())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip140 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip140 {{ pri140: {=u8:?} }}", self.pri140())
    }
}
#[doc = "Interrupt Priority Register 141"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip141(pub u8);
impl Nvicip141 {
    #[doc = "Priority of the INT_PWM2_FAULT interrupt 141"]
    #[must_use]
    #[inline(always)]
    pub const fn pri141(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM2_FAULT interrupt 141"]
    #[inline(always)]
    pub const fn set_pri141(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip141 {
    #[inline(always)]
    fn default() -> Nvicip141 {
        Nvicip141(0)
    }
}
impl core::fmt::Debug for Nvicip141 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip141")
            .field("pri141", &self.pri141())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip141 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip141 {{ pri141: {=u8:?} }}", self.pri141())
    }
}
#[doc = "Interrupt Priority Register 142"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip142(pub u8);
impl Nvicip142 {
    #[doc = "Priority of the INT_PWM3_0 interrupt 142"]
    #[must_use]
    #[inline(always)]
    pub const fn pri142(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM3_0 interrupt 142"]
    #[inline(always)]
    pub const fn set_pri142(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip142 {
    #[inline(always)]
    fn default() -> Nvicip142 {
        Nvicip142(0)
    }
}
impl core::fmt::Debug for Nvicip142 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip142")
            .field("pri142", &self.pri142())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip142 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip142 {{ pri142: {=u8:?} }}", self.pri142())
    }
}
#[doc = "Interrupt Priority Register 143"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip143(pub u8);
impl Nvicip143 {
    #[doc = "Priority of the INT_PWM3_1 interrupt 143"]
    #[must_use]
    #[inline(always)]
    pub const fn pri143(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM3_1 interrupt 143"]
    #[inline(always)]
    pub const fn set_pri143(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip143 {
    #[inline(always)]
    fn default() -> Nvicip143 {
        Nvicip143(0)
    }
}
impl core::fmt::Debug for Nvicip143 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip143")
            .field("pri143", &self.pri143())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip143 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip143 {{ pri143: {=u8:?} }}", self.pri143())
    }
}
#[doc = "Interrupt Priority Register 144"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip144(pub u8);
impl Nvicip144 {
    #[doc = "Priority of the INT_PWM3_2 interrupt 144"]
    #[must_use]
    #[inline(always)]
    pub const fn pri144(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM3_2 interrupt 144"]
    #[inline(always)]
    pub const fn set_pri144(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip144 {
    #[inline(always)]
    fn default() -> Nvicip144 {
        Nvicip144(0)
    }
}
impl core::fmt::Debug for Nvicip144 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip144")
            .field("pri144", &self.pri144())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip144 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip144 {{ pri144: {=u8:?} }}", self.pri144())
    }
}
#[doc = "Interrupt Priority Register 145"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip145(pub u8);
impl Nvicip145 {
    #[doc = "Priority of the INT_PWM3_3 interrupt 145"]
    #[must_use]
    #[inline(always)]
    pub const fn pri145(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM3_3 interrupt 145"]
    #[inline(always)]
    pub const fn set_pri145(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip145 {
    #[inline(always)]
    fn default() -> Nvicip145 {
        Nvicip145(0)
    }
}
impl core::fmt::Debug for Nvicip145 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip145")
            .field("pri145", &self.pri145())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip145 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip145 {{ pri145: {=u8:?} }}", self.pri145())
    }
}
#[doc = "Interrupt Priority Register 146"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip146(pub u8);
impl Nvicip146 {
    #[doc = "Priority of the INT_PWM3_FAULT interrupt 146"]
    #[must_use]
    #[inline(always)]
    pub const fn pri146(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM3_FAULT interrupt 146"]
    #[inline(always)]
    pub const fn set_pri146(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip146 {
    #[inline(always)]
    fn default() -> Nvicip146 {
        Nvicip146(0)
    }
}
impl core::fmt::Debug for Nvicip146 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip146")
            .field("pri146", &self.pri146())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip146 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip146 {{ pri146: {=u8:?} }}", self.pri146())
    }
}
#[doc = "Interrupt Priority Register 147"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip147(pub u8);
impl Nvicip147 {
    #[doc = "Priority of the INT_PWM4_0 interrupt 147"]
    #[must_use]
    #[inline(always)]
    pub const fn pri147(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM4_0 interrupt 147"]
    #[inline(always)]
    pub const fn set_pri147(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip147 {
    #[inline(always)]
    fn default() -> Nvicip147 {
        Nvicip147(0)
    }
}
impl core::fmt::Debug for Nvicip147 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip147")
            .field("pri147", &self.pri147())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip147 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip147 {{ pri147: {=u8:?} }}", self.pri147())
    }
}
#[doc = "Interrupt Priority Register 148"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip148(pub u8);
impl Nvicip148 {
    #[doc = "Priority of the INT_PWM4_1 interrupt 148"]
    #[must_use]
    #[inline(always)]
    pub const fn pri148(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM4_1 interrupt 148"]
    #[inline(always)]
    pub const fn set_pri148(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip148 {
    #[inline(always)]
    fn default() -> Nvicip148 {
        Nvicip148(0)
    }
}
impl core::fmt::Debug for Nvicip148 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip148")
            .field("pri148", &self.pri148())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip148 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip148 {{ pri148: {=u8:?} }}", self.pri148())
    }
}
#[doc = "Interrupt Priority Register 149"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip149(pub u8);
impl Nvicip149 {
    #[doc = "Priority of the INT_PWM4_2 interrupt 149"]
    #[must_use]
    #[inline(always)]
    pub const fn pri149(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM4_2 interrupt 149"]
    #[inline(always)]
    pub const fn set_pri149(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip149 {
    #[inline(always)]
    fn default() -> Nvicip149 {
        Nvicip149(0)
    }
}
impl core::fmt::Debug for Nvicip149 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip149")
            .field("pri149", &self.pri149())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip149 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip149 {{ pri149: {=u8:?} }}", self.pri149())
    }
}
#[doc = "Interrupt Priority Register 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip15(pub u8);
impl Nvicip15 {
    #[doc = "Priority of the INT_DMA15_DMA31 interrupt 15"]
    #[must_use]
    #[inline(always)]
    pub const fn pri15(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA15_DMA31 interrupt 15"]
    #[inline(always)]
    pub const fn set_pri15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip15 {
    #[inline(always)]
    fn default() -> Nvicip15 {
        Nvicip15(0)
    }
}
impl core::fmt::Debug for Nvicip15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip15")
            .field("pri15", &self.pri15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip15 {{ pri15: {=u8:?} }}", self.pri15())
    }
}
#[doc = "Interrupt Priority Register 150"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip150(pub u8);
impl Nvicip150 {
    #[doc = "Priority of the INT_PWM4_3 interrupt 150"]
    #[must_use]
    #[inline(always)]
    pub const fn pri150(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM4_3 interrupt 150"]
    #[inline(always)]
    pub const fn set_pri150(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip150 {
    #[inline(always)]
    fn default() -> Nvicip150 {
        Nvicip150(0)
    }
}
impl core::fmt::Debug for Nvicip150 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip150")
            .field("pri150", &self.pri150())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip150 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip150 {{ pri150: {=u8:?} }}", self.pri150())
    }
}
#[doc = "Interrupt Priority Register 151"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip151(pub u8);
impl Nvicip151 {
    #[doc = "Priority of the INT_PWM4_FAULT interrupt 151"]
    #[must_use]
    #[inline(always)]
    pub const fn pri151(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM4_FAULT interrupt 151"]
    #[inline(always)]
    pub const fn set_pri151(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip151 {
    #[inline(always)]
    fn default() -> Nvicip151 {
        Nvicip151(0)
    }
}
impl core::fmt::Debug for Nvicip151 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip151")
            .field("pri151", &self.pri151())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip151 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip151 {{ pri151: {=u8:?} }}", self.pri151())
    }
}
#[doc = "Interrupt Priority Register 152"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip152(pub u8);
impl Nvicip152 {
    #[doc = "Priority of the INT_ENET2 interrupt 152"]
    #[must_use]
    #[inline(always)]
    pub const fn pri152(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENET2 interrupt 152"]
    #[inline(always)]
    pub const fn set_pri152(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip152 {
    #[inline(always)]
    fn default() -> Nvicip152 {
        Nvicip152(0)
    }
}
impl core::fmt::Debug for Nvicip152 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip152")
            .field("pri152", &self.pri152())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip152 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip152 {{ pri152: {=u8:?} }}", self.pri152())
    }
}
#[doc = "Interrupt Priority Register 153"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip153(pub u8);
impl Nvicip153 {
    #[doc = "Priority of the INT_ENET2_1588_Timer interrupt 153"]
    #[must_use]
    #[inline(always)]
    pub const fn pri153(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ENET2_1588_Timer interrupt 153"]
    #[inline(always)]
    pub const fn set_pri153(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip153 {
    #[inline(always)]
    fn default() -> Nvicip153 {
        Nvicip153(0)
    }
}
impl core::fmt::Debug for Nvicip153 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip153")
            .field("pri153", &self.pri153())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip153 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip153 {{ pri153: {=u8:?} }}", self.pri153())
    }
}
#[doc = "Interrupt Priority Register 154"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip154(pub u8);
impl Nvicip154 {
    #[doc = "Priority of the INT_CAN3 interrupt 154"]
    #[must_use]
    #[inline(always)]
    pub const fn pri154(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CAN3 interrupt 154"]
    #[inline(always)]
    pub const fn set_pri154(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip154 {
    #[inline(always)]
    fn default() -> Nvicip154 {
        Nvicip154(0)
    }
}
impl core::fmt::Debug for Nvicip154 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip154")
            .field("pri154", &self.pri154())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip154 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip154 {{ pri154: {=u8:?} }}", self.pri154())
    }
}
#[doc = "Interrupt Priority Register 155"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip155(pub u8);
impl Nvicip155 {
    #[doc = "Priority of the INT_Reserved171 interrupt 155"]
    #[must_use]
    #[inline(always)]
    pub const fn pri155(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved171 interrupt 155"]
    #[inline(always)]
    pub const fn set_pri155(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip155 {
    #[inline(always)]
    fn default() -> Nvicip155 {
        Nvicip155(0)
    }
}
impl core::fmt::Debug for Nvicip155 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip155")
            .field("pri155", &self.pri155())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip155 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip155 {{ pri155: {=u8:?} }}", self.pri155())
    }
}
#[doc = "Interrupt Priority Register 156"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip156(pub u8);
impl Nvicip156 {
    #[doc = "Priority of the INT_FLEXIO3 interrupt 156"]
    #[must_use]
    #[inline(always)]
    pub const fn pri156(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXIO3 interrupt 156"]
    #[inline(always)]
    pub const fn set_pri156(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip156 {
    #[inline(always)]
    fn default() -> Nvicip156 {
        Nvicip156(0)
    }
}
impl core::fmt::Debug for Nvicip156 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip156")
            .field("pri156", &self.pri156())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip156 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip156 {{ pri156: {=u8:?} }}", self.pri156())
    }
}
#[doc = "Interrupt Priority Register 157"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip157(pub u8);
impl Nvicip157 {
    #[doc = "Priority of the INT_GPIO6_7_8_9 interrupt 157"]
    #[must_use]
    #[inline(always)]
    pub const fn pri157(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO6_7_8_9 interrupt 157"]
    #[inline(always)]
    pub const fn set_pri157(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip157 {
    #[inline(always)]
    fn default() -> Nvicip157 {
        Nvicip157(0)
    }
}
impl core::fmt::Debug for Nvicip157 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip157")
            .field("pri157", &self.pri157())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip157 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip157 {{ pri157: {=u8:?} }}", self.pri157())
    }
}
#[doc = "Interrupt Priority Register 16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip16(pub u8);
impl Nvicip16 {
    #[doc = "Priority of the INT_DMA_ERROR interrupt 16"]
    #[must_use]
    #[inline(always)]
    pub const fn pri16(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA_ERROR interrupt 16"]
    #[inline(always)]
    pub const fn set_pri16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip16 {
    #[inline(always)]
    fn default() -> Nvicip16 {
        Nvicip16(0)
    }
}
impl core::fmt::Debug for Nvicip16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip16")
            .field("pri16", &self.pri16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip16 {{ pri16: {=u8:?} }}", self.pri16())
    }
}
#[doc = "Interrupt Priority Register 17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip17(pub u8);
impl Nvicip17 {
    #[doc = "Priority of the INT_CTI0_ERROR interrupt 17"]
    #[must_use]
    #[inline(always)]
    pub const fn pri17(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CTI0_ERROR interrupt 17"]
    #[inline(always)]
    pub const fn set_pri17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip17 {
    #[inline(always)]
    fn default() -> Nvicip17 {
        Nvicip17(0)
    }
}
impl core::fmt::Debug for Nvicip17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip17")
            .field("pri17", &self.pri17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip17 {{ pri17: {=u8:?} }}", self.pri17())
    }
}
#[doc = "Interrupt Priority Register 18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip18(pub u8);
impl Nvicip18 {
    #[doc = "Priority of the INT_CTI1_ERROR interrupt 18"]
    #[must_use]
    #[inline(always)]
    pub const fn pri18(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CTI1_ERROR interrupt 18"]
    #[inline(always)]
    pub const fn set_pri18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip18 {
    #[inline(always)]
    fn default() -> Nvicip18 {
        Nvicip18(0)
    }
}
impl core::fmt::Debug for Nvicip18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip18")
            .field("pri18", &self.pri18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip18 {{ pri18: {=u8:?} }}", self.pri18())
    }
}
#[doc = "Interrupt Priority Register 19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip19(pub u8);
impl Nvicip19 {
    #[doc = "Priority of the INT_CORE interrupt 19"]
    #[must_use]
    #[inline(always)]
    pub const fn pri19(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CORE interrupt 19"]
    #[inline(always)]
    pub const fn set_pri19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip19 {
    #[inline(always)]
    fn default() -> Nvicip19 {
        Nvicip19(0)
    }
}
impl core::fmt::Debug for Nvicip19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip19")
            .field("pri19", &self.pri19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip19 {{ pri19: {=u8:?} }}", self.pri19())
    }
}
#[doc = "Interrupt Priority Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip2(pub u8);
impl Nvicip2 {
    #[doc = "Priority of the INT_DMA2_DMA18 interrupt 2"]
    #[must_use]
    #[inline(always)]
    pub const fn pri2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA2_DMA18 interrupt 2"]
    #[inline(always)]
    pub const fn set_pri2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip2 {
    #[inline(always)]
    fn default() -> Nvicip2 {
        Nvicip2(0)
    }
}
impl core::fmt::Debug for Nvicip2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip2")
            .field("pri2", &self.pri2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip2 {{ pri2: {=u8:?} }}", self.pri2())
    }
}
#[doc = "Interrupt Priority Register 20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip20(pub u8);
impl Nvicip20 {
    #[doc = "Priority of the INT_LPUART1 interrupt 20"]
    #[must_use]
    #[inline(always)]
    pub const fn pri20(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART1 interrupt 20"]
    #[inline(always)]
    pub const fn set_pri20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip20 {
    #[inline(always)]
    fn default() -> Nvicip20 {
        Nvicip20(0)
    }
}
impl core::fmt::Debug for Nvicip20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip20")
            .field("pri20", &self.pri20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip20 {{ pri20: {=u8:?} }}", self.pri20())
    }
}
#[doc = "Interrupt Priority Register 21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip21(pub u8);
impl Nvicip21 {
    #[doc = "Priority of the INT_LPUART2 interrupt 21"]
    #[must_use]
    #[inline(always)]
    pub const fn pri21(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART2 interrupt 21"]
    #[inline(always)]
    pub const fn set_pri21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip21 {
    #[inline(always)]
    fn default() -> Nvicip21 {
        Nvicip21(0)
    }
}
impl core::fmt::Debug for Nvicip21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip21")
            .field("pri21", &self.pri21())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip21 {{ pri21: {=u8:?} }}", self.pri21())
    }
}
#[doc = "Interrupt Priority Register 22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip22(pub u8);
impl Nvicip22 {
    #[doc = "Priority of the INT_LPUART3 interrupt 22"]
    #[must_use]
    #[inline(always)]
    pub const fn pri22(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART3 interrupt 22"]
    #[inline(always)]
    pub const fn set_pri22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip22 {
    #[inline(always)]
    fn default() -> Nvicip22 {
        Nvicip22(0)
    }
}
impl core::fmt::Debug for Nvicip22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip22")
            .field("pri22", &self.pri22())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip22 {{ pri22: {=u8:?} }}", self.pri22())
    }
}
#[doc = "Interrupt Priority Register 23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip23(pub u8);
impl Nvicip23 {
    #[doc = "Priority of the INT_LPUART4 interrupt 23"]
    #[must_use]
    #[inline(always)]
    pub const fn pri23(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART4 interrupt 23"]
    #[inline(always)]
    pub const fn set_pri23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip23 {
    #[inline(always)]
    fn default() -> Nvicip23 {
        Nvicip23(0)
    }
}
impl core::fmt::Debug for Nvicip23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip23")
            .field("pri23", &self.pri23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip23 {{ pri23: {=u8:?} }}", self.pri23())
    }
}
#[doc = "Interrupt Priority Register 24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip24(pub u8);
impl Nvicip24 {
    #[doc = "Priority of the INT_LPUART5 interrupt 24"]
    #[must_use]
    #[inline(always)]
    pub const fn pri24(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART5 interrupt 24"]
    #[inline(always)]
    pub const fn set_pri24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip24 {
    #[inline(always)]
    fn default() -> Nvicip24 {
        Nvicip24(0)
    }
}
impl core::fmt::Debug for Nvicip24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip24")
            .field("pri24", &self.pri24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip24 {{ pri24: {=u8:?} }}", self.pri24())
    }
}
#[doc = "Interrupt Priority Register 25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip25(pub u8);
impl Nvicip25 {
    #[doc = "Priority of the INT_LPUART6 interrupt 25"]
    #[must_use]
    #[inline(always)]
    pub const fn pri25(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART6 interrupt 25"]
    #[inline(always)]
    pub const fn set_pri25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip25 {
    #[inline(always)]
    fn default() -> Nvicip25 {
        Nvicip25(0)
    }
}
impl core::fmt::Debug for Nvicip25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip25")
            .field("pri25", &self.pri25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip25 {{ pri25: {=u8:?} }}", self.pri25())
    }
}
#[doc = "Interrupt Priority Register 26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip26(pub u8);
impl Nvicip26 {
    #[doc = "Priority of the INT_LPUART7 interrupt 26"]
    #[must_use]
    #[inline(always)]
    pub const fn pri26(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART7 interrupt 26"]
    #[inline(always)]
    pub const fn set_pri26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip26 {
    #[inline(always)]
    fn default() -> Nvicip26 {
        Nvicip26(0)
    }
}
impl core::fmt::Debug for Nvicip26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip26")
            .field("pri26", &self.pri26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip26 {{ pri26: {=u8:?} }}", self.pri26())
    }
}
#[doc = "Interrupt Priority Register 27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip27(pub u8);
impl Nvicip27 {
    #[doc = "Priority of the INT_LPUART8 interrupt 27"]
    #[must_use]
    #[inline(always)]
    pub const fn pri27(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART8 interrupt 27"]
    #[inline(always)]
    pub const fn set_pri27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip27 {
    #[inline(always)]
    fn default() -> Nvicip27 {
        Nvicip27(0)
    }
}
impl core::fmt::Debug for Nvicip27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip27")
            .field("pri27", &self.pri27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip27 {{ pri27: {=u8:?} }}", self.pri27())
    }
}
#[doc = "Interrupt Priority Register 28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip28(pub u8);
impl Nvicip28 {
    #[doc = "Priority of the INT_LPI2C1 interrupt 28"]
    #[must_use]
    #[inline(always)]
    pub const fn pri28(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPI2C1 interrupt 28"]
    #[inline(always)]
    pub const fn set_pri28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip28 {
    #[inline(always)]
    fn default() -> Nvicip28 {
        Nvicip28(0)
    }
}
impl core::fmt::Debug for Nvicip28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip28")
            .field("pri28", &self.pri28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip28 {{ pri28: {=u8:?} }}", self.pri28())
    }
}
#[doc = "Interrupt Priority Register 29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip29(pub u8);
impl Nvicip29 {
    #[doc = "Priority of the INT_LPI2C2 interrupt 29"]
    #[must_use]
    #[inline(always)]
    pub const fn pri29(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPI2C2 interrupt 29"]
    #[inline(always)]
    pub const fn set_pri29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip29 {
    #[inline(always)]
    fn default() -> Nvicip29 {
        Nvicip29(0)
    }
}
impl core::fmt::Debug for Nvicip29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip29")
            .field("pri29", &self.pri29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip29 {{ pri29: {=u8:?} }}", self.pri29())
    }
}
#[doc = "Interrupt Priority Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip3(pub u8);
impl Nvicip3 {
    #[doc = "Priority of the INT_DMA3_DMA19 interrupt 3"]
    #[must_use]
    #[inline(always)]
    pub const fn pri3(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA3_DMA19 interrupt 3"]
    #[inline(always)]
    pub const fn set_pri3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip3 {
    #[inline(always)]
    fn default() -> Nvicip3 {
        Nvicip3(0)
    }
}
impl core::fmt::Debug for Nvicip3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip3")
            .field("pri3", &self.pri3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip3 {{ pri3: {=u8:?} }}", self.pri3())
    }
}
#[doc = "Interrupt Priority Register 30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip30(pub u8);
impl Nvicip30 {
    #[doc = "Priority of the INT_LPI2C3 interrupt 30"]
    #[must_use]
    #[inline(always)]
    pub const fn pri30(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPI2C3 interrupt 30"]
    #[inline(always)]
    pub const fn set_pri30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip30 {
    #[inline(always)]
    fn default() -> Nvicip30 {
        Nvicip30(0)
    }
}
impl core::fmt::Debug for Nvicip30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip30")
            .field("pri30", &self.pri30())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip30 {{ pri30: {=u8:?} }}", self.pri30())
    }
}
#[doc = "Interrupt Priority Register 31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip31(pub u8);
impl Nvicip31 {
    #[doc = "Priority of the INT_LPI2C4 interrupt 31"]
    #[must_use]
    #[inline(always)]
    pub const fn pri31(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPI2C4 interrupt 31"]
    #[inline(always)]
    pub const fn set_pri31(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip31 {
    #[inline(always)]
    fn default() -> Nvicip31 {
        Nvicip31(0)
    }
}
impl core::fmt::Debug for Nvicip31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip31")
            .field("pri31", &self.pri31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip31 {{ pri31: {=u8:?} }}", self.pri31())
    }
}
#[doc = "Interrupt Priority Register 32"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip32(pub u8);
impl Nvicip32 {
    #[doc = "Priority of the INT_LPSPI1 interrupt 32"]
    #[must_use]
    #[inline(always)]
    pub const fn pri32(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPSPI1 interrupt 32"]
    #[inline(always)]
    pub const fn set_pri32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip32 {
    #[inline(always)]
    fn default() -> Nvicip32 {
        Nvicip32(0)
    }
}
impl core::fmt::Debug for Nvicip32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip32")
            .field("pri32", &self.pri32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip32 {{ pri32: {=u8:?} }}", self.pri32())
    }
}
#[doc = "Interrupt Priority Register 33"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip33(pub u8);
impl Nvicip33 {
    #[doc = "Priority of the INT_LPSPI2 interrupt 33"]
    #[must_use]
    #[inline(always)]
    pub const fn pri33(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPSPI2 interrupt 33"]
    #[inline(always)]
    pub const fn set_pri33(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip33 {
    #[inline(always)]
    fn default() -> Nvicip33 {
        Nvicip33(0)
    }
}
impl core::fmt::Debug for Nvicip33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip33")
            .field("pri33", &self.pri33())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip33 {{ pri33: {=u8:?} }}", self.pri33())
    }
}
#[doc = "Interrupt Priority Register 34"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip34(pub u8);
impl Nvicip34 {
    #[doc = "Priority of the INT_LPSPI3 interrupt 34"]
    #[must_use]
    #[inline(always)]
    pub const fn pri34(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPSPI3 interrupt 34"]
    #[inline(always)]
    pub const fn set_pri34(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip34 {
    #[inline(always)]
    fn default() -> Nvicip34 {
        Nvicip34(0)
    }
}
impl core::fmt::Debug for Nvicip34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip34")
            .field("pri34", &self.pri34())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip34 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip34 {{ pri34: {=u8:?} }}", self.pri34())
    }
}
#[doc = "Interrupt Priority Register 35"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip35(pub u8);
impl Nvicip35 {
    #[doc = "Priority of the INT_LPSPI4 interrupt 35"]
    #[must_use]
    #[inline(always)]
    pub const fn pri35(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPSPI4 interrupt 35"]
    #[inline(always)]
    pub const fn set_pri35(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip35 {
    #[inline(always)]
    fn default() -> Nvicip35 {
        Nvicip35(0)
    }
}
impl core::fmt::Debug for Nvicip35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip35")
            .field("pri35", &self.pri35())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip35 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip35 {{ pri35: {=u8:?} }}", self.pri35())
    }
}
#[doc = "Interrupt Priority Register 36"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip36(pub u8);
impl Nvicip36 {
    #[doc = "Priority of the INT_CAN1 interrupt 36"]
    #[must_use]
    #[inline(always)]
    pub const fn pri36(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CAN1 interrupt 36"]
    #[inline(always)]
    pub const fn set_pri36(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip36 {
    #[inline(always)]
    fn default() -> Nvicip36 {
        Nvicip36(0)
    }
}
impl core::fmt::Debug for Nvicip36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip36")
            .field("pri36", &self.pri36())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip36 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip36 {{ pri36: {=u8:?} }}", self.pri36())
    }
}
#[doc = "Interrupt Priority Register 37"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip37(pub u8);
impl Nvicip37 {
    #[doc = "Priority of the INT_CAN2 interrupt 37"]
    #[must_use]
    #[inline(always)]
    pub const fn pri37(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CAN2 interrupt 37"]
    #[inline(always)]
    pub const fn set_pri37(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip37 {
    #[inline(always)]
    fn default() -> Nvicip37 {
        Nvicip37(0)
    }
}
impl core::fmt::Debug for Nvicip37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip37")
            .field("pri37", &self.pri37())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip37 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip37 {{ pri37: {=u8:?} }}", self.pri37())
    }
}
#[doc = "Interrupt Priority Register 38"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip38(pub u8);
impl Nvicip38 {
    #[doc = "Priority of the INT_FLEXRAM interrupt 38"]
    #[must_use]
    #[inline(always)]
    pub const fn pri38(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXRAM interrupt 38"]
    #[inline(always)]
    pub const fn set_pri38(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip38 {
    #[inline(always)]
    fn default() -> Nvicip38 {
        Nvicip38(0)
    }
}
impl core::fmt::Debug for Nvicip38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip38")
            .field("pri38", &self.pri38())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip38 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip38 {{ pri38: {=u8:?} }}", self.pri38())
    }
}
#[doc = "Interrupt Priority Register 39"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip39(pub u8);
impl Nvicip39 {
    #[doc = "Priority of the INT_KPP interrupt 39"]
    #[must_use]
    #[inline(always)]
    pub const fn pri39(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_KPP interrupt 39"]
    #[inline(always)]
    pub const fn set_pri39(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip39 {
    #[inline(always)]
    fn default() -> Nvicip39 {
        Nvicip39(0)
    }
}
impl core::fmt::Debug for Nvicip39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip39")
            .field("pri39", &self.pri39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip39 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip39 {{ pri39: {=u8:?} }}", self.pri39())
    }
}
#[doc = "Interrupt Priority Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip4(pub u8);
impl Nvicip4 {
    #[doc = "Priority of the INT_DMA4_DMA20 interrupt 4"]
    #[must_use]
    #[inline(always)]
    pub const fn pri4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA4_DMA20 interrupt 4"]
    #[inline(always)]
    pub const fn set_pri4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip4 {
    #[inline(always)]
    fn default() -> Nvicip4 {
        Nvicip4(0)
    }
}
impl core::fmt::Debug for Nvicip4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip4")
            .field("pri4", &self.pri4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip4 {{ pri4: {=u8:?} }}", self.pri4())
    }
}
#[doc = "Interrupt Priority Register 40"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip40(pub u8);
impl Nvicip40 {
    #[doc = "Priority of the INT_TSC_DIG interrupt 40"]
    #[must_use]
    #[inline(always)]
    pub const fn pri40(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TSC_DIG interrupt 40"]
    #[inline(always)]
    pub const fn set_pri40(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip40 {
    #[inline(always)]
    fn default() -> Nvicip40 {
        Nvicip40(0)
    }
}
impl core::fmt::Debug for Nvicip40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip40")
            .field("pri40", &self.pri40())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip40 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip40 {{ pri40: {=u8:?} }}", self.pri40())
    }
}
#[doc = "Interrupt Priority Register 41"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip41(pub u8);
impl Nvicip41 {
    #[doc = "Priority of the INT_GPR_IRQ interrupt 41"]
    #[must_use]
    #[inline(always)]
    pub const fn pri41(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPR_IRQ interrupt 41"]
    #[inline(always)]
    pub const fn set_pri41(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip41 {
    #[inline(always)]
    fn default() -> Nvicip41 {
        Nvicip41(0)
    }
}
impl core::fmt::Debug for Nvicip41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip41")
            .field("pri41", &self.pri41())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip41 {{ pri41: {=u8:?} }}", self.pri41())
    }
}
#[doc = "Interrupt Priority Register 42"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip42(pub u8);
impl Nvicip42 {
    #[doc = "Priority of interrupt 42"]
    #[must_use]
    #[inline(always)]
    pub const fn pri42(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of interrupt 42"]
    #[inline(always)]
    pub const fn set_pri42(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip42 {
    #[inline(always)]
    fn default() -> Nvicip42 {
        Nvicip42(0)
    }
}
impl core::fmt::Debug for Nvicip42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip42")
            .field("pri42", &self.pri42())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip42 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip42 {{ pri42: {=u8:?} }}", self.pri42())
    }
}
#[doc = "Interrupt Priority Register 43"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip43(pub u8);
impl Nvicip43 {
    #[doc = "Priority of interrupt 43"]
    #[must_use]
    #[inline(always)]
    pub const fn pri43(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of interrupt 43"]
    #[inline(always)]
    pub const fn set_pri43(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip43 {
    #[inline(always)]
    fn default() -> Nvicip43 {
        Nvicip43(0)
    }
}
impl core::fmt::Debug for Nvicip43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip43")
            .field("pri43", &self.pri43())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip43 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip43 {{ pri43: {=u8:?} }}", self.pri43())
    }
}
#[doc = "Interrupt Priority Register 44"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip44(pub u8);
impl Nvicip44 {
    #[doc = "Priority of interrupt 44"]
    #[must_use]
    #[inline(always)]
    pub const fn pri44(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of interrupt 44"]
    #[inline(always)]
    pub const fn set_pri44(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip44 {
    #[inline(always)]
    fn default() -> Nvicip44 {
        Nvicip44(0)
    }
}
impl core::fmt::Debug for Nvicip44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip44")
            .field("pri44", &self.pri44())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip44 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip44 {{ pri44: {=u8:?} }}", self.pri44())
    }
}
#[doc = "Interrupt Priority Register 45"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip45(pub u8);
impl Nvicip45 {
    #[doc = "Priority of the INT_WDOG2 interrupt 45"]
    #[must_use]
    #[inline(always)]
    pub const fn pri45(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_WDOG2 interrupt 45"]
    #[inline(always)]
    pub const fn set_pri45(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip45 {
    #[inline(always)]
    fn default() -> Nvicip45 {
        Nvicip45(0)
    }
}
impl core::fmt::Debug for Nvicip45 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip45")
            .field("pri45", &self.pri45())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip45 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip45 {{ pri45: {=u8:?} }}", self.pri45())
    }
}
#[doc = "Interrupt Priority Register 46"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip46(pub u8);
impl Nvicip46 {
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER interrupt 46"]
    #[must_use]
    #[inline(always)]
    pub const fn pri46(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER interrupt 46"]
    #[inline(always)]
    pub const fn set_pri46(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip46 {
    #[inline(always)]
    fn default() -> Nvicip46 {
        Nvicip46(0)
    }
}
impl core::fmt::Debug for Nvicip46 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip46")
            .field("pri46", &self.pri46())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip46 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip46 {{ pri46: {=u8:?} }}", self.pri46())
    }
}
#[doc = "Interrupt Priority Register 47"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip47(pub u8);
impl Nvicip47 {
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER_TZ interrupt 47"]
    #[must_use]
    #[inline(always)]
    pub const fn pri47(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER_TZ interrupt 47"]
    #[inline(always)]
    pub const fn set_pri47(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip47 {
    #[inline(always)]
    fn default() -> Nvicip47 {
        Nvicip47(0)
    }
}
impl core::fmt::Debug for Nvicip47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip47")
            .field("pri47", &self.pri47())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip47 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip47 {{ pri47: {=u8:?} }}", self.pri47())
    }
}
#[doc = "Interrupt Priority Register 48"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip48(pub u8);
impl Nvicip48 {
    #[doc = "Priority of the INT_SNVS_LP_WRAPPER interrupt 48"]
    #[must_use]
    #[inline(always)]
    pub const fn pri48(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SNVS_LP_WRAPPER interrupt 48"]
    #[inline(always)]
    pub const fn set_pri48(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip48 {
    #[inline(always)]
    fn default() -> Nvicip48 {
        Nvicip48(0)
    }
}
impl core::fmt::Debug for Nvicip48 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip48")
            .field("pri48", &self.pri48())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip48 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip48 {{ pri48: {=u8:?} }}", self.pri48())
    }
}
#[doc = "Interrupt Priority Register 49"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip49(pub u8);
impl Nvicip49 {
    #[doc = "Priority of the INT_CSU interrupt 49"]
    #[must_use]
    #[inline(always)]
    pub const fn pri49(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CSU interrupt 49"]
    #[inline(always)]
    pub const fn set_pri49(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip49 {
    #[inline(always)]
    fn default() -> Nvicip49 {
        Nvicip49(0)
    }
}
impl core::fmt::Debug for Nvicip49 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip49")
            .field("pri49", &self.pri49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip49 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip49 {{ pri49: {=u8:?} }}", self.pri49())
    }
}
#[doc = "Interrupt Priority Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip5(pub u8);
impl Nvicip5 {
    #[doc = "Priority of the INT_DMA5_DMA21 interrupt 5"]
    #[must_use]
    #[inline(always)]
    pub const fn pri5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA5_DMA21 interrupt 5"]
    #[inline(always)]
    pub const fn set_pri5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip5 {
    #[inline(always)]
    fn default() -> Nvicip5 {
        Nvicip5(0)
    }
}
impl core::fmt::Debug for Nvicip5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip5")
            .field("pri5", &self.pri5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip5 {{ pri5: {=u8:?} }}", self.pri5())
    }
}
#[doc = "Interrupt Priority Register 50"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip50(pub u8);
impl Nvicip50 {
    #[doc = "Priority of the INT_DCP interrupt 50"]
    #[must_use]
    #[inline(always)]
    pub const fn pri50(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DCP interrupt 50"]
    #[inline(always)]
    pub const fn set_pri50(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip50 {
    #[inline(always)]
    fn default() -> Nvicip50 {
        Nvicip50(0)
    }
}
impl core::fmt::Debug for Nvicip50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip50")
            .field("pri50", &self.pri50())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip50 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip50 {{ pri50: {=u8:?} }}", self.pri50())
    }
}
#[doc = "Interrupt Priority Register 51"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip51(pub u8);
impl Nvicip51 {
    #[doc = "Priority of the INT_DCP_VMI interrupt 51"]
    #[must_use]
    #[inline(always)]
    pub const fn pri51(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DCP_VMI interrupt 51"]
    #[inline(always)]
    pub const fn set_pri51(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip51 {
    #[inline(always)]
    fn default() -> Nvicip51 {
        Nvicip51(0)
    }
}
impl core::fmt::Debug for Nvicip51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip51")
            .field("pri51", &self.pri51())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip51 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip51 {{ pri51: {=u8:?} }}", self.pri51())
    }
}
#[doc = "Interrupt Priority Register 52"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip52(pub u8);
impl Nvicip52 {
    #[doc = "Priority of the INT_Reserved68 interrupt 52"]
    #[must_use]
    #[inline(always)]
    pub const fn pri52(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved68 interrupt 52"]
    #[inline(always)]
    pub const fn set_pri52(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip52 {
    #[inline(always)]
    fn default() -> Nvicip52 {
        Nvicip52(0)
    }
}
impl core::fmt::Debug for Nvicip52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip52")
            .field("pri52", &self.pri52())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip52 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip52 {{ pri52: {=u8:?} }}", self.pri52())
    }
}
#[doc = "Interrupt Priority Register 53"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip53(pub u8);
impl Nvicip53 {
    #[doc = "Priority of the INT_TRNG interrupt 53"]
    #[must_use]
    #[inline(always)]
    pub const fn pri53(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TRNG interrupt 53"]
    #[inline(always)]
    pub const fn set_pri53(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip53 {
    #[inline(always)]
    fn default() -> Nvicip53 {
        Nvicip53(0)
    }
}
impl core::fmt::Debug for Nvicip53 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip53")
            .field("pri53", &self.pri53())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip53 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip53 {{ pri53: {=u8:?} }}", self.pri53())
    }
}
#[doc = "Interrupt Priority Register 54"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip54(pub u8);
impl Nvicip54 {
    #[doc = "Priority of the INT_SJC interrupt 54"]
    #[must_use]
    #[inline(always)]
    pub const fn pri54(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SJC interrupt 54"]
    #[inline(always)]
    pub const fn set_pri54(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip54 {
    #[inline(always)]
    fn default() -> Nvicip54 {
        Nvicip54(0)
    }
}
impl core::fmt::Debug for Nvicip54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip54")
            .field("pri54", &self.pri54())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip54 {{ pri54: {=u8:?} }}", self.pri54())
    }
}
#[doc = "Interrupt Priority Register 55"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip55(pub u8);
impl Nvicip55 {
    #[doc = "Priority of the INT_BEE interrupt 55"]
    #[must_use]
    #[inline(always)]
    pub const fn pri55(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_BEE interrupt 55"]
    #[inline(always)]
    pub const fn set_pri55(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip55 {
    #[inline(always)]
    fn default() -> Nvicip55 {
        Nvicip55(0)
    }
}
impl core::fmt::Debug for Nvicip55 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip55")
            .field("pri55", &self.pri55())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip55 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip55 {{ pri55: {=u8:?} }}", self.pri55())
    }
}
#[doc = "Interrupt Priority Register 56"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip56(pub u8);
impl Nvicip56 {
    #[doc = "Priority of the INT_SAI1 interrupt 56"]
    #[must_use]
    #[inline(always)]
    pub const fn pri56(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI1 interrupt 56"]
    #[inline(always)]
    pub const fn set_pri56(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip56 {
    #[inline(always)]
    fn default() -> Nvicip56 {
        Nvicip56(0)
    }
}
impl core::fmt::Debug for Nvicip56 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip56")
            .field("pri56", &self.pri56())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip56 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip56 {{ pri56: {=u8:?} }}", self.pri56())
    }
}
#[doc = "Interrupt Priority Register 57"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip57(pub u8);
impl Nvicip57 {
    #[doc = "Priority of the INT_SAI2 interrupt 57"]
    #[must_use]
    #[inline(always)]
    pub const fn pri57(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI2 interrupt 57"]
    #[inline(always)]
    pub const fn set_pri57(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip57 {
    #[inline(always)]
    fn default() -> Nvicip57 {
        Nvicip57(0)
    }
}
impl core::fmt::Debug for Nvicip57 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip57")
            .field("pri57", &self.pri57())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip57 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip57 {{ pri57: {=u8:?} }}", self.pri57())
    }
}
#[doc = "Interrupt Priority Register 58"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip58(pub u8);
impl Nvicip58 {
    #[doc = "Priority of the INT_SAI3_RX interrupt 58"]
    #[must_use]
    #[inline(always)]
    pub const fn pri58(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI3_RX interrupt 58"]
    #[inline(always)]
    pub const fn set_pri58(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip58 {
    #[inline(always)]
    fn default() -> Nvicip58 {
        Nvicip58(0)
    }
}
impl core::fmt::Debug for Nvicip58 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip58")
            .field("pri58", &self.pri58())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip58 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip58 {{ pri58: {=u8:?} }}", self.pri58())
    }
}
#[doc = "Interrupt Priority Register 59"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip59(pub u8);
impl Nvicip59 {
    #[doc = "Priority of the INT_SAI3_TX interrupt 59"]
    #[must_use]
    #[inline(always)]
    pub const fn pri59(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI3_TX interrupt 59"]
    #[inline(always)]
    pub const fn set_pri59(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip59 {
    #[inline(always)]
    fn default() -> Nvicip59 {
        Nvicip59(0)
    }
}
impl core::fmt::Debug for Nvicip59 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip59")
            .field("pri59", &self.pri59())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip59 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip59 {{ pri59: {=u8:?} }}", self.pri59())
    }
}
#[doc = "Interrupt Priority Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip6(pub u8);
impl Nvicip6 {
    #[doc = "Priority of the INT_DMA6_DMA22 interrupt 6"]
    #[must_use]
    #[inline(always)]
    pub const fn pri6(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA6_DMA22 interrupt 6"]
    #[inline(always)]
    pub const fn set_pri6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip6 {
    #[inline(always)]
    fn default() -> Nvicip6 {
        Nvicip6(0)
    }
}
impl core::fmt::Debug for Nvicip6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip6")
            .field("pri6", &self.pri6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip6 {{ pri6: {=u8:?} }}", self.pri6())
    }
}
#[doc = "Interrupt Priority Register 60"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip60(pub u8);
impl Nvicip60 {
    #[doc = "Priority of the INT_SPDIF interrupt 60"]
    #[must_use]
    #[inline(always)]
    pub const fn pri60(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SPDIF interrupt 60"]
    #[inline(always)]
    pub const fn set_pri60(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip60 {
    #[inline(always)]
    fn default() -> Nvicip60 {
        Nvicip60(0)
    }
}
impl core::fmt::Debug for Nvicip60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip60")
            .field("pri60", &self.pri60())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip60 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip60 {{ pri60: {=u8:?} }}", self.pri60())
    }
}
#[doc = "Interrupt Priority Register 61"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip61(pub u8);
impl Nvicip61 {
    #[doc = "Priority of the INT_PMU_EVENT interrupt 61"]
    #[must_use]
    #[inline(always)]
    pub const fn pri61(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PMU_EVENT interrupt 61"]
    #[inline(always)]
    pub const fn set_pri61(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip61 {
    #[inline(always)]
    fn default() -> Nvicip61 {
        Nvicip61(0)
    }
}
impl core::fmt::Debug for Nvicip61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip61")
            .field("pri61", &self.pri61())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip61 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip61 {{ pri61: {=u8:?} }}", self.pri61())
    }
}
#[doc = "Interrupt Priority Register 62"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip62(pub u8);
impl Nvicip62 {
    #[doc = "Priority of the INT_Reserved78 interrupt 62"]
    #[must_use]
    #[inline(always)]
    pub const fn pri62(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved78 interrupt 62"]
    #[inline(always)]
    pub const fn set_pri62(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip62 {
    #[inline(always)]
    fn default() -> Nvicip62 {
        Nvicip62(0)
    }
}
impl core::fmt::Debug for Nvicip62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip62")
            .field("pri62", &self.pri62())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip62 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip62 {{ pri62: {=u8:?} }}", self.pri62())
    }
}
#[doc = "Interrupt Priority Register 63"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip63(pub u8);
impl Nvicip63 {
    #[doc = "Priority of the INT_TEMP_LOW_HIGH interrupt 63"]
    #[must_use]
    #[inline(always)]
    pub const fn pri63(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TEMP_LOW_HIGH interrupt 63"]
    #[inline(always)]
    pub const fn set_pri63(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip63 {
    #[inline(always)]
    fn default() -> Nvicip63 {
        Nvicip63(0)
    }
}
impl core::fmt::Debug for Nvicip63 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip63")
            .field("pri63", &self.pri63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip63 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip63 {{ pri63: {=u8:?} }}", self.pri63())
    }
}
#[doc = "Interrupt Priority Register 64"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip64(pub u8);
impl Nvicip64 {
    #[doc = "Priority of the INT_TEMP_PANIC interrupt 64"]
    #[must_use]
    #[inline(always)]
    pub const fn pri64(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TEMP_PANIC interrupt 64"]
    #[inline(always)]
    pub const fn set_pri64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip64 {
    #[inline(always)]
    fn default() -> Nvicip64 {
        Nvicip64(0)
    }
}
impl core::fmt::Debug for Nvicip64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip64")
            .field("pri64", &self.pri64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip64 {{ pri64: {=u8:?} }}", self.pri64())
    }
}
#[doc = "Interrupt Priority Register 65"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip65(pub u8);
impl Nvicip65 {
    #[doc = "Priority of the INT_USB_PHY1 interrupt 65"]
    #[must_use]
    #[inline(always)]
    pub const fn pri65(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USB_PHY1 interrupt 65"]
    #[inline(always)]
    pub const fn set_pri65(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip65 {
    #[inline(always)]
    fn default() -> Nvicip65 {
        Nvicip65(0)
    }
}
impl core::fmt::Debug for Nvicip65 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip65")
            .field("pri65", &self.pri65())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip65 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip65 {{ pri65: {=u8:?} }}", self.pri65())
    }
}
#[doc = "Interrupt Priority Register 66"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip66(pub u8);
impl Nvicip66 {
    #[doc = "Priority of the INT_USB_PHY2 interrupt 66"]
    #[must_use]
    #[inline(always)]
    pub const fn pri66(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USB_PHY2 interrupt 66"]
    #[inline(always)]
    pub const fn set_pri66(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip66 {
    #[inline(always)]
    fn default() -> Nvicip66 {
        Nvicip66(0)
    }
}
impl core::fmt::Debug for Nvicip66 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip66")
            .field("pri66", &self.pri66())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip66 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip66 {{ pri66: {=u8:?} }}", self.pri66())
    }
}
#[doc = "Interrupt Priority Register 67"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip67(pub u8);
impl Nvicip67 {
    #[doc = "Priority of the INT_ADC1 interrupt 67"]
    #[must_use]
    #[inline(always)]
    pub const fn pri67(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC1 interrupt 67"]
    #[inline(always)]
    pub const fn set_pri67(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip67 {
    #[inline(always)]
    fn default() -> Nvicip67 {
        Nvicip67(0)
    }
}
impl core::fmt::Debug for Nvicip67 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip67")
            .field("pri67", &self.pri67())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip67 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip67 {{ pri67: {=u8:?} }}", self.pri67())
    }
}
#[doc = "Interrupt Priority Register 68"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip68(pub u8);
impl Nvicip68 {
    #[doc = "Priority of the INT_ADC2 interrupt 68"]
    #[must_use]
    #[inline(always)]
    pub const fn pri68(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC2 interrupt 68"]
    #[inline(always)]
    pub const fn set_pri68(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip68 {
    #[inline(always)]
    fn default() -> Nvicip68 {
        Nvicip68(0)
    }
}
impl core::fmt::Debug for Nvicip68 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip68")
            .field("pri68", &self.pri68())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip68 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip68 {{ pri68: {=u8:?} }}", self.pri68())
    }
}
#[doc = "Interrupt Priority Register 69"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip69(pub u8);
impl Nvicip69 {
    #[doc = "Priority of the INT_DCDC interrupt 69"]
    #[must_use]
    #[inline(always)]
    pub const fn pri69(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DCDC interrupt 69"]
    #[inline(always)]
    pub const fn set_pri69(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip69 {
    #[inline(always)]
    fn default() -> Nvicip69 {
        Nvicip69(0)
    }
}
impl core::fmt::Debug for Nvicip69 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip69")
            .field("pri69", &self.pri69())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip69 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip69 {{ pri69: {=u8:?} }}", self.pri69())
    }
}
#[doc = "Interrupt Priority Register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip7(pub u8);
impl Nvicip7 {
    #[doc = "Priority of the INT_DMA7_DMA23 interrupt 7"]
    #[must_use]
    #[inline(always)]
    pub const fn pri7(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA7_DMA23 interrupt 7"]
    #[inline(always)]
    pub const fn set_pri7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip7 {
    #[inline(always)]
    fn default() -> Nvicip7 {
        Nvicip7(0)
    }
}
impl core::fmt::Debug for Nvicip7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip7")
            .field("pri7", &self.pri7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip7 {{ pri7: {=u8:?} }}", self.pri7())
    }
}
#[doc = "Interrupt Priority Register 70"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip70(pub u8);
impl Nvicip70 {
    #[doc = "Priority of the INT_Reserved86 interrupt 70"]
    #[must_use]
    #[inline(always)]
    pub const fn pri70(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved86 interrupt 70"]
    #[inline(always)]
    pub const fn set_pri70(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip70 {
    #[inline(always)]
    fn default() -> Nvicip70 {
        Nvicip70(0)
    }
}
impl core::fmt::Debug for Nvicip70 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip70")
            .field("pri70", &self.pri70())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip70 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip70 {{ pri70: {=u8:?} }}", self.pri70())
    }
}
#[doc = "Interrupt Priority Register 71"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip71(pub u8);
impl Nvicip71 {
    #[doc = "Priority of the INT_GPIO10_Combined_0_31 interrupt 71"]
    #[must_use]
    #[inline(always)]
    pub const fn pri71(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO10_Combined_0_31 interrupt 71"]
    #[inline(always)]
    pub const fn set_pri71(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip71 {
    #[inline(always)]
    fn default() -> Nvicip71 {
        Nvicip71(0)
    }
}
impl core::fmt::Debug for Nvicip71 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip71")
            .field("pri71", &self.pri71())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip71 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip71 {{ pri71: {=u8:?} }}", self.pri71())
    }
}
#[doc = "Interrupt Priority Register 72"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip72(pub u8);
impl Nvicip72 {
    #[doc = "Priority of the INT_GPIO1_INT0 interrupt 72"]
    #[must_use]
    #[inline(always)]
    pub const fn pri72(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT0 interrupt 72"]
    #[inline(always)]
    pub const fn set_pri72(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip72 {
    #[inline(always)]
    fn default() -> Nvicip72 {
        Nvicip72(0)
    }
}
impl core::fmt::Debug for Nvicip72 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip72")
            .field("pri72", &self.pri72())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip72 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip72 {{ pri72: {=u8:?} }}", self.pri72())
    }
}
#[doc = "Interrupt Priority Register 73"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip73(pub u8);
impl Nvicip73 {
    #[doc = "Priority of the INT_GPIO1_INT1 interrupt 73"]
    #[must_use]
    #[inline(always)]
    pub const fn pri73(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT1 interrupt 73"]
    #[inline(always)]
    pub const fn set_pri73(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip73 {
    #[inline(always)]
    fn default() -> Nvicip73 {
        Nvicip73(0)
    }
}
impl core::fmt::Debug for Nvicip73 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip73")
            .field("pri73", &self.pri73())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip73 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip73 {{ pri73: {=u8:?} }}", self.pri73())
    }
}
#[doc = "Interrupt Priority Register 74"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip74(pub u8);
impl Nvicip74 {
    #[doc = "Priority of the INT_GPIO1_INT2 interrupt 74"]
    #[must_use]
    #[inline(always)]
    pub const fn pri74(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT2 interrupt 74"]
    #[inline(always)]
    pub const fn set_pri74(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip74 {
    #[inline(always)]
    fn default() -> Nvicip74 {
        Nvicip74(0)
    }
}
impl core::fmt::Debug for Nvicip74 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip74")
            .field("pri74", &self.pri74())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip74 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip74 {{ pri74: {=u8:?} }}", self.pri74())
    }
}
#[doc = "Interrupt Priority Register 75"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip75(pub u8);
impl Nvicip75 {
    #[doc = "Priority of the INT_GPIO1_INT3 interrupt 75"]
    #[must_use]
    #[inline(always)]
    pub const fn pri75(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT3 interrupt 75"]
    #[inline(always)]
    pub const fn set_pri75(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip75 {
    #[inline(always)]
    fn default() -> Nvicip75 {
        Nvicip75(0)
    }
}
impl core::fmt::Debug for Nvicip75 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip75")
            .field("pri75", &self.pri75())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip75 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip75 {{ pri75: {=u8:?} }}", self.pri75())
    }
}
#[doc = "Interrupt Priority Register 76"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip76(pub u8);
impl Nvicip76 {
    #[doc = "Priority of the INT_GPIO1_INT4 interrupt 76"]
    #[must_use]
    #[inline(always)]
    pub const fn pri76(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT4 interrupt 76"]
    #[inline(always)]
    pub const fn set_pri76(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip76 {
    #[inline(always)]
    fn default() -> Nvicip76 {
        Nvicip76(0)
    }
}
impl core::fmt::Debug for Nvicip76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip76")
            .field("pri76", &self.pri76())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip76 {{ pri76: {=u8:?} }}", self.pri76())
    }
}
#[doc = "Interrupt Priority Register 77"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip77(pub u8);
impl Nvicip77 {
    #[doc = "Priority of the INT_GPIO1_INT5 interrupt 77"]
    #[must_use]
    #[inline(always)]
    pub const fn pri77(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT5 interrupt 77"]
    #[inline(always)]
    pub const fn set_pri77(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip77 {
    #[inline(always)]
    fn default() -> Nvicip77 {
        Nvicip77(0)
    }
}
impl core::fmt::Debug for Nvicip77 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip77")
            .field("pri77", &self.pri77())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip77 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip77 {{ pri77: {=u8:?} }}", self.pri77())
    }
}
#[doc = "Interrupt Priority Register 78"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip78(pub u8);
impl Nvicip78 {
    #[doc = "Priority of the INT_GPIO1_INT6 interrupt 78"]
    #[must_use]
    #[inline(always)]
    pub const fn pri78(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT6 interrupt 78"]
    #[inline(always)]
    pub const fn set_pri78(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip78 {
    #[inline(always)]
    fn default() -> Nvicip78 {
        Nvicip78(0)
    }
}
impl core::fmt::Debug for Nvicip78 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip78")
            .field("pri78", &self.pri78())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip78 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip78 {{ pri78: {=u8:?} }}", self.pri78())
    }
}
#[doc = "Interrupt Priority Register 79"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip79(pub u8);
impl Nvicip79 {
    #[doc = "Priority of the INT_GPIO1_INT7 interrupt 79"]
    #[must_use]
    #[inline(always)]
    pub const fn pri79(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_INT7 interrupt 79"]
    #[inline(always)]
    pub const fn set_pri79(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip79 {
    #[inline(always)]
    fn default() -> Nvicip79 {
        Nvicip79(0)
    }
}
impl core::fmt::Debug for Nvicip79 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip79")
            .field("pri79", &self.pri79())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip79 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip79 {{ pri79: {=u8:?} }}", self.pri79())
    }
}
#[doc = "Interrupt Priority Register 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip8(pub u8);
impl Nvicip8 {
    #[doc = "Priority of the INT_DMA8_DMA24 interrupt 8"]
    #[must_use]
    #[inline(always)]
    pub const fn pri8(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA8_DMA24 interrupt 8"]
    #[inline(always)]
    pub const fn set_pri8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip8 {
    #[inline(always)]
    fn default() -> Nvicip8 {
        Nvicip8(0)
    }
}
impl core::fmt::Debug for Nvicip8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip8")
            .field("pri8", &self.pri8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip8 {{ pri8: {=u8:?} }}", self.pri8())
    }
}
#[doc = "Interrupt Priority Register 80"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip80(pub u8);
impl Nvicip80 {
    #[doc = "Priority of the INT_GPIO1_Combined_0_15 interrupt 80"]
    #[must_use]
    #[inline(always)]
    pub const fn pri80(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_Combined_0_15 interrupt 80"]
    #[inline(always)]
    pub const fn set_pri80(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip80 {
    #[inline(always)]
    fn default() -> Nvicip80 {
        Nvicip80(0)
    }
}
impl core::fmt::Debug for Nvicip80 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip80")
            .field("pri80", &self.pri80())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip80 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip80 {{ pri80: {=u8:?} }}", self.pri80())
    }
}
#[doc = "Interrupt Priority Register 81"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip81(pub u8);
impl Nvicip81 {
    #[doc = "Priority of the INT_GPIO1_Combined_16_31 interrupt 81"]
    #[must_use]
    #[inline(always)]
    pub const fn pri81(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_Combined_16_31 interrupt 81"]
    #[inline(always)]
    pub const fn set_pri81(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip81 {
    #[inline(always)]
    fn default() -> Nvicip81 {
        Nvicip81(0)
    }
}
impl core::fmt::Debug for Nvicip81 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip81")
            .field("pri81", &self.pri81())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip81 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip81 {{ pri81: {=u8:?} }}", self.pri81())
    }
}
#[doc = "Interrupt Priority Register 82"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip82(pub u8);
impl Nvicip82 {
    #[doc = "Priority of the INT_GPIO2_Combined_0_15 interrupt 82"]
    #[must_use]
    #[inline(always)]
    pub const fn pri82(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO2_Combined_0_15 interrupt 82"]
    #[inline(always)]
    pub const fn set_pri82(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip82 {
    #[inline(always)]
    fn default() -> Nvicip82 {
        Nvicip82(0)
    }
}
impl core::fmt::Debug for Nvicip82 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip82")
            .field("pri82", &self.pri82())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip82 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip82 {{ pri82: {=u8:?} }}", self.pri82())
    }
}
#[doc = "Interrupt Priority Register 83"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip83(pub u8);
impl Nvicip83 {
    #[doc = "Priority of the INT_GPIO2_Combined_16_31 interrupt 83"]
    #[must_use]
    #[inline(always)]
    pub const fn pri83(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO2_Combined_16_31 interrupt 83"]
    #[inline(always)]
    pub const fn set_pri83(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip83 {
    #[inline(always)]
    fn default() -> Nvicip83 {
        Nvicip83(0)
    }
}
impl core::fmt::Debug for Nvicip83 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip83")
            .field("pri83", &self.pri83())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip83 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip83 {{ pri83: {=u8:?} }}", self.pri83())
    }
}
#[doc = "Interrupt Priority Register 84"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip84(pub u8);
impl Nvicip84 {
    #[doc = "Priority of the INT_GPIO3_Combined_0_15 interrupt 84"]
    #[must_use]
    #[inline(always)]
    pub const fn pri84(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO3_Combined_0_15 interrupt 84"]
    #[inline(always)]
    pub const fn set_pri84(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip84 {
    #[inline(always)]
    fn default() -> Nvicip84 {
        Nvicip84(0)
    }
}
impl core::fmt::Debug for Nvicip84 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip84")
            .field("pri84", &self.pri84())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip84 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip84 {{ pri84: {=u8:?} }}", self.pri84())
    }
}
#[doc = "Interrupt Priority Register 85"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip85(pub u8);
impl Nvicip85 {
    #[doc = "Priority of the INT_GPIO3_Combined_16_31 interrupt 85"]
    #[must_use]
    #[inline(always)]
    pub const fn pri85(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO3_Combined_16_31 interrupt 85"]
    #[inline(always)]
    pub const fn set_pri85(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip85 {
    #[inline(always)]
    fn default() -> Nvicip85 {
        Nvicip85(0)
    }
}
impl core::fmt::Debug for Nvicip85 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip85")
            .field("pri85", &self.pri85())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip85 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip85 {{ pri85: {=u8:?} }}", self.pri85())
    }
}
#[doc = "Interrupt Priority Register 86"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip86(pub u8);
impl Nvicip86 {
    #[doc = "Priority of the INT_GPIO4_Combined_0_15 interrupt 86"]
    #[must_use]
    #[inline(always)]
    pub const fn pri86(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO4_Combined_0_15 interrupt 86"]
    #[inline(always)]
    pub const fn set_pri86(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip86 {
    #[inline(always)]
    fn default() -> Nvicip86 {
        Nvicip86(0)
    }
}
impl core::fmt::Debug for Nvicip86 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip86")
            .field("pri86", &self.pri86())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip86 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip86 {{ pri86: {=u8:?} }}", self.pri86())
    }
}
#[doc = "Interrupt Priority Register 87"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip87(pub u8);
impl Nvicip87 {
    #[doc = "Priority of the INT_GPIO4_Combined_16_31 interrupt 87"]
    #[must_use]
    #[inline(always)]
    pub const fn pri87(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO4_Combined_16_31 interrupt 87"]
    #[inline(always)]
    pub const fn set_pri87(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip87 {
    #[inline(always)]
    fn default() -> Nvicip87 {
        Nvicip87(0)
    }
}
impl core::fmt::Debug for Nvicip87 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip87")
            .field("pri87", &self.pri87())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip87 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip87 {{ pri87: {=u8:?} }}", self.pri87())
    }
}
#[doc = "Interrupt Priority Register 88"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip88(pub u8);
impl Nvicip88 {
    #[doc = "Priority of the INT_GPIO5_Combined_0_15 interrupt 88"]
    #[must_use]
    #[inline(always)]
    pub const fn pri88(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO5_Combined_0_15 interrupt 88"]
    #[inline(always)]
    pub const fn set_pri88(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip88 {
    #[inline(always)]
    fn default() -> Nvicip88 {
        Nvicip88(0)
    }
}
impl core::fmt::Debug for Nvicip88 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip88")
            .field("pri88", &self.pri88())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip88 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip88 {{ pri88: {=u8:?} }}", self.pri88())
    }
}
#[doc = "Interrupt Priority Register 89"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip89(pub u8);
impl Nvicip89 {
    #[doc = "Priority of the INT_GPIO5_Combined_16_31 interrupt 89"]
    #[must_use]
    #[inline(always)]
    pub const fn pri89(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO5_Combined_16_31 interrupt 89"]
    #[inline(always)]
    pub const fn set_pri89(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip89 {
    #[inline(always)]
    fn default() -> Nvicip89 {
        Nvicip89(0)
    }
}
impl core::fmt::Debug for Nvicip89 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip89")
            .field("pri89", &self.pri89())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip89 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip89 {{ pri89: {=u8:?} }}", self.pri89())
    }
}
#[doc = "Interrupt Priority Register 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip9(pub u8);
impl Nvicip9 {
    #[doc = "Priority of the INT_DMA9_DMA25 interrupt 9"]
    #[must_use]
    #[inline(always)]
    pub const fn pri9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA9_DMA25 interrupt 9"]
    #[inline(always)]
    pub const fn set_pri9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip9 {
    #[inline(always)]
    fn default() -> Nvicip9 {
        Nvicip9(0)
    }
}
impl core::fmt::Debug for Nvicip9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip9")
            .field("pri9", &self.pri9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip9 {{ pri9: {=u8:?} }}", self.pri9())
    }
}
#[doc = "Interrupt Priority Register 90"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip90(pub u8);
impl Nvicip90 {
    #[doc = "Priority of the INT_FLEXIO1 interrupt 90"]
    #[must_use]
    #[inline(always)]
    pub const fn pri90(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXIO1 interrupt 90"]
    #[inline(always)]
    pub const fn set_pri90(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip90 {
    #[inline(always)]
    fn default() -> Nvicip90 {
        Nvicip90(0)
    }
}
impl core::fmt::Debug for Nvicip90 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip90")
            .field("pri90", &self.pri90())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip90 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip90 {{ pri90: {=u8:?} }}", self.pri90())
    }
}
#[doc = "Interrupt Priority Register 91"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip91(pub u8);
impl Nvicip91 {
    #[doc = "Priority of the INT_FLEXIO2 interrupt 91"]
    #[must_use]
    #[inline(always)]
    pub const fn pri91(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXIO2 interrupt 91"]
    #[inline(always)]
    pub const fn set_pri91(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip91 {
    #[inline(always)]
    fn default() -> Nvicip91 {
        Nvicip91(0)
    }
}
impl core::fmt::Debug for Nvicip91 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip91")
            .field("pri91", &self.pri91())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip91 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip91 {{ pri91: {=u8:?} }}", self.pri91())
    }
}
#[doc = "Interrupt Priority Register 92"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip92(pub u8);
impl Nvicip92 {
    #[doc = "Priority of the INT_WDOG1 interrupt 92"]
    #[must_use]
    #[inline(always)]
    pub const fn pri92(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_WDOG1 interrupt 92"]
    #[inline(always)]
    pub const fn set_pri92(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip92 {
    #[inline(always)]
    fn default() -> Nvicip92 {
        Nvicip92(0)
    }
}
impl core::fmt::Debug for Nvicip92 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip92")
            .field("pri92", &self.pri92())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip92 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip92 {{ pri92: {=u8:?} }}", self.pri92())
    }
}
#[doc = "Interrupt Priority Register 93"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip93(pub u8);
impl Nvicip93 {
    #[doc = "Priority of the INT_RTWDOG interrupt 93"]
    #[must_use]
    #[inline(always)]
    pub const fn pri93(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_RTWDOG interrupt 93"]
    #[inline(always)]
    pub const fn set_pri93(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip93 {
    #[inline(always)]
    fn default() -> Nvicip93 {
        Nvicip93(0)
    }
}
impl core::fmt::Debug for Nvicip93 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip93")
            .field("pri93", &self.pri93())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip93 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip93 {{ pri93: {=u8:?} }}", self.pri93())
    }
}
#[doc = "Interrupt Priority Register 94"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip94(pub u8);
impl Nvicip94 {
    #[doc = "Priority of the INT_EWM interrupt 94"]
    #[must_use]
    #[inline(always)]
    pub const fn pri94(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_EWM interrupt 94"]
    #[inline(always)]
    pub const fn set_pri94(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip94 {
    #[inline(always)]
    fn default() -> Nvicip94 {
        Nvicip94(0)
    }
}
impl core::fmt::Debug for Nvicip94 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip94")
            .field("pri94", &self.pri94())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip94 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip94 {{ pri94: {=u8:?} }}", self.pri94())
    }
}
#[doc = "Interrupt Priority Register 95"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip95(pub u8);
impl Nvicip95 {
    #[doc = "Priority of the INT_CCM_1 interrupt 95"]
    #[must_use]
    #[inline(always)]
    pub const fn pri95(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CCM_1 interrupt 95"]
    #[inline(always)]
    pub const fn set_pri95(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip95 {
    #[inline(always)]
    fn default() -> Nvicip95 {
        Nvicip95(0)
    }
}
impl core::fmt::Debug for Nvicip95 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip95")
            .field("pri95", &self.pri95())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip95 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip95 {{ pri95: {=u8:?} }}", self.pri95())
    }
}
#[doc = "Interrupt Priority Register 96"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip96(pub u8);
impl Nvicip96 {
    #[doc = "Priority of the INT_CCM_2 interrupt 96"]
    #[must_use]
    #[inline(always)]
    pub const fn pri96(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CCM_2 interrupt 96"]
    #[inline(always)]
    pub const fn set_pri96(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip96 {
    #[inline(always)]
    fn default() -> Nvicip96 {
        Nvicip96(0)
    }
}
impl core::fmt::Debug for Nvicip96 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip96")
            .field("pri96", &self.pri96())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip96 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip96 {{ pri96: {=u8:?} }}", self.pri96())
    }
}
#[doc = "Interrupt Priority Register 97"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip97(pub u8);
impl Nvicip97 {
    #[doc = "Priority of the INT_GPC interrupt 97"]
    #[must_use]
    #[inline(always)]
    pub const fn pri97(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPC interrupt 97"]
    #[inline(always)]
    pub const fn set_pri97(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip97 {
    #[inline(always)]
    fn default() -> Nvicip97 {
        Nvicip97(0)
    }
}
impl core::fmt::Debug for Nvicip97 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip97")
            .field("pri97", &self.pri97())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip97 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip97 {{ pri97: {=u8:?} }}", self.pri97())
    }
}
#[doc = "Interrupt Priority Register 98"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip98(pub u8);
impl Nvicip98 {
    #[doc = "Priority of the INT_SRC interrupt 98"]
    #[must_use]
    #[inline(always)]
    pub const fn pri98(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SRC interrupt 98"]
    #[inline(always)]
    pub const fn set_pri98(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip98 {
    #[inline(always)]
    fn default() -> Nvicip98 {
        Nvicip98(0)
    }
}
impl core::fmt::Debug for Nvicip98 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip98")
            .field("pri98", &self.pri98())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip98 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip98 {{ pri98: {=u8:?} }}", self.pri98())
    }
}
#[doc = "Interrupt Priority Register 99"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicip99(pub u8);
impl Nvicip99 {
    #[doc = "Priority of the INT_Reserved115 interrupt 99"]
    #[must_use]
    #[inline(always)]
    pub const fn pri99(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved115 interrupt 99"]
    #[inline(always)]
    pub const fn set_pri99(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip99 {
    #[inline(always)]
    fn default() -> Nvicip99 {
        Nvicip99(0)
    }
}
impl core::fmt::Debug for Nvicip99 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicip99")
            .field("pri99", &self.pri99())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicip99 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicip99 {{ pri99: {=u8:?} }}", self.pri99())
    }
}
#[doc = "Software Trigger Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvicstir(pub u32);
impl Nvicstir {
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
    #[must_use]
    #[inline(always)]
    pub const fn intid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
    #[inline(always)]
    pub const fn set_intid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Nvicstir {
    #[inline(always)]
    fn default() -> Nvicstir {
        Nvicstir(0)
    }
}
impl core::fmt::Debug for Nvicstir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nvicstir")
            .field("intid", &self.intid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nvicstir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nvicstir {{ intid: {=u16:?} }}", self.intid())
    }
}
