#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodf {
    #[doc = "divide by 1"]
    AHB_PODF_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPodf {
    #[doc = "divide by 1"]
    ARM_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    ARM_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    ARM_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    ARM_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    ARM_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    ARM_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    ARM_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    ARM_PODF_7 = 0x07,
}
impl ArmPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPodf {
    #[inline(always)]
    fn from(val: u8) -> ArmPodf {
        ArmPodf::from_bits(val)
    }
}
impl From<ArmPodf> for u8 {
    #[inline(always)]
    fn from(val: ArmPodf) -> u8 {
        ArmPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
impl CanClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanClkPodf {
    #[inline(always)]
    fn from(val: u8) -> CanClkPodf {
        CanClkPodf::from_bits(val)
    }
}
impl From<CanClkPodf> for u8 {
    #[inline(always)]
    fn from(val: CanClkPodf) -> u8 {
        CanClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanClkSel {
    #[doc = "derive clock from pll3_sw_clk divided clock (60M)"]
    CAN_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk (24M)"]
    CAN_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from pll3_sw_clk divided clock (80M)"]
    CAN_CLK_SEL_2 = 0x02,
    #[doc = "Disable FlexCAN clock"]
    CAN_CLK_SEL_3 = 0x03,
}
impl CanClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanClkSel {
    #[inline(always)]
    fn from(val: u8) -> CanClkSel {
        CanClkSel::from_bits(val)
    }
}
impl From<CanClkSel> for u8 {
    #[inline(always)]
    fn from(val: CanClkSel) -> u8 {
        CanClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1div {
    #[doc = "divide by 1"]
    CLKO1_DIV_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1sel {
    #[doc = "USB1 PLL clock (divided by 2)"]
    CLKO1_SEL_0 = 0x0,
    #[doc = "SYS PLL clock (divided by 2)"]
    CLKO1_SEL_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "VIDEO PLL clock (divided by 2)"]
    CLKO1_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "semc_clk_root"]
    CLKO1_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "lcdif_pix_clk_root"]
    CLKO1_SEL_10 = 0x0a,
    #[doc = "ahb_clk_root"]
    CLKO1_SEL_11 = 0x0b,
    #[doc = "ipg_clk_root"]
    CLKO1_SEL_12 = 0x0c,
    #[doc = "perclk_root"]
    CLKO1_SEL_13 = 0x0d,
    #[doc = "ckil_sync_clk_root"]
    CLKO1_SEL_14 = 0x0e,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2div {
    #[doc = "divide by 1"]
    CLKO2_DIV_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "usdhc1_clk_root"]
    CLKO2_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "lpi2c_clk_root"]
    CLKO2_SEL_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "csi_clk_root"]
    CLKO2_SEL_11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "osc_clk"]
    CLKO2_SEL_14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "usdhc2_clk_root"]
    CLKO2_SEL_17 = 0x11,
    #[doc = "sai1_clk_root"]
    CLKO2_SEL_18 = 0x12,
    #[doc = "sai2_clk_root"]
    CLKO2_SEL_19 = 0x13,
    #[doc = "sai3_clk_root (shared with ADC1 and ADC2 alt_clk root)"]
    CLKO2_SEL_20 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "can_clk_root (FlexCAN, shared with CANFD)"]
    CLKO2_SEL_23 = 0x17,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiClkSel {
    #[doc = "derive clock from osc_clk (24M)"]
    CSI_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD2"]
    CSI_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from pll3_120M"]
    CSI_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL3 PFD1"]
    CSI_CLK_SEL_3 = 0x03,
}
impl CsiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiClkSel {
    #[inline(always)]
    fn from(val: u8) -> CsiClkSel {
        CsiClkSel::from_bits(val)
    }
}
impl From<CsiClkSel> for u8 {
    #[inline(always)]
    fn from(val: CsiClkSel) -> u8 {
        CsiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiPodf {
    #[doc = "divide by 1"]
    CSI_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    CSI_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    CSI_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    CSI_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    CSI_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    CSI_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    CSI_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    CSI_PODF_7 = 0x07,
}
impl CsiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiPodf {
    #[inline(always)]
    fn from(val: u8) -> CsiPodf {
        CsiPodf::from_bits(val)
    }
}
impl From<CsiPodf> for u8 {
    #[inline(always)]
    fn from(val: CsiPodf) -> u8 {
        CsiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
impl Flexio1clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1clkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1clkPred {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1clkSel {
    #[doc = "derive clock from PLL4"]
    FLEXIO1_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2"]
    FLEXIO1_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5"]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
impl Flexio2clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2clkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2clkPodf {
    #[inline(always)]
    fn from(val: u8) -> Flexio2clkPodf {
        Flexio2clkPodf::from_bits(val)
    }
}
impl From<Flexio2clkPodf> for u8 {
    #[inline(always)]
    fn from(val: Flexio2clkPodf) -> u8 {
        Flexio2clkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2clkPred {
    #[doc = "divide by 1"]
    FLEXIO2_CLK_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    FLEXIO2_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXIO2_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXIO2_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXIO2_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXIO2_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXIO2_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXIO2_CLK_PRED_7 = 0x07,
}
impl Flexio2clkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2clkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2clkPred {
    #[inline(always)]
    fn from(val: u8) -> Flexio2clkPred {
        Flexio2clkPred::from_bits(val)
    }
}
impl From<Flexio2clkPred> for u8 {
    #[inline(always)]
    fn from(val: Flexio2clkPred) -> u8 {
        Flexio2clkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2clkSel {
    #[doc = "derive clock from PLL4 divided clock"]
    FLEXIO2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2 clock"]
    FLEXIO2_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5 clock"]
    FLEXIO2_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk"]
    FLEXIO2_CLK_SEL_3 = 0x03,
}
impl Flexio2clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2clkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexio2clkSel {
        Flexio2clkSel::from_bits(val)
    }
}
impl From<Flexio2clkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexio2clkSel) -> u8 {
        Flexio2clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2clkSel {
    #[doc = "derive clock from PLL2 PFD2"]
    FLEXSPI2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD0"]
    FLEXSPI2_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL3 PFD1"]
    FLEXSPI2_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 (pll2_main_clk)"]
    FLEXSPI2_CLK_SEL_3 = 0x03,
}
impl Flexspi2clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2clkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2clkSel {
        Flexspi2clkSel::from_bits(val)
    }
}
impl From<Flexspi2clkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2clkSel) -> u8 {
        Flexspi2clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2podf {
    #[doc = "divide by 1"]
    FLEXSPI2_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    FLEXSPI2_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    FLEXSPI2_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    FLEXSPI2_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    FLEXSPI2_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    FLEXSPI2_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    FLEXSPI2_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    FLEXSPI2_PODF_7 = 0x07,
}
impl Flexspi2podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2podf {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2podf {
        Flexspi2podf::from_bits(val)
    }
}
impl From<Flexspi2podf> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2podf) -> u8 {
        Flexspi2podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiClkSel {
    #[doc = "derive clock from semc_clk_root_pre"]
    FLEXSPI_CLK_SEL_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiPodf {
    #[doc = "divide by 1"]
    FLEXSPI_PODF_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpgPodf {
    #[doc = "divide by 1"]
    IPG_PODF_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdifPodf {
    #[doc = "divide by 1"]
    LCDIF_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    LCDIF_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    LCDIF_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    LCDIF_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    LCDIF_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    LCDIF_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    LCDIF_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    LCDIF_PODF_7 = 0x07,
}
impl LcdifPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdifPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdifPodf {
    #[inline(always)]
    fn from(val: u8) -> LcdifPodf {
        LcdifPodf::from_bits(val)
    }
}
impl From<LcdifPodf> for u8 {
    #[inline(always)]
    fn from(val: LcdifPodf) -> u8 {
        LcdifPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdifPreClkSel {
    #[doc = "derive clock from PLL2"]
    LCDIF_PRE_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD3"]
    LCDIF_PRE_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5"]
    LCDIF_PRE_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from PLL2 PFD0"]
    LCDIF_PRE_CLK_SEL_3 = 0x03,
    #[doc = "derive clock from PLL2 PFD1"]
    LCDIF_PRE_CLK_SEL_4 = 0x04,
    #[doc = "derive clock from PLL3 PFD1"]
    LCDIF_PRE_CLK_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LcdifPreClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdifPreClkSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdifPreClkSel {
    #[inline(always)]
    fn from(val: u8) -> LcdifPreClkSel {
        LcdifPreClkSel::from_bits(val)
    }
}
impl From<LcdifPreClkSel> for u8 {
    #[inline(always)]
    fn from(val: LcdifPreClkSel) -> u8 {
        LcdifPreClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdifPred {
    #[doc = "divide by 1"]
    LCDIF_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    LCDIF_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    LCDIF_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    LCDIF_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    LCDIF_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    LCDIF_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    LCDIF_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    LCDIF_PRED_7 = 0x07,
}
impl LcdifPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdifPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdifPred {
    #[inline(always)]
    fn from(val: u8) -> LcdifPred {
        LcdifPred::from_bits(val)
    }
}
impl From<LcdifPred> for u8 {
    #[inline(always)]
    fn from(val: LcdifPred) -> u8 {
        LcdifPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpm {
    #[doc = "Remain in run mode"]
    LPM_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiClkSel {
    #[doc = "derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiPodf {
    #[doc = "divide by 1"]
    LPSPI_PODF_0 = 0x0,
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
}
impl LpspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClk2podf {
    #[doc = "divide by 1"]
    PERIPH_CLK2_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    PERIPH_CLK2_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    PERIPH_CLK2_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    PERIPH_CLK2_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    PERIPH_CLK2_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    PERIPH_CLK2_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    PERIPH_CLK2_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    PERIPH_CLK2_PODF_7 = 0x07,
}
impl PeriphClk2podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClk2podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClk2podf {
    #[inline(always)]
    fn from(val: u8) -> PeriphClk2podf {
        PeriphClk2podf::from_bits(val)
    }
}
impl From<PeriphClk2podf> for u8 {
    #[inline(always)]
    fn from(val: PeriphClk2podf) -> u8 {
        PeriphClk2podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClk2sel {
    #[doc = "derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0 = 0x0,
    #[doc = "derive clock from osc_clk (pll1_ref_clk)"]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrePeriphClkSel {
    #[doc = "derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL2 PFD2"]
    PRE_PERIPH_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL2 PFD0"]
    PRE_PERIPH_CLK_SEL_2 = 0x02,
    #[doc = "derive clock from divided PLL1"]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegBypassCount {
    #[doc = "no delay"]
    REG_BYPASS_COUNT_0 = 0x0,
    #[doc = "1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "63 CKIL clock periods delay"]
    REG_BYPASS_COUNT_63 = 0x3f,
}
impl RegBypassCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegBypassCount {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkPred {
    #[doc = "divide by 1"]
    SAI1_CLK_PRED_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI1_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL5"]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
impl Sai2clkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2clkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2clkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai2clkPodf {
        Sai2clkPodf::from_bits(val)
    }
}
impl From<Sai2clkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai2clkPodf) -> u8 {
        Sai2clkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2clkPred {
    #[doc = "divide by 1"]
    SAI2_CLK_PRED_0 = 0x0,
    #[doc = "divide by 2"]
    SAI2_CLK_PRED_1 = 0x01,
    #[doc = "divide by 3"]
    SAI2_CLK_PRED_2 = 0x02,
    #[doc = "divide by 4"]
    SAI2_CLK_PRED_3 = 0x03,
    #[doc = "divide by 5"]
    SAI2_CLK_PRED_4 = 0x04,
    #[doc = "divide by 6"]
    SAI2_CLK_PRED_5 = 0x05,
    #[doc = "divide by 7"]
    SAI2_CLK_PRED_6 = 0x06,
    #[doc = "divide by 8"]
    SAI2_CLK_PRED_7 = 0x07,
}
impl Sai2clkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2clkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2clkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai2clkPred {
        Sai2clkPred::from_bits(val)
    }
}
impl From<Sai2clkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai2clkPred) -> u8 {
        Sai2clkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2clkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI2_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL5"]
    SAI2_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL4"]
    SAI2_CLK_SEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai2clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2clkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2clkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai2clkSel {
        Sai2clkSel::from_bits(val)
    }
}
impl From<Sai2clkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai2clkSel) -> u8 {
        Sai2clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3clkPred {
    #[doc = "divide by 1"]
    SAI3_CLK_PRED_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3clkSel {
    #[doc = "derive clock from PLL3 PFD2"]
    SAI3_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL5"]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcPodf {
    #[doc = "divide by 1"]
    SEMC_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    SEMC_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    SEMC_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    SEMC_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    SEMC_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    SEMC_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    SEMC_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    SEMC_PODF_7 = 0x07,
}
impl SemcPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcPodf {
    #[inline(always)]
    fn from(val: u8) -> SemcPodf {
        SemcPodf::from_bits(val)
    }
}
impl From<SemcPodf> for u8 {
    #[inline(always)]
    fn from(val: SemcPodf) -> u8 {
        SemcPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0clkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0clkPred {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0clkSel {
    #[doc = "derive clock from PLL4"]
    SPDIF0_CLK_SEL_0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1 = 0x01,
    #[doc = "derive clock from PLL5"]
    SPDIF0_CLK_SEL_2 = 0x02,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StbyCount {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysMemDsCtrl {
    #[doc = "Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceClkSel {
    #[doc = "derive clock from PLL2"]
    TRACE_CLK_SEL_0 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TracePodf {
    #[doc = "divide by 1"]
    TRACE_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    TRACE_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    TRACE_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    TRACE_PODF_3 = 0x03,
}
impl TracePodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TracePodf {
        unsafe { core::mem::transmute(val & 0x03) }
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartClkPodf {
    #[doc = "Divide by 1"]
    DIVIDE_1 = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1podf {
    #[doc = "divide by 1"]
    USDHC1_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    USDHC1_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    USDHC1_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    USDHC1_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    USDHC1_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    USDHC1_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    USDHC1_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    USDHC1_PODF_7 = 0x07,
}
impl Usdhc1podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1podf {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1podf {
        Usdhc1podf::from_bits(val)
    }
}
impl From<Usdhc1podf> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1podf) -> u8 {
        Usdhc1podf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2podf {
    #[doc = "divide by 1"]
    USDHC2_PODF_0 = 0x0,
    #[doc = "divide by 2"]
    USDHC2_PODF_1 = 0x01,
    #[doc = "divide by 3"]
    USDHC2_PODF_2 = 0x02,
    #[doc = "divide by 4"]
    USDHC2_PODF_3 = 0x03,
    #[doc = "divide by 5"]
    USDHC2_PODF_4 = 0x04,
    #[doc = "divide by 6"]
    USDHC2_PODF_5 = 0x05,
    #[doc = "divide by 7"]
    USDHC2_PODF_6 = 0x06,
    #[doc = "divide by 8"]
    USDHC2_PODF_7 = 0x07,
}
impl Usdhc2podf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2podf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2podf {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2podf {
        Usdhc2podf::from_bits(val)
    }
}
impl From<Usdhc2podf> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2podf) -> u8 {
        Usdhc2podf::to_bits(val)
    }
}
