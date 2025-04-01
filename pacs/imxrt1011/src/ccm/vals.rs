#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AdcAclkPodf {
    _RESERVED_0 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "pll3_sw_clk / 8"]
    ADC_ACLK_PODF_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "pll3_sw_clk / 12"]
    ADC_ACLK_PODF_11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "pll3_sw_clk / 16"]
    ADC_ACLK_PODF_15 = 0x0f,
}
impl AdcAclkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcAclkPodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcAclkPodf {
    #[inline(always)]
    fn from(val: u8) -> AdcAclkPodf {
        AdcAclkPodf::from_bits(val)
    }
}
impl From<AdcAclkPodf> for u8 {
    #[inline(always)]
    fn from(val: AdcAclkPodf) -> u8 {
        AdcAclkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AhbPodf {
    #[doc = "divide by 1"]
    AHB_PODF_0 = 0,
    #[doc = "divide by 2"]
    AHB_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    AHB_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    AHB_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    AHB_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    AHB_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    AHB_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    AHB_PODF_7 = 0x07,
}
impl AhbPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodf {
    #[inline(always)]
    fn from(val: u8) -> AhbPodf {
        AhbPodf::from_bits(val)
    }
}
impl From<AhbPodf> for u8 {
    #[inline(always)]
    fn from(val: AhbPodf) -> u8 {
        AhbPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clko1div {
    #[doc = "divide by 1"]
    CLKO1_DIV_0 = 0,
    #[doc = "divide by 2"]
    CLKO1_DIV_1 = 0x01,
    #[doc = "divide by 3"]
    CLKO1_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    CLKO1_DIV_3 = 0x03,
    #[doc = "divide by 5"]
    CLKO1_DIV_4 = 0x04,
    #[doc = "divide by 6"]
    CLKO1_DIV_5 = 0x05,
    #[doc = "divide by 7"]
    CLKO1_DIV_6 = 0x06,
    #[doc = "divide by 8"]
    CLKO1_DIV_7 = 0x07,
}
impl Clko1div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1div {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1div {
    #[inline(always)]
    fn from(val: u8) -> Clko1div {
        Clko1div::from_bits(val)
    }
}
impl From<Clko1div> for u8 {
    #[inline(always)]
    fn from(val: Clko1div) -> u8 {
        Clko1div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clko1sel {
    #[doc = "pll3_sw_clk (divided by 2)"]
    CLKO1_SEL_0 = 0,
    #[doc = "PLL2 (divided by 2)"]
    CLKO1_SEL_1 = 0x01,
    #[doc = "ENET PLL (divided by 2)"]
    CLKO1_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "core_clk_root"]
    CLKO1_SEL_11 = 0x0b,
    #[doc = "ipg_clk_root"]
    CLKO1_SEL_12 = 0x0c,
    #[doc = "perclk_root"]
    CLKO1_SEL_13 = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "pll4_main_clk"]
    CLKO1_SEL_15 = 0x0f,
}
impl Clko1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1sel {
    #[inline(always)]
    fn from(val: u8) -> Clko1sel {
        Clko1sel::from_bits(val)
    }
}
impl From<Clko1sel> for u8 {
    #[inline(always)]
    fn from(val: Clko1sel) -> u8 {
        Clko1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clko2div {
    #[doc = "divide by 1"]
    CLKO2_DIV_0 = 0,
    #[doc = "divide by 2"]
    CLKO2_DIV_1 = 0x01,
    #[doc = "divide by 3"]
    CLKO2_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    CLKO2_DIV_3 = 0x03,
    #[doc = "divide by 5"]
    CLKO2_DIV_4 = 0x04,
    #[doc = "divide by 6"]
    CLKO2_DIV_5 = 0x05,
    #[doc = "divide by 7"]
    CLKO2_DIV_6 = 0x06,
    #[doc = "divide by 8"]
    CLKO2_DIV_7 = 0x07,
}
impl Clko2div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2div {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2div {
    #[inline(always)]
    fn from(val: u8) -> Clko2div {
        Clko2div::from_bits(val)
    }
}
impl From<Clko2div> for u8 {
    #[inline(always)]
    fn from(val: Clko2div) -> u8 {
        Clko2div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clko2sel {
    _RESERVED_0 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "lpi2c_clk_root"]
    CLKO2_SEL_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "osc_clk"]
    CLKO2_SEL_14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "lpspi_clk_root"]
    CLKO2_SEL_16 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "sai1_clk_root"]
    CLKO2_SEL_18 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "sai3_clk_root"]
    CLKO2_SEL_20 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "trace_clk_root"]
    CLKO2_SEL_22 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "flexspi_clk_root"]
    CLKO2_SEL_27 = 0x1b,
    #[doc = "uart_clk_root"]
    CLKO2_SEL_28 = 0x1c,
    #[doc = "spdif0_clk_root"]
    CLKO2_SEL_29 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Clko2sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2sel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2sel {
    #[inline(always)]
    fn from(val: u8) -> Clko2sel {
        Clko2sel::from_bits(val)
    }
}
impl From<Clko2sel> for u8 {
    #[inline(always)]
    fn from(val: Clko2sel) -> u8 {
        Clko2sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flexio1clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
}
impl Flexio1clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1clkPodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1clkPodf {
    #[inline(always)]
    fn from(val: u8) -> Flexio1clkPodf {
        Flexio1clkPodf::from_bits(val)
    }
}
impl From<Flexio1clkPodf> for u8 {
    #[inline(always)]
    fn from(val: Flexio1clkPodf) -> u8 {
        Flexio1clkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flexio1clkPred {
    #[doc = "divide by 1"]
    FLEXIO1_CLK_PRED_0 = 0,
    #[doc = "divide by 2"]
    FLEXIO1_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXIO1_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXIO1_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXIO1_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXIO1_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXIO1_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXIO1_CLK_PRED_7 = 0x07,
}
impl Flexio1clkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1clkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1clkPred {
    #[inline(always)]
    fn from(val: u8) -> Flexio1clkPred {
        Flexio1clkPred::from_bits(val)
    }
}
impl From<Flexio1clkPred> for u8 {
    #[inline(always)]
    fn from(val: Flexio1clkPred) -> u8 {
        Flexio1clkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flexio1clkSel {
    #[doc = "derive clock from PLL4 divided clock"]
    FLEXIO1_CLK_SEL_0 = 0,
    #[doc = "derive clock from PLL3 PFD2 clock"]
    FLEXIO1_CLK_SEL_1 = 0x01,
    #[doc = "derive from PLL2"]
    FLEXIO1_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO1_CLK_SEL_3 = 0x03,
}
impl Flexio1clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1clkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexio1clkSel {
        Flexio1clkSel::from_bits(val)
    }
}
impl From<Flexio1clkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexio1clkSel) -> u8 {
        Flexio1clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlexspiClkSel {
    #[doc = "derive clock from PLL2"]
    FLEXSPI_CLK_SEL_0 = 0,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXSPI_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD2"]
    FLEXSPI_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL3 PFD0"]
    FLEXSPI_CLK_SEL_3 = 0x03,
}
impl FlexspiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiClkSel {
    #[inline(always)]
    fn from(val: u8) -> FlexspiClkSel {
        FlexspiClkSel::from_bits(val)
    }
}
impl From<FlexspiClkSel> for u8 {
    #[inline(always)]
    fn from(val: FlexspiClkSel) -> u8 {
        FlexspiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlexspiPodf {
    #[doc = "divide by 1"]
    FLEXSPI_PODF_0 = 0,
    #[doc = "divide by 2"]
    FLEXSPI_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXSPI_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXSPI_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXSPI_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXSPI_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXSPI_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXSPI_PODF_7 = 0x07,
}
impl FlexspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiPodf {
    #[inline(always)]
    fn from(val: u8) -> FlexspiPodf {
        FlexspiPodf::from_bits(val)
    }
}
impl From<FlexspiPodf> for u8 {
    #[inline(always)]
    fn from(val: FlexspiPodf) -> u8 {
        FlexspiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum IpgPodf {
    #[doc = "divide by 1"]
    IPG_PODF_0 = 0,
    #[doc = "divide by 2"]
    IPG_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    IPG_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    IPG_PODF_3 = 0x03,
}
impl IpgPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpgPodf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpgPodf {
    #[inline(always)]
    fn from(val: u8) -> IpgPodf {
        IpgPodf::from_bits(val)
    }
}
impl From<IpgPodf> for u8 {
    #[inline(always)]
    fn from(val: IpgPodf) -> u8 {
        IpgPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpi2cClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Lpi2cClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkPodf {
        Lpi2cClkPodf::from_bits(val)
    }
}
impl From<Lpi2cClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkPodf) -> u8 {
        Lpi2cClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lpm {
    #[doc = "Remain in run mode"]
    LPM_0 = 0,
    #[doc = "Transfer to wait mode"]
    LPM_1 = 0x01,
    #[doc = "Transfer to stop mode"]
    LPM_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpm {
    #[inline(always)]
    fn from(val: u8) -> Lpm {
        Lpm::from_bits(val)
    }
}
impl From<Lpm> for u8 {
    #[inline(always)]
    fn from(val: Lpm) -> u8 {
        Lpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LpspiClkSel {
    #[doc = "derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0 = 0,
    #[doc = "derive clock from PLL3 PFD0"]
    LPSPI_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2"]
    LPSPI_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 PFD2"]
    LPSPI_CLK_SEL_3 = 0x03,
}
impl LpspiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiClkSel {
    #[inline(always)]
    fn from(val: u8) -> LpspiClkSel {
        LpspiClkSel::from_bits(val)
    }
}
impl From<LpspiClkSel> for u8 {
    #[inline(always)]
    fn from(val: LpspiClkSel) -> u8 {
        LpspiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LpspiPodf {
    #[doc = "divide by 1"]
    LPSPI_PODF_0 = 0,
    #[doc = "divide by 2"]
    LPSPI_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    LPSPI_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    LPSPI_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    LPSPI_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    LPSPI_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    LPSPI_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    LPSPI_PODF_7 = 0x07,
    #[doc = "divide by 9"]
    LPSPI_PODF_8 = 0x08,
    #[doc = "divide by 10"]
    LPSPI_PODF_9 = 0x09,
    #[doc = "divide by 11"]
    LPSPI_PODF_10 = 0x0a,
    #[doc = "divide by 12"]
    LPSPI_PODF_11 = 0x0b,
    #[doc = "divide by 13"]
    LPSPI_PODF_12 = 0x0c,
    #[doc = "divide by 14"]
    LPSPI_PODF_13 = 0x0d,
    #[doc = "divide by 15"]
    LPSPI_PODF_14 = 0x0e,
    #[doc = "divide by 16"]
    LPSPI_PODF_15 = 0x0f,
}
impl LpspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiPodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiPodf {
    #[inline(always)]
    fn from(val: u8) -> LpspiPodf {
        LpspiPodf::from_bits(val)
    }
}
impl From<LpspiPodf> for u8 {
    #[inline(always)]
    fn from(val: LpspiPodf) -> u8 {
        LpspiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PerclkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl PerclkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkPodf {
    #[inline(always)]
    fn from(val: u8) -> PerclkPodf {
        PerclkPodf::from_bits(val)
    }
}
impl From<PerclkPodf> for u8 {
    #[inline(always)]
    fn from(val: PerclkPodf) -> u8 {
        PerclkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PeriphClk2sel {
    #[doc = "derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0 = 0,
    #[doc = "derive clock from osc_clk"]
    PERIPH_CLK2_SEL_1 = 0x01,
    #[doc = "derive clock from pll2_bypass_clk"]
    PERIPH_CLK2_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PeriphClk2sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClk2sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClk2sel {
    #[inline(always)]
    fn from(val: u8) -> PeriphClk2sel {
        PeriphClk2sel::from_bits(val)
    }
}
impl From<PeriphClk2sel> for u8 {
    #[inline(always)]
    fn from(val: PeriphClk2sel) -> u8 {
        PeriphClk2sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PrePeriphClkSel {
    #[doc = "derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0 = 0,
    #[doc = "derive clock from PLL3 PFD3"]
    PRE_PERIPH_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD3"]
    PRE_PERIPH_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL6"]
    PRE_PERIPH_CLK_SEL_3 = 0x03,
}
impl PrePeriphClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrePeriphClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrePeriphClkSel {
    #[inline(always)]
    fn from(val: u8) -> PrePeriphClkSel {
        PrePeriphClkSel::from_bits(val)
    }
}
impl From<PrePeriphClkSel> for u8 {
    #[inline(always)]
    fn from(val: PrePeriphClkSel) -> u8 {
        PrePeriphClkSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RegBypassCount(pub u8);
impl RegBypassCount {
    #[doc = "no delay"]
    pub const REG_BYPASS_COUNT_0: Self = Self(0);
    #[doc = "1 CKIL clock period delay"]
    pub const REG_BYPASS_COUNT_1: Self = Self(0x01);
    #[doc = "63 CKIL clock periods delay"]
    pub const REG_BYPASS_COUNT_63: Self = Self(0x3f);
}
impl RegBypassCount {
    pub const fn from_bits(val: u8) -> RegBypassCount {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for RegBypassCount {
    #[inline(always)]
    fn from(val: u8) -> RegBypassCount {
        RegBypassCount::from_bits(val)
    }
}
impl From<RegBypassCount> for u8 {
    #[inline(always)]
    fn from(val: RegBypassCount) -> u8 {
        RegBypassCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Sai1clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkPodf {
        Sai1clkPodf::from_bits(val)
    }
}
impl From<Sai1clkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkPodf) -> u8 {
        Sai1clkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1clkPred {
    #[doc = "divide by 1"]
    SAI1_CLK_PRED_0 = 0,
    #[doc = "divide by 2"]
    SAI1_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    SAI1_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    SAI1_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    SAI1_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    SAI1_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    SAI1_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    SAI1_CLK_PRED_7 = 0x07,
}
impl Sai1clkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkPred {
        Sai1clkPred::from_bits(val)
    }
}
impl From<Sai1clkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkPred) -> u8 {
        Sai1clkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai1clkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI1_CLK_SEL_0 = 0,
    #[doc = "derive from pll3_sw_clk"]
    SAI1_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL4"]
    SAI1_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkSel {
        Sai1clkSel::from_bits(val)
    }
}
impl From<Sai1clkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkSel) -> u8 {
        Sai1clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai3clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl Sai3clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3clkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3clkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai3clkPodf {
        Sai3clkPodf::from_bits(val)
    }
}
impl From<Sai3clkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai3clkPodf) -> u8 {
        Sai3clkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai3clkPred {
    #[doc = "divide by 1"]
    SAI3_CLK_PRED_0 = 0,
    #[doc = "divide by 2"]
    SAI3_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    SAI3_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    SAI3_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    SAI3_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    SAI3_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    SAI3_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    SAI3_CLK_PRED_7 = 0x07,
}
impl Sai3clkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3clkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3clkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai3clkPred {
        Sai3clkPred::from_bits(val)
    }
}
impl From<Sai3clkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai3clkPred) -> u8 {
        Sai3clkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sai3clkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI3_CLK_SEL_0 = 0,
    #[doc = "derive from pll3_sw_clk"]
    SAI3_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL4"]
    SAI3_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai3clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3clkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai3clkSel {
        Sai3clkSel::from_bits(val)
    }
}
impl From<Sai3clkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai3clkSel) -> u8 {
        Sai3clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spdif0clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Spdif0clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0clkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0clkPodf {
    #[inline(always)]
    fn from(val: u8) -> Spdif0clkPodf {
        Spdif0clkPodf::from_bits(val)
    }
}
impl From<Spdif0clkPodf> for u8 {
    #[inline(always)]
    fn from(val: Spdif0clkPodf) -> u8 {
        Spdif0clkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spdif0clkPred {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
}
impl Spdif0clkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0clkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0clkPred {
    #[inline(always)]
    fn from(val: u8) -> Spdif0clkPred {
        Spdif0clkPred::from_bits(val)
    }
}
impl From<Spdif0clkPred> for u8 {
    #[inline(always)]
    fn from(val: Spdif0clkPred) -> u8 {
        Spdif0clkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Spdif0clkSel {
    #[doc = "derive clock from PLL4"]
    SPDIF0_CLK_SEL_0 = 0,
    #[doc = "derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk"]
    SPDIF0_CLK_SEL_3 = 0x03,
}
impl Spdif0clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0clkSel {
    #[inline(always)]
    fn from(val: u8) -> Spdif0clkSel {
        Spdif0clkSel::from_bits(val)
    }
}
impl From<Spdif0clkSel> for u8 {
    #[inline(always)]
    fn from(val: Spdif0clkSel) -> u8 {
        Spdif0clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StbyCount {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0 = 0,
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_1 = 0x01,
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_2 = 0x02,
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_3 = 0x03,
}
impl StbyCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StbyCount {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StbyCount {
    #[inline(always)]
    fn from(val: u8) -> StbyCount {
        StbyCount::from_bits(val)
    }
}
impl From<StbyCount> for u8 {
    #[inline(always)]
    fn from(val: StbyCount) -> u8 {
        StbyCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SysMemDsCtrl {
    #[doc = "Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0 = 0,
    #[doc = "Enable memory (outside Arm platform) DS mode when system STOP and PLL are disabled"]
    SYS_MEM_DS_CTRL_1 = 0x01,
    #[doc = "enable memory (outside Arm platform) DS mode when system is in STOP mode"]
    SYS_MEM_DS_CTRL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SysMemDsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysMemDsCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysMemDsCtrl {
    #[inline(always)]
    fn from(val: u8) -> SysMemDsCtrl {
        SysMemDsCtrl::from_bits(val)
    }
}
impl From<SysMemDsCtrl> for u8 {
    #[inline(always)]
    fn from(val: SysMemDsCtrl) -> u8 {
        SysMemDsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TraceClkSel {
    #[doc = "derive clock from PLL2"]
    TRACE_CLK_SEL_0 = 0,
    #[doc = "derive clock from PLL2 PFD2"]
    TRACE_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD0"]
    TRACE_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 PFD1"]
    TRACE_CLK_SEL_3 = 0x03,
}
impl TraceClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceClkSel {
    #[inline(always)]
    fn from(val: u8) -> TraceClkSel {
        TraceClkSel::from_bits(val)
    }
}
impl From<TraceClkSel> for u8 {
    #[inline(always)]
    fn from(val: TraceClkSel) -> u8 {
        TraceClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TracePodf {
    #[doc = "divide by 1"]
    TRACE_PODF_0 = 0,
    #[doc = "divide by 2"]
    TRACE_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    TRACE_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    TRACE_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    TRACE_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    TRACE_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    TRACE_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    TRACE_PODF_7 = 0x07,
    #[doc = "divide by 9"]
    TRACE_PODF_8 = 0x08,
    #[doc = "divide by 10"]
    TRACE_PODF_9 = 0x09,
    #[doc = "divide by 11"]
    TRACE_PODF_10 = 0x0a,
    #[doc = "divide by 12"]
    TRACE_PODF_11 = 0x0b,
    #[doc = "divide by 13"]
    TRACE_PODF_12 = 0x0c,
    #[doc = "divide by 14"]
    TRACE_PODF_13 = 0x0d,
    #[doc = "divide by 15"]
    TRACE_PODF_14 = 0x0e,
    #[doc = "divide by 16"]
    TRACE_PODF_15 = 0x0f,
}
impl TracePodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TracePodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TracePodf {
    #[inline(always)]
    fn from(val: u8) -> TracePodf {
        TracePodf::from_bits(val)
    }
}
impl From<TracePodf> for u8 {
    #[inline(always)]
    fn from(val: TracePodf) -> u8 {
        TracePodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UartClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "Divide by 2"]
    DIVIDE_2 = 0x01,
    #[doc = "Divide by 3"]
    DIVIDE_3 = 0x02,
    #[doc = "Divide by 4"]
    DIVIDE_4 = 0x03,
    #[doc = "Divide by 5"]
    DIVIDE_5 = 0x04,
    #[doc = "Divide by 6"]
    DIVIDE_6 = 0x05,
    #[doc = "Divide by 7"]
    DIVIDE_7 = 0x06,
    #[doc = "Divide by 8"]
    DIVIDE_8 = 0x07,
    #[doc = "Divide by 9"]
    DIVIDE_9 = 0x08,
    #[doc = "Divide by 10"]
    DIVIDE_10 = 0x09,
    #[doc = "Divide by 11"]
    DIVIDE_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIVIDE_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIVIDE_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIVIDE_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIVIDE_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIVIDE_16 = 0x0f,
    #[doc = "Divide by 17"]
    DIVIDE_17 = 0x10,
    #[doc = "Divide by 18"]
    DIVIDE_18 = 0x11,
    #[doc = "Divide by 19"]
    DIVIDE_19 = 0x12,
    #[doc = "Divide by 20"]
    DIVIDE_20 = 0x13,
    #[doc = "Divide by 21"]
    DIVIDE_21 = 0x14,
    #[doc = "Divide by 22"]
    DIVIDE_22 = 0x15,
    #[doc = "Divide by 23"]
    DIVIDE_23 = 0x16,
    #[doc = "Divide by 24"]
    DIVIDE_24 = 0x17,
    #[doc = "Divide by 25"]
    DIVIDE_25 = 0x18,
    #[doc = "Divide by 26"]
    DIVIDE_26 = 0x19,
    #[doc = "Divide by 27"]
    DIVIDE_27 = 0x1a,
    #[doc = "Divide by 28"]
    DIVIDE_28 = 0x1b,
    #[doc = "Divide by 29"]
    DIVIDE_29 = 0x1c,
    #[doc = "Divide by 30"]
    DIVIDE_30 = 0x1d,
    #[doc = "Divide by 31"]
    DIVIDE_31 = 0x1e,
    #[doc = "Divide by 32"]
    DIVIDE_32 = 0x1f,
    #[doc = "Divide by 33"]
    DIVIDE_33 = 0x20,
    #[doc = "Divide by 34"]
    DIVIDE_34 = 0x21,
    #[doc = "Divide by 35"]
    DIVIDE_35 = 0x22,
    #[doc = "Divide by 36"]
    DIVIDE_36 = 0x23,
    #[doc = "Divide by 37"]
    DIVIDE_37 = 0x24,
    #[doc = "Divide by 38"]
    DIVIDE_38 = 0x25,
    #[doc = "Divide by 39"]
    DIVIDE_39 = 0x26,
    #[doc = "Divide by 40"]
    DIVIDE_40 = 0x27,
    #[doc = "Divide by 41"]
    DIVIDE_41 = 0x28,
    #[doc = "Divide by 42"]
    DIVIDE_42 = 0x29,
    #[doc = "Divide by 43"]
    DIVIDE_43 = 0x2a,
    #[doc = "Divide by 44"]
    DIVIDE_44 = 0x2b,
    #[doc = "Divide by 45"]
    DIVIDE_45 = 0x2c,
    #[doc = "Divide by 46"]
    DIVIDE_46 = 0x2d,
    #[doc = "Divide by 47"]
    DIVIDE_47 = 0x2e,
    #[doc = "Divide by 48"]
    DIVIDE_48 = 0x2f,
    #[doc = "Divide by 49"]
    DIVIDE_49 = 0x30,
    #[doc = "Divide by 50"]
    DIVIDE_50 = 0x31,
    #[doc = "Divide by 51"]
    DIVIDE_51 = 0x32,
    #[doc = "Divide by 52"]
    DIVIDE_52 = 0x33,
    #[doc = "Divide by 53"]
    DIVIDE_53 = 0x34,
    #[doc = "Divide by 54"]
    DIVIDE_54 = 0x35,
    #[doc = "Divide by 55"]
    DIVIDE_55 = 0x36,
    #[doc = "Divide by 56"]
    DIVIDE_56 = 0x37,
    #[doc = "Divide by 57"]
    DIVIDE_57 = 0x38,
    #[doc = "Divide by 58"]
    DIVIDE_58 = 0x39,
    #[doc = "Divide by 59"]
    DIVIDE_59 = 0x3a,
    #[doc = "Divide by 60"]
    DIVIDE_60 = 0x3b,
    #[doc = "Divide by 61"]
    DIVIDE_61 = 0x3c,
    #[doc = "Divide by 62"]
    DIVIDE_62 = 0x3d,
    #[doc = "Divide by 63"]
    DIVIDE_63 = 0x3e,
    #[doc = "Divide by 64"]
    DIVIDE_64 = 0x3f,
}
impl UartClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartClkPodf {
    #[inline(always)]
    fn from(val: u8) -> UartClkPodf {
        UartClkPodf::from_bits(val)
    }
}
impl From<UartClkPodf> for u8 {
    #[inline(always)]
    fn from(val: UartClkPodf) -> u8 {
        UartClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UartClkSel {
    #[doc = "derive clock from pll3_80m"]
    UART_CLK_SEL_0 = 0,
    #[doc = "derive clock from osc_clk"]
    UART_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from per_clk_root"]
    UART_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl UartClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartClkSel {
    #[inline(always)]
    fn from(val: u8) -> UartClkSel {
        UartClkSel::from_bits(val)
    }
}
impl From<UartClkSel> for u8 {
    #[inline(always)]
    fn from(val: UartClkSel) -> u8 {
        UartClkSel::to_bits(val)
    }
}
