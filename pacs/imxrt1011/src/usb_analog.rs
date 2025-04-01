#[doc = "USB Analog"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UsbAnalog {
    ptr: *mut u8,
}
unsafe impl Send for UsbAnalog {}
unsafe impl Sync for UsbAnalog {}
impl UsbAnalog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1vbusDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(416usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1vbusDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(420usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1vbusDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(424usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1vbusDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(428usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1chrgDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1chrgDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(436usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1chrgDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(440usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1chrgDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(444usize) as _) }
    }
    #[doc = "USB VBUS Detect Status Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_stat(
        self,
    ) -> crate::common::Reg<regs::Usb1vbusDetectStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(448usize) as _) }
    }
    #[doc = "USB Charger Detect Status Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_stat(
        self,
    ) -> crate::common::Reg<regs::Usb1chrgDetectStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(464usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback(self) -> crate::common::Reg<regs::Usb1loopback, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(480usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback_set(
        self,
    ) -> crate::common::Reg<regs::Usb1loopbackSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(484usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1loopbackClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(488usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1loopbackTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(492usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc(self) -> crate::common::Reg<regs::Usb1misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc_set(self) -> crate::common::Reg<regs::Usb1miscSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(500usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc_clr(self) -> crate::common::Reg<regs::Usb1miscClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(504usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc_tog(self) -> crate::common::Reg<regs::Usb1miscTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(508usize) as _) }
    }
    #[doc = "Chip Silicon Version"]
    #[inline(always)]
    pub const fn digprog(self) -> crate::common::Reg<regs::Digprog, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(608usize) as _) }
    }
}
pub mod regs;
pub mod vals;
