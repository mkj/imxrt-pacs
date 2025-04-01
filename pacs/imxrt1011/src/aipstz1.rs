#[doc = "AIPSTZ Control Registers"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Aipstz1 {
    ptr: *mut u8,
}
unsafe impl Send for Aipstz1 {}
unsafe impl Sync for Aipstz1 {}
impl Aipstz1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Priviledge Registers"]
    #[inline(always)]
    pub const fn mpr(self) -> crate::common::Reg<regs::Mpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr(self) -> crate::common::Reg<regs::Opacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr1(self) -> crate::common::Reg<regs::Opacr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr2(self) -> crate::common::Reg<regs::Opacr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr3(self) -> crate::common::Reg<regs::Opacr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr4(self) -> crate::common::Reg<regs::Opacr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
}
pub mod regs;
pub mod vals;
