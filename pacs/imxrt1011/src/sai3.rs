#[doc = "SAI"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sai3 {
    ptr: *mut u8,
}
unsafe impl Send for Sai3 {}
unsafe impl Sync for Sai3 {}
impl Sai3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Transmit Control"]
    #[inline(always)]
    pub const fn tcsr(self) -> crate::common::Reg<regs::Tcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Transmit Configuration 1"]
    #[inline(always)]
    pub const fn tcr1(self) -> crate::common::Reg<regs::Tcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Transmit Configuration 2"]
    #[inline(always)]
    pub const fn tcr2(self) -> crate::common::Reg<regs::Tcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Transmit Configuration 3"]
    #[inline(always)]
    pub const fn tcr3(self) -> crate::common::Reg<regs::Tcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Transmit Configuration 4"]
    #[inline(always)]
    pub const fn tcr4(self) -> crate::common::Reg<regs::Tcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Transmit Configuration 5"]
    #[inline(always)]
    pub const fn tcr5(self) -> crate::common::Reg<regs::Tcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn tdr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Transmit FIFO"]
    #[inline(always)]
    pub const fn tfr0(self) -> crate::common::Reg<regs::Tfr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Transmit Mask"]
    #[inline(always)]
    pub const fn tmr(self) -> crate::common::Reg<regs::Tmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Receive Control"]
    #[inline(always)]
    pub const fn rcsr(self) -> crate::common::Reg<regs::Rcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Receive Configuration 1"]
    #[inline(always)]
    pub const fn rcr1(self) -> crate::common::Reg<regs::Rcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Receive Configuration 2"]
    #[inline(always)]
    pub const fn rcr2(self) -> crate::common::Reg<regs::Rcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Receive Configuration 3"]
    #[inline(always)]
    pub const fn rcr3(self) -> crate::common::Reg<regs::Rcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "Receive Configuration 4"]
    #[inline(always)]
    pub const fn rcr4(self) -> crate::common::Reg<regs::Rcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "Receive Configuration 5"]
    #[inline(always)]
    pub const fn rcr5(self) -> crate::common::Reg<regs::Rcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn rdr0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "Receive FIFO"]
    #[inline(always)]
    pub const fn rfr0(self) -> crate::common::Reg<regs::Rfr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Receive Mask"]
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::Rmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
}
pub mod regs;
pub mod vals;
