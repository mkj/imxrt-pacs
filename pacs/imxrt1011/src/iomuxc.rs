#[doc = "IOMUXC"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_14(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_13(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_12(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_11(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_10(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_09(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_08(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_07(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_06(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_05(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_04(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_03(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_02(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_01(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_ad_00(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlAd00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_14(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_13(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_12(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_11(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_10(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_09(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_08(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_07(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_06(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_05(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_04(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_03(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_02(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_01(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_sd_00(
        self,
    ) -> crate::common::Reg<regs::GpioMuxCtlSd00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_13(self) -> crate::common::Reg<regs::GpioMuxCtl13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_12(self) -> crate::common::Reg<regs::GpioMuxCtl12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_11(self) -> crate::common::Reg<regs::GpioMuxCtl11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_10(self) -> crate::common::Reg<regs::GpioMuxCtl10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_09(self) -> crate::common::Reg<regs::GpioMuxCtl09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_08(self) -> crate::common::Reg<regs::GpioMuxCtl08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_07(self) -> crate::common::Reg<regs::GpioMuxCtl07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_06(self) -> crate::common::Reg<regs::GpioMuxCtl06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_05(self) -> crate::common::Reg<regs::GpioMuxCtl05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_04(self) -> crate::common::Reg<regs::GpioMuxCtl04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_03(self) -> crate::common::Reg<regs::GpioMuxCtl03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_02(self) -> crate::common::Reg<regs::GpioMuxCtl02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_01(self) -> crate::common::Reg<regs::GpioMuxCtl01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn gpio_mux_00(self) -> crate::common::Reg<regs::GpioMuxCtl00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn gpio_ctl_ad_x(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize + n * 4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn gpio_ctl_sd_x(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize + n * 4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn gpio_ctl_x(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::GpioCtl, crate::common::RW> {
        assert!(n < 14usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize + n * 4usize) as _) }
    }
    #[doc = "USB_OTG_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg_id_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtgIdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmaSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmaSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmaSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwma_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmaSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmbSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmbSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmbSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(396usize) as _) }
    }
    #[doc = "FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_pwmb_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1pwmbSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(400usize) as _) }
    }
    #[doc = "FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi_dqs_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiDqsFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(404usize) as _) }
    }
    #[doc = "FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi_dqs_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::FlexspiDqsFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(408usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_0(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(412usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_1(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(416usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_2(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(420usize) as _) }
    }
    #[doc = "KPP_COL_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_col_select_input_3(
        self,
    ) -> crate::common::Reg<regs::KppColSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(424usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_0(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(428usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_1(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_2(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(436usize) as _) }
    }
    #[doc = "KPP_ROW_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_row_select_input_3(
        self,
    ) -> crate::common::Reg<regs::KppRowSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(440usize) as _) }
    }
    #[doc = "LPI2C1_HREQ_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_hreq_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1hreqSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(444usize) as _) }
    }
    #[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1sclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(448usize) as _) }
    }
    #[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1sdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(452usize) as _) }
    }
    #[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2sclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(456usize) as _) }
    }
    #[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2sdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(460usize) as _) }
    }
    #[doc = "LPSPI1_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi1pcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(464usize) as _) }
    }
    #[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1sckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(468usize) as _) }
    }
    #[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1sdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(472usize) as _) }
    }
    #[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1sdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(476usize) as _) }
    }
    #[doc = "LPSPI2_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi2pcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(480usize) as _) }
    }
    #[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2sckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(484usize) as _) }
    }
    #[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2sdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(488usize) as _) }
    }
    #[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2sdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(492usize) as _) }
    }
    #[doc = "LPUART1_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1rxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "LPUART1_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1txdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(500usize) as _) }
    }
    #[doc = "LPUART2_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2rxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(504usize) as _) }
    }
    #[doc = "LPUART2_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2txdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(508usize) as _) }
    }
    #[doc = "LPUART3_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3rxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "LPUART3_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3txdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "LPUART4_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4rxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "LPUART4_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4txdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn nmi_glue_nmi_select_input(
        self,
    ) -> crate::common::Reg<regs::NmiGlueNmiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[doc = "SPDIF_IN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_in1_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifIn1selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "SPDIF_TX_CLK2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_tx_clk2_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifTxClk2selectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize) as _) }
    }
    #[doc = "USB_OTG_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_otg_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbOtgOcSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[doc = "XEV_GLUE_RXEV_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xev_glue_rxev_select_input(
        self,
    ) -> crate::common::Reg<regs::XevGlueRxevSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
}
pub mod regs;
pub mod vals;
