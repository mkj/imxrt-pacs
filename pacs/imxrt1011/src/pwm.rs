#[doc = "PWM"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn sm(self, n: usize) -> Sm {
        assert!(n < 4usize);
        unsafe { Sm::from_ptr(self.ptr.add(0usize + n * 96usize) as _) }
    }
    #[doc = "Output Enable Register"]
    #[inline(always)]
    pub const fn outen(self) -> crate::common::Reg<regs::Outen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Mask Register"]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(386usize) as _) }
    }
    #[doc = "Software Controlled Output Register"]
    #[inline(always)]
    pub const fn swcout(self) -> crate::common::Reg<regs::Swcout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "PWM Source Select Register"]
    #[inline(always)]
    pub const fn dtsrcsel(self) -> crate::common::Reg<regs::Dtsrcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(390usize) as _) }
    }
    #[doc = "Master Control Register"]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "Master Control 2 Register"]
    #[inline(always)]
    pub const fn mctrl2(self) -> crate::common::Reg<regs::Mctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(394usize) as _) }
    }
    #[doc = "Fault Control Register"]
    #[inline(always)]
    pub const fn fctrl0(self) -> crate::common::Reg<regs::Fctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(396usize) as _) }
    }
    #[doc = "Fault Status Register"]
    #[inline(always)]
    pub const fn fsts0(self) -> crate::common::Reg<regs::Fsts0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(398usize) as _) }
    }
    #[doc = "Fault Filter Register"]
    #[inline(always)]
    pub const fn ffilt0(self) -> crate::common::Reg<regs::Ffilt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(400usize) as _) }
    }
    #[doc = "Fault Test Register"]
    #[inline(always)]
    pub const fn ftst0(self) -> crate::common::Reg<regs::Ftst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(402usize) as _) }
    }
    #[doc = "Fault Control 2 Register"]
    #[inline(always)]
    pub const fn fctrl20(self) -> crate::common::Reg<regs::Fctrl20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(404usize) as _) }
    }
}
#[doc = "no description available"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sm {
    ptr: *mut u8,
}
unsafe impl Send for Sm {}
unsafe impl Sync for Sm {}
impl Sm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn smcnt(self) -> crate::common::Reg<regs::Smcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sminit(self) -> crate::common::Reg<regs::Sminit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn smctrl2(self) -> crate::common::Reg<regs::Smctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn smctrl(self) -> crate::common::Reg<regs::Smctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(6usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn smval0(self) -> crate::common::Reg<regs::Smval0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(10usize) as _) }
    }
    #[doc = "Fractional Value Register 1"]
    #[inline(always)]
    pub const fn smfracval1(self) -> crate::common::Reg<regs::Smfracval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn smval1(self) -> crate::common::Reg<regs::Smval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(14usize) as _) }
    }
    #[doc = "Fractional Value Register 2"]
    #[inline(always)]
    pub const fn smfracval2(self) -> crate::common::Reg<regs::Smfracval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn smval2(self) -> crate::common::Reg<regs::Smval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(18usize) as _) }
    }
    #[doc = "Fractional Value Register 3"]
    #[inline(always)]
    pub const fn smfracval3(self) -> crate::common::Reg<regs::Smfracval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn smval3(self) -> crate::common::Reg<regs::Smval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(22usize) as _) }
    }
    #[doc = "Fractional Value Register 4"]
    #[inline(always)]
    pub const fn smfracval4(self) -> crate::common::Reg<regs::Smfracval4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn smval4(self) -> crate::common::Reg<regs::Smval4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(26usize) as _) }
    }
    #[doc = "Fractional Value Register 5"]
    #[inline(always)]
    pub const fn smfracval5(self) -> crate::common::Reg<regs::Smfracval5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn smval5(self) -> crate::common::Reg<regs::Smval5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(30usize) as _) }
    }
    #[doc = "Fractional Control Register"]
    #[inline(always)]
    pub const fn smfrctrl(self) -> crate::common::Reg<regs::Smfrctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn smoctrl(self) -> crate::common::Reg<regs::Smoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(34usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn smsts(self) -> crate::common::Reg<regs::Smsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sminten(self) -> crate::common::Reg<regs::Sminten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(38usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn smdmaen(self) -> crate::common::Reg<regs::Smdmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn smtctrl(self) -> crate::common::Reg<regs::Smtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(42usize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn smdismap0(self) -> crate::common::Reg<regs::Smdismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn smdtcnt0(self) -> crate::common::Reg<regs::Smdtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn smdtcnt1(self) -> crate::common::Reg<regs::Smdtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(50usize) as _) }
    }
    #[doc = "Capture Control A Register"]
    #[inline(always)]
    pub const fn smcaptctrla(self) -> crate::common::Reg<regs::Smcaptctrla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Capture Compare A Register"]
    #[inline(always)]
    pub const fn smcaptcompa(self) -> crate::common::Reg<regs::Smcaptcompa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(54usize) as _) }
    }
    #[doc = "Capture Control B Register"]
    #[inline(always)]
    pub const fn smcaptctrlb(self) -> crate::common::Reg<regs::Smcaptctrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Capture Compare B Register"]
    #[inline(always)]
    pub const fn smcaptcompb(self) -> crate::common::Reg<regs::Smcaptcompb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(58usize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn smcaptctrlx(self) -> crate::common::Reg<regs::Smcaptctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn smcaptcompx(self) -> crate::common::Reg<regs::Smcaptcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(62usize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn smcval0(self) -> crate::common::Reg<regs::Smcval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn smcval0cyc(self) -> crate::common::Reg<regs::Smcval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(66usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn smcval1(self) -> crate::common::Reg<regs::Smcval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn smcval1cyc(self) -> crate::common::Reg<regs::Smcval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(70usize) as _) }
    }
    #[doc = "Capture Value 2 Register"]
    #[inline(always)]
    pub const fn smcval2(self) -> crate::common::Reg<regs::Smcval2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    #[inline(always)]
    pub const fn smcval2cyc(self) -> crate::common::Reg<regs::Smcval2cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(74usize) as _) }
    }
    #[doc = "Capture Value 3 Register"]
    #[inline(always)]
    pub const fn smcval3(self) -> crate::common::Reg<regs::Smcval3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    #[inline(always)]
    pub const fn smcval3cyc(self) -> crate::common::Reg<regs::Smcval3cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(78usize) as _) }
    }
    #[doc = "Capture Value 4 Register"]
    #[inline(always)]
    pub const fn smcval4(self) -> crate::common::Reg<regs::Smcval4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    #[inline(always)]
    pub const fn smcval4cyc(self) -> crate::common::Reg<regs::Smcval4cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(82usize) as _) }
    }
    #[doc = "Capture Value 5 Register"]
    #[inline(always)]
    pub const fn smcval5(self) -> crate::common::Reg<regs::Smcval5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    #[inline(always)]
    pub const fn smcval5cyc(self) -> crate::common::Reg<regs::Smcval5cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(86usize) as _) }
    }
}
pub mod regs;
pub mod vals;
