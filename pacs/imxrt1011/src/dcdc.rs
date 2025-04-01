#[doc = "DCDC"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dcdc {
    ptr: *mut u8,
}
unsafe impl Send for Dcdc {}
unsafe impl Sync for Dcdc {}
impl Dcdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DCDC Register 0"]
    #[inline(always)]
    pub const fn reg0(self) -> crate::common::Reg<regs::Reg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "DCDC Register 1"]
    #[inline(always)]
    pub const fn reg1(self) -> crate::common::Reg<regs::Reg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "DCDC Register 2"]
    #[inline(always)]
    pub const fn reg2(self) -> crate::common::Reg<regs::Reg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DCDC Register 3"]
    #[inline(always)]
    pub const fn reg3(self) -> crate::common::Reg<regs::Reg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
}
pub mod regs;
pub mod vals;
