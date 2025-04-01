#[doc = "XTALOSC24M"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Xtalosc24m {
    ptr: *mut u8,
}
unsafe impl Send for Xtalosc24m {}
unsafe impl Sync for Xtalosc24m {}
impl Xtalosc24m {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0(self) -> crate::common::Reg<regs::Misc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_set(self) -> crate::common::Reg<regs::Misc0set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_clr(self) -> crate::common::Reg<regs::Misc0clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_tog(self) -> crate::common::Reg<regs::Misc0tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl(self) -> crate::common::Reg<regs::LowpwrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(624usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl_set(
        self,
    ) -> crate::common::Reg<regs::LowpwrCtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(628usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl_clr(
        self,
    ) -> crate::common::Reg<regs::LowpwrCtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(632usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl_tog(
        self,
    ) -> crate::common::Reg<regs::LowpwrCtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(636usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0(self) -> crate::common::Reg<regs::OscConfig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0_set(
        self,
    ) -> crate::common::Reg<regs::OscConfig0set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(676usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0_clr(
        self,
    ) -> crate::common::Reg<regs::OscConfig0clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(680usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0_tog(
        self,
    ) -> crate::common::Reg<regs::OscConfig0tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(684usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1(self) -> crate::common::Reg<regs::OscConfig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1_set(
        self,
    ) -> crate::common::Reg<regs::OscConfig1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(692usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1_clr(
        self,
    ) -> crate::common::Reg<regs::OscConfig1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(696usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1_tog(
        self,
    ) -> crate::common::Reg<regs::OscConfig1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(700usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2(self) -> crate::common::Reg<regs::OscConfig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(704usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2_set(
        self,
    ) -> crate::common::Reg<regs::OscConfig2set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(708usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2_clr(
        self,
    ) -> crate::common::Reg<regs::OscConfig2clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(712usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2_tog(
        self,
    ) -> crate::common::Reg<regs::OscConfig2tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(716usize) as _) }
    }
}
pub mod regs;
pub mod vals;
