#[doc = "DCP register reference index"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dcp {
    ptr: *mut u8,
}
unsafe impl Send for Dcp {}
unsafe impl Sync for Dcp {}
impl Dcp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DCP control register 0"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat_set(self) -> crate::common::Reg<regs::StatSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat_clr(self) -> crate::common::Reg<regs::StatClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "DCP status register"]
    #[inline(always)]
    pub const fn stat_tog(self) -> crate::common::Reg<regs::StatTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl(self) -> crate::common::Reg<regs::Channelctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl_set(
        self,
    ) -> crate::common::Reg<regs::ChannelctrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl_clr(
        self,
    ) -> crate::common::Reg<regs::ChannelctrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "DCP channel control register"]
    #[inline(always)]
    pub const fn channelctrl_tog(
        self,
    ) -> crate::common::Reg<regs::ChannelctrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "DCP capability 0 register"]
    #[inline(always)]
    pub const fn capability0(self) -> crate::common::Reg<regs::Capability0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "DCP capability 1 register"]
    #[inline(always)]
    pub const fn capability1(self) -> crate::common::Reg<regs::Capability1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "DCP context buffer pointer"]
    #[inline(always)]
    pub const fn context(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "DCP key index"]
    #[inline(always)]
    pub const fn key(self) -> crate::common::Reg<regs::Key, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "DCP key data"]
    #[inline(always)]
    pub const fn keydata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "DCP work packet 0 status register"]
    #[inline(always)]
    pub const fn packet0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "DCP work packet 1 status register"]
    #[inline(always)]
    pub const fn packet1(self) -> crate::common::Reg<regs::Packet1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "DCP work packet 2 status register"]
    #[inline(always)]
    pub const fn packet2(self) -> crate::common::Reg<regs::Packet2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "DCP work packet 3 status register"]
    #[inline(always)]
    pub const fn packet3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "DCP work packet 4 status register"]
    #[inline(always)]
    pub const fn packet4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "DCP work packet 5 status register"]
    #[inline(always)]
    pub const fn packet5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "DCP work packet 6 status register"]
    #[inline(always)]
    pub const fn packet6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "DCP channel 0 command pointer address register"]
    #[inline(always)]
    pub const fn ch0cmdptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "DCP channel 0 semaphore register"]
    #[inline(always)]
    pub const fn ch0sema(self) -> crate::common::Reg<regs::Ch0sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat(self) -> crate::common::Reg<regs::Ch0stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat_set(self) -> crate::common::Reg<regs::Ch0statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat_clr(self) -> crate::common::Reg<regs::Ch0statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[doc = "DCP channel 0 status register"]
    #[inline(always)]
    pub const fn ch0stat_tog(self) -> crate::common::Reg<regs::Ch0statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts(self) -> crate::common::Reg<regs::Ch0opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts_set(self) -> crate::common::Reg<regs::Ch0optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts_clr(self) -> crate::common::Reg<regs::Ch0optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "DCP channel 0 options register"]
    #[inline(always)]
    pub const fn ch0opts_tog(self) -> crate::common::Reg<regs::Ch0optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "DCP channel 1 command pointer address register"]
    #[inline(always)]
    pub const fn ch1cmdptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "DCP channel 1 semaphore register"]
    #[inline(always)]
    pub const fn ch1sema(self) -> crate::common::Reg<regs::Ch1sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat(self) -> crate::common::Reg<regs::Ch1stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat_set(self) -> crate::common::Reg<regs::Ch1statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat_clr(self) -> crate::common::Reg<regs::Ch1statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "DCP channel 1 status register"]
    #[inline(always)]
    pub const fn ch1stat_tog(self) -> crate::common::Reg<regs::Ch1statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts(self) -> crate::common::Reg<regs::Ch1opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts_set(self) -> crate::common::Reg<regs::Ch1optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts_clr(self) -> crate::common::Reg<regs::Ch1optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "DCP channel 1 options register"]
    #[inline(always)]
    pub const fn ch1opts_tog(self) -> crate::common::Reg<regs::Ch1optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
    #[doc = "DCP channel 2 command pointer address register"]
    #[inline(always)]
    pub const fn ch2cmdptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "DCP channel 2 semaphore register"]
    #[inline(always)]
    pub const fn ch2sema(self) -> crate::common::Reg<regs::Ch2sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(400usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat(self) -> crate::common::Reg<regs::Ch2stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(416usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat_set(self) -> crate::common::Reg<regs::Ch2statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(420usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat_clr(self) -> crate::common::Reg<regs::Ch2statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(424usize) as _) }
    }
    #[doc = "DCP channel 2 status register"]
    #[inline(always)]
    pub const fn ch2stat_tog(self) -> crate::common::Reg<regs::Ch2statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(428usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts(self) -> crate::common::Reg<regs::Ch2opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts_set(self) -> crate::common::Reg<regs::Ch2optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(436usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts_clr(self) -> crate::common::Reg<regs::Ch2optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(440usize) as _) }
    }
    #[doc = "DCP channel 2 options register"]
    #[inline(always)]
    pub const fn ch2opts_tog(self) -> crate::common::Reg<regs::Ch2optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(444usize) as _) }
    }
    #[doc = "DCP channel 3 command pointer address register"]
    #[inline(always)]
    pub const fn ch3cmdptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(448usize) as _) }
    }
    #[doc = "DCP channel 3 semaphore register"]
    #[inline(always)]
    pub const fn ch3sema(self) -> crate::common::Reg<regs::Ch3sema, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(464usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat(self) -> crate::common::Reg<regs::Ch3stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(480usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat_set(self) -> crate::common::Reg<regs::Ch3statSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(484usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat_clr(self) -> crate::common::Reg<regs::Ch3statClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(488usize) as _) }
    }
    #[doc = "DCP channel 3 status register"]
    #[inline(always)]
    pub const fn ch3stat_tog(self) -> crate::common::Reg<regs::Ch3statTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(492usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts(self) -> crate::common::Reg<regs::Ch3opts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts_set(self) -> crate::common::Reg<regs::Ch3optsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(500usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts_clr(self) -> crate::common::Reg<regs::Ch3optsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(504usize) as _) }
    }
    #[doc = "DCP channel 3 options register"]
    #[inline(always)]
    pub const fn ch3opts_tog(self) -> crate::common::Reg<regs::Ch3optsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(508usize) as _) }
    }
    #[doc = "DCP debug select register"]
    #[inline(always)]
    pub const fn dbgselect(self) -> crate::common::Reg<regs::Dbgselect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "DCP debug data register"]
    #[inline(always)]
    pub const fn dbgdata(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1040usize) as _) }
    }
    #[doc = "DCP page table register"]
    #[inline(always)]
    pub const fn pagetable(self) -> crate::common::Reg<regs::Pagetable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1056usize) as _) }
    }
    #[doc = "DCP version register"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1072usize) as _) }
    }
}
pub mod regs;
pub mod vals;
