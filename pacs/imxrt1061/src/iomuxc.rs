#[doc = "IOMUXC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iomuxc {
    ptr: *mut u8,
}
unsafe impl Send for Iomuxc {}
unsafe impl Sync for Iomuxc {}
impl Iomuxc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_16(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_17(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_18(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_19(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_20(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_21(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_22(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_23(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_24(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_25(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_26(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_27(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_28(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_29(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_30(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_31(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_32(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_33(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_34(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_35(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_36(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_37(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_38(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_39(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_40(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_41(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmc41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB006, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB007, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB008, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB009, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b0_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_b1_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAdB115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB006, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB007, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB008, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB009, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b0_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_b1_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_14(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_15(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_16(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_17(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_18(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_19(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_20(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_21(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_22(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x025cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_23(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_24(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_25(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_26(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_27(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_28(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0274usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_29(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_30(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x027cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_31(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_32(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_33(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_34(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_35(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_36(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_37(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_38(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x029cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_39(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_40(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_41(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmc41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02acusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB006, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB007, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB008, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB009, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_14(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b0_15(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02fcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_14(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_b1_15(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAdB115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_00(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_01(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_02(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_03(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_04(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_05(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_06(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_07(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_08(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_09(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0350usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_10(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0354usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_11(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0358usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_12(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_13(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0360usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_14(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0364usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b0_15(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0368usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_00(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_01(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_02(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0374usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_03(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_04(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_05(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_06(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_07(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_08(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_09(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_10(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_11(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0398usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_12(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_13(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_14(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_15(
        self,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03acusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
    #[doc = "ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn anatop_usb_otg1_id_select_input(
        self,
    ) -> crate::common::Reg<regs::AnatopUsbOtg1idSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[doc = "ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn anatop_usb_otg2_id_select_input(
        self,
    ) -> crate::common::Reg<regs::AnatopUsbOtg2idSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f8usize) as _) }
    }
    #[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ccm_pmic_ready_select_input(
        self,
    ) -> crate::common::Reg<regs::CcmPmicReadySelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
    #[doc = "CSI_DATA02_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data02_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData02selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "CSI_DATA03_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data03_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData03selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "CSI_DATA04_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data04_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData04selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "CSI_DATA05_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data05_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData05selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "CSI_DATA06_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data06_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData06selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "CSI_DATA07_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data07_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData07selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "CSI_DATA08_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data08_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData08selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "CSI_DATA09_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_data09_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiData09selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "CSI_HSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_hsync_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiHsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "CSI_PIXCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_pixclk_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiPixclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "CSI_VSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn csi_vsync_select_input(
        self,
    ) -> crate::common::Reg<regs::CsiVsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[doc = "ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_ipg_clk_rmii_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetIpgClkRmiiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
    #[doc = "ENET_MDIO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_mdio_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetMdioSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[doc = "ENET0_RXDATA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet0_rxdata_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet0rxdataSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0434usize) as _) }
    }
    #[doc = "ENET1_RXDATA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet1_rxdata_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet1rxdataSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[doc = "ENET_RXEN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_rxen_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetRxenSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[doc = "ENET_RXERR_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_rxerr_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetRxerrSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "ENET0_TIMER_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet0_timer_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet0timerSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[doc = "ENET_TXCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet_txclk_select_input(
        self,
    ) -> crate::common::Reg<regs::EnetTxclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0448usize) as _) }
    }
    #[doc = "FLEXCAN1_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexcan1_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexcan1rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[doc = "FLEXCAN2_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexcan2_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexcan2rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwma3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwma0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwma1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x045cusize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwma2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmb3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmb0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmb1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x046cusize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmb2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwma3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0474usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwma0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0478usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwma1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x047cusize) as _) }
    }
    #[doc = "FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwma2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwma2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwmb3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0484usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwmb0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0488usize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwmb1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x048cusize) as _) }
    }
    #[doc = "FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_pwmb2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2pwmb2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0490usize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4pwma0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0494usize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4pwma1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0498usize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4pwma2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x049cusize) as _) }
    }
    #[doc = "FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm4_pwma3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexpwm4pwma3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[doc = "FLEXSPIA_DQS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_dqs_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaDqsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a4usize) as _) }
    }
    #[doc = "FLEXSPIA_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a8usize) as _) }
    }
    #[doc = "FLEXSPIA_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04acusize) as _) }
    }
    #[doc = "FLEXSPIA_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b0usize) as _) }
    }
    #[doc = "FLEXSPIA_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaData3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b4usize) as _) }
    }
    #[doc = "FLEXSPIB_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b8usize) as _) }
    }
    #[doc = "FLEXSPIB_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04bcusize) as _) }
    }
    #[doc = "FLEXSPIB_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize) as _) }
    }
    #[doc = "FLEXSPIB_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspib_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspibData3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
    }
    #[doc = "FLEXSPIA_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspia_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiaSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c8usize) as _) }
    }
    #[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1sclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04ccusize) as _) }
    }
    #[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1sdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
    }
    #[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2sclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d4usize) as _) }
    }
    #[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2sdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d8usize) as _) }
    }
    #[doc = "LPI2C3_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c3_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c3sclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04dcusize) as _) }
    }
    #[doc = "LPI2C3_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c3_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c3sdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[doc = "LPI2C4_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c4_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c4sclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e4usize) as _) }
    }
    #[doc = "LPI2C4_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c4_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c4sdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e8usize) as _) }
    }
    #[doc = "LPSPI1_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1pcs0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04ecusize) as _) }
    }
    #[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1sckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f0usize) as _) }
    }
    #[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1sdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f4usize) as _) }
    }
    #[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1sdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f8usize) as _) }
    }
    #[doc = "LPSPI2_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2pcs0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04fcusize) as _) }
    }
    #[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2sckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2sdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2sdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "LPSPI3_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3pcs0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "LPSPI3_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3sckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "LPSPI3_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3sdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "LPSPI3_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3sdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "LPSPI4_PCS0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_pcs0_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4pcs0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "LPSPI4_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4sckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "LPSPI4_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4sdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "LPSPI4_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4sdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "LPUART2_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "LPUART2_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "LPUART3_CTS_B_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_cts_b_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3ctsBselectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[doc = "LPUART3_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "LPUART3_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
    }
    #[doc = "LPUART4_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "LPUART4_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "LPUART5_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "LPUART5_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[doc = "LPUART6_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "LPUART6_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[doc = "LPUART7_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
    }
    #[doc = "LPUART7_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
    }
    #[doc = "LPUART8_RX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_rx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8rxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "LPUART8_TX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_tx_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8txSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
    }
    #[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn nmi_select_input(
        self,
    ) -> crate::common::Reg<regs::NmiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
    }
    #[doc = "QTIMER2_TIMER0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer0_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2timer0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
    }
    #[doc = "QTIMER2_TIMER1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer1_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2timer1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[doc = "QTIMER2_TIMER2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer2_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2timer2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0574usize) as _) }
    }
    #[doc = "QTIMER2_TIMER3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_timer3_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2timer3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
    }
    #[doc = "QTIMER3_TIMER0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer0_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3timer0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x057cusize) as _) }
    }
    #[doc = "QTIMER3_TIMER1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer1_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3timer1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "QTIMER3_TIMER2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer2_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3timer2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
    }
    #[doc = "QTIMER3_TIMER3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_timer3_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3timer3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
    #[doc = "SAI1_MCLK2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_mclk2_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1mclk2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x058cusize) as _) }
    }
    #[doc = "SAI1_RX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1rxBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[doc = "SAI1_RX_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1rxData0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
    }
    #[doc = "SAI1_RX_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1rxData1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0598usize) as _) }
    }
    #[doc = "SAI1_RX_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1rxData2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x059cusize) as _) }
    }
    #[doc = "SAI1_RX_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1rxData3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[doc = "SAI1_RX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_rx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1rxSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
    }
    #[doc = "SAI1_TX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_tx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1txBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a8usize) as _) }
    }
    #[doc = "SAI1_TX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_tx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1txSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05acusize) as _) }
    }
    #[doc = "SAI2_MCLK2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_mclk2_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2mclk2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b0usize) as _) }
    }
    #[doc = "SAI2_RX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_rx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2rxBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b4usize) as _) }
    }
    #[doc = "SAI2_RX_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_rx_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2rxData0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b8usize) as _) }
    }
    #[doc = "SAI2_RX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_rx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2rxSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05bcusize) as _) }
    }
    #[doc = "SAI2_TX_BCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_tx_bclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2txBclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[doc = "SAI2_TX_SYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai2_tx_sync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai2txSyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c4usize) as _) }
    }
    #[doc = "SPDIF_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_in_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c8usize) as _) }
    }
    #[doc = "USB_OTG2_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg2_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtg2ocSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ccusize) as _) }
    }
    #[doc = "USB_OTG1_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg1_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtg1ocSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d0usize) as _) }
    }
    #[doc = "USDHC1_CD_B_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc1_cd_b_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc1cdBselectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d4usize) as _) }
    }
    #[doc = "USDHC1_WP_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc1_wp_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc1wpSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d8usize) as _) }
    }
    #[doc = "USDHC2_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2clkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05dcusize) as _) }
    }
    #[doc = "USDHC2_CD_B_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_cd_b_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2cdBselectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[doc = "USDHC2_CMD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_cmd_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2cmdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e4usize) as _) }
    }
    #[doc = "USDHC2_DATA0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data0_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e8usize) as _) }
    }
    #[doc = "USDHC2_DATA1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data1_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ecusize) as _) }
    }
    #[doc = "USDHC2_DATA2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data2_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[doc = "USDHC2_DATA3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data3_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
    #[doc = "USDHC2_DATA4_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data4_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data4selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
    }
    #[doc = "USDHC2_DATA5_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data5_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data5selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05fcusize) as _) }
    }
    #[doc = "USDHC2_DATA6_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data6_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data6selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "USDHC2_DATA7_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_data7_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2data7selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "USDHC2_WP_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_wp_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2wpSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "XBAR1_IN02_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in02_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in02selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[doc = "XBAR1_IN03_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in03_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in03selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[doc = "XBAR1_IN04_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in04_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in04selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[doc = "XBAR1_IN05_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in05_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in05selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[doc = "XBAR1_IN06_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in06_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in06selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x061cusize) as _) }
    }
    #[doc = "XBAR1_IN07_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in07_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in07selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[doc = "XBAR1_IN08_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in08_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in08selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
    #[doc = "XBAR1_IN09_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in09_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in09selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0628usize) as _) }
    }
    #[doc = "XBAR1_IN17_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in17_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in17selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x062cusize) as _) }
    }
    #[doc = "XBAR1_IN18_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in18_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in18selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0630usize) as _) }
    }
    #[doc = "XBAR1_IN20_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in20_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in20selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0634usize) as _) }
    }
    #[doc = "XBAR1_IN22_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in22_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in22selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0638usize) as _) }
    }
    #[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in23_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in23selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x063cusize) as _) }
    }
    #[doc = "XBAR1_IN24_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in24_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in24selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "XBAR1_IN14_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in14_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in14selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0644usize) as _) }
    }
    #[doc = "XBAR1_IN15_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in15_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in15selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0648usize) as _) }
    }
    #[doc = "XBAR1_IN16_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in16_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in16selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x064cusize) as _) }
    }
    #[doc = "XBAR1_IN25_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in25_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in25selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
    }
    #[doc = "XBAR1_IN19_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in19_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in19selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0654usize) as _) }
    }
    #[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_in21_select_input(
        self,
    ) -> crate::common::Reg<regs::Xbar1in21selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0658usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x065cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0664usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0668usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x066cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0670usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB006, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0674usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB007, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0678usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB008, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x067cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB009, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0684usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0688usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x068cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b0_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0690usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0694usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0698usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x069cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06acusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_spi_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSpiB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB002, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB003, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB004, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB005, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB006, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB007, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB008, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB009, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b0_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06fcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_spi_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSpiB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0708usize) as _) }
    }
    #[doc = "ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipg_clk_rmii_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2ipgClkRmiiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x070cusize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_mdio_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0mdioSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxdata_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0rxdataSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0714usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxdata_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0rxdataSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0718usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxen_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0rxenSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x071cusize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_rxerr_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0rxerrSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0720usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_timer_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0timerSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0724usize) as _) }
    }
    #[doc = "ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn enet2_ipp_ind_mac0_txclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Enet2ippIndMac0txclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0728usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_dqs_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndDqsFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x072cusize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fa_bit0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFaBit0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0730usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fa_bit1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFaBit1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0734usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fa_bit2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFaBit2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0738usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fa_bit3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFaBit3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x073cusize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fb_bit0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFbBit0selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0740usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fb_bit1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFbBit1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0744usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fb_bit2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFbBit2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0748usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_io_fb_bit3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndIoFbBit3selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x074cusize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_sck_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndSckFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0750usize) as _) }
    }
    #[doc = "FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_ipp_ind_sck_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2ippIndSckFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0754usize) as _) }
    }
    #[doc = "GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt1_ipp_ind_capin1_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt1ippIndCapin1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0758usize) as _) }
    }
    #[doc = "GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt1_ipp_ind_capin2_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt1ippIndCapin2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x075cusize) as _) }
    }
    #[doc = "GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt1_ipp_ind_clkin_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt1ippIndClkinSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0760usize) as _) }
    }
    #[doc = "GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt2_ipp_ind_capin1_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt2ippIndCapin1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0764usize) as _) }
    }
    #[doc = "GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt2_ipp_ind_capin2_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt2ippIndCapin2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0768usize) as _) }
    }
    #[doc = "GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn gpt2_ipp_ind_clkin_select_input(
        self,
    ) -> crate::common::Reg<regs::Gpt2ippIndClkinSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x076cusize) as _) }
    }
    #[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipg_clk_sai_mclk_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Sai3ipgClkSaiMclkSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0770usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_rxbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3ippIndSaiRxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0774usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_rxdata_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sai3ippIndSaiRxdataSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0778usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_rxsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3ippIndSaiRxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x077cusize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_txbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3ippIndSaiTxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize) as _) }
    }
    #[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai3_ipp_ind_sai_txsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai3ippIndSaiTxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0784usize) as _) }
    }
    #[doc = "SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn semc_i_ipp_ind_dqs4_select_input(
        self,
    ) -> crate::common::Reg<regs::SemcIippIndDqs4selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0788usize) as _) }
    }
    #[doc = "CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn canfd_ipp_ind_canrx_select_input(
        self,
    ) -> crate::common::Reg<regs::CanfdIppIndCanrxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x078cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
