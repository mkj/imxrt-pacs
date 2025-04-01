#[doc = "FLEXIO"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexio {
    ptr: *mut u8,
}
unsafe impl Send for Flexio {}
unsafe impl Sync for Flexio {}
impl Flexio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Parameter Register"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "FlexIO Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Pin State Register"]
    #[inline(always)]
    pub const fn pin(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Shifter Status Register"]
    #[inline(always)]
    pub const fn shiftstat(self) -> crate::common::Reg<regs::Shiftstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Shifter Error Register"]
    #[inline(always)]
    pub const fn shifterr(self) -> crate::common::Reg<regs::Shifterr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Timer Status Register"]
    #[inline(always)]
    pub const fn timstat(self) -> crate::common::Reg<regs::Timstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn shiftsien(self) -> crate::common::Reg<regs::Shiftsien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn shifteien(self) -> crate::common::Reg<regs::Shifteien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Timer Interrupt Enable Register"]
    #[inline(always)]
    pub const fn timien(self) -> crate::common::Reg<regs::Timien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn shiftsden(self) -> crate::common::Reg<regs::Shiftsden, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Shifter State Register"]
    #[inline(always)]
    pub const fn shiftstate(self) -> crate::common::Reg<regs::Shiftstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Shifter Control N Register"]
    #[inline(always)]
    pub const fn shiftctl(self, n: usize) -> crate::common::Reg<regs::Shiftctl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Configuration N Register"]
    #[inline(always)]
    pub const fn shiftcfg(self, n: usize) -> crate::common::Reg<regs::Shiftcfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Register"]
    #[inline(always)]
    pub const fn shiftbuf(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Bit Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufbis(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Byte Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufbys(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Bit Byte Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufbbs(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(896usize + n * 4usize) as _) }
    }
    #[doc = "Timer Control N Register"]
    #[inline(always)]
    pub const fn timctl(self, n: usize) -> crate::common::Reg<regs::Timctl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize + n * 4usize) as _) }
    }
    #[doc = "Timer Configuration N Register"]
    #[inline(always)]
    pub const fn timcfg(self, n: usize) -> crate::common::Reg<regs::Timcfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1152usize + n * 4usize) as _) }
    }
    #[doc = "Timer Compare N Register"]
    #[inline(always)]
    pub const fn timcmp(self, n: usize) -> crate::common::Reg<regs::Timcmp, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufnbs(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1664usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Half Word Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufhws(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1792usize + n * 4usize) as _) }
    }
    #[doc = "Shifter Buffer N Nibble Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufnis(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1920usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
