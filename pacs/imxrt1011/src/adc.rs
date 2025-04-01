#[doc = "Analog-to-Digital Converter"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register for hardware triggers"]
    #[inline(always)]
    pub const fn hc0(self) -> crate::common::Reg<regs::Hc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Control register for hardware triggers"]
    #[inline(always)]
    pub const fn hc(self, n: usize) -> crate::common::Reg<regs::Hc, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize + n * 4usize) as _) }
    }
    #[doc = "Status register for HW triggers"]
    #[inline(always)]
    pub const fn hs(self) -> crate::common::Reg<regs::Hs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Data result register for HW triggers"]
    #[inline(always)]
    pub const fn r0(self) -> crate::common::Reg<regs::R0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Data result register for HW triggers"]
    #[inline(always)]
    pub const fn r(self, n: usize) -> crate::common::Reg<regs::R, crate::common::R> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize + n * 4usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "General control register"]
    #[inline(always)]
    pub const fn gc(self) -> crate::common::Reg<regs::Gc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "General status register"]
    #[inline(always)]
    pub const fn gs(self) -> crate::common::Reg<regs::Gs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Compare value register"]
    #[inline(always)]
    pub const fn cv(self) -> crate::common::Reg<regs::Cv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Offset correction value register"]
    #[inline(always)]
    pub const fn ofs(self) -> crate::common::Reg<regs::Ofs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Calibration value register"]
    #[inline(always)]
    pub const fn cal(self) -> crate::common::Reg<regs::Cal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
}
pub mod regs;
pub mod vals;
