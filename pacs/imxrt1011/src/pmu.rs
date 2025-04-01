#[doc = "PMU"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pmu {
    ptr: *mut u8,
}
unsafe impl Send for Pmu {}
unsafe impl Sync for Pmu {}
impl Pmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1(self) -> crate::common::Reg<regs::Reg1p1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1_set(self) -> crate::common::Reg<regs::Reg1p1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1_clr(self) -> crate::common::Reg<regs::Reg1p1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(280usize) as _) }
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1_tog(self) -> crate::common::Reg<regs::Reg1p1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(284usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0(self) -> crate::common::Reg<regs::Reg3p0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0_set(self) -> crate::common::Reg<regs::Reg3p0set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0_clr(self) -> crate::common::Reg<regs::Reg3p0clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0_tog(self) -> crate::common::Reg<regs::Reg3p0tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5(self) -> crate::common::Reg<regs::Reg2p5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5_set(self) -> crate::common::Reg<regs::Reg2p5set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5_clr(self) -> crate::common::Reg<regs::Reg2p5clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5_tog(self) -> crate::common::Reg<regs::Reg2p5tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core(self) -> crate::common::Reg<regs::RegCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core_set(self) -> crate::common::Reg<regs::RegCoreSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core_clr(self) -> crate::common::Reg<regs::RegCoreClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(328usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core_tog(self) -> crate::common::Reg<regs::RegCoreTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize) as _) }
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
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1(self) -> crate::common::Reg<regs::Misc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_set(self) -> crate::common::Reg<regs::Misc1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_clr(self) -> crate::common::Reg<regs::Misc1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_tog(self) -> crate::common::Reg<regs::Misc1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2(self) -> crate::common::Reg<regs::Misc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2_set(self) -> crate::common::Reg<regs::Misc2set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2_clr(self) -> crate::common::Reg<regs::Misc2clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2_tog(self) -> crate::common::Reg<regs::Misc2tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
}
pub mod regs;
pub mod vals;
