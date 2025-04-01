#[doc = "CCM_ANALOG"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CcmAnalog {
    ptr: *mut u8,
}
unsafe impl Send for CcmAnalog {}
unsafe impl Sync for CcmAnalog {}
impl CcmAnalog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1(self) -> crate::common::Reg<regs::PllUsb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1_set(self) -> crate::common::Reg<regs::PllUsb1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1_clr(self) -> crate::common::Reg<regs::PllUsb1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    #[inline(always)]
    pub const fn pll_usb1_tog(self) -> crate::common::Reg<regs::PllUsb1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys(self) -> crate::common::Reg<regs::PllSys, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys_set(self) -> crate::common::Reg<regs::PllSysSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys_clr(self) -> crate::common::Reg<regs::PllSysClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Analog System PLL Control Register"]
    #[inline(always)]
    pub const fn pll_sys_tog(self) -> crate::common::Reg<regs::PllSysTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "528MHz System PLL Spread Spectrum Register"]
    #[inline(always)]
    pub const fn pll_sys_ss(self) -> crate::common::Reg<regs::PllSysSs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_sys_num(self) -> crate::common::Reg<regs::PllSysNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_sys_denom(self) -> crate::common::Reg<regs::PllSysDenom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio(self) -> crate::common::Reg<regs::PllAudio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio_set(self) -> crate::common::Reg<regs::PllAudioSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio_clr(self) -> crate::common::Reg<regs::PllAudioClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "Analog Audio PLL control Register"]
    #[inline(always)]
    pub const fn pll_audio_tog(self) -> crate::common::Reg<regs::PllAudioTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_audio_num(self) -> crate::common::Reg<regs::PllAudioNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
    #[inline(always)]
    pub const fn pll_audio_denom(
        self,
    ) -> crate::common::Reg<regs::PllAudioDenom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet(self) -> crate::common::Reg<regs::PllEnet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet_set(self) -> crate::common::Reg<regs::PllEnetSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet_clr(self) -> crate::common::Reg<regs::PllEnetClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
    #[doc = "Analog ENET PLL Control Register"]
    #[inline(always)]
    pub const fn pll_enet_tog(self) -> crate::common::Reg<regs::PllEnetTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(236usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480(self) -> crate::common::Reg<regs::Pfd480, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480_set(self) -> crate::common::Reg<regs::Pfd480set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480_clr(self) -> crate::common::Reg<regs::Pfd480clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_480_tog(self) -> crate::common::Reg<regs::Pfd480tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528(self) -> crate::common::Reg<regs::Pfd528, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528_set(self) -> crate::common::Reg<regs::Pfd528set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528_clr(self) -> crate::common::Reg<regs::Pfd528clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    #[inline(always)]
    pub const fn pfd_528_tog(self) -> crate::common::Reg<regs::Pfd528tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0(self) -> crate::common::Reg<regs::Misc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_set(self) -> crate::common::Reg<regs::Misc0set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_clr(self) -> crate::common::Reg<regs::Misc0clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_tog(self) -> crate::common::Reg<regs::Misc0tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1(self) -> crate::common::Reg<regs::Misc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_set(self) -> crate::common::Reg<regs::Misc1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_clr(self) -> crate::common::Reg<regs::Misc1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_tog(self) -> crate::common::Reg<regs::Misc1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2(self) -> crate::common::Reg<regs::Misc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2_set(self) -> crate::common::Reg<regs::Misc2set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2_clr(self) -> crate::common::Reg<regs::Misc2clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "Miscellaneous Register 2"]
    #[inline(always)]
    pub const fn misc2_tog(self) -> crate::common::Reg<regs::Misc2tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
}
pub mod regs;
pub mod vals;
