#[doc = "IOMUXC_GPR"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IomuxcGpr {
    ptr: *mut u8,
}
unsafe impl Send for IomuxcGpr {}
unsafe impl Sync for IomuxcGpr {}
impl IomuxcGpr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPR0 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "GPR1 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr1(self) -> crate::common::Reg<regs::Gpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "GPR2 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr2(self) -> crate::common::Reg<regs::Gpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "GPR3 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr3(self) -> crate::common::Reg<regs::Gpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "GPR4 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr4(self) -> crate::common::Reg<regs::Gpr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "GPR5 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr5(self) -> crate::common::Reg<regs::Gpr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "GPR6 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr6(self) -> crate::common::Reg<regs::Gpr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "GPR7 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr7(self) -> crate::common::Reg<regs::Gpr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "GPR8 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr8(self) -> crate::common::Reg<regs::Gpr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "GPR9 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr9(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "GPR10 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr10(self) -> crate::common::Reg<regs::Gpr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "GPR11 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr11(self) -> crate::common::Reg<regs::Gpr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "GPR12 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr12(self) -> crate::common::Reg<regs::Gpr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "GPR13 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr13(self) -> crate::common::Reg<regs::Gpr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "GPR14 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr14(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "GPR15 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr15(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "GPR16 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr16(self) -> crate::common::Reg<regs::Gpr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "GPR17 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr17(self) -> crate::common::Reg<regs::Gpr17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "GPR18 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr18(self) -> crate::common::Reg<regs::Gpr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "GPR19 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr19(self) -> crate::common::Reg<regs::Gpr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "GPR20 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr20(self) -> crate::common::Reg<regs::Gpr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "GPR21 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr21(self) -> crate::common::Reg<regs::Gpr21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "GPR22 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr22(self) -> crate::common::Reg<regs::Gpr22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "GPR23 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr23(self) -> crate::common::Reg<regs::Gpr23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "GPR24 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr24(self) -> crate::common::Reg<regs::Gpr24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "GPR25 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr25(self) -> crate::common::Reg<regs::Gpr25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "GPR26 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr26(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "GPR27 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr27(self) -> crate::common::Reg<regs::Gpr27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "GPR28 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr28(self) -> crate::common::Reg<regs::Gpr28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "GPR29 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr29(self) -> crate::common::Reg<regs::Gpr29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
}
pub mod regs;
pub mod vals;
