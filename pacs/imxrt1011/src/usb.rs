#[doc = "USB"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Identification register"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Hardware General"]
    #[inline(always)]
    pub const fn hwgeneral(self) -> crate::common::Reg<regs::Hwgeneral, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Host Hardware Parameters"]
    #[inline(always)]
    pub const fn hwhost(self) -> crate::common::Reg<regs::Hwhost, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Device Hardware Parameters"]
    #[inline(always)]
    pub const fn hwdevice(self) -> crate::common::Reg<regs::Hwdevice, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "TX Buffer Hardware Parameters"]
    #[inline(always)]
    pub const fn hwtxbuf(self) -> crate::common::Reg<regs::Hwtxbuf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "RX Buffer Hardware Parameters"]
    #[inline(always)]
    pub const fn hwrxbuf(self) -> crate::common::Reg<regs::Hwrxbuf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "General Purpose Timer #0 Load"]
    #[inline(always)]
    pub const fn gptimer0ld(self) -> crate::common::Reg<regs::Gptimer0ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "General Purpose Timer #0 Controller"]
    #[inline(always)]
    pub const fn gptimer0ctrl(self) -> crate::common::Reg<regs::Gptimer0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "General Purpose Timer #1 Load"]
    #[inline(always)]
    pub const fn gptimer1ld(self) -> crate::common::Reg<regs::Gptimer1ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "General Purpose Timer #1 Controller"]
    #[inline(always)]
    pub const fn gptimer1ctrl(self) -> crate::common::Reg<regs::Gptimer1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "System Bus Config"]
    #[inline(always)]
    pub const fn sbuscfg(self) -> crate::common::Reg<regs::Sbuscfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Capability Registers Length"]
    #[inline(always)]
    pub const fn caplength(self) -> crate::common::Reg<regs::Caplength, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Host Controller Interface Version"]
    #[inline(always)]
    pub const fn hciversion(self) -> crate::common::Reg<regs::Hciversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(258usize) as _) }
    }
    #[doc = "Host Controller Structural Parameters"]
    #[inline(always)]
    pub const fn hcsparams(self) -> crate::common::Reg<regs::Hcsparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "Host Controller Capability Parameters"]
    #[inline(always)]
    pub const fn hccparams(self) -> crate::common::Reg<regs::Hccparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Device Controller Interface Version"]
    #[inline(always)]
    pub const fn dciversion(self) -> crate::common::Reg<regs::Dciversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "Device Controller Capability Parameters"]
    #[inline(always)]
    pub const fn dccparams(self) -> crate::common::Reg<regs::Dccparams, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "USB Command Register"]
    #[inline(always)]
    pub const fn usbcmd(self) -> crate::common::Reg<regs::Usbcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "USB Status Register"]
    #[inline(always)]
    pub const fn usbsts(self) -> crate::common::Reg<regs::Usbsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn usbintr(self) -> crate::common::Reg<regs::Usbintr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(328usize) as _) }
    }
    #[doc = "USB Frame Index"]
    #[inline(always)]
    pub const fn frindex(self) -> crate::common::Reg<regs::Frindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize) as _) }
    }
    #[doc = "Device Address"]
    #[inline(always)]
    pub const fn deviceaddr(self) -> crate::common::Reg<regs::Deviceaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Frame List Base Address"]
    #[inline(always)]
    pub const fn periodiclistbase(
        self,
    ) -> crate::common::Reg<regs::Periodiclistbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Next Asynch. Address"]
    #[inline(always)]
    pub const fn asynclistaddr(self) -> crate::common::Reg<regs::Asynclistaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Endpoint List Address"]
    #[inline(always)]
    pub const fn endptlistaddr(self) -> crate::common::Reg<regs::Endptlistaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Programmable Burst Size"]
    #[inline(always)]
    pub const fn burstsize(self) -> crate::common::Reg<regs::Burstsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "TX FIFO Fill Tuning"]
    #[inline(always)]
    pub const fn txfilltuning(self) -> crate::common::Reg<regs::Txfilltuning, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "Endpoint NAK"]
    #[inline(always)]
    pub const fn endptnak(self) -> crate::common::Reg<regs::Endptnak, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "Endpoint NAK Enable"]
    #[inline(always)]
    pub const fn endptnaken(self) -> crate::common::Reg<regs::Endptnaken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
    #[doc = "Configure Flag Register"]
    #[inline(always)]
    pub const fn configflag(self) -> crate::common::Reg<regs::Configflag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Port Status & Control"]
    #[inline(always)]
    pub const fn portsc1(self) -> crate::common::Reg<regs::Portsc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "On-The-Go Status & control"]
    #[inline(always)]
    pub const fn otgsc(self) -> crate::common::Reg<regs::Otgsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(420usize) as _) }
    }
    #[doc = "USB Device Mode"]
    #[inline(always)]
    pub const fn usbmode(self) -> crate::common::Reg<regs::Usbmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(424usize) as _) }
    }
    #[doc = "Endpoint Setup Status"]
    #[inline(always)]
    pub const fn endptsetupstat(
        self,
    ) -> crate::common::Reg<regs::Endptsetupstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(428usize) as _) }
    }
    #[doc = "Endpoint Prime"]
    #[inline(always)]
    pub const fn endptprime(self) -> crate::common::Reg<regs::Endptprime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "Endpoint Flush"]
    #[inline(always)]
    pub const fn endptflush(self) -> crate::common::Reg<regs::Endptflush, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(436usize) as _) }
    }
    #[doc = "Endpoint Status"]
    #[inline(always)]
    pub const fn endptstat(self) -> crate::common::Reg<regs::Endptstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(440usize) as _) }
    }
    #[doc = "Endpoint Complete"]
    #[inline(always)]
    pub const fn endptcomplete(self) -> crate::common::Reg<regs::Endptcomplete, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(444usize) as _) }
    }
    #[doc = "Endpoint Control0"]
    #[inline(always)]
    pub const fn endptctrl_x(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EndptctrlX, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(448usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
