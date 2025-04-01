#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmaSelectInput0(pub u32);
impl Flexpwm1pwmaSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmaSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmaSelectInput0 {
        Flexpwm1pwmaSelectInput0(0)
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmaSelectInput1(pub u32);
impl Flexpwm1pwmaSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmaSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmaSelectInput1 {
        Flexpwm1pwmaSelectInput1(0)
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmaSelectInput2(pub u32);
impl Flexpwm1pwmaSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmaSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmaSelectInput2 {
        Flexpwm1pwmaSelectInput2(0)
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmaSelectInput3(pub u32);
impl Flexpwm1pwmaSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmaSelectInput3 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmaSelectInput3 {
        Flexpwm1pwmaSelectInput3(0)
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmbSelectInput0(pub u32);
impl Flexpwm1pwmbSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmbSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmbSelectInput0 {
        Flexpwm1pwmbSelectInput0(0)
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmbSelectInput1(pub u32);
impl Flexpwm1pwmbSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmbSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmbSelectInput1 {
        Flexpwm1pwmbSelectInput1(0)
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmbSelectInput2(pub u32);
impl Flexpwm1pwmbSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmbSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmbSelectInput2 {
        Flexpwm1pwmbSelectInput2(0)
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Flexpwm1pwmbSelectInput3(pub u32);
impl Flexpwm1pwmbSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1pwmbSelectInput3 {
    #[inline(always)]
    fn default() -> Flexpwm1pwmbSelectInput3 {
        Flexpwm1pwmbSelectInput3(0)
    }
}
#[doc = "FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FlexspiDqsFaSelectInput(pub u32);
impl FlexspiDqsFaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiDqsFaSelectInput {
    #[inline(always)]
    fn default() -> FlexspiDqsFaSelectInput {
        FlexspiDqsFaSelectInput(0)
    }
}
#[doc = "FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FlexspiDqsFbSelectInput(pub u32);
impl FlexspiDqsFbSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiDqsFbSelectInput {
    #[inline(always)]
    fn default() -> FlexspiDqsFbSelectInput {
        FlexspiDqsFbSelectInput(0)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioCtl(pub u32);
impl GpioCtl {
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::GpioDse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::GpioDse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::GpioDse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::GpioSpeed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::GpioSpeed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::GpioSpeed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn ode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn pke(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::GpioPus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::GpioPus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::GpioPus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn hys(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GpioCtl {
    #[inline(always)]
    fn default() -> GpioCtl {
        GpioCtl(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl00(pub u32);
impl GpioMuxCtl00 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode00 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode00::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode00) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl00 {
    #[inline(always)]
    fn default() -> GpioMuxCtl00 {
        GpioMuxCtl00(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl01(pub u32);
impl GpioMuxCtl01 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode01 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode01::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode01) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl01 {
    #[inline(always)]
    fn default() -> GpioMuxCtl01 {
        GpioMuxCtl01(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl02(pub u32);
impl GpioMuxCtl02 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode02 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode02::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode02) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl02 {
    #[inline(always)]
    fn default() -> GpioMuxCtl02 {
        GpioMuxCtl02(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl03(pub u32);
impl GpioMuxCtl03 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode03 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode03::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode03) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl03 {
    #[inline(always)]
    fn default() -> GpioMuxCtl03 {
        GpioMuxCtl03(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl04(pub u32);
impl GpioMuxCtl04 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode04 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode04::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode04) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl04 {
    #[inline(always)]
    fn default() -> GpioMuxCtl04 {
        GpioMuxCtl04(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl05(pub u32);
impl GpioMuxCtl05 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode05 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode05::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode05) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl05 {
    #[inline(always)]
    fn default() -> GpioMuxCtl05 {
        GpioMuxCtl05(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl06(pub u32);
impl GpioMuxCtl06 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode06 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode06::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode06) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl06 {
    #[inline(always)]
    fn default() -> GpioMuxCtl06 {
        GpioMuxCtl06(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl07(pub u32);
impl GpioMuxCtl07 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode07 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode07::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode07) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl07 {
    #[inline(always)]
    fn default() -> GpioMuxCtl07 {
        GpioMuxCtl07(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl08(pub u32);
impl GpioMuxCtl08 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode08 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode08::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode08) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl08 {
    #[inline(always)]
    fn default() -> GpioMuxCtl08 {
        GpioMuxCtl08(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl09(pub u32);
impl GpioMuxCtl09 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode09 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode09::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode09) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl09 {
    #[inline(always)]
    fn default() -> GpioMuxCtl09 {
        GpioMuxCtl09(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl10(pub u32);
impl GpioMuxCtl10 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode10 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode10::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode10) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl10 {
    #[inline(always)]
    fn default() -> GpioMuxCtl10 {
        GpioMuxCtl10(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl11(pub u32);
impl GpioMuxCtl11 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode11 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode11::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode11) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl11 {
    #[inline(always)]
    fn default() -> GpioMuxCtl11 {
        GpioMuxCtl11(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl12(pub u32);
impl GpioMuxCtl12 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode12 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode12::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode12) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl12 {
    #[inline(always)]
    fn default() -> GpioMuxCtl12 {
        GpioMuxCtl12(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtl13(pub u32);
impl GpioMuxCtl13 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxMode13 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxMode13::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxMode13) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtl13 {
    #[inline(always)]
    fn default() -> GpioMuxCtl13 {
        GpioMuxCtl13(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd00(pub u32);
impl GpioMuxCtlAd00 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd00 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd00::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd00) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd00 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd00 {
        GpioMuxCtlAd00(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd01(pub u32);
impl GpioMuxCtlAd01 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd01 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd01::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd01) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd01 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd01 {
        GpioMuxCtlAd01(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd02(pub u32);
impl GpioMuxCtlAd02 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd02 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd02::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd02) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd02 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd02 {
        GpioMuxCtlAd02(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd03(pub u32);
impl GpioMuxCtlAd03 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd03 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd03::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd03) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd03 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd03 {
        GpioMuxCtlAd03(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd04(pub u32);
impl GpioMuxCtlAd04 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd04 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd04::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd04) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd04 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd04 {
        GpioMuxCtlAd04(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd05(pub u32);
impl GpioMuxCtlAd05 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd05 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd05::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd05) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd05 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd05 {
        GpioMuxCtlAd05(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd06(pub u32);
impl GpioMuxCtlAd06 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd06 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd06::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd06) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd06 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd06 {
        GpioMuxCtlAd06(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd07(pub u32);
impl GpioMuxCtlAd07 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd07 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd07::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd07) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd07 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd07 {
        GpioMuxCtlAd07(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd08(pub u32);
impl GpioMuxCtlAd08 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd08 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd08::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd08) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd08 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd08 {
        GpioMuxCtlAd08(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd09(pub u32);
impl GpioMuxCtlAd09 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd09 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd09::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd09) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd09 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd09 {
        GpioMuxCtlAd09(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd10(pub u32);
impl GpioMuxCtlAd10 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd10 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd10::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd10) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd10 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd10 {
        GpioMuxCtlAd10(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd11(pub u32);
impl GpioMuxCtlAd11 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd11 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd11::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd11) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd11 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd11 {
        GpioMuxCtlAd11(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd12(pub u32);
impl GpioMuxCtlAd12 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd12 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd12::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd12) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd12 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd12 {
        GpioMuxCtlAd12(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd13(pub u32);
impl GpioMuxCtlAd13 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd13 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd13::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd13) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd13 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd13 {
        GpioMuxCtlAd13(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlAd14(pub u32);
impl GpioMuxCtlAd14 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeAd14 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeAd14::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeAd14) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlAd14 {
    #[inline(always)]
    fn default() -> GpioMuxCtlAd14 {
        GpioMuxCtlAd14(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd00(pub u32);
impl GpioMuxCtlSd00 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd00 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd00::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd00) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd00 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd00 {
        GpioMuxCtlSd00(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd01(pub u32);
impl GpioMuxCtlSd01 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd01 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd01::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd01) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd01 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd01 {
        GpioMuxCtlSd01(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd02(pub u32);
impl GpioMuxCtlSd02 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd02 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd02::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd02) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd02 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd02 {
        GpioMuxCtlSd02(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd03(pub u32);
impl GpioMuxCtlSd03 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd03 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd03::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd03) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd03 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd03 {
        GpioMuxCtlSd03(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd04(pub u32);
impl GpioMuxCtlSd04 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd04 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd04::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd04) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd04 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd04 {
        GpioMuxCtlSd04(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd05(pub u32);
impl GpioMuxCtlSd05 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd05 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd05::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd05) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd05 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd05 {
        GpioMuxCtlSd05(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd06(pub u32);
impl GpioMuxCtlSd06 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd06 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd06::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd06) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd06 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd06 {
        GpioMuxCtlSd06(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd07(pub u32);
impl GpioMuxCtlSd07 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd07 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd07::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd07) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd07 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd07 {
        GpioMuxCtlSd07(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd08(pub u32);
impl GpioMuxCtlSd08 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd08 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd08::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd08) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd08 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd08 {
        GpioMuxCtlSd08(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd09(pub u32);
impl GpioMuxCtlSd09 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd09 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd09::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd09) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd09 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd09 {
        GpioMuxCtlSd09(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd10(pub u32);
impl GpioMuxCtlSd10 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd10 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd10::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd10) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd10 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd10 {
        GpioMuxCtlSd10(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd11(pub u32);
impl GpioMuxCtlSd11 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd11 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd11::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd11) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd11 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd11 {
        GpioMuxCtlSd11(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd12(pub u32);
impl GpioMuxCtlSd12 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd12 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd12::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd12) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd12 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd12 {
        GpioMuxCtlSd12(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd13(pub u32);
impl GpioMuxCtlSd13 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::GpioMuxModeSd13 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::GpioMuxModeSd13::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::GpioMuxModeSd13) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd13 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd13 {
        GpioMuxCtlSd13(0)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GpioMuxCtlSd14(pub u32);
impl GpioMuxCtlSd14 {
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn mux_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GpioMuxCtlSd14 {
    #[inline(always)]
    fn default() -> GpioMuxCtlSd14 {
        GpioMuxCtlSd14(0)
    }
}
#[doc = "KPP_COL_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppColSelectInput0(pub u32);
impl KppColSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput0 {
    #[inline(always)]
    fn default() -> KppColSelectInput0 {
        KppColSelectInput0(0)
    }
}
#[doc = "KPP_COL_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppColSelectInput1(pub u32);
impl KppColSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput1 {
    #[inline(always)]
    fn default() -> KppColSelectInput1 {
        KppColSelectInput1(0)
    }
}
#[doc = "KPP_COL_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppColSelectInput2(pub u32);
impl KppColSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput2 {
    #[inline(always)]
    fn default() -> KppColSelectInput2 {
        KppColSelectInput2(0)
    }
}
#[doc = "KPP_COL_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppColSelectInput3(pub u32);
impl KppColSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput3 {
    #[inline(always)]
    fn default() -> KppColSelectInput3 {
        KppColSelectInput3(0)
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppRowSelectInput0(pub u32);
impl KppRowSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput0 {
    #[inline(always)]
    fn default() -> KppRowSelectInput0 {
        KppRowSelectInput0(0)
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppRowSelectInput1(pub u32);
impl KppRowSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput1 {
    #[inline(always)]
    fn default() -> KppRowSelectInput1 {
        KppRowSelectInput1(0)
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppRowSelectInput2(pub u32);
impl KppRowSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput2 {
    #[inline(always)]
    fn default() -> KppRowSelectInput2 {
        KppRowSelectInput2(0)
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KppRowSelectInput3(pub u32);
impl KppRowSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput3 {
    #[inline(always)]
    fn default() -> KppRowSelectInput3 {
        KppRowSelectInput3(0)
    }
}
#[doc = "LPI2C1_HREQ_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpi2c1hreqSelectInput(pub u32);
impl Lpi2c1hreqSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c1hreqSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1hreqSelectInput {
        Lpi2c1hreqSelectInput(0)
    }
}
#[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpi2c1sclSelectInput(pub u32);
impl Lpi2c1sclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1sclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c1sclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1sclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c1sclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1sclSelectInput {
        Lpi2c1sclSelectInput(0)
    }
}
#[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpi2c1sdaSelectInput(pub u32);
impl Lpi2c1sdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1sdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c1sdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1sdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c1sdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1sdaSelectInput {
        Lpi2c1sdaSelectInput(0)
    }
}
#[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpi2c2sclSelectInput(pub u32);
impl Lpi2c2sclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2sclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c2sclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2sclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c2sclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2sclSelectInput {
        Lpi2c2sclSelectInput(0)
    }
}
#[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpi2c2sdaSelectInput(pub u32);
impl Lpi2c2sdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2sdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c2sdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2sdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c2sdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2sdaSelectInput {
        Lpi2c2sdaSelectInput(0)
    }
}
#[doc = "LPSPI1_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi1pcsSelectInput0(pub u32);
impl Lpspi1pcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1pcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi1pcsSelectInput0 {
        Lpspi1pcsSelectInput0(0)
    }
}
#[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi1sckSelectInput(pub u32);
impl Lpspi1sckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1sckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1sckSelectInput {
        Lpspi1sckSelectInput(0)
    }
}
#[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi1sdiSelectInput(pub u32);
impl Lpspi1sdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1sdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1sdiSelectInput {
        Lpspi1sdiSelectInput(0)
    }
}
#[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi1sdoSelectInput(pub u32);
impl Lpspi1sdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1sdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1sdoSelectInput {
        Lpspi1sdoSelectInput(0)
    }
}
#[doc = "LPSPI2_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi2pcsSelectInput0(pub u32);
impl Lpspi2pcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2pcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi2pcsSelectInput0 {
        Lpspi2pcsSelectInput0(0)
    }
}
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi2sckSelectInput(pub u32);
impl Lpspi2sckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2sckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2sckSelectInput {
        Lpspi2sckSelectInput(0)
    }
}
#[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi2sdiSelectInput(pub u32);
impl Lpspi2sdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2sdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2sdiSelectInput {
        Lpspi2sdiSelectInput(0)
    }
}
#[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpspi2sdoSelectInput(pub u32);
impl Lpspi2sdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2sdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2sdoSelectInput {
        Lpspi2sdoSelectInput(0)
    }
}
#[doc = "LPUART1_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart1rxdSelectInput(pub u32);
impl Lpuart1rxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1rxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1rxdSelectInput {
        Lpuart1rxdSelectInput(0)
    }
}
#[doc = "LPUART1_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart1txdSelectInput(pub u32);
impl Lpuart1txdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1txdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1txdSelectInput {
        Lpuart1txdSelectInput(0)
    }
}
#[doc = "LPUART2_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart2rxdSelectInput(pub u32);
impl Lpuart2rxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2rxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2rxdSelectInput {
        Lpuart2rxdSelectInput(0)
    }
}
#[doc = "LPUART2_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart2txdSelectInput(pub u32);
impl Lpuart2txdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2txdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2txdSelectInput {
        Lpuart2txdSelectInput(0)
    }
}
#[doc = "LPUART3_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart3rxdSelectInput(pub u32);
impl Lpuart3rxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3rxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart3rxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3rxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart3rxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3rxdSelectInput {
        Lpuart3rxdSelectInput(0)
    }
}
#[doc = "LPUART3_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart3txdSelectInput(pub u32);
impl Lpuart3txdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3txdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart3txdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3txdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart3txdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3txdSelectInput {
        Lpuart3txdSelectInput(0)
    }
}
#[doc = "LPUART4_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart4rxdSelectInput(pub u32);
impl Lpuart4rxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart4rxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4rxdSelectInput {
        Lpuart4rxdSelectInput(0)
    }
}
#[doc = "LPUART4_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpuart4txdSelectInput(pub u32);
impl Lpuart4txdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart4txdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4txdSelectInput {
        Lpuart4txdSelectInput(0)
    }
}
#[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NmiGlueNmiSelectInput(pub u32);
impl NmiGlueNmiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for NmiGlueNmiSelectInput {
    #[inline(always)]
    fn default() -> NmiGlueNmiSelectInput {
        NmiGlueNmiSelectInput(0)
    }
}
#[doc = "SPDIF_IN1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SpdifIn1selectInput(pub u32);
impl SpdifIn1selectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SpdifIn1selectInput {
    #[inline(always)]
    fn default() -> SpdifIn1selectInput {
        SpdifIn1selectInput(0)
    }
}
#[doc = "SPDIF_TX_CLK2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SpdifTxClk2selectInput(pub u32);
impl SpdifTxClk2selectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SpdifTxClk2selectInput {
    #[inline(always)]
    fn default() -> SpdifTxClk2selectInput {
        SpdifTxClk2selectInput(0)
    }
}
#[doc = "USB_OTG_ID_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UsbOtgIdSelectInput(pub u32);
impl UsbOtgIdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbOtgIdSelectInput {
    #[inline(always)]
    fn default() -> UsbOtgIdSelectInput {
        UsbOtgIdSelectInput(0)
    }
}
#[doc = "USB_OTG_OC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UsbOtgOcSelectInput(pub u32);
impl UsbOtgOcSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbOtgOcSelectInput {
    #[inline(always)]
    fn default() -> UsbOtgOcSelectInput {
        UsbOtgOcSelectInput(0)
    }
}
#[doc = "XEV_GLUE_RXEV_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct XevGlueRxevSelectInput(pub u32);
impl XevGlueRxevSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn daisy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for XevGlueRxevSelectInput {
    #[inline(always)]
    fn default() -> XevGlueRxevSelectInput {
        XevGlueRxevSelectInput(0)
    }
}
