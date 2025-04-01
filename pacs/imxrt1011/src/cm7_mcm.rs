#[doc = "CM7_MCM"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7mcm {
    ptr: *mut u8,
}
unsafe impl Send for Cm7mcm {}
unsafe impl Sync for Cm7mcm {}
impl Cm7mcm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Status and Control Register"]
    #[inline(always)]
    pub const fn iscr(self) -> crate::common::Reg<regs::Iscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
}
pub mod regs;
