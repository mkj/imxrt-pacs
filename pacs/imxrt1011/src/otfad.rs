#[doc = "no description available"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctx {
    ptr: *mut u8,
}
unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}
impl Ctx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AES Key Word"]
    #[inline(always)]
    pub const fn ctx_key(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
    #[doc = "AES Counter Word"]
    #[inline(always)]
    pub const fn ctx_ctr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize + n * 4usize) as _) }
    }
    #[doc = "AES Region Descriptor Word0"]
    #[inline(always)]
    pub const fn ctx_rgd_w0(self) -> crate::common::Reg<regs::CtxRgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "AES Region Descriptor Word1"]
    #[inline(always)]
    pub const fn ctx_rgd_w1(self) -> crate::common::Reg<regs::CtxRgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
}
#[doc = "OTFAD"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Otfad {
    ptr: *mut u8,
}
unsafe impl Send for Otfad {}
unsafe impl Sync for Otfad {}
impl Otfad {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3072usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3076usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ctx(self, n: usize) -> Ctx {
        assert!(n < 4usize);
        unsafe { Ctx::from_ptr(self.ptr.add(3328usize + n * 64usize) as _) }
    }
}
pub mod regs;
pub mod vals;
