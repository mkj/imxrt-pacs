#[doc = "USBPHY Register Reference Index"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usbphy {
    ptr: *mut u8,
}
unsafe impl Send for Usbphy {}
unsafe impl Sync for Usbphy {}
impl Usbphy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd(self) -> crate::common::Reg<regs::Pwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_set(self) -> crate::common::Reg<regs::PwdSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_clr(self) -> crate::common::Reg<regs::PwdClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_tog(self) -> crate::common::Reg<regs::PwdTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx(self) -> crate::common::Reg<regs::Tx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_set(self) -> crate::common::Reg<regs::TxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_clr(self) -> crate::common::Reg<regs::TxClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_tog(self) -> crate::common::Reg<regs::TxTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx(self) -> crate::common::Reg<regs::Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_set(self) -> crate::common::Reg<regs::RxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_clr(self) -> crate::common::Reg<regs::RxClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_tog(self) -> crate::common::Reg<regs::RxTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "USB PHY Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug(self) -> crate::common::Reg<regs::Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug_set(self) -> crate::common::Reg<regs::DebugSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug_clr(self) -> crate::common::Reg<regs::DebugClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug_tog(self) -> crate::common::Reg<regs::DebugTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 0"]
    #[inline(always)]
    pub const fn debug0_status(self) -> crate::common::Reg<regs::Debug0status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1(self) -> crate::common::Reg<regs::Debug1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_set(self) -> crate::common::Reg<regs::Debug1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_clr(self) -> crate::common::Reg<regs::Debug1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_tog(self) -> crate::common::Reg<regs::Debug1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "UTMI RTL Version"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
}
pub mod regs;
