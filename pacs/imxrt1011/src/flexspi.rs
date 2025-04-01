#[doc = "FlexSPI"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexspi {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi {}
unsafe impl Sync for Flexspi {}
impl Flexspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Control Register 0"]
    #[inline(always)]
    pub const fn mcr0(self) -> crate::common::Reg<regs::Mcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Module Control Register 1"]
    #[inline(always)]
    pub const fn mcr1(self) -> crate::common::Reg<regs::Mcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Module Control Register 2"]
    #[inline(always)]
    pub const fn mcr2(self) -> crate::common::Reg<regs::Mcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "AHB Bus Control Register"]
    #[inline(always)]
    pub const fn ahbcr(self) -> crate::common::Reg<regs::Ahbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "LUT Key Register"]
    #[inline(always)]
    pub const fn lutkey(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "LUT Control Register"]
    #[inline(always)]
    pub const fn lutcr(self) -> crate::common::Reg<regs::Lutcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "AHB RX Buffer 0 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbufcr0(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AhbrxbufCr0, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flsha1cr0(self) -> crate::common::Reg<regs::Flsha1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flsha2cr0(self) -> crate::common::Reg<regs::Flsha2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flshb1cr0(self) -> crate::common::Reg<regs::Flshb1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flshb2cr0(self) -> crate::common::Reg<regs::Flshb2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Flash Control Register 1"]
    #[inline(always)]
    pub const fn flshcr1(self, n: usize) -> crate::common::Reg<regs::Flshcr1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control Register 2"]
    #[inline(always)]
    pub const fn flshcr2(self, n: usize) -> crate::common::Reg<regs::Flshcr2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control Register 4"]
    #[inline(always)]
    pub const fn flshcr4(self) -> crate::common::Reg<regs::Flshcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "IP Control Register 0"]
    #[inline(always)]
    pub const fn ipcr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "IP Control Register 1"]
    #[inline(always)]
    pub const fn ipcr1(self) -> crate::common::Reg<regs::Ipcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "IP Command Register"]
    #[inline(always)]
    pub const fn ipcmd(self) -> crate::common::Reg<regs::Ipcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "IP RX FIFO Control Register"]
    #[inline(always)]
    pub const fn iprxfcr(self) -> crate::common::Reg<regs::Iprxfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "IP TX FIFO Control Register"]
    #[inline(always)]
    pub const fn iptxfcr(self) -> crate::common::Reg<regs::Iptxfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "DLL Control Register 0"]
    #[inline(always)]
    pub const fn dllcr(self, n: usize) -> crate::common::Reg<regs::Dllcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize + n * 4usize) as _) }
    }
    #[doc = "Status Register 0"]
    #[inline(always)]
    pub const fn sts0(self) -> crate::common::Reg<regs::Sts0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Status Register 1"]
    #[inline(always)]
    pub const fn sts1(self) -> crate::common::Reg<regs::Sts1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "Status Register 2"]
    #[inline(always)]
    pub const fn sts2(self) -> crate::common::Reg<regs::Sts2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
    #[doc = "AHB Suspend Status Register"]
    #[inline(always)]
    pub const fn ahbspndsts(self) -> crate::common::Reg<regs::Ahbspndsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(236usize) as _) }
    }
    #[doc = "IP RX FIFO Status Register"]
    #[inline(always)]
    pub const fn iprxfsts(self) -> crate::common::Reg<regs::Iprxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "IP TX FIFO Status Register"]
    #[inline(always)]
    pub const fn iptxfsts(self) -> crate::common::Reg<regs::Iptxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "IP RX FIFO Data Register x"]
    #[inline(always)]
    pub const fn rfdr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "IP TX FIFO Data Register x"]
    #[inline(always)]
    pub const fn tfdr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize + n * 4usize) as _) }
    }
    #[doc = "LUT x"]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> crate::common::Reg<regs::Lut, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
