#[doc = "SNVS"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Snvs {
    ptr: *mut u8,
}
unsafe impl Send for Snvs {}
unsafe impl Sync for Snvs {}
impl Snvs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SNVS_HP Lock Register"]
    #[inline(always)]
    pub const fn hplr(self) -> crate::common::Reg<regs::Hplr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "SNVS_HP Command Register"]
    #[inline(always)]
    pub const fn hpcomr(self) -> crate::common::Reg<regs::Hpcomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "SNVS_HP Control Register"]
    #[inline(always)]
    pub const fn hpcr(self) -> crate::common::Reg<regs::Hpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "SNVS_HP Security Interrupt Control Register"]
    #[inline(always)]
    pub const fn hpsicr(self) -> crate::common::Reg<regs::Hpsicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "SNVS_HP Security Violation Control Register"]
    #[inline(always)]
    pub const fn hpsvcr(self) -> crate::common::Reg<regs::Hpsvcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "SNVS_HP Status Register"]
    #[inline(always)]
    pub const fn hpsr(self) -> crate::common::Reg<regs::Hpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "SNVS_HP Security Violation Status Register"]
    #[inline(always)]
    pub const fn hpsvsr(self) -> crate::common::Reg<regs::Hpsvsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "SNVS_HP High Assurance Counter IV Register"]
    #[inline(always)]
    pub const fn hphacivr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "SNVS_HP High Assurance Counter Register"]
    #[inline(always)]
    pub const fn hphacr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "SNVS_HP Real Time Counter MSB Register"]
    #[inline(always)]
    pub const fn hprtcmr(self) -> crate::common::Reg<regs::Hprtcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "SNVS_HP Real Time Counter LSB Register"]
    #[inline(always)]
    pub const fn hprtclr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "SNVS_HP Time Alarm MSB Register"]
    #[inline(always)]
    pub const fn hptamr(self) -> crate::common::Reg<regs::Hptamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "SNVS_HP Time Alarm LSB Register"]
    #[inline(always)]
    pub const fn hptalr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "SNVS_LP Lock Register"]
    #[inline(always)]
    pub const fn lplr(self) -> crate::common::Reg<regs::Lplr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "SNVS_LP Control Register"]
    #[inline(always)]
    pub const fn lpcr(self) -> crate::common::Reg<regs::Lpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "SNVS_LP Master Key Control Register"]
    #[inline(always)]
    pub const fn lpmkcr(self) -> crate::common::Reg<regs::Lpmkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "SNVS_LP Security Violation Control Register"]
    #[inline(always)]
    pub const fn lpsvcr(self) -> crate::common::Reg<regs::Lpsvcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "SNVS_LP Tamper Glitch Filters Configuration Register"]
    #[inline(always)]
    pub const fn lptgfcr(self) -> crate::common::Reg<regs::Lptgfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "SNVS_LP Tamper Detect Configuration Register"]
    #[inline(always)]
    pub const fn lptdcr(self) -> crate::common::Reg<regs::Lptdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "SNVS_LP Status Register"]
    #[inline(always)]
    pub const fn lpsr(self) -> crate::common::Reg<regs::Lpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
    #[inline(always)]
    pub const fn lpsrtcmr(self) -> crate::common::Reg<regs::Lpsrtcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
    #[inline(always)]
    pub const fn lpsrtclr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "SNVS_LP Time Alarm Register"]
    #[inline(always)]
    pub const fn lptar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
    #[inline(always)]
    pub const fn lpsmcmr(self) -> crate::common::Reg<regs::Lpsmcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
    #[inline(always)]
    pub const fn lpsmclr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "SNVS_LP Digital Low-Voltage Detector Register"]
    #[inline(always)]
    pub const fn lplvdr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
    #[inline(always)]
    pub const fn lpgpr0_legacy_alias(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "SNVS_LP Zeroizable Master Key Register"]
    #[inline(always)]
    pub const fn lpzmkr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize + n * 4usize) as _) }
    }
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    #[inline(always)]
    pub const fn lpgpr_alias(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize + n * 4usize) as _) }
    }
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    #[inline(always)]
    pub const fn lpgpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "SNVS_HP Version ID Register 1"]
    #[inline(always)]
    pub const fn hpvidr1(self) -> crate::common::Reg<regs::Hpvidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3064usize) as _) }
    }
    #[doc = "SNVS_HP Version ID Register 2"]
    #[inline(always)]
    pub const fn hpvidr2(self) -> crate::common::Reg<regs::Hpvidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3068usize) as _) }
    }
}
pub mod regs;
pub mod vals;
