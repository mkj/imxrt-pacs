#[doc = "GPT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "GPT Enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPT Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPT Enable mode"]
    #[inline(always)]
    pub const fn enmod(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPT Enable mode"]
    #[inline(always)]
    pub const fn set_enmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPT debug mode enable"]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPT debug mode enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPT Wait Mode enable"]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPT Wait Mode enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPT Doze Mode Enable"]
    #[inline(always)]
    pub const fn dozeen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPT Doze Mode Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPT Stop Mode enable"]
    #[inline(always)]
    pub const fn stopen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPT Stop Mode enable"]
    #[inline(always)]
    pub const fn set_stopen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Source select"]
    #[inline(always)]
    pub const fn clksrc(&self) -> super::vals::Clksrc {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Clksrc::from_bits(val as u8)
    }
    #[doc = "Clock Source select"]
    #[inline(always)]
    pub const fn set_clksrc(&mut self, val: super::vals::Clksrc) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Free-Run or Restart mode"]
    #[inline(always)]
    pub const fn frr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Free-Run or Restart mode"]
    #[inline(always)]
    pub const fn set_frr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    pub const fn en_24m(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    pub const fn set_en_24m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Software reset"]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "See IM2"]
    #[inline(always)]
    pub const fn im1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "See IM2"]
    #[inline(always)]
    pub const fn set_im1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    pub const fn im2(&self) -> super::vals::Im2 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Im2::from_bits(val as u8)
    }
    #[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    pub const fn set_im2(&mut self, val: super::vals::Im2) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "See OM3"]
    #[inline(always)]
    pub const fn om1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "See OM3"]
    #[inline(always)]
    pub const fn set_om1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "See OM3"]
    #[inline(always)]
    pub const fn om2(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "See OM3"]
    #[inline(always)]
    pub const fn set_om2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    pub const fn om3(&self) -> super::vals::Om3 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Om3::from_bits(val as u8)
    }
    #[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    pub const fn set_om3(&mut self, val: super::vals::Om3) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "See F03"]
    #[inline(always)]
    pub const fn fo1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "See F03"]
    #[inline(always)]
    pub const fn set_fo1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "See F03"]
    #[inline(always)]
    pub const fn fo2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See F03"]
    #[inline(always)]
    pub const fn set_fo2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    pub const fn fo3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    pub const fn set_fo3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "GPT Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "See OF3IE"]
    #[inline(always)]
    pub const fn of1ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3IE"]
    #[inline(always)]
    pub const fn set_of1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See OF3IE"]
    #[inline(always)]
    pub const fn of2ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3IE"]
    #[inline(always)]
    pub const fn set_of2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    pub const fn of3ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    pub const fn set_of3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "See IF2IE"]
    #[inline(always)]
    pub const fn if1ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See IF2IE"]
    #[inline(always)]
    pub const fn set_if1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    pub const fn if2ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_if2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    pub const fn rovie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    pub const fn set_rovie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
#[doc = "GPT Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescaler bits"]
    #[inline(always)]
    pub const fn prescaler(&self) -> super::vals::Prescaler {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::Prescaler::from_bits(val as u16)
    }
    #[doc = "Prescaler bits"]
    #[inline(always)]
    pub const fn set_prescaler(&mut self, val: super::vals::Prescaler) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Prescaler bits"]
    #[inline(always)]
    pub const fn prescaler24m(&self) -> super::vals::Prescaler24m {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Prescaler24m::from_bits(val as u8)
    }
    #[doc = "Prescaler bits"]
    #[inline(always)]
    pub const fn set_prescaler24m(&mut self, val: super::vals::Prescaler24m) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
#[doc = "GPT Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "See OF3"]
    #[inline(always)]
    pub const fn of1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3"]
    #[inline(always)]
    pub const fn set_of1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See OF3"]
    #[inline(always)]
    pub const fn of2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3"]
    #[inline(always)]
    pub const fn set_of2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    pub const fn of3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    pub const fn set_of3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "See IF2"]
    #[inline(always)]
    pub const fn if1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See IF2"]
    #[inline(always)]
    pub const fn set_if1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    pub const fn if2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    pub const fn set_if2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rollover Flag"]
    #[inline(always)]
    pub const fn rov(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rollover Flag"]
    #[inline(always)]
    pub const fn set_rov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
