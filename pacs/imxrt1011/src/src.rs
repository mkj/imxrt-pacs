#[doc = "SRC"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Src {
    ptr: *mut u8,
}
unsafe impl Send for Src {}
unsafe impl Sync for Src {}
impl Src {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRC Control Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "SRC Boot Mode Register 1"]
    #[inline(always)]
    pub const fn sbmr1(self) -> crate::common::Reg<regs::Sbmr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "SRC Reset Status Register"]
    #[inline(always)]
    pub const fn srsr(self) -> crate::common::Reg<regs::Srsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "SRC Boot Mode Register 2"]
    #[inline(always)]
    pub const fn sbmr2(self) -> crate::common::Reg<regs::Sbmr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "SRC General Purpose Register 1"]
    #[inline(always)]
    pub const fn gpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "SRC General Purpose Register 2"]
    #[inline(always)]
    pub const fn gpr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "SRC General Purpose Register 3"]
    #[inline(always)]
    pub const fn gpr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "SRC General Purpose Register 4"]
    #[inline(always)]
    pub const fn gpr4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "SRC General Purpose Register 5"]
    #[inline(always)]
    pub const fn gpr5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "SRC General Purpose Register 6"]
    #[inline(always)]
    pub const fn gpr6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "SRC General Purpose Register 7"]
    #[inline(always)]
    pub const fn gpr7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "SRC General Purpose Register 8"]
    #[inline(always)]
    pub const fn gpr8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "SRC General Purpose Register 9"]
    #[inline(always)]
    pub const fn gpr9(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "SRC General Purpose Register 10"]
    #[inline(always)]
    pub const fn gpr10(self) -> crate::common::Reg<regs::Gpr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
}
pub mod regs;
pub mod vals;
