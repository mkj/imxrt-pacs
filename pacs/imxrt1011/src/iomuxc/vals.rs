#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioDse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_ = 0x01,
    #[doc = "R0/2"]
    DSE_2_R0_2 = 0x02,
    #[doc = "R0/3"]
    DSE_3_R0_3 = 0x03,
    #[doc = "R0/4"]
    DSE_4_R0_4 = 0x04,
    #[doc = "R0/5"]
    DSE_5_R0_5 = 0x05,
    #[doc = "R0/6"]
    DSE_6_R0_6 = 0x06,
    #[doc = "R0/7"]
    DSE_7_R0_7 = 0x07,
}
impl GpioDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioDse {
    #[inline(always)]
    fn from(val: u8) -> GpioDse {
        GpioDse::from_bits(val)
    }
}
impl From<GpioDse> for u8 {
    #[inline(always)]
    fn from(val: GpioDse) -> u8 {
        GpioDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode00 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DQS of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_MCLK of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_PCS3 of instance: LPSPI2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI1_PCS3 of instance: LPSPI1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: PIT_TRIGGER00 of instance: PIT"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO00 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode00 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode00 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode00 {
        GpioMuxMode00::from_bits(val)
    }
}
impl From<GpioMuxMode00> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode00) -> u8 {
        GpioMuxMode00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode01 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_RX_BCLK of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: WDOG1_ANY of instance: WDOG1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: LPI2C1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW03 of instance: KPP"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO01 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode01 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode01 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode01 {
        GpioMuxMode01::from_bits(val)
    }
}
impl From<GpioMuxMode01> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode01) -> u8 {
        GpioMuxMode01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode02 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_RX_SYNC of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: WDOG2_B of instance: WDOG2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: LPI2C1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_COL03 of instance: KPP"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO02 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode02 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode02 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode02 {
        GpioMuxMode02::from_bits(val)
    }
}
impl From<GpioMuxMode02> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode02) -> u8 {
        GpioMuxMode02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode03 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_RX_DATA00 of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_COMPARE3 of instance: GPT1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SPDIF_SR_CLK of instance: SPDIF"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO03 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode03 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode03 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode03 {
        GpioMuxMode03::from_bits(val)
    }
}
impl From<GpioMuxMode03> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode03) -> u8 {
        GpioMuxMode03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode04 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_DATA00 of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_CAPTURE2 of instance: GPT1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SPDIF_IN of instance: SPDIF"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO04 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode04 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode04 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode04 {
        GpioMuxMode04::from_bits(val)
    }
}
impl From<GpioMuxMode04> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode04) -> u8 {
        GpioMuxMode04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode05 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_DATA01 of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_COMPARE2 of instance: GPT1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_RXD of instance: LPUART4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SPDIF_OUT of instance: SPDIF"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO05 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode05 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode05 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode05 {
        GpioMuxMode05::from_bits(val)
    }
}
impl From<GpioMuxMode05> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode05) -> u8 {
        GpioMuxMode05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode06 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_BCLK of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_CAPTURE1 of instance: GPT1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_TXD of instance: LPUART4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SPDIF_EXT_CLK of instance: SPDIF"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO06 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode06 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode06 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode06 {
        GpioMuxMode06::from_bits(val)
    }
}
impl From<GpioMuxMode06> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode06) -> u8 {
        GpioMuxMode06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode07 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_TX_SYNC of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_COMPARE1 of instance: GPT1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RXD of instance: LPUART3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SPDIF_LOCK of instance: SPDIF"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO07 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART1_RTS_B of instance: LPUART1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode07 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode07 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode07 {
        GpioMuxMode07::from_bits(val)
    }
}
impl From<GpioMuxMode07> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode07) -> u8 {
        GpioMuxMode07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode08 {
    #[doc = "Select mux mode: ALT0 mux port: SAI1_MCLK of instance: SAI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_CLK of instance: GPT1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_TXD of instance: LPUART3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO00 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO08 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART1_CTS_B of instance: LPUART1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode08 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode08 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode08 {
        GpioMuxMode08::from_bits(val)
    }
}
impl From<GpioMuxMode08> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode08) -> u8 {
        GpioMuxMode08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode09 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART1_RXD of instance: LPUART1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: WDOG1_B of instance: WDOG1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXSPI_A_SS1_B of instance: FLEXSPI"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: LPI2C2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO01 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO09 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SPDIF_SR_CLK of instance: SPDIF"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode09 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode09 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode09 {
        GpioMuxMode09::from_bits(val)
    }
}
impl From<GpioMuxMode09> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode09) -> u8 {
        GpioMuxMode09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode10 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART1_TXD of instance: LPUART1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_HREQ of instance: LPI2C1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: EWM_OUT_B of instance: EWM"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: LPI2C2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO02 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO10 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SPDIF_IN of instance: SPDIF"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxMode10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode10 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode10 {
        GpioMuxMode10::from_bits(val)
    }
}
impl From<GpioMuxMode10> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode10) -> u8 {
        GpioMuxMode10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode11 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART3_RXD of instance: LPUART3"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_ROW00 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI_B_SS1_B of instance: FLEXSPI"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO03 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO11 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SPDIF_OUT of instance: SPDIF"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ARM_TRACE3 of instance: cm7_mxrt"]
    ALT7 = 0x07,
}
impl GpioMuxMode11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode11 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode11 {
        GpioMuxMode11::from_bits(val)
    }
}
impl From<GpioMuxMode11> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode11) -> u8 {
        GpioMuxMode11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode12 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART3_TXD of instance: LPUART3"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_COL00 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_OC of instance: USB"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO04 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO12 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SPDIF_EXT_CLK of instance: SPDIF"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ARM_TRACE2 of instance: cm7_mxrt"]
    ALT7 = 0x07,
}
impl GpioMuxMode12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode12 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode12 {
        GpioMuxMode12::from_bits(val)
    }
}
impl From<GpioMuxMode12> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode12) -> u8 {
        GpioMuxMode12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxMode13 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART2_RXD of instance: LPUART2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS2 of instance: LPSPI2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_ROW03 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_ID of instance: anatop"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO05 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO13 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SPDIF_LOCK of instance: SPDIF"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ARM_TRACE1 of instance: cm7_mxrt"]
    ALT7 = 0x07,
}
impl GpioMuxMode13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxMode13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxMode13 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxMode13 {
        GpioMuxMode13::from_bits(val)
    }
}
impl From<GpioMuxMode13> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxMode13) -> u8 {
        GpioMuxMode13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd00 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART2_TXD of instance: LPUART2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI1_PCS2 of instance: LPSPI1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_COL03 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: USB"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO20 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO14 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ARM_NMI of instance: NMI_GLUE"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ARM_TRACE0 of instance: cm7_mxrt"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd00 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd00 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd00 {
        GpioMuxModeAd00::from_bits(val)
    }
}
impl From<GpioMuxModeAd00> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd00) -> u8 {
        GpioMuxModeAd00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd01 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART4_RXD of instance: LPUART4"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: LPSPI2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: WDOG1_ANY of instance: WDOG1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: LPI2C2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: MQS_LEFT of instance: MQS"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO15 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_OC of instance: USB"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ARM_TRACE_SWO of instance: cm7_mxrt"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd01 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd01 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd01 {
        GpioMuxModeAd01::from_bits(val)
    }
}
impl From<GpioMuxModeAd01> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd01) -> u8 {
        GpioMuxModeAd01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd02 {
    #[doc = "Select mux mode: ALT0 mux port: LPUART4_TXD of instance: LPUART4"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI1_PCS1 of instance: LPSPI1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: WDOG2_B of instance: WDOG2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: LPI2C2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: MQS_RIGHT of instance: MQS"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO16 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ARM_TRACE_CLK of instance: cm7_mxrt"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd02 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd02 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd02 {
        GpioMuxModeAd02::from_bits(val)
    }
}
impl From<GpioMuxModeAd02> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd02) -> u8 {
        GpioMuxModeAd02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd03 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SDI of instance: LPSPI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER03 of instance: PIT"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW02 of instance: KPP"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT2_CLK of instance: GPT2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO17 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SNVS_VIO_5_B of instance: snvs_hp"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_DE_B of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd03 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd03 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd03 {
        GpioMuxModeAd03::from_bits(val)
    }
}
impl From<GpioMuxModeAd03> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd03) -> u8 {
        GpioMuxModeAd03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd04 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SDO of instance: LPSPI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER02 of instance: PIT"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL02 of instance: KPP"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT2_COMPARE1 of instance: GPT2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO18 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SNVS_VIO_5_CTL of instance: snvs_hp"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeAd04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd04 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd04 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd04 {
        GpioMuxModeAd04::from_bits(val)
    }
}
impl From<GpioMuxModeAd04> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd04) -> u8 {
        GpioMuxModeAd04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd05 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_PCS0 of instance: LPSPI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER01 of instance: PIT"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW01 of instance: KPP"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT2_CAPTURE1 of instance: GPT2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO19 of instance: GPIOMUX"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeAd05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd05 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd05 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd05 {
        GpioMuxModeAd05::from_bits(val)
    }
}
impl From<GpioMuxModeAd05> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd05) -> u8 {
        GpioMuxModeAd05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd06 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SCK of instance: LPSPI1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: PIT_TRIGGER00 of instance: PIT"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL01 of instance: KPP"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT2_COMPARE2 of instance: GPT2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO20 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: LPI2C1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeAd06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd06 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd06 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd06 {
        GpioMuxModeAd06::from_bits(val)
    }
}
impl From<GpioMuxModeAd06> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd06) -> u8 {
        GpioMuxModeAd06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd07 {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C2_SDA of instance: LPI2C2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART3_RXD of instance: LPUART3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_RXEV of instance: cm7_mxrt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART2_RTS_B of instance: LPUART2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT2_CAPTURE2 of instance: GPT2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO21 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: OCOTP_FUSE_LATCHED of instance: OCOTP"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: XBAR1_INOUT03 of instance: XBAR1"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd07 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd07 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd07 {
        GpioMuxModeAd07::from_bits(val)
    }
}
impl From<GpioMuxModeAd07> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd07) -> u8 {
        GpioMuxModeAd07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd08 {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C2_SCL of instance: LPI2C2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART3_TXD of instance: LPUART3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_TXEV of instance: cm7_mxrt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART2_CTS_B of instance: LPUART2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT2_COMPARE3 of instance: GPT2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO22 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: EWM_OUT_B of instance: EWM"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_TRSTB of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd08 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd08 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd08 {
        GpioMuxModeAd08::from_bits(val)
    }
}
impl From<GpioMuxModeAd08> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd08) -> u8 {
        GpioMuxModeAd08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd09 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI2_SDI of instance: LPSPI2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM3_X of instance: FLEXPWM1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_ROW02 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ARM_TRACE_SWO of instance: cm7_mxrt"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO21 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO23 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: REF_CLK_32K of instance: XTAL OSC"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_TDO of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd09 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd09 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd09 {
        GpioMuxModeAd09::from_bits(val)
    }
}
impl From<GpioMuxModeAd09> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd09) -> u8 {
        GpioMuxModeAd09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd10 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI2_SDO of instance: LPSPI2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM2_X of instance: FLEXPWM1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_COL02 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: PIT_TRIGGER03 of instance: PIT"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO22 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO24 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_ID of instance: anatop"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_TDI of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd10 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd10 {
        GpioMuxModeAd10::from_bits(val)
    }
}
impl From<GpioMuxModeAd10> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd10) -> u8 {
        GpioMuxModeAd10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd11 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI2_PCS0 of instance: LPSPI2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM1_X of instance: FLEXPWM1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_ROW01 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: PIT_TRIGGER02 of instance: PIT"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO23 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO25 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: WDOG1_B of instance: WDOG1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_MOD of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd11 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd11 {
        GpioMuxModeAd11::from_bits(val)
    }
}
impl From<GpioMuxModeAd11> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd11) -> u8 {
        GpioMuxModeAd11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd12 {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI2_SCK of instance: LPSPI2"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWM0_X of instance: FLEXPWM1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_COL01 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: PIT_TRIGGER01 of instance: PIT"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO24 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO26 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USB_OTG1_PWR of instance: USB"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_TCK of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd12 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd12 {
        GpioMuxModeAd12::from_bits(val)
    }
}
impl From<GpioMuxModeAd12> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd12) -> u8 {
        GpioMuxModeAd12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd13 {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C1_SDA of instance: LPI2C1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART3_RTS_B of instance: LPUART3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_ROW00 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_RTS_B of instance: LPUART4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO25 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO27 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ARM_NMI of instance: NMI_GLUE"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: JTAG_TMS of instance: JTAG"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd13 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd13 {
        GpioMuxModeAd13::from_bits(val)
    }
}
impl From<GpioMuxModeAd13> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd13) -> u8 {
        GpioMuxModeAd13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeAd14 {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C1_SCL of instance: LPI2C1"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART3_CTS_B of instance: LPUART3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: KPP_COL00 of instance: KPP"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_CTS_B of instance: LPUART4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO26 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIOMUX_IO28 of instance: GPIOMUX"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: REF_CLK_24M of instance: XTAL OSC"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: XBAR1_INOUT02 of instance: XBAR1"]
    ALT7 = 0x07,
}
impl GpioMuxModeAd14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeAd14 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeAd14 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeAd14 {
        GpioMuxModeAd14::from_bits(val)
    }
}
impl From<GpioMuxModeAd14> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeAd14) -> u8 {
        GpioMuxModeAd14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd00 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_SS0_B of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_TX_SYNC of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_RXEV of instance: cm7_mxrt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_STOP of instance: CCM"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO06 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG02 of instance: SRC"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd00 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd00 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd00 {
        GpioMuxModeSd00::from_bits(val)
    }
}
impl From<GpioMuxModeSd00> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd00) -> u8 {
        GpioMuxModeSd00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd01 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA01 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_TX_BCLK of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_CLKO2 of instance: CCM"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO07 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG01 of instance: SRC"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd01 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd01 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd01 {
        GpioMuxModeSd01::from_bits(val)
    }
}
impl From<GpioMuxModeSd01> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd01) -> u8 {
        GpioMuxModeSd01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd02 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA02 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_TX_DATA of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_CLKO1 of instance: CCM"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO08 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG00 of instance: SRC"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd02 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd02 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd02 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd02 {
        GpioMuxModeSd02::from_bits(val)
    }
}
impl From<GpioMuxModeSd02> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd02) -> u8 {
        GpioMuxModeSd02::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd03 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA00 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_RX_DATA of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_REF_EN_B of instance: CCM"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO09 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_MODE01 of instance: SRC"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd03 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd03 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd03 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd03 {
        GpioMuxModeSd03::from_bits(val)
    }
}
impl From<GpioMuxModeSd03> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd03) -> u8 {
        GpioMuxModeSd03::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd04 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_DATA03 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_RX_SYNC of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_WAIT of instance: CCM"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO10 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_MODE00 of instance: SRC"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd04 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd04 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd04 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd04 {
        GpioMuxModeSd04::from_bits(val)
    }
}
impl From<GpioMuxModeSd04> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd04) -> u8 {
        GpioMuxModeSd04::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd05 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_SS1_B of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_SDI of instance: LPSPI1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO11 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: GPIO2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd05 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd05 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd05 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd05 {
        GpioMuxModeSd05::from_bits(val)
    }
}
impl From<GpioMuxModeSd05> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd05) -> u8 {
        GpioMuxModeSd05::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd06 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_SS0_B of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_SDO of instance: LPSPI1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO12 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: GPIO2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd06 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd06 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd06 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd06 {
        GpioMuxModeSd06::from_bits(val)
    }
}
impl From<GpioMuxModeSd06> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd06) -> u8 {
        GpioMuxModeSd06::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd07 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA1 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SDA of instance: LPI2C2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_PCS0 of instance: LPSPI1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO13 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: GPIO2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd07 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd07 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd07 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd07 {
        GpioMuxModeSd07::from_bits(val)
    }
}
impl From<GpioMuxModeSd07> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd07) -> u8 {
        GpioMuxModeSd07::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd08 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA2 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SCL of instance: LPI2C2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_SCK of instance: LPSPI1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO14 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: GPIO2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd08 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd08 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd08 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd08 {
        GpioMuxModeSd08::from_bits(val)
    }
}
impl From<GpioMuxModeSd08> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd08) -> u8 {
        GpioMuxModeSd08::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd09 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA0 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDI of instance: LPSPI2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RXD of instance: LPUART2"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO15 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: GPIO2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd09 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd09 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd09 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd09 {
        GpioMuxModeSd09::from_bits(val)
    }
}
impl From<GpioMuxModeSd09> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd09) -> u8 {
        GpioMuxModeSd09::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd10 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_SCLK of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDO of instance: LPSPI2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_TXD of instance: LPUART2"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO16 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: GPIO2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd10 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd10 {
        GpioMuxModeSd10::from_bits(val)
    }
}
impl From<GpioMuxModeSd10> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd10) -> u8 {
        GpioMuxModeSd10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd11 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DATA3 of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SCK of instance: LPSPI2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_RXD of instance: LPUART1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO17 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: WDOG1_RST_B_DEB of instance: WDOG1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd11 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd11 {
        GpioMuxModeSd11::from_bits(val)
    }
}
impl From<GpioMuxModeSd11> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd11) -> u8 {
        GpioMuxModeSd11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd12 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_A_DQS of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: LPSPI2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_TXD of instance: LPUART1"]
    ALT2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO18 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: WDOG2_RST_B_DEB of instance: WDOG2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd12 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd12 {
        GpioMuxModeSd12::from_bits(val)
    }
}
impl From<GpioMuxModeSd12> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd12) -> u8 {
        GpioMuxModeSd12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioMuxModeSd13 {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI_B_SCLK of instance: FLEXSPI"]
    ALT0 = 0,
    #[doc = "Select mux mode: ALT1 mux port: SAI3_RX_BCLK of instance: SAI3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_CM7_TXEV of instance: cm7_mxrt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_PMIC_RDY of instance: CCM"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_IO19 of instance: FLEXIO1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: GPIO2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BT_CFG03 of instance: SRC"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GpioMuxModeSd13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeSd13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeSd13 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeSd13 {
        GpioMuxModeSd13::from_bits(val)
    }
}
impl From<GpioMuxModeSd13> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeSd13) -> u8 {
        GpioMuxModeSd13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl GpioPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioPus {
    #[inline(always)]
    fn from(val: u8) -> GpioPus {
        GpioPus::from_bits(val)
    }
}
impl From<GpioPus> for u8 {
    #[inline(always)]
    fn from(val: GpioPus) -> u8 {
        GpioPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GpioSpeed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl GpioSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSpeed {
    #[inline(always)]
    fn from(val: u8) -> GpioSpeed {
        GpioSpeed::from_bits(val)
    }
}
impl From<GpioSpeed> for u8 {
    #[inline(always)]
    fn from(val: GpioSpeed) -> u8 {
        GpioSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpi2c1sclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT0"]
    GPIO_AD_14_ALT0 = 0,
    #[doc = "Selecting Pad: GPIO_SD_06 for Mode: ALT1"]
    GPIO_SD_06_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT1"]
    GPIO_12_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_02 for Mode: ALT3"]
    GPIO_02_ALT3 = 0x03,
}
impl Lpi2c1sclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1sclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1sclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1sclSelectInputDaisy {
        Lpi2c1sclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1sclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1sclSelectInputDaisy) -> u8 {
        Lpi2c1sclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpi2c1sdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT0"]
    GPIO_AD_13_ALT0 = 0,
    #[doc = "Selecting Pad: GPIO_SD_05 for Mode: ALT1"]
    GPIO_SD_05_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_11 for Mode: ALT1"]
    GPIO_11_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_01 for Mode: ALT3"]
    GPIO_01_ALT3 = 0x03,
}
impl Lpi2c1sdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1sdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1sdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1sdaSelectInputDaisy {
        Lpi2c1sdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1sdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1sdaSelectInputDaisy) -> u8 {
        Lpi2c1sdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpi2c2sclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT0"]
    GPIO_AD_08_ALT0 = 0,
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT3"]
    GPIO_AD_02_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_08 for Mode: ALT1"]
    GPIO_SD_08_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_10 for Mode: ALT3"]
    GPIO_10_ALT3 = 0x03,
}
impl Lpi2c2sclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2sclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2sclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2sclSelectInputDaisy {
        Lpi2c2sclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c2sclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2sclSelectInputDaisy) -> u8 {
        Lpi2c2sclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpi2c2sdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT0"]
    GPIO_AD_07_ALT0 = 0,
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT3"]
    GPIO_AD_01_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_07 for Mode: ALT1"]
    GPIO_SD_07_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_09 for Mode: ALT3"]
    GPIO_09_ALT3 = 0x03,
}
impl Lpi2c2sdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2sdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2sdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2sdaSelectInputDaisy {
        Lpi2c2sdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c2sdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2sdaSelectInputDaisy) -> u8 {
        Lpi2c2sdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpuart3rxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT1"]
    GPIO_AD_07_ALT1 = 0,
    #[doc = "Selecting Pad: GPIO_11 for Mode: ALT0"]
    GPIO_11_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_07 for Mode: ALT3"]
    GPIO_07_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3rxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3rxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3rxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3rxdSelectInputDaisy {
        Lpuart3rxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3rxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3rxdSelectInputDaisy) -> u8 {
        Lpuart3rxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpuart3txdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT1"]
    GPIO_AD_08_ALT1 = 0,
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT0"]
    GPIO_12_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_08 for Mode: ALT3"]
    GPIO_08_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3txdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3txdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3txdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3txdSelectInputDaisy {
        Lpuart3txdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3txdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3txdSelectInputDaisy) -> u8 {
        Lpuart3txdSelectInputDaisy::to_bits(val)
    }
}
