#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanfdIppIndCanrxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT9"]
    GPIO_EMC_37_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT8"]
    GPIO_AD_B0_15_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT8"]
    GPIO_AD_B0_11_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CanfdIppIndCanrxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanfdIppIndCanrxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanfdIppIndCanrxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CanfdIppIndCanrxSelectInputDaisy {
        CanfdIppIndCanrxSelectInputDaisy::from_bits(val)
    }
}
impl From<CanfdIppIndCanrxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CanfdIppIndCanrxSelectInputDaisy) -> u8 {
        CanfdIppIndCanrxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcmPmicReadySelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT6"]
    GPIO_SD_B1_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_12 for Mode: ALT1"]
    GPIO_AD_B0_12_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_01 for Mode: ALT4"]
    GPIO_AD_B1_01_ALT4 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B1_08 for Mode: ALT3"]
    GPIO_AD_B1_08_ALT3 = 0x03,
    #[doc = "Selecting Pad: GPIO_EMC_32 for Mode: ALT3"]
    GPIO_EMC_32_ALT3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CcmPmicReadySelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcmPmicReadySelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcmPmicReadySelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CcmPmicReadySelectInputDaisy {
        CcmPmicReadySelectInputDaisy::from_bits(val)
    }
}
impl From<CcmPmicReadySelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CcmPmicReadySelectInputDaisy) -> u8 {
        CcmPmicReadySelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiHsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT4"]
    GPIO_AD_B0_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT4"]
    GPIO_AD_B1_07_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_14 for Mode: ALT2"]
    GPIO_B1_14_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CsiHsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiHsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiHsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiHsyncSelectInputDaisy {
        CsiHsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<CsiHsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiHsyncSelectInputDaisy) -> u8 {
        CsiHsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiVsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_14 for Mode: ALT4"]
    GPIO_AD_B0_14_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT4"]
    GPIO_AD_B1_06_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT2"]
    GPIO_B1_13_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CsiVsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiVsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiVsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiVsyncSelectInputDaisy {
        CsiVsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<CsiVsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiVsyncSelectInputDaisy) -> u8 {
        CsiVsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet0timerSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT3"]
    GPIO_AD_B0_15_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT7"]
    GPIO_AD_B0_11_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT3"]
    GPIO_B1_12_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet0timerSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet0timerSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet0timerSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet0timerSelectInputDaisy {
        Enet0timerSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet0timerSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet0timerSelectInputDaisy) -> u8 {
        Enet0timerSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ipgClkRmiiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT9"]
    GPIO_EMC_33_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT9"]
    GPIO_SD_B0_01_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_15 for Mode: ALT9"]
    GPIO_B0_15_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2ipgClkRmiiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ipgClkRmiiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ipgClkRmiiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2ipgClkRmiiSelectInputDaisy {
        Enet2ipgClkRmiiSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2ipgClkRmiiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2ipgClkRmiiSelectInputDaisy) -> u8 {
        Enet2ipgClkRmiiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ippIndMac0rxdataSelectInput0daisy {
    #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT8"]
    GPIO_EMC_35_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_03 for Mode: ALT8"]
    GPIO_SD_B0_03_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT8"]
    GPIO_B1_01_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2ippIndMac0rxdataSelectInput0daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ippIndMac0rxdataSelectInput0daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ippIndMac0rxdataSelectInput0daisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2ippIndMac0rxdataSelectInput0daisy {
        Enet2ippIndMac0rxdataSelectInput0daisy::from_bits(val)
    }
}
impl From<Enet2ippIndMac0rxdataSelectInput0daisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2ippIndMac0rxdataSelectInput0daisy) -> u8 {
        Enet2ippIndMac0rxdataSelectInput0daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ippIndMac0rxdataSelectInput1daisy {
    #[doc = "Selecting Pad: GPIO_EMC_36 for Mode: ALT8"]
    GPIO_EMC_36_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_04 for Mode: ALT8"]
    GPIO_SD_B0_04_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT8"]
    GPIO_B1_02_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2ippIndMac0rxdataSelectInput1daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ippIndMac0rxdataSelectInput1daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ippIndMac0rxdataSelectInput1daisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2ippIndMac0rxdataSelectInput1daisy {
        Enet2ippIndMac0rxdataSelectInput1daisy::from_bits(val)
    }
}
impl From<Enet2ippIndMac0rxdataSelectInput1daisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2ippIndMac0rxdataSelectInput1daisy) -> u8 {
        Enet2ippIndMac0rxdataSelectInput1daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ippIndMac0rxenSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT8"]
    GPIO_EMC_37_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_05 for Mode: ALT8"]
    GPIO_SD_B0_05_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT8"]
    GPIO_B1_03_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2ippIndMac0rxenSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ippIndMac0rxenSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ippIndMac0rxenSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2ippIndMac0rxenSelectInputDaisy {
        Enet2ippIndMac0rxenSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2ippIndMac0rxenSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2ippIndMac0rxenSelectInputDaisy) -> u8 {
        Enet2ippIndMac0rxenSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ippIndMac0rxerrSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_34 for Mode: ALT8"]
    GPIO_EMC_34_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_02 for Mode: ALT8"]
    GPIO_SD_B0_02_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT8"]
    GPIO_B1_00_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2ippIndMac0rxerrSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ippIndMac0rxerrSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ippIndMac0rxerrSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2ippIndMac0rxerrSelectInputDaisy {
        Enet2ippIndMac0rxerrSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2ippIndMac0rxerrSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2ippIndMac0rxerrSelectInputDaisy) -> u8 {
        Enet2ippIndMac0rxerrSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ippIndMac0txclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT8"]
    GPIO_EMC_33_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT8"]
    GPIO_SD_B0_01_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_15 for Mode: ALT8"]
    GPIO_B0_15_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2ippIndMac0txclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ippIndMac0txclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ippIndMac0txclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2ippIndMac0txclkSelectInputDaisy {
        Enet2ippIndMac0txclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2ippIndMac0txclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2ippIndMac0txclkSelectInputDaisy) -> u8 {
        Enet2ippIndMac0txclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetMdioSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_05 for Mode: ALT1"]
    GPIO_AD_B1_05_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_41 for Mode: ALT4"]
    GPIO_EMC_41_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_15 for Mode: ALT0"]
    GPIO_B1_15_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EnetMdioSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetMdioSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetMdioSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EnetMdioSelectInputDaisy {
        EnetMdioSelectInputDaisy::from_bits(val)
    }
}
impl From<EnetMdioSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EnetMdioSelectInputDaisy) -> u8 {
        EnetMdioSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1rxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT4"]
    GPIO_SD_B1_03_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT3"]
    GPIO_EMC_18_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT2"]
    GPIO_AD_B1_09_ALT2 = 0x02,
    #[doc = "Selecting Pad: GPIO_B0_03 for Mode: ALT2"]
    GPIO_B0_03_ALT2 = 0x03,
}
impl Flexcan1rxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1rxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1rxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1rxSelectInputDaisy {
        Flexcan1rxSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexcan1rxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1rxSelectInputDaisy) -> u8 {
        Flexcan1rxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan2rxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT3"]
    GPIO_EMC_10_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT0"]
    GPIO_AD_B0_03_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT6"]
    GPIO_AD_B0_15_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT6"]
    GPIO_B1_09_ALT6 = 0x03,
}
impl Flexcan2rxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan2rxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan2rxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexcan2rxSelectInputDaisy {
        Flexcan2rxSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexcan2rxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexcan2rxSelectInputDaisy) -> u8 {
        Flexcan2rxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1pwma3selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT2"]
    GPIO_SD_B1_00_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT4"]
    GPIO_EMC_12_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_38 for Mode: ALT1"]
    GPIO_EMC_38_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B0_10 for Mode: ALT1"]
    GPIO_AD_B0_10_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT6"]
    GPIO_B1_00_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flexpwm1pwma3selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1pwma3selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1pwma3selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1pwma3selectInputDaisy {
        Flexpwm1pwma3selectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1pwma3selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1pwma3selectInputDaisy) -> u8 {
        Flexpwm1pwma3selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1pwmb3selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    GPIO_SD_B1_01_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    GPIO_EMC_13_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    GPIO_EMC_39_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    GPIO_AD_B0_11_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    GPIO_B1_01_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flexpwm1pwmb3selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1pwmb3selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1pwmb3selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1pwmb3selectInputDaisy {
        Flexpwm1pwmb3selectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1pwmb3selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1pwmb3selectInputDaisy) -> u8 {
        Flexpwm1pwmb3selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2pwma3selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT2"]
    GPIO_SD_B1_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT1"]
    GPIO_EMC_19_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_00 for Mode: ALT0"]
    GPIO_AD_B0_00_ALT0 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT1"]
    GPIO_AD_B0_09_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT6"]
    GPIO_B1_02_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flexpwm2pwma3selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2pwma3selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2pwma3selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2pwma3selectInputDaisy {
        Flexpwm2pwma3selectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2pwma3selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2pwma3selectInputDaisy) -> u8 {
        Flexpwm2pwma3selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2pwmb3selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT2"]
    GPIO_SD_B1_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT1"]
    GPIO_EMC_20_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_01 for Mode: ALT0"]
    GPIO_AD_B0_01_ALT0 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT6"]
    GPIO_B1_03_ALT6 = 0x03,
}
impl Flexpwm2pwmb3selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2pwmb3selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2pwmb3selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2pwmb3selectInputDaisy {
        Flexpwm2pwmb3selectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2pwmb3selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2pwmb3selectInputDaisy) -> u8 {
        Flexpwm2pwmb3selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ippIndDqsFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_00 for Mode: ALT0"]
    GPIO_SPI_B1_00_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT8"]
    GPIO_EMC_23_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_09 for Mode: ALT0"]
    GPIO_SPI_B0_09_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2ippIndDqsFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ippIndDqsFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ippIndDqsFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ippIndDqsFaSelectInputDaisy {
        Flexspi2ippIndDqsFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2ippIndDqsFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ippIndDqsFaSelectInputDaisy) -> u8 {
        Flexspi2ippIndDqsFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ippIndIoFaBit0selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_04 for Mode: ALT0"]
    GPIO_SPI_B1_04_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT8"]
    GPIO_EMC_26_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_02 for Mode: ALT0"]
    GPIO_SPI_B0_02_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2ippIndIoFaBit0selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ippIndIoFaBit0selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ippIndIoFaBit0selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ippIndIoFaBit0selectInputDaisy {
        Flexspi2ippIndIoFaBit0selectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2ippIndIoFaBit0selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ippIndIoFaBit0selectInputDaisy) -> u8 {
        Flexspi2ippIndIoFaBit0selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ippIndIoFaBit1selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_03 for Mode: ALT0"]
    GPIO_SPI_B1_03_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT8"]
    GPIO_EMC_27_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_12 for Mode: ALT0"]
    GPIO_SPI_B0_12_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2ippIndIoFaBit1selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ippIndIoFaBit1selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ippIndIoFaBit1selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ippIndIoFaBit1selectInputDaisy {
        Flexspi2ippIndIoFaBit1selectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2ippIndIoFaBit1selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ippIndIoFaBit1selectInputDaisy) -> u8 {
        Flexspi2ippIndIoFaBit1selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ippIndIoFaBit2selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_02 for Mode: ALT0"]
    GPIO_SPI_B1_02_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_28 for Mode: ALT8"]
    GPIO_EMC_28_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_06 for Mode: ALT0"]
    GPIO_SPI_B0_06_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2ippIndIoFaBit2selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ippIndIoFaBit2selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ippIndIoFaBit2selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ippIndIoFaBit2selectInputDaisy {
        Flexspi2ippIndIoFaBit2selectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2ippIndIoFaBit2selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ippIndIoFaBit2selectInputDaisy) -> u8 {
        Flexspi2ippIndIoFaBit2selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ippIndIoFaBit3selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_01 for Mode: ALT0"]
    GPIO_SPI_B1_01_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_29 for Mode: ALT8"]
    GPIO_EMC_29_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_10 for Mode: ALT0"]
    GPIO_SPI_B0_10_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2ippIndIoFaBit3selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ippIndIoFaBit3selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ippIndIoFaBit3selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ippIndIoFaBit3selectInputDaisy {
        Flexspi2ippIndIoFaBit3selectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2ippIndIoFaBit3selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ippIndIoFaBit3selectInputDaisy) -> u8 {
        Flexspi2ippIndIoFaBit3selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2ippIndSckFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_05 for Mode: ALT0"]
    GPIO_SPI_B1_05_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT8"]
    GPIO_EMC_25_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_08 for Mode: ALT0"]
    GPIO_SPI_B0_08_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2ippIndSckFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2ippIndSckFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2ippIndSckFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2ippIndSckFaSelectInputDaisy {
        Flexspi2ippIndSckFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2ippIndSckFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2ippIndSckFaSelectInputDaisy) -> u8 {
        Flexspi2ippIndSckFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioDse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB000 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_CLK of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MQS_RIGHT of instance: mqs"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO00 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SEMC_CSX01 of instance: semc"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_MDC of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB000 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB000 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB000 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB000 {
        GpioMuxModeB000::from_bits(val)
    }
}
impl From<GpioMuxModeB000> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB000) -> u8 {
        GpioMuxModeB000::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB001 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_ENABLE of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MQS_LEFT of instance: mqs"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SDI of instance: lpspi4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO01 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SEMC_CSX02 of instance: semc"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_MDIO of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB001 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB001 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB001 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB001 {
        GpioMuxModeB001::from_bits(val)
    }
}
impl From<GpioMuxModeB001> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB001) -> u8 {
        GpioMuxModeB001::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB002 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_HSYNC of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER2 of instance: qtimer1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXCAN1_TX of instance: flexcan1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SDO of instance: lpspi4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO02 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SEMC_CSX03 of instance: semc"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_OUT of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB002 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB002 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB002 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB002 {
        GpioMuxModeB002::from_bits(val)
    }
}
impl From<GpioMuxModeB002> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB002) -> u8 {
        GpioMuxModeB002::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB003 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_VSYNC of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXCAN1_RX of instance: flexcan1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SCK of instance: lpspi4"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO03 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: WDOG2_RESET_B_DEB of instance: wdog2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_IN of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB003 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB003 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB003 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB003 {
        GpioMuxModeB003::from_bits(val)
    }
}
impl From<GpioMuxModeB003> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB003) -> u8 {
        GpioMuxModeB003::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB004 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA00 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C2_SCL of instance: lpi2c2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ARM_TRACE0 of instance: cm7_mx6rt"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO04 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG00 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA03 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB004 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB004 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB004 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB004 {
        GpioMuxModeB004::from_bits(val)
    }
}
impl From<GpioMuxModeB004> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB004) -> u8 {
        GpioMuxModeB004::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB005 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA01 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER2_TIMER2 of instance: qtimer2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C2_SDA of instance: lpi2c2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ARM_TRACE1 of instance: cm7_mx6rt"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO05 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG01 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA02 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB005 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB005 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB005 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB005 {
        GpioMuxModeB005::from_bits(val)
    }
}
impl From<GpioMuxModeB005> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB005) -> u8 {
        GpioMuxModeB005::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB006 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA02 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ARM_TRACE2 of instance: cm7_mx6rt"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO06 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG02 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_CLK of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB006 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB006 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB006 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB006 {
        GpioMuxModeB006::from_bits(val)
    }
}
impl From<GpioMuxModeB006> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB006) -> u8 {
        GpioMuxModeB006::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB007 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA03 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ARM_TRACE3 of instance: cm7_mx6rt"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO07 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG03 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_ER of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB007 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB007 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB007 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB007 {
        GpioMuxModeB007::from_bits(val)
    }
}
impl From<GpioMuxModeB007> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB007) -> u8 {
        GpioMuxModeB007::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB008 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA04 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_TX of instance: lpuart3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO08 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG04 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA03 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB008 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB008 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB008 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB008 {
        GpioMuxModeB008::from_bits(val)
    }
}
impl From<GpioMuxModeB008> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB008) -> u8 {
        GpioMuxModeB008::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB009 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA05 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RX of instance: lpuart3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO09 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG05 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA02 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB009 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB009 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB009 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB009 {
        GpioMuxModeB009::from_bits(val)
    }
}
impl From<GpioMuxModeB009> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB009) -> u8 {
        GpioMuxModeB009::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB010 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA06 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA03 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO10 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG06 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_CRS of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB010 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB010 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB010 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB010 {
        GpioMuxModeB010::from_bits(val)
    }
}
impl From<GpioMuxModeB010> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB010) -> u8 {
        GpioMuxModeB010::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB011 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA07 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER4_TIMER2 of instance: qtimer4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA02 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO11 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG07 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_COL of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB011 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB011 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB011 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB011 {
        GpioMuxModeB011::from_bits(val)
    }
}
impl From<GpioMuxModeB011> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB011) -> u8 {
        GpioMuxModeB011::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB012 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA08 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT10 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_TRACE_CLK of instance: cm7_mx6rt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO12 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG08 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA00 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB012 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB012 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB012 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB012 {
        GpioMuxModeB012::from_bits(val)
    }
}
impl From<GpioMuxModeB012> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB012) -> u8 {
        GpioMuxModeB012::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB013 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA09 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT11 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_TRACE_SWO of instance: cm7_mx6rt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO13 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG09 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA01 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB013 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB013 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB013 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB013 {
        GpioMuxModeB013::from_bits(val)
    }
}
impl From<GpioMuxModeB013> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB013) -> u8 {
        GpioMuxModeB013::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB014 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA10 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT12 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_TXEV of instance: cm7_mx6rt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO14 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO14 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG10 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_EN of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB014 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB014 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB014 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB014 {
        GpioMuxModeB014::from_bits(val)
    }
}
impl From<GpioMuxModeB014> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB014) -> u8 {
        GpioMuxModeB014::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB015 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA11 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT13 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ARM_RXEV of instance: cm7_mx6rt"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO15 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO15 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SRC_BOOT_CFG11 of instance: src"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_CLK of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: ENET2_REF_CLK2 of instance: enet2"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB015 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB015 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB015 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB015 {
        GpioMuxModeB015::from_bits(val)
    }
}
impl From<GpioMuxModeB015> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB015) -> u8 {
        GpioMuxModeB015::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB100 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA12 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT14 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO16 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO16 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_ER of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO16 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB100 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB100 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB100 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB100 {
        GpioMuxModeB100::from_bits(val)
    }
}
impl From<GpioMuxModeB100> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB100) -> u8 {
        GpioMuxModeB100::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB101 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA13 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT15 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO17 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO17 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA00 of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO17 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB101 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB101 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB101 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB101 {
        GpioMuxModeB101::from_bits(val)
    }
}
impl From<GpioMuxModeB101> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB101) -> u8 {
        GpioMuxModeB101::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB102 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA14 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI4_PCS2 of instance: lpspi4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO18 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO18 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO18 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB102 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB102 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB102 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB102 {
        GpioMuxModeB102::from_bits(val)
    }
}
impl From<GpioMuxModeB102> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB102) -> u8 {
        GpioMuxModeB102::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB103 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA15 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT17 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI4_PCS1 of instance: lpspi4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO19 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO19 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_EN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO19 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB103 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB103 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB103 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB103 {
        GpioMuxModeB103::from_bits(val)
    }
}
impl From<GpioMuxModeB103> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB103) -> u8 {
        GpioMuxModeB103::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB104 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA16 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA15 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RX_DATA00 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO20 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO20 of instance: gpio2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT1_CLK of instance: gpt1"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO20 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB104 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB104 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB104 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB104 {
        GpioMuxModeB104::from_bits(val)
    }
}
impl From<GpioMuxModeB104> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB104) -> u8 {
        GpioMuxModeB104::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB105 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA17 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI4_SDI of instance: lpspi4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA14 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RX_DATA01 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO21 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO21 of instance: gpio2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT1_CAPTURE1 of instance: gpt1"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO21 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB105 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB105 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB105 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB105 {
        GpioMuxModeB105::from_bits(val)
    }
}
impl From<GpioMuxModeB105> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB105) -> u8 {
        GpioMuxModeB105::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB106 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA18 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI4_SDO of instance: lpspi4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA13 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RX_EN of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO22 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO22 of instance: gpio2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT1_CAPTURE2 of instance: gpt1"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO22 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB106 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB106 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB106 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB106 {
        GpioMuxModeB106::from_bits(val)
    }
}
impl From<GpioMuxModeB106> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB106) -> u8 {
        GpioMuxModeB106::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB107 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA19 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI4_SCK of instance: lpspi4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA12 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TX_DATA00 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO23 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO23 of instance: gpio2"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT1_COMPARE1 of instance: gpt1"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO23 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB107 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB107 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB107 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB107 {
        GpioMuxModeB107::from_bits(val)
    }
}
impl From<GpioMuxModeB107> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB107) -> u8 {
        GpioMuxModeB107::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB108 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA20 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER1_TIMER3 of instance: qtimer1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA11 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TX_DATA01 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO24 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO24 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXCAN2_TX of instance: flexcan2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT1_COMPARE2 of instance: gpt1"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO24 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB108 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB108 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB108 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB108 {
        GpioMuxModeB108::from_bits(val)
    }
}
impl From<GpioMuxModeB108> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB108) -> u8 {
        GpioMuxModeB108::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB109 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA21 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER2_TIMER3 of instance: qtimer2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA10 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TX_EN of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO25 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO25 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXCAN2_RX of instance: flexcan2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT1_COMPARE3 of instance: gpt1"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO25 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB109 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB109 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB109 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB109 {
        GpioMuxModeB109::from_bits(val)
    }
}
impl From<GpioMuxModeB109> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB109) -> u8 {
        GpioMuxModeB109::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB110 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA22 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER3 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA00 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TX_CLK of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO26 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO26 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ENET_REF_CLK of instance: enet"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO26 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB110 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB110 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB110 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB110 {
        GpioMuxModeB110::from_bits(val)
    }
}
impl From<GpioMuxModeB110> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB110) -> u8 {
        GpioMuxModeB110::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB111 {
    #[doc = "Select mux mode: ALT0 mux port: LCD_DATA23 of instance: lcdif"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER4_TIMER3 of instance: qtimer4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_DATA01 of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RX_ER of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO27 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO27 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI4_PCS3 of instance: lpspi4"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO27 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB111 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB111 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB111 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB111 {
        GpioMuxModeB111::from_bits(val)
    }
}
impl From<GpioMuxModeB111> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB111) -> u8 {
        GpioMuxModeB111::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB112 {
    _RESERVED_0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART5_TX of instance: lpuart5"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_PIXCLK of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_1588_EVENT0_IN of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO28 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO28 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_CD_B of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO28 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB112 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB112 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB112 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB112 {
        GpioMuxModeB112::from_bits(val)
    }
}
impl From<GpioMuxModeB112> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB112) -> u8 {
        GpioMuxModeB112::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB113 {
    #[doc = "Select mux mode: ALT0 mux port: WDOG1_B of instance: wdog1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART5_RX of instance: lpuart5"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_VSYNC of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_1588_EVENT0_OUT of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO29 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO29 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SEMC_DQS4 of instance: semc"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO29 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB113 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB113 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB113 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB113 {
        GpioMuxModeB113::from_bits(val)
    }
}
impl From<GpioMuxModeB113> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB113) -> u8 {
        GpioMuxModeB113::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB114 {
    #[doc = "Select mux mode: ALT0 mux port: ENET_MDC of instance: enet"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA02 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_HSYNC of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_IN02 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO30 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO30 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_VSELECT of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA00 of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO30 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB114 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB114 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB114 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB114 {
        GpioMuxModeB114::from_bits(val)
    }
}
impl From<GpioMuxModeB114> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB114) -> u8 {
        GpioMuxModeB114::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioMuxModeB115 {
    #[doc = "Select mux mode: ALT0 mux port: ENET_MDIO of instance: enet"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA03 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CSI_MCLK of instance: csi"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_IN03 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO31 of instance: flexio2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO31 of instance: gpio2"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_RESET_B of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA01 of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO31 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl GpioMuxModeB115 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioMuxModeB115 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioMuxModeB115 {
    #[inline(always)]
    fn from(val: u8) -> GpioMuxModeB115 {
        GpioMuxModeB115::from_bits(val)
    }
}
impl From<GpioMuxModeB115> for u8 {
    #[inline(always)]
    fn from(val: GpioMuxModeB115) -> u8 {
        GpioMuxModeB115::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSpeed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3sclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_22 for Mode: ALT2"]
    GPIO_EMC_22_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT2"]
    GPIO_SD_B0_00_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT1"]
    GPIO_AD_B1_07_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c3sclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3sclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3sclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3sclSelectInputDaisy {
        Lpi2c3sclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c3sclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3sclSelectInputDaisy) -> u8 {
        Lpi2c3sclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3sdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_21 for Mode: ALT2"]
    GPIO_EMC_21_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT2"]
    GPIO_SD_B0_01_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT1"]
    GPIO_AD_B1_06_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c3sdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3sdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3sdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3sdaSelectInputDaisy {
        Lpi2c3sdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c3sdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3sdaSelectInputDaisy) -> u8 {
        Lpi2c3sdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3rxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT2"]
    GPIO_AD_B1_07_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_14 for Mode: ALT2"]
    GPIO_EMC_14_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_09 for Mode: ALT3"]
    GPIO_B0_09_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3rxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3rxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3rxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3rxSelectInputDaisy {
        Lpuart3rxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3rxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3rxSelectInputDaisy) -> u8 {
        Lpuart3rxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3txSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT2"]
    GPIO_AD_B1_06_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT2"]
    GPIO_EMC_13_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_08 for Mode: ALT3"]
    GPIO_B0_08_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3txSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3txSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3txSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3txSelectInputDaisy {
        Lpuart3txSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3txSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3txSelectInputDaisy) -> u8 {
        Lpuart3txSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4rxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT4"]
    GPIO_SD_B1_01_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT2"]
    GPIO_EMC_20_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT2"]
    GPIO_B1_01_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart4rxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4rxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4rxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4rxSelectInputDaisy {
        Lpuart4rxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4rxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4rxSelectInputDaisy) -> u8 {
        Lpuart4rxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4txSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT4"]
    GPIO_SD_B1_00_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT2"]
    GPIO_EMC_19_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT2"]
    GPIO_B1_00_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart4txSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4txSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4txSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4txSelectInputDaisy {
        Lpuart4txSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4txSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4txSelectInputDaisy) -> u8 {
        Lpuart4txSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8rxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_05 for Mode: ALT2"]
    GPIO_SD_B0_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT2"]
    GPIO_AD_B1_11_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT2"]
    GPIO_EMC_39_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart8rxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8rxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8rxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8rxSelectInputDaisy {
        Lpuart8rxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8rxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8rxSelectInputDaisy) -> u8 {
        Lpuart8rxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8txSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_04 for Mode: ALT2"]
    GPIO_SD_B0_04_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT2"]
    GPIO_AD_B1_10_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_38 for Mode: ALT2"]
    GPIO_EMC_38_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart8txSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8txSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8txSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8txSelectInputDaisy {
        Lpuart8txSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8txSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8txSelectInputDaisy) -> u8 {
        Lpuart8txSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3timer0selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_15 for Mode: ALT4"]
    GPIO_EMC_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_00 for Mode: ALT1"]
    GPIO_AD_B1_00_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_06 for Mode: ALT1"]
    GPIO_B0_06_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3timer0selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3timer0selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3timer0selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3timer0selectInputDaisy {
        Qtimer3timer0selectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3timer0selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3timer0selectInputDaisy) -> u8 {
        Qtimer3timer0selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3timer1selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_01 for Mode: ALT1"]
    GPIO_AD_B1_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_16 for Mode: ALT4"]
    GPIO_EMC_16_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_07 for Mode: ALT1"]
    GPIO_B0_07_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3timer1selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3timer1selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3timer1selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3timer1selectInputDaisy {
        Qtimer3timer1selectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3timer1selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3timer1selectInputDaisy) -> u8 {
        Qtimer3timer1selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3timer2selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_17 for Mode: ALT4"]
    GPIO_EMC_17_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT1"]
    GPIO_AD_B1_02_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_08 for Mode: ALT1"]
    GPIO_B0_08_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3timer2selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3timer2selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3timer2selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3timer2selectInputDaisy {
        Qtimer3timer2selectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3timer2selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3timer2selectInputDaisy) -> u8 {
        Qtimer3timer2selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3timer3selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT4"]
    GPIO_EMC_18_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_03 for Mode: ALT1"]
    GPIO_AD_B1_03_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT1"]
    GPIO_B1_10_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3timer3selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3timer3selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3timer3selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3timer3selectInputDaisy {
        Qtimer3timer3selectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3timer3selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3timer3selectInputDaisy) -> u8 {
        Qtimer3timer3selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1mclk2selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT3"]
    GPIO_SD_B1_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT3"]
    GPIO_AD_B1_09_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_13 for Mode: ALT3"]
    GPIO_B0_13_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1mclk2selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1mclk2selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1mclk2selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1mclk2selectInputDaisy {
        Sai1mclk2selectInputDaisy::from_bits(val)
    }
}
impl From<Sai1mclk2selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1mclk2selectInputDaisy) -> u8 {
        Sai1mclk2selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1rxBclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT3"]
    GPIO_SD_B1_05_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT3"]
    GPIO_AD_B1_11_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_15 for Mode: ALT3"]
    GPIO_B0_15_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1rxBclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1rxBclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1rxBclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1rxBclkSelectInputDaisy {
        Sai1rxBclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1rxBclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1rxBclkSelectInputDaisy) -> u8 {
        Sai1rxBclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1rxData0selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT3"]
    GPIO_SD_B1_06_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT3"]
    GPIO_AD_B1_12_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT3"]
    GPIO_B1_00_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1rxData0selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1rxData0selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1rxData0selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1rxData0selectInputDaisy {
        Sai1rxData0selectInputDaisy::from_bits(val)
    }
}
impl From<Sai1rxData0selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1rxData0selectInputDaisy) -> u8 {
        Sai1rxData0selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1rxSyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT3"]
    GPIO_SD_B1_04_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT3"]
    GPIO_AD_B1_10_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_14 for Mode: ALT3"]
    GPIO_B0_14_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1rxSyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1rxSyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1rxSyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1rxSyncSelectInputDaisy {
        Sai1rxSyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1rxSyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1rxSyncSelectInputDaisy) -> u8 {
        Sai1rxSyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1txBclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT3"]
    GPIO_SD_B1_08_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT3"]
    GPIO_AD_B1_14_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT3"]
    GPIO_B1_02_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1txBclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1txBclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1txBclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1txBclkSelectInputDaisy {
        Sai1txBclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1txBclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1txBclkSelectInputDaisy) -> u8 {
        Sai1txBclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1txSyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT3"]
    GPIO_SD_B1_09_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_15 for Mode: ALT3"]
    GPIO_AD_B1_15_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT3"]
    GPIO_B1_03_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1txSyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1txSyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1txSyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1txSyncSelectInputDaisy {
        Sai1txSyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1txSyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1txSyncSelectInputDaisy) -> u8 {
        Sai1txSyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcIippIndDqs4selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT9"]
    GPIO_SD_B0_00_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT9"]
    GPIO_EMC_39_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT9"]
    GPIO_AD_B0_09_ALT9 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT8"]
    GPIO_B1_13_ALT8 = 0x03,
}
impl SemcIippIndDqs4selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcIippIndDqs4selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcIippIndDqs4selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> SemcIippIndDqs4selectInputDaisy {
        SemcIippIndDqs4selectInputDaisy::from_bits(val)
    }
}
impl From<SemcIippIndDqs4selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: SemcIippIndDqs4selectInputDaisy) -> u8 {
        SemcIippIndDqs4selectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB000muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT14 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: REF_CLK_32K of instance: xtalosc"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG2_ID of instance: usb"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPI2C1_SCLS of instance: lpi2c1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_RESET_B of instance: usdhc1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SCK of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB000muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB000muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB000muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB000muxMode {
        SwMuxCtlPadGpioAdB000muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB000muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB000muxMode) -> u8 {
        SwMuxCtlPadGpioAdB000muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB001muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT15 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: REF_CLK_24M of instance: anatop"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_ID of instance: anatop"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPI2C1_SDAS of instance: lpi2c1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO01 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: EWM_OUT_B of instance: ewm"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDO of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB001muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB001muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB001muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB001muxMode {
        SwMuxCtlPadGpioAdB001muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB001muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB001muxMode) -> u8 {
        SwMuxCtlPadGpioAdB001muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB002muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXCAN2_TX of instance: flexcan2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: usb"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX00 of instance: flexpwm1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: lpi2c1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDI of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB002muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB002muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB002muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB002muxMode {
        SwMuxCtlPadGpioAdB002muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB002muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB002muxMode) -> u8 {
        SwMuxCtlPadGpioAdB002muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB003muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXCAN2_RX of instance: flexcan2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT17 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RX of instance: lpuart6"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_OC of instance: usb"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX01 of instance: flexpwm1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO03 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: REF_CLK_24M of instance: anatop"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB003muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB003muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB003muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB003muxMode {
        SwMuxCtlPadGpioAdB003muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB003muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB003muxMode) -> u8 {
        SwMuxCtlPadGpioAdB003muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB004muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SRC_BOOT_MODE00 of instance: src"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: MQS_RIGHT of instance: mqs"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_TX_DATA03 of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_TX_SYNC of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA09 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO04 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: PIT_TRIGGER00 of instance: pit"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB004muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB004muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB004muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB004muxMode {
        SwMuxCtlPadGpioAdB004muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB004muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB004muxMode) -> u8 {
        SwMuxCtlPadGpioAdB004muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB005muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SRC_BOOT_MODE01 of instance: src"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: MQS_LEFT of instance: mqs"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_TX_DATA02 of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_TX_BCLK of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA08 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO05 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_INOUT17 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS2 of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB005muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB005muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB005muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB005muxMode {
        SwMuxCtlPadGpioAdB005muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB005muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB005muxMode) -> u8 {
        SwMuxCtlPadGpioAdB005muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB006muxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_TMS of instance: jtag_mux"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: GPT2_COMPARE1 of instance: gpt2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_RX_CLK of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_RX_BCLK of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA07 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO06 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_INOUT18 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS3 of instance: lpspi3"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB006muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB006muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB006muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB006muxMode {
        SwMuxCtlPadGpioAdB006muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB006muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB006muxMode) -> u8 {
        SwMuxCtlPadGpioAdB006muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB007muxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_TCK of instance: jtag_mux"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: GPT2_COMPARE2 of instance: gpt2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_TX_ER of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_RX_SYNC of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA06 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO07 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_INOUT19 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ENET_1588_EVENT3_OUT of instance: enet"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB007muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB007muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB007muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB007muxMode {
        SwMuxCtlPadGpioAdB007muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB007muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB007muxMode) -> u8 {
        SwMuxCtlPadGpioAdB007muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB008muxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_MOD of instance: jtag_mux"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: GPT2_COMPARE3 of instance: gpt2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_RX_DATA03 of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_RX_DATA of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA05 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO08 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_IN20 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ENET_1588_EVENT3_IN of instance: enet"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB008muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB008muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB008muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB008muxMode {
        SwMuxCtlPadGpioAdB008muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB008muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB008muxMode) -> u8 {
        SwMuxCtlPadGpioAdB008muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB009muxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_TDI of instance: jtag_mux"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_RX_DATA02 of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_TX_DATA of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA04 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO09 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_IN21 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: GPT2_CLK of instance: gpt2"]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_DQS4 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB009muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB009muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB009muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB009muxMode {
        SwMuxCtlPadGpioAdB009muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB009muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB009muxMode) -> u8 {
        SwMuxCtlPadGpioAdB009muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB010muxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_TDO of instance: jtag_mux"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_CRS of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI2_MCLK of instance: sai2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA03 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO10 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_IN22 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ENET_1588_EVENT0_OUT of instance: enet"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXCAN3_TX of instance: flexcan3/canfd"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: ARM_TRACE_SWO of instance: cm7_mx6rt"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB010muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB010muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB010muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB010muxMode {
        SwMuxCtlPadGpioAdB010muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB010muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB010muxMode) -> u8 {
        SwMuxCtlPadGpioAdB010muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB011muxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_TRSTB of instance: jtag_mux"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: ENET_COL of instance: enet"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: WDOG1_WDOG_B of instance: wdog1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA02 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO11 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_IN23 of instance: xbar1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: ENET_1588_EVENT0_IN of instance: enet"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXCAN3_RX of instance: flexcan3/canfd"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_CLK6 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB011muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB011muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB011muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB011muxMode {
        SwMuxCtlPadGpioAdB011muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB011muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB011muxMode) -> u8 {
        SwMuxCtlPadGpioAdB011muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB012muxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C4_SCL of instance: lpi2c4"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CCM_PMIC_READY of instance: ccm"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_TX of instance: lpuart1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: WDOG2_WDOG_B of instance: wdog2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX02 of instance: flexpwm1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO12 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ENET_1588_EVENT1_OUT of instance: enet"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: nmi_glue"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB012muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB012muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB012muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB012muxMode {
        SwMuxCtlPadGpioAdB012muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB012muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB012muxMode) -> u8 {
        SwMuxCtlPadGpioAdB012muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB013muxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C4_SDA of instance: lpi2c4"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: GPT1_CLK of instance: gpt1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_RX of instance: lpuart1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: EWM_OUT_B of instance: ewm"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX03 of instance: flexpwm1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO13 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ENET_1588_EVENT1_IN of instance: enet"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: REF_CLK_24M of instance: anatop"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadGpioAdB013muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB013muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB013muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB013muxMode {
        SwMuxCtlPadGpioAdB013muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB013muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB013muxMode) -> u8 {
        SwMuxCtlPadGpioAdB013muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB014muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG2_OC of instance: usb"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN24 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_CTS_B of instance: lpuart1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_1588_EVENT0_OUT of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_VSYNC of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO14 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXCAN2_TX of instance: flexcan2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXCAN3_TX of instance: flexcan3/canfd"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB014muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB014muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB014muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB014muxMode {
        SwMuxCtlPadGpioAdB014muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB014muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB014muxMode) -> u8 {
        SwMuxCtlPadGpioAdB014muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB015muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG2_PWR of instance: usb"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN25 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_RTS_B of instance: lpuart1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_1588_EVENT0_IN of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_HSYNC of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO15 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXCAN2_RX of instance: flexcan2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: WDOG1_WDOG_RST_B_DEB of instance: wdog1"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXCAN3_RX of instance: flexcan3/canfd"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB015muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB015muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB015muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB015muxMode {
        SwMuxCtlPadGpioAdB015muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB015muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB015muxMode) -> u8 {
        SwMuxCtlPadGpioAdB015muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB100muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG2_ID of instance: anatop"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_CTS_B of instance: lpuart2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: lpi2c1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: WDOG1_B of instance: wdog1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO16 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW07 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_OUT of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO00 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB100muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB100muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB100muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB100muxMode {
        SwMuxCtlPadGpioAdB100muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB100muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB100muxMode) -> u8 {
        SwMuxCtlPadGpioAdB100muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB101muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_PWR of instance: usb"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RTS_B of instance: lpuart2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: lpi2c1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CCM_PMIC_READY of instance: ccm"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO17 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_VSELECT of instance: usdhc1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL07 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_IN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO01 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB101muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB101muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB101muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB101muxMode {
        SwMuxCtlPadGpioAdB101muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB101muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB101muxMode) -> u8 {
        SwMuxCtlPadGpioAdB101muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB102muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_ID of instance: anatop"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_OUT of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: ENET_1588_EVENT2_OUT of instance: enet"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO18 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_CD_B of instance: usdhc1"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW06 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT2_CLK of instance: gpt2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO02 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB102muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB102muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB102muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB102muxMode {
        SwMuxCtlPadGpioAdB102muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB102muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB102muxMode) -> u8 {
        SwMuxCtlPadGpioAdB102muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB103muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_OC of instance: usb"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: QTIMER3_TIMER3 of instance: qtimer3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_IN of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: ENET_1588_EVENT2_IN of instance: enet"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO19 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_CD_B of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL06 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT2_CAPTURE1 of instance: gpt2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO03 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB103muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB103muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB103muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB103muxMode {
        SwMuxCtlPadGpioAdB103muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB103muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB103muxMode) -> u8 {
        SwMuxCtlPadGpioAdB103muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB104muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIB_DATA03 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ENET_MDC of instance: enet"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_CTS_B of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_SR_CLK of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_PIXCLK of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO20 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA0 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW05 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT2_CAPTURE2 of instance: gpt2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO04 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB104muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB104muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB104muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB104muxMode {
        SwMuxCtlPadGpioAdB104muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB104muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB104muxMode) -> u8 {
        SwMuxCtlPadGpioAdB104muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB105muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIB_DATA02 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ENET_MDIO of instance: enet"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_RTS_B of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_OUT of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_MCLK of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO21 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA1 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL05 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT2_COMPARE1 of instance: gpt2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO05 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB105muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB105muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB105muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB105muxMode {
        SwMuxCtlPadGpioAdB105muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB105muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB105muxMode) -> u8 {
        SwMuxCtlPadGpioAdB105muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB106muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIB_DATA01 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C3_SDA of instance: lpi2c3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_LOCK of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_VSYNC of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO22 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA2 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW04 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT2_COMPARE2 of instance: gpt2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO06 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB106muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB106muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB106muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB106muxMode {
        SwMuxCtlPadGpioAdB106muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB106muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB106muxMode) -> u8 {
        SwMuxCtlPadGpioAdB106muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB107muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIB_DATA00 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C3_SCL of instance: lpi2c3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_EXT_CLK of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_HSYNC of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO23 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA3 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL04 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: GPT2_COMPARE3 of instance: gpt2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO07 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB107muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB107muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB107muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB107muxMode {
        SwMuxCtlPadGpioAdB107muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB107muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB107muxMode) -> u8 {
        SwMuxCtlPadGpioAdB107muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB108muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_SS1_B of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA00 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXCAN1_TX of instance: flexcan1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_PMIC_READY of instance: ccm"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA09 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO24 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_CMD of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW03 of instance: kpp"]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO08 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB108muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB108muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB108muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB108muxMode {
        SwMuxCtlPadGpioAdB108muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB108muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB108muxMode) -> u8 {
        SwMuxCtlPadGpioAdB108muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB109muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_DQS of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA01 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXCAN1_RX of instance: flexcan1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA08 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO25 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_CLK of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL03 of instance: kpp"]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO09 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB109muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB109muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB109muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB109muxMode {
        SwMuxCtlPadGpioAdB109muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB109muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB109muxMode) -> u8 {
        SwMuxCtlPadGpioAdB109muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB110muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_DATA03 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: WDOG1_B of instance: wdog1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA07 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO26 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_WP of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW02 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT1_OUT of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO10 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB110muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB110muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB110muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB110muxMode {
        SwMuxCtlPadGpioAdB110muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB110muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB110muxMode) -> u8 {
        SwMuxCtlPadGpioAdB110muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB111muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_DATA02 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: EWM_OUT_B of instance: ewm"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA06 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO27 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL02 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT1_IN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO11 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB111muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB111muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB111muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB111muxMode {
        SwMuxCtlPadGpioAdB111muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB111muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB111muxMode) -> u8 {
        SwMuxCtlPadGpioAdB111muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB112muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_DATA01 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT00 of instance: acmp"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA05 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO28 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA4 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW01 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT2_OUT of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO12 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB112muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB112muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB112muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB112muxMode {
        SwMuxCtlPadGpioAdB112muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB112muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB112muxMode) -> u8 {
        SwMuxCtlPadGpioAdB112muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB113muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_DATA00 of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT01 of instance: acmp"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI3_SDI of instance: lpspi3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA04 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO29 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA5 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL01 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT2_IN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO13 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB113muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB113muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB113muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB113muxMode {
        SwMuxCtlPadGpioAdB113muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB113muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB113muxMode) -> u8 {
        SwMuxCtlPadGpioAdB113muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB114muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_SCLK of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT02 of instance: acmp"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI3_SDO of instance: lpspi3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA03 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO30 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA6 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_ROW00 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT3_OUT of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO14 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB114muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB114muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB114muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB114muxMode {
        SwMuxCtlPadGpioAdB114muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB114muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB114muxMode) -> u8 {
        SwMuxCtlPadGpioAdB114muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAdB115muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPIA_SS0_B of instance: flexspi"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP_OUT03 of instance: acmp"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI3_SCK of instance: lpspi3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA02 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO31 of instance: gpio1"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_DATA7 of instance: usdhc2"]
    ALT6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: KPP_COL00 of instance: kpp"]
    ALT7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_1588_EVENT3_IN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO15 of instance: flexio3"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAdB115muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAdB115muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAdB115muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAdB115muxMode {
        SwMuxCtlPadGpioAdB115muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAdB115muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAdB115muxMode) -> u8 {
        SwMuxCtlPadGpioAdB115muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc00muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA00 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA00 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_SCK of instance: lpspi2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_XBAR_IN02 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO00 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO00 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc00muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc00muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc00muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc00muxMode {
        SwMuxCtlPadGpioEmc00muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc00muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc00muxMode) -> u8 {
        SwMuxCtlPadGpioEmc00muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc01muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA01 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB00 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_IN03 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO01 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO01 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc01muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc01muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc01muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc01muxMode {
        SwMuxCtlPadGpioEmc01muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc01muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc01muxMode) -> u8 {
        SwMuxCtlPadGpioEmc01muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc02muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA02 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA01 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_SDO of instance: lpspi2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT04 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO02 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO02 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc02muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc02muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc02muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc02muxMode {
        SwMuxCtlPadGpioEmc02muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc02muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc02muxMode) -> u8 {
        SwMuxCtlPadGpioEmc02muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc03muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA03 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB01 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_SDI of instance: lpspi2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT05 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO03 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO03 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc03muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc03muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc03muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc03muxMode {
        SwMuxCtlPadGpioEmc03muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc03muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc03muxMode) -> u8 {
        SwMuxCtlPadGpioEmc03muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc04muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA04 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA02 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_DATA of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT06 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO04 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO04 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc04muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc04muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc04muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc04muxMode {
        SwMuxCtlPadGpioEmc04muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc04muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc04muxMode) -> u8 {
        SwMuxCtlPadGpioEmc04muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc05muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA05 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB02 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_SYNC of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT07 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO05 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO05 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc05muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc05muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc05muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc05muxMode {
        SwMuxCtlPadGpioEmc05muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc05muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc05muxMode) -> u8 {
        SwMuxCtlPadGpioEmc05muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc06muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA06 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_BCLK of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT08 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO06 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO06 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc06muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc06muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc06muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc06muxMode {
        SwMuxCtlPadGpioEmc06muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc06muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc06muxMode) -> u8 {
        SwMuxCtlPadGpioEmc06muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc07muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA07 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_MCLK of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT09 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO07 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO07 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc07muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc07muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc07muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc07muxMode {
        SwMuxCtlPadGpioEmc07muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc07muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc07muxMode) -> u8 {
        SwMuxCtlPadGpioEmc07muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc08muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DM00 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_DATA of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT17 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO08 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO08 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc08muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc08muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc08muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc08muxMode {
        SwMuxCtlPadGpioEmc08muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc08muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc08muxMode) -> u8 {
        SwMuxCtlPadGpioEmc08muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc09muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR00 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_SYNC of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXCAN2_TX of instance: flexcan2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO09 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO09 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_SS1_B of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc09muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc09muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc09muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc09muxMode {
        SwMuxCtlPadGpioEmc09muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc09muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc09muxMode) -> u8 {
        SwMuxCtlPadGpioEmc09muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc10muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR01 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_BCLK of instance: sai2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXCAN2_RX of instance: flexcan2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO10 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO10 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_SS0_B of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc10muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc10muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc10muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc10muxMode {
        SwMuxCtlPadGpioEmc10muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc10muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc10muxMode) -> u8 {
        SwMuxCtlPadGpioEmc10muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc11muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR02 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C4_SDA of instance: lpi2c4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO11 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO11 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_DQS of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc11muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc11muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc11muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc11muxMode {
        SwMuxCtlPadGpioEmc11muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc11muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc11muxMode) -> u8 {
        SwMuxCtlPadGpioEmc11muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc12muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR03 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN24 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C4_SCL of instance: lpi2c4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USDHC1_WP of instance: usdhc1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO12 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_SCLK of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc12muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc12muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc12muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc12muxMode {
        SwMuxCtlPadGpioEmc12muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc12muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc12muxMode) -> u8 {
        SwMuxCtlPadGpioEmc12muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc13muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR04 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN25 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: MQS_RIGHT of instance: mqs"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO13 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_DATA00 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc13muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc13muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc13muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc13muxMode {
        SwMuxCtlPadGpioEmc13muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc13muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc13muxMode) -> u8 {
        SwMuxCtlPadGpioEmc13muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc14muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR05 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT19 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: MQS_LEFT of instance: mqs"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS1 of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO14 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_DATA01 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc14muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc14muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc14muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc14muxMode {
        SwMuxCtlPadGpioEmc14muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc14muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc14muxMode) -> u8 {
        SwMuxCtlPadGpioEmc14muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc15muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR06 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN20 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_CTS_B of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_OUT of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO15 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_DATA02 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc15muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc15muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc15muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc15muxMode {
        SwMuxCtlPadGpioEmc15muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc15muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc15muxMode) -> u8 {
        SwMuxCtlPadGpioEmc15muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc16muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR07 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN21 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART3_RTS_B of instance: lpuart3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SPDIF_IN of instance: spdif"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO16 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_B_DATA03 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc16muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc16muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc16muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc16muxMode {
        SwMuxCtlPadGpioEmc16muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc16muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc16muxMode) -> u8 {
        SwMuxCtlPadGpioEmc16muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc17muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR08 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA03 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_CTS_B of instance: lpuart4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXCAN1_TX of instance: flexcan1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO17 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc17muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc17muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc17muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc17muxMode {
        SwMuxCtlPadGpioEmc17muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc17muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc17muxMode) -> u8 {
        SwMuxCtlPadGpioEmc17muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc18muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR09 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB03 of instance: flexpwm4"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_RTS_B of instance: lpuart4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXCAN1_RX of instance: flexcan1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER3_TIMER3 of instance: qtimer3"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO18 of instance: gpio4"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SNVS_VIO_5_CTL of instance: snvs_hp"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc18muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc18muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc18muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc18muxMode {
        SwMuxCtlPadGpioEmc18muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc18muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc18muxMode) -> u8 {
        SwMuxCtlPadGpioEmc18muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc19muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR11 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RDATA01 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO19 of instance: gpio4"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SNVS_VIO_5 of instance: snvs_hp"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc19muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc19muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc19muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc19muxMode {
        SwMuxCtlPadGpioEmc19muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc19muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc19muxMode) -> u8 {
        SwMuxCtlPadGpioEmc19muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc20muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR12 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RDATA00 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO20 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc20muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc20muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc20muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc20muxMode {
        SwMuxCtlPadGpioEmc20muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc20muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc20muxMode) -> u8 {
        SwMuxCtlPadGpioEmc20muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc21muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_BA0 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA03 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C3_SDA of instance: lpi2c3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TDATA01 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER2_TIMER2 of instance: qtimer2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO21 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc21muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc21muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc21muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc21muxMode {
        SwMuxCtlPadGpioEmc21muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc21muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc21muxMode) -> u8 {
        SwMuxCtlPadGpioEmc21muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc22muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_BA1 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB03 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C3_SCL of instance: lpi2c3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TDATA00 of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: QTIMER2_TIMER3 of instance: qtimer2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO22 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_SS1_B of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc22muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc22muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc22muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc22muxMode {
        SwMuxCtlPadGpioEmc22muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc22muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc22muxMode) -> u8 {
        SwMuxCtlPadGpioEmc22muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc23muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR10 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RX_EN of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT1_CAPTURE2 of instance: gpt1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO23 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_DQS of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc23muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc23muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc23muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc23muxMode {
        SwMuxCtlPadGpioEmc23muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc23muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc23muxMode) -> u8 {
        SwMuxCtlPadGpioEmc23muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc24muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CAS of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TX_EN of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: GPT1_CAPTURE1 of instance: gpt1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO24 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_SS0_B of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc24muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc24muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc24muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc24muxMode {
        SwMuxCtlPadGpioEmc24muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc24muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc24muxMode) -> u8 {
        SwMuxCtlPadGpioEmc24muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc25muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_RAS of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_TX_CLK of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: ENET_REF_CLK of instance: enet"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO25 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_SCLK of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc25muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc25muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc25muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc25muxMode {
        SwMuxCtlPadGpioEmc25muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc25muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc25muxMode) -> u8 {
        SwMuxCtlPadGpioEmc25muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc26muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CLK of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RX of instance: lpuart6"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: ENET_RX_ER of instance: enet"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO12 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO26 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_DATA00 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc26muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc26muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc26muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc26muxMode {
        SwMuxCtlPadGpioEmc26muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc26muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc26muxMode) -> u8 {
        SwMuxCtlPadGpioEmc26muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc27muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CKE of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RTS_B of instance: lpuart5"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI1_SCK of instance: lpspi1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO13 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO27 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_DATA01 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc27muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc27muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc27muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc27muxMode {
        SwMuxCtlPadGpioEmc27muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc27muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc27muxMode) -> u8 {
        SwMuxCtlPadGpioEmc27muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc28muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_WE of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_CTS_B of instance: lpuart5"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI1_SDO of instance: lpspi1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO14 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO28 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_DATA02 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc28muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc28muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc28muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc28muxMode {
        SwMuxCtlPadGpioEmc28muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc28muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc28muxMode) -> u8 {
        SwMuxCtlPadGpioEmc28muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc29muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CS0 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA00 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RTS_B of instance: lpuart6"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI1_SDI of instance: lpspi1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO15 of instance: flexio1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO29 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_A_DATA03 of instance: flexspi2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc29muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc29muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc29muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc29muxMode {
        SwMuxCtlPadGpioEmc29muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc29muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc29muxMode) -> u8 {
        SwMuxCtlPadGpioEmc29muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc30muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA08 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB00 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_CTS_B of instance: lpuart6"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI1_PCS0 of instance: lpspi1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA23 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO30 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA00 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc30muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc30muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc30muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc30muxMode {
        SwMuxCtlPadGpioEmc30muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc30muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc30muxMode) -> u8 {
        SwMuxCtlPadGpioEmc30muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc31muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA09 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA01 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI1_PCS1 of instance: lpspi1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA22 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO31 of instance: gpio4"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TDATA01 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc31muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc31muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc31muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc31muxMode {
        SwMuxCtlPadGpioEmc31muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc31muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc31muxMode) -> u8 {
        SwMuxCtlPadGpioEmc31muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc32muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA10 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB01 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_PMIC_RDY of instance: ccm"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA21 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO18 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_EN of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc32muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc32muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc32muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc32muxMode {
        SwMuxCtlPadGpioEmc32muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc32muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc32muxMode) -> u8 {
        SwMuxCtlPadGpioEmc32muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc33muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA11 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA02 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_RESET_B of instance: usdhc1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_RX_DATA of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA20 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO19 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_CLK of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: ENET2_REF_CLK2 of instance: enet2"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc33muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc33muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc33muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc33muxMode {
        SwMuxCtlPadGpioEmc33muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc33muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc33muxMode) -> u8 {
        SwMuxCtlPadGpioEmc33muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc34muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA12 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB02 of instance: flexpwm3"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_VSELECT of instance: usdhc1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_RX_SYNC of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA19 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO20 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_ER of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc34muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc34muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc34muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc34muxMode {
        SwMuxCtlPadGpioEmc34muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc34muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc34muxMode) -> u8 {
        SwMuxCtlPadGpioEmc34muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc35muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA13 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT18 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE1 of instance: gpt1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_RX_BCLK of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA18 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO21 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_CD_B of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA00 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc35muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc35muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc35muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc35muxMode {
        SwMuxCtlPadGpioEmc35muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc35muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc35muxMode) -> u8 {
        SwMuxCtlPadGpioEmc35muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc36muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA14 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN22 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: gpt1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA17 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXCAN3_TX of instance: flexcan3/canfd"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc36muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc36muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc36muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc36muxMode {
        SwMuxCtlPadGpioEmc36muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc36muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc36muxMode) -> u8 {
        SwMuxCtlPadGpioEmc36muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc37muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA15 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN23 of instance: xbar1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE3 of instance: gpt1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_MCLK of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA16 of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO23 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_WP of instance: usdhc2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_EN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXCAN3_RX of instance: flexcan3/canfd"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc37muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc37muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc37muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc37muxMode {
        SwMuxCtlPadGpioEmc37muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc37muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc37muxMode) -> u8 {
        SwMuxCtlPadGpioEmc37muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc38muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DM01 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: CSI_FIELD of instance: csi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO24 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_VSELECT of instance: usdhc2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_MDC of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc38muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc38muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc38muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc38muxMode {
        SwMuxCtlPadGpioEmc38muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc38muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc38muxMode) -> u8 {
        SwMuxCtlPadGpioEmc38muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc39muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DQS of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: sai3"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: WDOG1_WDOG_B of instance: wdog1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO25 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_CD_B of instance: usdhc2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_MDIO of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_DQS4 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc39muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc39muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc39muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc39muxMode {
        SwMuxCtlPadGpioEmc39muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc39muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc39muxMode) -> u8 {
        SwMuxCtlPadGpioEmc39muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc40muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_RDY of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: GPT2_CAPTURE2 of instance: gpt2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_PCS2 of instance: lpspi1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG2_OC of instance: usb"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: ENET_MDC of instance: enet"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO26 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_CLK5 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmc40muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc40muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc40muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc40muxMode {
        SwMuxCtlPadGpioEmc40muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc40muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc40muxMode) -> u8 {
        SwMuxCtlPadGpioEmc40muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmc41muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CSX00 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: GPT2_CAPTURE1 of instance: gpt2"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_PCS3 of instance: lpspi1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG2_PWR of instance: usb"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: ENET_MDIO of instance: enet"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO27 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_VSELECT of instance: usdhc1"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioEmc41muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmc41muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmc41muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmc41muxMode {
        SwMuxCtlPadGpioEmc41muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmc41muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmc41muxMode) -> u8 {
        SwMuxCtlPadGpioEmc41muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB000muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_CMD of instance: usdhc1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C3_SCL of instance: lpi2c3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT04 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI1_SCK of instance: lpspi1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO12 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXSPIA_SS1_B of instance: flexspi"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_EN of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_DQS4 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB000muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB000muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB000muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB000muxMode {
        SwMuxCtlPadGpioSdB000muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB000muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB000muxMode) -> u8 {
        SwMuxCtlPadGpioSdB000muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB001muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_CLK of instance: usdhc1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C3_SDA of instance: lpi2c3"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT05 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI1_PCS0 of instance: lpspi1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO13 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXSPIB_SS1_B of instance: flexspi"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_TX_CLK of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: ENET2_REF_CLK2 of instance: enet2"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB001muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB001muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB001muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB001muxMode {
        SwMuxCtlPadGpioSdB001muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB001muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB001muxMode) -> u8 {
        SwMuxCtlPadGpioSdB001muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB002muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA0 of instance: usdhc1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_CTS_B of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT06 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI1_SDO of instance: lpspi1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO14 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_ER of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_CLK5 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB002muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB002muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB002muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB002muxMode {
        SwMuxCtlPadGpioSdB002muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB002muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB002muxMode) -> u8 {
        SwMuxCtlPadGpioSdB002muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB003muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA1 of instance: usdhc1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_RTS_B of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT07 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI1_SDI of instance: lpspi1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO15 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA00 of instance: enet2"]
    ALT8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SEMC_CLK6 of instance: semc"]
    ALT9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB003muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB003muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB003muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB003muxMode {
        SwMuxCtlPadGpioSdB003muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB003muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB003muxMode) -> u8 {
        SwMuxCtlPadGpioSdB003muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB004muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA2 of instance: usdhc1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT08 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXSPIB_SS0_B of instance: flexspi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO16 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CCM_CLKO1 of instance: ccm"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB004muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB004muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB004muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB004muxMode {
        SwMuxCtlPadGpioSdB004muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB004muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB004muxMode) -> u8 {
        SwMuxCtlPadGpioSdB004muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB005muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA3 of instance: usdhc1"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_INOUT09 of instance: xbar1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXSPIB_DQS of instance: flexspi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO17 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CCM_CLKO2 of instance: ccm"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RX_EN of instance: enet2"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB005muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB005muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB005muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB005muxMode {
        SwMuxCtlPadGpioSdB005muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB005muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB005muxMode) -> u8 {
        SwMuxCtlPadGpioSdB005muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB100muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA3 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIB_DATA03 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA03 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART4_TX of instance: lpuart4"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO00 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_DATA of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB100muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB100muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB100muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB100muxMode {
        SwMuxCtlPadGpioSdB100muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB100muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB100muxMode) -> u8 {
        SwMuxCtlPadGpioSdB100muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB101muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA2 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIB_DATA02 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA02 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART4_RX of instance: lpuart4"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO01 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_DATA of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB101muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB101muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB101muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB101muxMode {
        SwMuxCtlPadGpioSdB101muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB101muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB101muxMode) -> u8 {
        SwMuxCtlPadGpioSdB101muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB102muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA1 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIB_DATA01 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXCAN1_TX of instance: flexcan1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO02 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CCM_WAIT of instance: ccm"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_SYNC of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB102muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB102muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB102muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB102muxMode {
        SwMuxCtlPadGpioSdB102muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB102muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB102muxMode) -> u8 {
        SwMuxCtlPadGpioSdB102muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB103muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA0 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIB_DATA00 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXCAN1_RX of instance: flexcan1"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO03 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CCM_PMIC_READY of instance: ccm"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_BCLK of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB103muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB103muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB103muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB103muxMode {
        SwMuxCtlPadGpioSdB103muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB103muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB103muxMode) -> u8 {
        SwMuxCtlPadGpioSdB103muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB104muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_CLK of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIB_SCLK of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C1_SCL of instance: lpi2c1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXSPIA_SS1_B of instance: flexspi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO04 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CCM_STOP of instance: ccm"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_MCLK of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB104muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB104muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB104muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB104muxMode {
        SwMuxCtlPadGpioSdB104muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB104muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB104muxMode) -> u8 {
        SwMuxCtlPadGpioSdB104muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB105muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_CMD of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_DQS of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C1_SDA of instance: lpi2c1"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXSPIB_SS0_B of instance: flexspi"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO05 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_SYNC of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB105muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB105muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB105muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB105muxMode {
        SwMuxCtlPadGpioSdB105muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB105muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB105muxMode) -> u8 {
        SwMuxCtlPadGpioSdB105muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB106muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_SS0_B of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_CTS_B of instance: lpuart7"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO06 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_BCLK of instance: sai3"]
    ALT8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB106muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB106muxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB106muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB106muxMode {
        SwMuxCtlPadGpioSdB106muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB106muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB106muxMode) -> u8 {
        SwMuxCtlPadGpioSdB106muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB107muxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CSX01 of instance: semc"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_SCLK of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RTS_B of instance: lpuart7"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_SCK of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO07 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSdB107muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB107muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB107muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB107muxMode {
        SwMuxCtlPadGpioSdB107muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB107muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB107muxMode) -> u8 {
        SwMuxCtlPadGpioSdB107muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB108muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA4 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_DATA00 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_SD0 of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO08 of instance: gpio3"]
    ALT5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SEMC_CSX02 of instance: semc"]
    ALT6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSdB108muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB108muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB108muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB108muxMode {
        SwMuxCtlPadGpioSdB108muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB108muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB108muxMode) -> u8 {
        SwMuxCtlPadGpioSdB108muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB109muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA5 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_DATA01 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_SDI of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO09 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSdB109muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB109muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB109muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB109muxMode {
        SwMuxCtlPadGpioSdB109muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB109muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB109muxMode) -> u8 {
        SwMuxCtlPadGpioSdB109muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB110muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA6 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_DATA02 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: lpi2c2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS2 of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO10 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSdB110muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB110muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB110muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB110muxMode {
        SwMuxCtlPadGpioSdB110muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB110muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB110muxMode) -> u8 {
        SwMuxCtlPadGpioSdB110muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB111muxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA7 of instance: usdhc2"]
    ALT0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPIA_DATA03 of instance: flexspi"]
    ALT1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2"]
    ALT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: lpi2c2"]
    ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI2_PCS3 of instance: lpspi2"]
    ALT4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO11 of instance: gpio3"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSdB111muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB111muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB111muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB111muxMode {
        SwMuxCtlPadGpioSdB111muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB111muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB111muxMode) -> u8 {
        SwMuxCtlPadGpioSdB111muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB000muxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO00 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB000muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB000muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB000muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB000muxMode {
        SwMuxCtlPadGpioSpiB000muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB000muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB000muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB000muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB001muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_B_SCLK of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO01 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB001muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB001muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB001muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB001muxMode {
        SwMuxCtlPadGpioSpiB001muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB001muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB001muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB001muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB002muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA00 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO02 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB002muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB002muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB002muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB002muxMode {
        SwMuxCtlPadGpioSpiB002muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB002muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB002muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB002muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB003muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_B_DATA02 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO03 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB003muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB003muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB003muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB003muxMode {
        SwMuxCtlPadGpioSpiB003muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB003muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB003muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB003muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB004muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_B_DATA03 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO04 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB004muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB004muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB004muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB004muxMode {
        SwMuxCtlPadGpioSpiB004muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB004muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB004muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB004muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB005muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_SS0_B of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO05 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB005muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB005muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB005muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB005muxMode {
        SwMuxCtlPadGpioSpiB005muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB005muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB005muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB005muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB006muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA02 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO06 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB006muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB006muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB006muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB006muxMode {
        SwMuxCtlPadGpioSpiB006muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB006muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB006muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB006muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB007muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_B_DATA01 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO07 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB007muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB007muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB007muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB007muxMode {
        SwMuxCtlPadGpioSpiB007muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB007muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB007muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB007muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB008muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_SCLK of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO08 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB008muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB008muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB008muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB008muxMode {
        SwMuxCtlPadGpioSpiB008muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB008muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB008muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB008muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB009muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DQS of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO09 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB009muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB009muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB009muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB009muxMode {
        SwMuxCtlPadGpioSpiB009muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB009muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB009muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB009muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB010muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA03 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO10 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB010muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB010muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB010muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB010muxMode {
        SwMuxCtlPadGpioSpiB010muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB010muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB010muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB010muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB011muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_B_DATA00 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO11 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB011muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB011muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB011muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB011muxMode {
        SwMuxCtlPadGpioSpiB011muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB011muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB011muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB011muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB012muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA01 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO12 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB012muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB012muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB012muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB012muxMode {
        SwMuxCtlPadGpioSpiB012muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB012muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB012muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB012muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB013muxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO13 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB013muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB013muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB013muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB013muxMode {
        SwMuxCtlPadGpioSpiB013muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB013muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB013muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB013muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB100muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DQS of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO14 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB100muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB100muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB100muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB100muxMode {
        SwMuxCtlPadGpioSpiB100muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB100muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB100muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB100muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB101muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA03 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO15 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB101muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB101muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB101muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB101muxMode {
        SwMuxCtlPadGpioSpiB101muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB101muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB101muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB101muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB102muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA02 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO16 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB102muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB102muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB102muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB102muxMode {
        SwMuxCtlPadGpioSpiB102muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB102muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB102muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB102muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB103muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA01 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO17 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB103muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB103muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB103muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB103muxMode {
        SwMuxCtlPadGpioSpiB103muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB103muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB103muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB103muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB104muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_DATA00 of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO18 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB104muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB104muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB104muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB104muxMode {
        SwMuxCtlPadGpioSpiB104muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB104muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB104muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB104muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB105muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_SCLK of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO19 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB105muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB105muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB105muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB105muxMode {
        SwMuxCtlPadGpioSpiB105muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB105muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB105muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB105muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB106muxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_A_SS0_B of instance: flexspi2"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO20 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB106muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB106muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB106muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB106muxMode {
        SwMuxCtlPadGpioSpiB106muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB106muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB106muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB106muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSpiB107muxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO10_IO21 of instance: gpio10"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSpiB107muxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSpiB107muxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSpiB107muxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSpiB107muxMode {
        SwMuxCtlPadGpioSpiB107muxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSpiB107muxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSpiB107muxMode) -> u8 {
        SwMuxCtlPadGpioSpiB107muxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB000dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB000dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB000dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB000dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB000dse {
        SwPadCtlPadGpioAdB000dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB000dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB000dse) -> u8 {
        SwPadCtlPadGpioAdB000dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB000pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB000pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB000pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB000pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB000pus {
        SwPadCtlPadGpioAdB000pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB000pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB000pus) -> u8 {
        SwPadCtlPadGpioAdB000pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB000speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB000speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB000speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB000speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB000speed {
        SwPadCtlPadGpioAdB000speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB000speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB000speed) -> u8 {
        SwPadCtlPadGpioAdB000speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB001dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB001dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB001dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB001dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB001dse {
        SwPadCtlPadGpioAdB001dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB001dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB001dse) -> u8 {
        SwPadCtlPadGpioAdB001dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB001pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB001pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB001pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB001pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB001pus {
        SwPadCtlPadGpioAdB001pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB001pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB001pus) -> u8 {
        SwPadCtlPadGpioAdB001pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB001speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB001speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB001speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB001speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB001speed {
        SwPadCtlPadGpioAdB001speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB001speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB001speed) -> u8 {
        SwPadCtlPadGpioAdB001speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB002dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB002dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB002dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB002dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB002dse {
        SwPadCtlPadGpioAdB002dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB002dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB002dse) -> u8 {
        SwPadCtlPadGpioAdB002dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB002pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB002pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB002pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB002pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB002pus {
        SwPadCtlPadGpioAdB002pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB002pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB002pus) -> u8 {
        SwPadCtlPadGpioAdB002pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB002speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB002speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB002speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB002speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB002speed {
        SwPadCtlPadGpioAdB002speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB002speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB002speed) -> u8 {
        SwPadCtlPadGpioAdB002speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB003dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB003dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB003dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB003dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB003dse {
        SwPadCtlPadGpioAdB003dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB003dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB003dse) -> u8 {
        SwPadCtlPadGpioAdB003dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB003pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB003pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB003pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB003pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB003pus {
        SwPadCtlPadGpioAdB003pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB003pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB003pus) -> u8 {
        SwPadCtlPadGpioAdB003pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB003speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB003speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB003speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB003speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB003speed {
        SwPadCtlPadGpioAdB003speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB003speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB003speed) -> u8 {
        SwPadCtlPadGpioAdB003speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB004dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB004dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB004dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB004dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB004dse {
        SwPadCtlPadGpioAdB004dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB004dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB004dse) -> u8 {
        SwPadCtlPadGpioAdB004dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB004pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB004pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB004pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB004pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB004pus {
        SwPadCtlPadGpioAdB004pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB004pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB004pus) -> u8 {
        SwPadCtlPadGpioAdB004pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB004speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB004speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB004speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB004speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB004speed {
        SwPadCtlPadGpioAdB004speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB004speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB004speed) -> u8 {
        SwPadCtlPadGpioAdB004speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB005dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB005dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB005dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB005dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB005dse {
        SwPadCtlPadGpioAdB005dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB005dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB005dse) -> u8 {
        SwPadCtlPadGpioAdB005dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB005pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB005pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB005pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB005pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB005pus {
        SwPadCtlPadGpioAdB005pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB005pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB005pus) -> u8 {
        SwPadCtlPadGpioAdB005pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB005speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB005speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB005speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB005speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB005speed {
        SwPadCtlPadGpioAdB005speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB005speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB005speed) -> u8 {
        SwPadCtlPadGpioAdB005speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB006dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB006dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB006dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB006dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB006dse {
        SwPadCtlPadGpioAdB006dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB006dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB006dse) -> u8 {
        SwPadCtlPadGpioAdB006dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB006pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB006pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB006pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB006pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB006pus {
        SwPadCtlPadGpioAdB006pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB006pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB006pus) -> u8 {
        SwPadCtlPadGpioAdB006pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB006speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB006speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB006speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB006speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB006speed {
        SwPadCtlPadGpioAdB006speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB006speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB006speed) -> u8 {
        SwPadCtlPadGpioAdB006speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB007dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB007dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB007dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB007dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB007dse {
        SwPadCtlPadGpioAdB007dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB007dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB007dse) -> u8 {
        SwPadCtlPadGpioAdB007dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB007pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB007pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB007pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB007pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB007pus {
        SwPadCtlPadGpioAdB007pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB007pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB007pus) -> u8 {
        SwPadCtlPadGpioAdB007pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB007speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB007speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB007speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB007speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB007speed {
        SwPadCtlPadGpioAdB007speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB007speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB007speed) -> u8 {
        SwPadCtlPadGpioAdB007speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB008dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB008dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB008dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB008dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB008dse {
        SwPadCtlPadGpioAdB008dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB008dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB008dse) -> u8 {
        SwPadCtlPadGpioAdB008dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB008pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB008pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB008pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB008pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB008pus {
        SwPadCtlPadGpioAdB008pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB008pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB008pus) -> u8 {
        SwPadCtlPadGpioAdB008pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB008speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB008speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB008speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB008speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB008speed {
        SwPadCtlPadGpioAdB008speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB008speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB008speed) -> u8 {
        SwPadCtlPadGpioAdB008speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB009dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB009dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB009dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB009dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB009dse {
        SwPadCtlPadGpioAdB009dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB009dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB009dse) -> u8 {
        SwPadCtlPadGpioAdB009dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB009pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB009pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB009pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB009pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB009pus {
        SwPadCtlPadGpioAdB009pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB009pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB009pus) -> u8 {
        SwPadCtlPadGpioAdB009pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB009speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB009speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB009speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB009speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB009speed {
        SwPadCtlPadGpioAdB009speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB009speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB009speed) -> u8 {
        SwPadCtlPadGpioAdB009speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB010dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB010dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB010dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB010dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB010dse {
        SwPadCtlPadGpioAdB010dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB010dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB010dse) -> u8 {
        SwPadCtlPadGpioAdB010dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB010pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB010pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB010pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB010pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB010pus {
        SwPadCtlPadGpioAdB010pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB010pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB010pus) -> u8 {
        SwPadCtlPadGpioAdB010pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB010speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB010speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB010speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB010speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB010speed {
        SwPadCtlPadGpioAdB010speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB010speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB010speed) -> u8 {
        SwPadCtlPadGpioAdB010speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB011dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB011dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB011dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB011dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB011dse {
        SwPadCtlPadGpioAdB011dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB011dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB011dse) -> u8 {
        SwPadCtlPadGpioAdB011dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB011pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB011pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB011pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB011pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB011pus {
        SwPadCtlPadGpioAdB011pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB011pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB011pus) -> u8 {
        SwPadCtlPadGpioAdB011pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB011speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB011speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB011speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB011speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB011speed {
        SwPadCtlPadGpioAdB011speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB011speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB011speed) -> u8 {
        SwPadCtlPadGpioAdB011speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB012dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB012dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB012dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB012dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB012dse {
        SwPadCtlPadGpioAdB012dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB012dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB012dse) -> u8 {
        SwPadCtlPadGpioAdB012dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB012pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB012pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB012pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB012pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB012pus {
        SwPadCtlPadGpioAdB012pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB012pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB012pus) -> u8 {
        SwPadCtlPadGpioAdB012pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB012speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB012speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB012speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB012speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB012speed {
        SwPadCtlPadGpioAdB012speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB012speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB012speed) -> u8 {
        SwPadCtlPadGpioAdB012speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB013dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB013dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB013dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB013dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB013dse {
        SwPadCtlPadGpioAdB013dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB013dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB013dse) -> u8 {
        SwPadCtlPadGpioAdB013dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB013pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB013pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB013pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB013pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB013pus {
        SwPadCtlPadGpioAdB013pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB013pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB013pus) -> u8 {
        SwPadCtlPadGpioAdB013pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB013speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB013speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB013speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB013speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB013speed {
        SwPadCtlPadGpioAdB013speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB013speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB013speed) -> u8 {
        SwPadCtlPadGpioAdB013speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB014dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB014dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB014dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB014dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB014dse {
        SwPadCtlPadGpioAdB014dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB014dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB014dse) -> u8 {
        SwPadCtlPadGpioAdB014dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB014pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB014pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB014pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB014pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB014pus {
        SwPadCtlPadGpioAdB014pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB014pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB014pus) -> u8 {
        SwPadCtlPadGpioAdB014pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB014speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB014speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB014speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB014speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB014speed {
        SwPadCtlPadGpioAdB014speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB014speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB014speed) -> u8 {
        SwPadCtlPadGpioAdB014speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB015dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB015dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB015dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB015dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB015dse {
        SwPadCtlPadGpioAdB015dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB015dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB015dse) -> u8 {
        SwPadCtlPadGpioAdB015dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB015pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB015pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB015pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB015pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB015pus {
        SwPadCtlPadGpioAdB015pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB015pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB015pus) -> u8 {
        SwPadCtlPadGpioAdB015pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB015speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB015speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB015speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB015speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB015speed {
        SwPadCtlPadGpioAdB015speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB015speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB015speed) -> u8 {
        SwPadCtlPadGpioAdB015speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB100dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB100dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB100dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB100dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB100dse {
        SwPadCtlPadGpioAdB100dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB100dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB100dse) -> u8 {
        SwPadCtlPadGpioAdB100dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB100pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB100pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB100pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB100pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB100pus {
        SwPadCtlPadGpioAdB100pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB100pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB100pus) -> u8 {
        SwPadCtlPadGpioAdB100pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB100speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB100speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB100speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB100speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB100speed {
        SwPadCtlPadGpioAdB100speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB100speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB100speed) -> u8 {
        SwPadCtlPadGpioAdB100speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB101dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB101dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB101dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB101dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB101dse {
        SwPadCtlPadGpioAdB101dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB101dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB101dse) -> u8 {
        SwPadCtlPadGpioAdB101dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB101pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB101pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB101pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB101pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB101pus {
        SwPadCtlPadGpioAdB101pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB101pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB101pus) -> u8 {
        SwPadCtlPadGpioAdB101pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB101speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB101speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB101speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB101speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB101speed {
        SwPadCtlPadGpioAdB101speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB101speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB101speed) -> u8 {
        SwPadCtlPadGpioAdB101speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB102dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB102dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB102dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB102dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB102dse {
        SwPadCtlPadGpioAdB102dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB102dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB102dse) -> u8 {
        SwPadCtlPadGpioAdB102dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB102pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB102pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB102pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB102pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB102pus {
        SwPadCtlPadGpioAdB102pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB102pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB102pus) -> u8 {
        SwPadCtlPadGpioAdB102pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB102speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB102speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB102speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB102speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB102speed {
        SwPadCtlPadGpioAdB102speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB102speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB102speed) -> u8 {
        SwPadCtlPadGpioAdB102speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB103dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB103dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB103dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB103dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB103dse {
        SwPadCtlPadGpioAdB103dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB103dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB103dse) -> u8 {
        SwPadCtlPadGpioAdB103dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB103pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB103pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB103pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB103pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB103pus {
        SwPadCtlPadGpioAdB103pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB103pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB103pus) -> u8 {
        SwPadCtlPadGpioAdB103pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB103speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB103speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB103speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB103speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB103speed {
        SwPadCtlPadGpioAdB103speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB103speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB103speed) -> u8 {
        SwPadCtlPadGpioAdB103speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB104dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB104dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB104dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB104dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB104dse {
        SwPadCtlPadGpioAdB104dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB104dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB104dse) -> u8 {
        SwPadCtlPadGpioAdB104dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB104pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB104pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB104pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB104pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB104pus {
        SwPadCtlPadGpioAdB104pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB104pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB104pus) -> u8 {
        SwPadCtlPadGpioAdB104pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB104speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB104speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB104speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB104speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB104speed {
        SwPadCtlPadGpioAdB104speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB104speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB104speed) -> u8 {
        SwPadCtlPadGpioAdB104speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB105dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB105dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB105dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB105dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB105dse {
        SwPadCtlPadGpioAdB105dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB105dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB105dse) -> u8 {
        SwPadCtlPadGpioAdB105dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB105pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB105pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB105pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB105pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB105pus {
        SwPadCtlPadGpioAdB105pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB105pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB105pus) -> u8 {
        SwPadCtlPadGpioAdB105pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB105speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB105speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB105speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB105speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB105speed {
        SwPadCtlPadGpioAdB105speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB105speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB105speed) -> u8 {
        SwPadCtlPadGpioAdB105speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB106dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB106dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB106dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB106dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB106dse {
        SwPadCtlPadGpioAdB106dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB106dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB106dse) -> u8 {
        SwPadCtlPadGpioAdB106dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB106pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB106pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB106pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB106pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB106pus {
        SwPadCtlPadGpioAdB106pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB106pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB106pus) -> u8 {
        SwPadCtlPadGpioAdB106pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB106speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB106speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB106speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB106speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB106speed {
        SwPadCtlPadGpioAdB106speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB106speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB106speed) -> u8 {
        SwPadCtlPadGpioAdB106speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB107dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB107dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB107dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB107dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB107dse {
        SwPadCtlPadGpioAdB107dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB107dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB107dse) -> u8 {
        SwPadCtlPadGpioAdB107dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB107pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB107pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB107pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB107pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB107pus {
        SwPadCtlPadGpioAdB107pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB107pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB107pus) -> u8 {
        SwPadCtlPadGpioAdB107pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB107speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB107speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB107speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB107speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB107speed {
        SwPadCtlPadGpioAdB107speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB107speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB107speed) -> u8 {
        SwPadCtlPadGpioAdB107speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB108dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB108dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB108dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB108dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB108dse {
        SwPadCtlPadGpioAdB108dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB108dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB108dse) -> u8 {
        SwPadCtlPadGpioAdB108dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB108pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB108pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB108pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB108pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB108pus {
        SwPadCtlPadGpioAdB108pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB108pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB108pus) -> u8 {
        SwPadCtlPadGpioAdB108pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB108speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB108speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB108speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB108speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB108speed {
        SwPadCtlPadGpioAdB108speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB108speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB108speed) -> u8 {
        SwPadCtlPadGpioAdB108speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB109dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB109dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB109dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB109dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB109dse {
        SwPadCtlPadGpioAdB109dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB109dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB109dse) -> u8 {
        SwPadCtlPadGpioAdB109dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB109pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB109pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB109pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB109pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB109pus {
        SwPadCtlPadGpioAdB109pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB109pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB109pus) -> u8 {
        SwPadCtlPadGpioAdB109pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB109speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB109speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB109speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB109speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB109speed {
        SwPadCtlPadGpioAdB109speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB109speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB109speed) -> u8 {
        SwPadCtlPadGpioAdB109speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB110dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB110dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB110dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB110dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB110dse {
        SwPadCtlPadGpioAdB110dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB110dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB110dse) -> u8 {
        SwPadCtlPadGpioAdB110dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB110pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB110pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB110pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB110pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB110pus {
        SwPadCtlPadGpioAdB110pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB110pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB110pus) -> u8 {
        SwPadCtlPadGpioAdB110pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB110speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB110speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB110speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB110speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB110speed {
        SwPadCtlPadGpioAdB110speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB110speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB110speed) -> u8 {
        SwPadCtlPadGpioAdB110speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB111dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB111dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB111dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB111dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB111dse {
        SwPadCtlPadGpioAdB111dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB111dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB111dse) -> u8 {
        SwPadCtlPadGpioAdB111dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB111pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB111pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB111pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB111pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB111pus {
        SwPadCtlPadGpioAdB111pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB111pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB111pus) -> u8 {
        SwPadCtlPadGpioAdB111pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB111speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB111speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB111speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB111speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB111speed {
        SwPadCtlPadGpioAdB111speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB111speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB111speed) -> u8 {
        SwPadCtlPadGpioAdB111speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB112dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB112dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB112dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB112dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB112dse {
        SwPadCtlPadGpioAdB112dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB112dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB112dse) -> u8 {
        SwPadCtlPadGpioAdB112dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB112pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB112pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB112pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB112pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB112pus {
        SwPadCtlPadGpioAdB112pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB112pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB112pus) -> u8 {
        SwPadCtlPadGpioAdB112pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB112speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB112speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB112speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB112speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB112speed {
        SwPadCtlPadGpioAdB112speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB112speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB112speed) -> u8 {
        SwPadCtlPadGpioAdB112speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB113dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB113dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB113dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB113dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB113dse {
        SwPadCtlPadGpioAdB113dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB113dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB113dse) -> u8 {
        SwPadCtlPadGpioAdB113dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB113pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB113pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB113pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB113pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB113pus {
        SwPadCtlPadGpioAdB113pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB113pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB113pus) -> u8 {
        SwPadCtlPadGpioAdB113pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB113speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB113speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB113speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB113speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB113speed {
        SwPadCtlPadGpioAdB113speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB113speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB113speed) -> u8 {
        SwPadCtlPadGpioAdB113speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB114dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB114dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB114dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB114dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB114dse {
        SwPadCtlPadGpioAdB114dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB114dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB114dse) -> u8 {
        SwPadCtlPadGpioAdB114dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB114pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB114pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB114pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB114pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB114pus {
        SwPadCtlPadGpioAdB114pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB114pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB114pus) -> u8 {
        SwPadCtlPadGpioAdB114pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB114speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB114speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB114speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB114speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB114speed {
        SwPadCtlPadGpioAdB114speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB114speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB114speed) -> u8 {
        SwPadCtlPadGpioAdB114speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB115dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioAdB115dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB115dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB115dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB115dse {
        SwPadCtlPadGpioAdB115dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB115dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB115dse) -> u8 {
        SwPadCtlPadGpioAdB115dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB115pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioAdB115pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB115pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB115pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB115pus {
        SwPadCtlPadGpioAdB115pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB115pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB115pus) -> u8 {
        SwPadCtlPadGpioAdB115pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAdB115speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioAdB115speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAdB115speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAdB115speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAdB115speed {
        SwPadCtlPadGpioAdB115speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAdB115speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAdB115speed) -> u8 {
        SwPadCtlPadGpioAdB115speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc00dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc00dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc00dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc00dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc00dse {
        SwPadCtlPadGpioEmc00dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc00dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc00dse) -> u8 {
        SwPadCtlPadGpioEmc00dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc00pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc00pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc00pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc00pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc00pus {
        SwPadCtlPadGpioEmc00pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc00pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc00pus) -> u8 {
        SwPadCtlPadGpioEmc00pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc00speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc00speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc00speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc00speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc00speed {
        SwPadCtlPadGpioEmc00speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc00speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc00speed) -> u8 {
        SwPadCtlPadGpioEmc00speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc01dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc01dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc01dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc01dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc01dse {
        SwPadCtlPadGpioEmc01dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc01dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc01dse) -> u8 {
        SwPadCtlPadGpioEmc01dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc01pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc01pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc01pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc01pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc01pus {
        SwPadCtlPadGpioEmc01pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc01pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc01pus) -> u8 {
        SwPadCtlPadGpioEmc01pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc01speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc01speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc01speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc01speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc01speed {
        SwPadCtlPadGpioEmc01speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc01speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc01speed) -> u8 {
        SwPadCtlPadGpioEmc01speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc02dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc02dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc02dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc02dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc02dse {
        SwPadCtlPadGpioEmc02dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc02dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc02dse) -> u8 {
        SwPadCtlPadGpioEmc02dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc02pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc02pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc02pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc02pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc02pus {
        SwPadCtlPadGpioEmc02pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc02pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc02pus) -> u8 {
        SwPadCtlPadGpioEmc02pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc02speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc02speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc02speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc02speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc02speed {
        SwPadCtlPadGpioEmc02speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc02speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc02speed) -> u8 {
        SwPadCtlPadGpioEmc02speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc03dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc03dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc03dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc03dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc03dse {
        SwPadCtlPadGpioEmc03dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc03dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc03dse) -> u8 {
        SwPadCtlPadGpioEmc03dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc03pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc03pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc03pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc03pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc03pus {
        SwPadCtlPadGpioEmc03pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc03pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc03pus) -> u8 {
        SwPadCtlPadGpioEmc03pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc03speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc03speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc03speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc03speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc03speed {
        SwPadCtlPadGpioEmc03speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc03speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc03speed) -> u8 {
        SwPadCtlPadGpioEmc03speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc04dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc04dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc04dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc04dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc04dse {
        SwPadCtlPadGpioEmc04dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc04dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc04dse) -> u8 {
        SwPadCtlPadGpioEmc04dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc04pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc04pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc04pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc04pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc04pus {
        SwPadCtlPadGpioEmc04pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc04pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc04pus) -> u8 {
        SwPadCtlPadGpioEmc04pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc04speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc04speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc04speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc04speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc04speed {
        SwPadCtlPadGpioEmc04speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc04speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc04speed) -> u8 {
        SwPadCtlPadGpioEmc04speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc05dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc05dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc05dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc05dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc05dse {
        SwPadCtlPadGpioEmc05dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc05dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc05dse) -> u8 {
        SwPadCtlPadGpioEmc05dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc05pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc05pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc05pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc05pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc05pus {
        SwPadCtlPadGpioEmc05pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc05pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc05pus) -> u8 {
        SwPadCtlPadGpioEmc05pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc05speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc05speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc05speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc05speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc05speed {
        SwPadCtlPadGpioEmc05speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc05speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc05speed) -> u8 {
        SwPadCtlPadGpioEmc05speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc06dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc06dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc06dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc06dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc06dse {
        SwPadCtlPadGpioEmc06dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc06dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc06dse) -> u8 {
        SwPadCtlPadGpioEmc06dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc06pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc06pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc06pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc06pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc06pus {
        SwPadCtlPadGpioEmc06pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc06pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc06pus) -> u8 {
        SwPadCtlPadGpioEmc06pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc06speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc06speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc06speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc06speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc06speed {
        SwPadCtlPadGpioEmc06speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc06speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc06speed) -> u8 {
        SwPadCtlPadGpioEmc06speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc07dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc07dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc07dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc07dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc07dse {
        SwPadCtlPadGpioEmc07dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc07dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc07dse) -> u8 {
        SwPadCtlPadGpioEmc07dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc07pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc07pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc07pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc07pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc07pus {
        SwPadCtlPadGpioEmc07pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc07pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc07pus) -> u8 {
        SwPadCtlPadGpioEmc07pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc07speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc07speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc07speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc07speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc07speed {
        SwPadCtlPadGpioEmc07speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc07speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc07speed) -> u8 {
        SwPadCtlPadGpioEmc07speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc08dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc08dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc08dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc08dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc08dse {
        SwPadCtlPadGpioEmc08dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc08dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc08dse) -> u8 {
        SwPadCtlPadGpioEmc08dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc08pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc08pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc08pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc08pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc08pus {
        SwPadCtlPadGpioEmc08pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc08pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc08pus) -> u8 {
        SwPadCtlPadGpioEmc08pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc08speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc08speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc08speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc08speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc08speed {
        SwPadCtlPadGpioEmc08speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc08speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc08speed) -> u8 {
        SwPadCtlPadGpioEmc08speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc09dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc09dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc09dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc09dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc09dse {
        SwPadCtlPadGpioEmc09dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc09dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc09dse) -> u8 {
        SwPadCtlPadGpioEmc09dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc09pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc09pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc09pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc09pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc09pus {
        SwPadCtlPadGpioEmc09pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc09pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc09pus) -> u8 {
        SwPadCtlPadGpioEmc09pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc09speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc09speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc09speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc09speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc09speed {
        SwPadCtlPadGpioEmc09speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc09speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc09speed) -> u8 {
        SwPadCtlPadGpioEmc09speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc10dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc10dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc10dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc10dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc10dse {
        SwPadCtlPadGpioEmc10dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc10dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc10dse) -> u8 {
        SwPadCtlPadGpioEmc10dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc10pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc10pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc10pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc10pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc10pus {
        SwPadCtlPadGpioEmc10pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc10pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc10pus) -> u8 {
        SwPadCtlPadGpioEmc10pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc10speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc10speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc10speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc10speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc10speed {
        SwPadCtlPadGpioEmc10speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc10speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc10speed) -> u8 {
        SwPadCtlPadGpioEmc10speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc11dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc11dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc11dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc11dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc11dse {
        SwPadCtlPadGpioEmc11dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc11dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc11dse) -> u8 {
        SwPadCtlPadGpioEmc11dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc11pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc11pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc11pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc11pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc11pus {
        SwPadCtlPadGpioEmc11pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc11pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc11pus) -> u8 {
        SwPadCtlPadGpioEmc11pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc11speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc11speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc11speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc11speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc11speed {
        SwPadCtlPadGpioEmc11speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc11speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc11speed) -> u8 {
        SwPadCtlPadGpioEmc11speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc12dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc12dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc12dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc12dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc12dse {
        SwPadCtlPadGpioEmc12dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc12dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc12dse) -> u8 {
        SwPadCtlPadGpioEmc12dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc12pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc12pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc12pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc12pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc12pus {
        SwPadCtlPadGpioEmc12pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc12pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc12pus) -> u8 {
        SwPadCtlPadGpioEmc12pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc12speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc12speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc12speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc12speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc12speed {
        SwPadCtlPadGpioEmc12speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc12speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc12speed) -> u8 {
        SwPadCtlPadGpioEmc12speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc13dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc13dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc13dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc13dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc13dse {
        SwPadCtlPadGpioEmc13dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc13dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc13dse) -> u8 {
        SwPadCtlPadGpioEmc13dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc13pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc13pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc13pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc13pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc13pus {
        SwPadCtlPadGpioEmc13pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc13pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc13pus) -> u8 {
        SwPadCtlPadGpioEmc13pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc13speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc13speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc13speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc13speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc13speed {
        SwPadCtlPadGpioEmc13speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc13speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc13speed) -> u8 {
        SwPadCtlPadGpioEmc13speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc14dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc14dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc14dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc14dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc14dse {
        SwPadCtlPadGpioEmc14dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc14dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc14dse) -> u8 {
        SwPadCtlPadGpioEmc14dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc14pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc14pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc14pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc14pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc14pus {
        SwPadCtlPadGpioEmc14pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc14pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc14pus) -> u8 {
        SwPadCtlPadGpioEmc14pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc14speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc14speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc14speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc14speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc14speed {
        SwPadCtlPadGpioEmc14speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc14speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc14speed) -> u8 {
        SwPadCtlPadGpioEmc14speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc15dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc15dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc15dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc15dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc15dse {
        SwPadCtlPadGpioEmc15dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc15dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc15dse) -> u8 {
        SwPadCtlPadGpioEmc15dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc15pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc15pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc15pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc15pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc15pus {
        SwPadCtlPadGpioEmc15pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc15pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc15pus) -> u8 {
        SwPadCtlPadGpioEmc15pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc15speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc15speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc15speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc15speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc15speed {
        SwPadCtlPadGpioEmc15speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc15speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc15speed) -> u8 {
        SwPadCtlPadGpioEmc15speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc16dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc16dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc16dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc16dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc16dse {
        SwPadCtlPadGpioEmc16dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc16dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc16dse) -> u8 {
        SwPadCtlPadGpioEmc16dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc16pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc16pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc16pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc16pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc16pus {
        SwPadCtlPadGpioEmc16pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc16pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc16pus) -> u8 {
        SwPadCtlPadGpioEmc16pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc16speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc16speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc16speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc16speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc16speed {
        SwPadCtlPadGpioEmc16speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc16speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc16speed) -> u8 {
        SwPadCtlPadGpioEmc16speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc17dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc17dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc17dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc17dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc17dse {
        SwPadCtlPadGpioEmc17dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc17dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc17dse) -> u8 {
        SwPadCtlPadGpioEmc17dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc17pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc17pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc17pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc17pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc17pus {
        SwPadCtlPadGpioEmc17pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc17pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc17pus) -> u8 {
        SwPadCtlPadGpioEmc17pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc17speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc17speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc17speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc17speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc17speed {
        SwPadCtlPadGpioEmc17speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc17speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc17speed) -> u8 {
        SwPadCtlPadGpioEmc17speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc18dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc18dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc18dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc18dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc18dse {
        SwPadCtlPadGpioEmc18dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc18dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc18dse) -> u8 {
        SwPadCtlPadGpioEmc18dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc18pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc18pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc18pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc18pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc18pus {
        SwPadCtlPadGpioEmc18pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc18pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc18pus) -> u8 {
        SwPadCtlPadGpioEmc18pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc18speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc18speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc18speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc18speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc18speed {
        SwPadCtlPadGpioEmc18speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc18speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc18speed) -> u8 {
        SwPadCtlPadGpioEmc18speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc19dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc19dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc19dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc19dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc19dse {
        SwPadCtlPadGpioEmc19dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc19dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc19dse) -> u8 {
        SwPadCtlPadGpioEmc19dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc19pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc19pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc19pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc19pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc19pus {
        SwPadCtlPadGpioEmc19pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc19pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc19pus) -> u8 {
        SwPadCtlPadGpioEmc19pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc19speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc19speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc19speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc19speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc19speed {
        SwPadCtlPadGpioEmc19speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc19speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc19speed) -> u8 {
        SwPadCtlPadGpioEmc19speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc20dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc20dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc20dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc20dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc20dse {
        SwPadCtlPadGpioEmc20dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc20dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc20dse) -> u8 {
        SwPadCtlPadGpioEmc20dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc20pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc20pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc20pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc20pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc20pus {
        SwPadCtlPadGpioEmc20pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc20pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc20pus) -> u8 {
        SwPadCtlPadGpioEmc20pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc20speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc20speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc20speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc20speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc20speed {
        SwPadCtlPadGpioEmc20speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc20speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc20speed) -> u8 {
        SwPadCtlPadGpioEmc20speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc21dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc21dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc21dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc21dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc21dse {
        SwPadCtlPadGpioEmc21dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc21dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc21dse) -> u8 {
        SwPadCtlPadGpioEmc21dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc21pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc21pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc21pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc21pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc21pus {
        SwPadCtlPadGpioEmc21pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc21pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc21pus) -> u8 {
        SwPadCtlPadGpioEmc21pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc21speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc21speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc21speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc21speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc21speed {
        SwPadCtlPadGpioEmc21speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc21speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc21speed) -> u8 {
        SwPadCtlPadGpioEmc21speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc22dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc22dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc22dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc22dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc22dse {
        SwPadCtlPadGpioEmc22dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc22dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc22dse) -> u8 {
        SwPadCtlPadGpioEmc22dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc22pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc22pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc22pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc22pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc22pus {
        SwPadCtlPadGpioEmc22pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc22pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc22pus) -> u8 {
        SwPadCtlPadGpioEmc22pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc22speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc22speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc22speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc22speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc22speed {
        SwPadCtlPadGpioEmc22speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc22speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc22speed) -> u8 {
        SwPadCtlPadGpioEmc22speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc23dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc23dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc23dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc23dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc23dse {
        SwPadCtlPadGpioEmc23dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc23dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc23dse) -> u8 {
        SwPadCtlPadGpioEmc23dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc23pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc23pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc23pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc23pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc23pus {
        SwPadCtlPadGpioEmc23pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc23pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc23pus) -> u8 {
        SwPadCtlPadGpioEmc23pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc23speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc23speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc23speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc23speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc23speed {
        SwPadCtlPadGpioEmc23speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc23speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc23speed) -> u8 {
        SwPadCtlPadGpioEmc23speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc24dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc24dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc24dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc24dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc24dse {
        SwPadCtlPadGpioEmc24dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc24dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc24dse) -> u8 {
        SwPadCtlPadGpioEmc24dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc24pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc24pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc24pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc24pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc24pus {
        SwPadCtlPadGpioEmc24pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc24pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc24pus) -> u8 {
        SwPadCtlPadGpioEmc24pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc24speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc24speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc24speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc24speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc24speed {
        SwPadCtlPadGpioEmc24speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc24speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc24speed) -> u8 {
        SwPadCtlPadGpioEmc24speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc25dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc25dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc25dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc25dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc25dse {
        SwPadCtlPadGpioEmc25dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc25dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc25dse) -> u8 {
        SwPadCtlPadGpioEmc25dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc25pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc25pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc25pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc25pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc25pus {
        SwPadCtlPadGpioEmc25pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc25pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc25pus) -> u8 {
        SwPadCtlPadGpioEmc25pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc25speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc25speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc25speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc25speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc25speed {
        SwPadCtlPadGpioEmc25speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc25speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc25speed) -> u8 {
        SwPadCtlPadGpioEmc25speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc26dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc26dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc26dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc26dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc26dse {
        SwPadCtlPadGpioEmc26dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc26dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc26dse) -> u8 {
        SwPadCtlPadGpioEmc26dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc26pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc26pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc26pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc26pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc26pus {
        SwPadCtlPadGpioEmc26pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc26pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc26pus) -> u8 {
        SwPadCtlPadGpioEmc26pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc26speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc26speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc26speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc26speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc26speed {
        SwPadCtlPadGpioEmc26speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc26speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc26speed) -> u8 {
        SwPadCtlPadGpioEmc26speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc27dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc27dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc27dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc27dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc27dse {
        SwPadCtlPadGpioEmc27dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc27dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc27dse) -> u8 {
        SwPadCtlPadGpioEmc27dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc27pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc27pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc27pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc27pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc27pus {
        SwPadCtlPadGpioEmc27pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc27pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc27pus) -> u8 {
        SwPadCtlPadGpioEmc27pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc27speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc27speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc27speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc27speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc27speed {
        SwPadCtlPadGpioEmc27speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc27speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc27speed) -> u8 {
        SwPadCtlPadGpioEmc27speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc28dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc28dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc28dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc28dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc28dse {
        SwPadCtlPadGpioEmc28dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc28dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc28dse) -> u8 {
        SwPadCtlPadGpioEmc28dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc28pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc28pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc28pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc28pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc28pus {
        SwPadCtlPadGpioEmc28pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc28pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc28pus) -> u8 {
        SwPadCtlPadGpioEmc28pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc28speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc28speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc28speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc28speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc28speed {
        SwPadCtlPadGpioEmc28speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc28speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc28speed) -> u8 {
        SwPadCtlPadGpioEmc28speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc29dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc29dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc29dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc29dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc29dse {
        SwPadCtlPadGpioEmc29dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc29dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc29dse) -> u8 {
        SwPadCtlPadGpioEmc29dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc29pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc29pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc29pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc29pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc29pus {
        SwPadCtlPadGpioEmc29pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc29pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc29pus) -> u8 {
        SwPadCtlPadGpioEmc29pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc29speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc29speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc29speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc29speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc29speed {
        SwPadCtlPadGpioEmc29speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc29speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc29speed) -> u8 {
        SwPadCtlPadGpioEmc29speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc30dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc30dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc30dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc30dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc30dse {
        SwPadCtlPadGpioEmc30dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc30dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc30dse) -> u8 {
        SwPadCtlPadGpioEmc30dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc30pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc30pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc30pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc30pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc30pus {
        SwPadCtlPadGpioEmc30pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc30pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc30pus) -> u8 {
        SwPadCtlPadGpioEmc30pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc30speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc30speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc30speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc30speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc30speed {
        SwPadCtlPadGpioEmc30speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc30speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc30speed) -> u8 {
        SwPadCtlPadGpioEmc30speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc31dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc31dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc31dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc31dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc31dse {
        SwPadCtlPadGpioEmc31dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc31dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc31dse) -> u8 {
        SwPadCtlPadGpioEmc31dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc31pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc31pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc31pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc31pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc31pus {
        SwPadCtlPadGpioEmc31pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc31pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc31pus) -> u8 {
        SwPadCtlPadGpioEmc31pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc31speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc31speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc31speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc31speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc31speed {
        SwPadCtlPadGpioEmc31speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc31speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc31speed) -> u8 {
        SwPadCtlPadGpioEmc31speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc32dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc32dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc32dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc32dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc32dse {
        SwPadCtlPadGpioEmc32dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc32dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc32dse) -> u8 {
        SwPadCtlPadGpioEmc32dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc32pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc32pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc32pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc32pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc32pus {
        SwPadCtlPadGpioEmc32pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc32pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc32pus) -> u8 {
        SwPadCtlPadGpioEmc32pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc32speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc32speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc32speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc32speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc32speed {
        SwPadCtlPadGpioEmc32speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc32speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc32speed) -> u8 {
        SwPadCtlPadGpioEmc32speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc33dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc33dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc33dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc33dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc33dse {
        SwPadCtlPadGpioEmc33dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc33dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc33dse) -> u8 {
        SwPadCtlPadGpioEmc33dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc33pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc33pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc33pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc33pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc33pus {
        SwPadCtlPadGpioEmc33pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc33pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc33pus) -> u8 {
        SwPadCtlPadGpioEmc33pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc33speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc33speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc33speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc33speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc33speed {
        SwPadCtlPadGpioEmc33speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc33speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc33speed) -> u8 {
        SwPadCtlPadGpioEmc33speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc34dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc34dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc34dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc34dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc34dse {
        SwPadCtlPadGpioEmc34dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc34dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc34dse) -> u8 {
        SwPadCtlPadGpioEmc34dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc34pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc34pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc34pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc34pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc34pus {
        SwPadCtlPadGpioEmc34pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc34pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc34pus) -> u8 {
        SwPadCtlPadGpioEmc34pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc34speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc34speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc34speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc34speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc34speed {
        SwPadCtlPadGpioEmc34speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc34speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc34speed) -> u8 {
        SwPadCtlPadGpioEmc34speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc35dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc35dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc35dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc35dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc35dse {
        SwPadCtlPadGpioEmc35dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc35dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc35dse) -> u8 {
        SwPadCtlPadGpioEmc35dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc35pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc35pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc35pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc35pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc35pus {
        SwPadCtlPadGpioEmc35pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc35pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc35pus) -> u8 {
        SwPadCtlPadGpioEmc35pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc35speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc35speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc35speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc35speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc35speed {
        SwPadCtlPadGpioEmc35speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc35speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc35speed) -> u8 {
        SwPadCtlPadGpioEmc35speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc36dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc36dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc36dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc36dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc36dse {
        SwPadCtlPadGpioEmc36dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc36dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc36dse) -> u8 {
        SwPadCtlPadGpioEmc36dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc36pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc36pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc36pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc36pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc36pus {
        SwPadCtlPadGpioEmc36pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc36pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc36pus) -> u8 {
        SwPadCtlPadGpioEmc36pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc36speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc36speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc36speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc36speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc36speed {
        SwPadCtlPadGpioEmc36speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc36speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc36speed) -> u8 {
        SwPadCtlPadGpioEmc36speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc37dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc37dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc37dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc37dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc37dse {
        SwPadCtlPadGpioEmc37dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc37dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc37dse) -> u8 {
        SwPadCtlPadGpioEmc37dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc37pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc37pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc37pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc37pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc37pus {
        SwPadCtlPadGpioEmc37pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc37pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc37pus) -> u8 {
        SwPadCtlPadGpioEmc37pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc37speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc37speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc37speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc37speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc37speed {
        SwPadCtlPadGpioEmc37speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc37speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc37speed) -> u8 {
        SwPadCtlPadGpioEmc37speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc38dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc38dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc38dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc38dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc38dse {
        SwPadCtlPadGpioEmc38dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc38dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc38dse) -> u8 {
        SwPadCtlPadGpioEmc38dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc38pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc38pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc38pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc38pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc38pus {
        SwPadCtlPadGpioEmc38pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc38pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc38pus) -> u8 {
        SwPadCtlPadGpioEmc38pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc38speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc38speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc38speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc38speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc38speed {
        SwPadCtlPadGpioEmc38speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc38speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc38speed) -> u8 {
        SwPadCtlPadGpioEmc38speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc39dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc39dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc39dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc39dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc39dse {
        SwPadCtlPadGpioEmc39dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc39dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc39dse) -> u8 {
        SwPadCtlPadGpioEmc39dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc39pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc39pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc39pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc39pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc39pus {
        SwPadCtlPadGpioEmc39pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc39pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc39pus) -> u8 {
        SwPadCtlPadGpioEmc39pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc39speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc39speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc39speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc39speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc39speed {
        SwPadCtlPadGpioEmc39speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc39speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc39speed) -> u8 {
        SwPadCtlPadGpioEmc39speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc40dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc40dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc40dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc40dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc40dse {
        SwPadCtlPadGpioEmc40dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc40dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc40dse) -> u8 {
        SwPadCtlPadGpioEmc40dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc40pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc40pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc40pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc40pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc40pus {
        SwPadCtlPadGpioEmc40pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc40pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc40pus) -> u8 {
        SwPadCtlPadGpioEmc40pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc40speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc40speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc40speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc40speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc40speed {
        SwPadCtlPadGpioEmc40speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc40speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc40speed) -> u8 {
        SwPadCtlPadGpioEmc40speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc41dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioEmc41dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc41dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc41dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc41dse {
        SwPadCtlPadGpioEmc41dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc41dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc41dse) -> u8 {
        SwPadCtlPadGpioEmc41dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc41pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioEmc41pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc41pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc41pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc41pus {
        SwPadCtlPadGpioEmc41pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc41pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc41pus) -> u8 {
        SwPadCtlPadGpioEmc41pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioEmc41speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioEmc41speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioEmc41speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioEmc41speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioEmc41speed {
        SwPadCtlPadGpioEmc41speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioEmc41speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioEmc41speed) -> u8 {
        SwPadCtlPadGpioEmc41speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB000dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB000dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB000dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB000dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB000dse {
        SwPadCtlPadGpioSdB000dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB000dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB000dse) -> u8 {
        SwPadCtlPadGpioSdB000dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB000pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB000pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB000pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB000pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB000pus {
        SwPadCtlPadGpioSdB000pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB000pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB000pus) -> u8 {
        SwPadCtlPadGpioSdB000pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB000speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB000speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB000speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB000speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB000speed {
        SwPadCtlPadGpioSdB000speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB000speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB000speed) -> u8 {
        SwPadCtlPadGpioSdB000speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB001dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB001dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB001dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB001dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB001dse {
        SwPadCtlPadGpioSdB001dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB001dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB001dse) -> u8 {
        SwPadCtlPadGpioSdB001dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB001pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB001pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB001pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB001pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB001pus {
        SwPadCtlPadGpioSdB001pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB001pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB001pus) -> u8 {
        SwPadCtlPadGpioSdB001pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB001speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB001speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB001speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB001speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB001speed {
        SwPadCtlPadGpioSdB001speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB001speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB001speed) -> u8 {
        SwPadCtlPadGpioSdB001speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB002dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB002dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB002dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB002dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB002dse {
        SwPadCtlPadGpioSdB002dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB002dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB002dse) -> u8 {
        SwPadCtlPadGpioSdB002dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB002pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB002pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB002pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB002pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB002pus {
        SwPadCtlPadGpioSdB002pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB002pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB002pus) -> u8 {
        SwPadCtlPadGpioSdB002pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB002speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB002speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB002speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB002speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB002speed {
        SwPadCtlPadGpioSdB002speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB002speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB002speed) -> u8 {
        SwPadCtlPadGpioSdB002speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB003dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB003dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB003dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB003dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB003dse {
        SwPadCtlPadGpioSdB003dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB003dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB003dse) -> u8 {
        SwPadCtlPadGpioSdB003dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB003pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB003pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB003pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB003pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB003pus {
        SwPadCtlPadGpioSdB003pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB003pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB003pus) -> u8 {
        SwPadCtlPadGpioSdB003pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB003speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB003speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB003speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB003speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB003speed {
        SwPadCtlPadGpioSdB003speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB003speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB003speed) -> u8 {
        SwPadCtlPadGpioSdB003speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB004dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB004dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB004dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB004dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB004dse {
        SwPadCtlPadGpioSdB004dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB004dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB004dse) -> u8 {
        SwPadCtlPadGpioSdB004dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB004pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB004pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB004pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB004pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB004pus {
        SwPadCtlPadGpioSdB004pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB004pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB004pus) -> u8 {
        SwPadCtlPadGpioSdB004pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB004speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB004speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB004speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB004speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB004speed {
        SwPadCtlPadGpioSdB004speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB004speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB004speed) -> u8 {
        SwPadCtlPadGpioSdB004speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB005dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB005dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB005dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB005dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB005dse {
        SwPadCtlPadGpioSdB005dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB005dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB005dse) -> u8 {
        SwPadCtlPadGpioSdB005dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB005pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB005pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB005pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB005pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB005pus {
        SwPadCtlPadGpioSdB005pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB005pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB005pus) -> u8 {
        SwPadCtlPadGpioSdB005pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB005speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB005speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB005speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB005speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB005speed {
        SwPadCtlPadGpioSdB005speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB005speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB005speed) -> u8 {
        SwPadCtlPadGpioSdB005speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB100dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB100dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB100dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB100dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB100dse {
        SwPadCtlPadGpioSdB100dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB100dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB100dse) -> u8 {
        SwPadCtlPadGpioSdB100dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB100pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB100pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB100pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB100pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB100pus {
        SwPadCtlPadGpioSdB100pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB100pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB100pus) -> u8 {
        SwPadCtlPadGpioSdB100pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB100speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB100speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB100speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB100speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB100speed {
        SwPadCtlPadGpioSdB100speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB100speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB100speed) -> u8 {
        SwPadCtlPadGpioSdB100speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB101dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB101dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB101dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB101dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB101dse {
        SwPadCtlPadGpioSdB101dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB101dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB101dse) -> u8 {
        SwPadCtlPadGpioSdB101dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB101pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB101pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB101pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB101pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB101pus {
        SwPadCtlPadGpioSdB101pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB101pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB101pus) -> u8 {
        SwPadCtlPadGpioSdB101pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB101speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB101speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB101speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB101speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB101speed {
        SwPadCtlPadGpioSdB101speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB101speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB101speed) -> u8 {
        SwPadCtlPadGpioSdB101speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB102dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB102dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB102dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB102dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB102dse {
        SwPadCtlPadGpioSdB102dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB102dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB102dse) -> u8 {
        SwPadCtlPadGpioSdB102dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB102pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB102pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB102pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB102pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB102pus {
        SwPadCtlPadGpioSdB102pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB102pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB102pus) -> u8 {
        SwPadCtlPadGpioSdB102pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB102speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB102speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB102speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB102speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB102speed {
        SwPadCtlPadGpioSdB102speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB102speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB102speed) -> u8 {
        SwPadCtlPadGpioSdB102speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB103dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB103dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB103dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB103dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB103dse {
        SwPadCtlPadGpioSdB103dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB103dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB103dse) -> u8 {
        SwPadCtlPadGpioSdB103dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB103pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB103pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB103pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB103pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB103pus {
        SwPadCtlPadGpioSdB103pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB103pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB103pus) -> u8 {
        SwPadCtlPadGpioSdB103pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB103speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB103speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB103speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB103speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB103speed {
        SwPadCtlPadGpioSdB103speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB103speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB103speed) -> u8 {
        SwPadCtlPadGpioSdB103speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB104dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB104dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB104dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB104dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB104dse {
        SwPadCtlPadGpioSdB104dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB104dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB104dse) -> u8 {
        SwPadCtlPadGpioSdB104dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB104pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB104pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB104pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB104pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB104pus {
        SwPadCtlPadGpioSdB104pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB104pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB104pus) -> u8 {
        SwPadCtlPadGpioSdB104pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB104speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB104speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB104speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB104speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB104speed {
        SwPadCtlPadGpioSdB104speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB104speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB104speed) -> u8 {
        SwPadCtlPadGpioSdB104speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB105dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB105dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB105dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB105dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB105dse {
        SwPadCtlPadGpioSdB105dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB105dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB105dse) -> u8 {
        SwPadCtlPadGpioSdB105dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB105pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB105pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB105pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB105pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB105pus {
        SwPadCtlPadGpioSdB105pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB105pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB105pus) -> u8 {
        SwPadCtlPadGpioSdB105pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB105speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB105speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB105speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB105speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB105speed {
        SwPadCtlPadGpioSdB105speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB105speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB105speed) -> u8 {
        SwPadCtlPadGpioSdB105speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB106dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB106dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB106dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB106dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB106dse {
        SwPadCtlPadGpioSdB106dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB106dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB106dse) -> u8 {
        SwPadCtlPadGpioSdB106dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB106pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB106pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB106pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB106pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB106pus {
        SwPadCtlPadGpioSdB106pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB106pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB106pus) -> u8 {
        SwPadCtlPadGpioSdB106pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB106speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB106speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB106speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB106speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB106speed {
        SwPadCtlPadGpioSdB106speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB106speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB106speed) -> u8 {
        SwPadCtlPadGpioSdB106speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB107dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB107dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB107dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB107dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB107dse {
        SwPadCtlPadGpioSdB107dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB107dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB107dse) -> u8 {
        SwPadCtlPadGpioSdB107dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB107pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB107pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB107pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB107pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB107pus {
        SwPadCtlPadGpioSdB107pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB107pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB107pus) -> u8 {
        SwPadCtlPadGpioSdB107pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB107speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB107speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB107speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB107speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB107speed {
        SwPadCtlPadGpioSdB107speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB107speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB107speed) -> u8 {
        SwPadCtlPadGpioSdB107speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB108dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB108dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB108dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB108dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB108dse {
        SwPadCtlPadGpioSdB108dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB108dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB108dse) -> u8 {
        SwPadCtlPadGpioSdB108dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB108pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB108pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB108pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB108pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB108pus {
        SwPadCtlPadGpioSdB108pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB108pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB108pus) -> u8 {
        SwPadCtlPadGpioSdB108pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB108speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB108speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB108speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB108speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB108speed {
        SwPadCtlPadGpioSdB108speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB108speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB108speed) -> u8 {
        SwPadCtlPadGpioSdB108speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB109dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB109dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB109dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB109dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB109dse {
        SwPadCtlPadGpioSdB109dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB109dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB109dse) -> u8 {
        SwPadCtlPadGpioSdB109dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB109pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB109pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB109pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB109pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB109pus {
        SwPadCtlPadGpioSdB109pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB109pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB109pus) -> u8 {
        SwPadCtlPadGpioSdB109pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB109speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB109speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB109speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB109speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB109speed {
        SwPadCtlPadGpioSdB109speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB109speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB109speed) -> u8 {
        SwPadCtlPadGpioSdB109speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB110dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB110dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB110dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB110dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB110dse {
        SwPadCtlPadGpioSdB110dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB110dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB110dse) -> u8 {
        SwPadCtlPadGpioSdB110dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB110pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB110pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB110pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB110pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB110pus {
        SwPadCtlPadGpioSdB110pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB110pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB110pus) -> u8 {
        SwPadCtlPadGpioSdB110pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB110speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB110speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB110speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB110speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB110speed {
        SwPadCtlPadGpioSdB110speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB110speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB110speed) -> u8 {
        SwPadCtlPadGpioSdB110speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB111dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
impl SwPadCtlPadGpioSdB111dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB111dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB111dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB111dse {
        SwPadCtlPadGpioSdB111dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB111dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB111dse) -> u8 {
        SwPadCtlPadGpioSdB111dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB111pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSdB111pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB111pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB111pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB111pus {
        SwPadCtlPadGpioSdB111pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB111pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB111pus) -> u8 {
        SwPadCtlPadGpioSdB111pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSdB111speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSdB111speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSdB111speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSdB111speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSdB111speed {
        SwPadCtlPadGpioSdB111speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSdB111speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSdB111speed) -> u8 {
        SwPadCtlPadGpioSdB111speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB000dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB000dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB000dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB000dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB000dse {
        SwPadCtlPadGpioSpiB000dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB000dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB000dse) -> u8 {
        SwPadCtlPadGpioSpiB000dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB000pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB000pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB000pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB000pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB000pus {
        SwPadCtlPadGpioSpiB000pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB000pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB000pus) -> u8 {
        SwPadCtlPadGpioSpiB000pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB000speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB000speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB000speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB000speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB000speed {
        SwPadCtlPadGpioSpiB000speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB000speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB000speed) -> u8 {
        SwPadCtlPadGpioSpiB000speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB001dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB001dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB001dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB001dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB001dse {
        SwPadCtlPadGpioSpiB001dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB001dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB001dse) -> u8 {
        SwPadCtlPadGpioSpiB001dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB001pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB001pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB001pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB001pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB001pus {
        SwPadCtlPadGpioSpiB001pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB001pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB001pus) -> u8 {
        SwPadCtlPadGpioSpiB001pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB001speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB001speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB001speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB001speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB001speed {
        SwPadCtlPadGpioSpiB001speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB001speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB001speed) -> u8 {
        SwPadCtlPadGpioSpiB001speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB002dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB002dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB002dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB002dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB002dse {
        SwPadCtlPadGpioSpiB002dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB002dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB002dse) -> u8 {
        SwPadCtlPadGpioSpiB002dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB002pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB002pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB002pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB002pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB002pus {
        SwPadCtlPadGpioSpiB002pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB002pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB002pus) -> u8 {
        SwPadCtlPadGpioSpiB002pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB002speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB002speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB002speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB002speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB002speed {
        SwPadCtlPadGpioSpiB002speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB002speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB002speed) -> u8 {
        SwPadCtlPadGpioSpiB002speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB003dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB003dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB003dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB003dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB003dse {
        SwPadCtlPadGpioSpiB003dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB003dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB003dse) -> u8 {
        SwPadCtlPadGpioSpiB003dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB003pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB003pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB003pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB003pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB003pus {
        SwPadCtlPadGpioSpiB003pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB003pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB003pus) -> u8 {
        SwPadCtlPadGpioSpiB003pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB003speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB003speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB003speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB003speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB003speed {
        SwPadCtlPadGpioSpiB003speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB003speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB003speed) -> u8 {
        SwPadCtlPadGpioSpiB003speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB004dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB004dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB004dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB004dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB004dse {
        SwPadCtlPadGpioSpiB004dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB004dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB004dse) -> u8 {
        SwPadCtlPadGpioSpiB004dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB004pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB004pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB004pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB004pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB004pus {
        SwPadCtlPadGpioSpiB004pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB004pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB004pus) -> u8 {
        SwPadCtlPadGpioSpiB004pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB004speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB004speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB004speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB004speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB004speed {
        SwPadCtlPadGpioSpiB004speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB004speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB004speed) -> u8 {
        SwPadCtlPadGpioSpiB004speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB005dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB005dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB005dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB005dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB005dse {
        SwPadCtlPadGpioSpiB005dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB005dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB005dse) -> u8 {
        SwPadCtlPadGpioSpiB005dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB005pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB005pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB005pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB005pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB005pus {
        SwPadCtlPadGpioSpiB005pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB005pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB005pus) -> u8 {
        SwPadCtlPadGpioSpiB005pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB005speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB005speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB005speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB005speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB005speed {
        SwPadCtlPadGpioSpiB005speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB005speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB005speed) -> u8 {
        SwPadCtlPadGpioSpiB005speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB006dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB006dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB006dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB006dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB006dse {
        SwPadCtlPadGpioSpiB006dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB006dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB006dse) -> u8 {
        SwPadCtlPadGpioSpiB006dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB006pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB006pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB006pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB006pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB006pus {
        SwPadCtlPadGpioSpiB006pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB006pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB006pus) -> u8 {
        SwPadCtlPadGpioSpiB006pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB006speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB006speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB006speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB006speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB006speed {
        SwPadCtlPadGpioSpiB006speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB006speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB006speed) -> u8 {
        SwPadCtlPadGpioSpiB006speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB007dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB007dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB007dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB007dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB007dse {
        SwPadCtlPadGpioSpiB007dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB007dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB007dse) -> u8 {
        SwPadCtlPadGpioSpiB007dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB007pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB007pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB007pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB007pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB007pus {
        SwPadCtlPadGpioSpiB007pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB007pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB007pus) -> u8 {
        SwPadCtlPadGpioSpiB007pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB007speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB007speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB007speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB007speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB007speed {
        SwPadCtlPadGpioSpiB007speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB007speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB007speed) -> u8 {
        SwPadCtlPadGpioSpiB007speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB008dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB008dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB008dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB008dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB008dse {
        SwPadCtlPadGpioSpiB008dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB008dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB008dse) -> u8 {
        SwPadCtlPadGpioSpiB008dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB008pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB008pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB008pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB008pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB008pus {
        SwPadCtlPadGpioSpiB008pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB008pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB008pus) -> u8 {
        SwPadCtlPadGpioSpiB008pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB008speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB008speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB008speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB008speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB008speed {
        SwPadCtlPadGpioSpiB008speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB008speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB008speed) -> u8 {
        SwPadCtlPadGpioSpiB008speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB009dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB009dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB009dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB009dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB009dse {
        SwPadCtlPadGpioSpiB009dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB009dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB009dse) -> u8 {
        SwPadCtlPadGpioSpiB009dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB009pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB009pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB009pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB009pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB009pus {
        SwPadCtlPadGpioSpiB009pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB009pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB009pus) -> u8 {
        SwPadCtlPadGpioSpiB009pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB009speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB009speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB009speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB009speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB009speed {
        SwPadCtlPadGpioSpiB009speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB009speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB009speed) -> u8 {
        SwPadCtlPadGpioSpiB009speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB010dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB010dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB010dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB010dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB010dse {
        SwPadCtlPadGpioSpiB010dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB010dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB010dse) -> u8 {
        SwPadCtlPadGpioSpiB010dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB010pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB010pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB010pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB010pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB010pus {
        SwPadCtlPadGpioSpiB010pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB010pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB010pus) -> u8 {
        SwPadCtlPadGpioSpiB010pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB010speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB010speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB010speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB010speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB010speed {
        SwPadCtlPadGpioSpiB010speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB010speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB010speed) -> u8 {
        SwPadCtlPadGpioSpiB010speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB011dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB011dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB011dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB011dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB011dse {
        SwPadCtlPadGpioSpiB011dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB011dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB011dse) -> u8 {
        SwPadCtlPadGpioSpiB011dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB011pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB011pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB011pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB011pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB011pus {
        SwPadCtlPadGpioSpiB011pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB011pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB011pus) -> u8 {
        SwPadCtlPadGpioSpiB011pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB011speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB011speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB011speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB011speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB011speed {
        SwPadCtlPadGpioSpiB011speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB011speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB011speed) -> u8 {
        SwPadCtlPadGpioSpiB011speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB012dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB012dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB012dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB012dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB012dse {
        SwPadCtlPadGpioSpiB012dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB012dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB012dse) -> u8 {
        SwPadCtlPadGpioSpiB012dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB012pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB012pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB012pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB012pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB012pus {
        SwPadCtlPadGpioSpiB012pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB012pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB012pus) -> u8 {
        SwPadCtlPadGpioSpiB012pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB012speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB012speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB012speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB012speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB012speed {
        SwPadCtlPadGpioSpiB012speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB012speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB012speed) -> u8 {
        SwPadCtlPadGpioSpiB012speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB013dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB013dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB013dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB013dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB013dse {
        SwPadCtlPadGpioSpiB013dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB013dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB013dse) -> u8 {
        SwPadCtlPadGpioSpiB013dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB013pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB013pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB013pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB013pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB013pus {
        SwPadCtlPadGpioSpiB013pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB013pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB013pus) -> u8 {
        SwPadCtlPadGpioSpiB013pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB013speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB013speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB013speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB013speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB013speed {
        SwPadCtlPadGpioSpiB013speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB013speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB013speed) -> u8 {
        SwPadCtlPadGpioSpiB013speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB100dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB100dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB100dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB100dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB100dse {
        SwPadCtlPadGpioSpiB100dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB100dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB100dse) -> u8 {
        SwPadCtlPadGpioSpiB100dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB100pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB100pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB100pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB100pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB100pus {
        SwPadCtlPadGpioSpiB100pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB100pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB100pus) -> u8 {
        SwPadCtlPadGpioSpiB100pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB100speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB100speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB100speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB100speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB100speed {
        SwPadCtlPadGpioSpiB100speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB100speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB100speed) -> u8 {
        SwPadCtlPadGpioSpiB100speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB101dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB101dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB101dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB101dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB101dse {
        SwPadCtlPadGpioSpiB101dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB101dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB101dse) -> u8 {
        SwPadCtlPadGpioSpiB101dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB101pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB101pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB101pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB101pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB101pus {
        SwPadCtlPadGpioSpiB101pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB101pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB101pus) -> u8 {
        SwPadCtlPadGpioSpiB101pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB101speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB101speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB101speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB101speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB101speed {
        SwPadCtlPadGpioSpiB101speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB101speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB101speed) -> u8 {
        SwPadCtlPadGpioSpiB101speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB102dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB102dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB102dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB102dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB102dse {
        SwPadCtlPadGpioSpiB102dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB102dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB102dse) -> u8 {
        SwPadCtlPadGpioSpiB102dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB102pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB102pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB102pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB102pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB102pus {
        SwPadCtlPadGpioSpiB102pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB102pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB102pus) -> u8 {
        SwPadCtlPadGpioSpiB102pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB102speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB102speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB102speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB102speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB102speed {
        SwPadCtlPadGpioSpiB102speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB102speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB102speed) -> u8 {
        SwPadCtlPadGpioSpiB102speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB103dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB103dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB103dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB103dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB103dse {
        SwPadCtlPadGpioSpiB103dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB103dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB103dse) -> u8 {
        SwPadCtlPadGpioSpiB103dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB103pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB103pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB103pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB103pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB103pus {
        SwPadCtlPadGpioSpiB103pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB103pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB103pus) -> u8 {
        SwPadCtlPadGpioSpiB103pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB103speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB103speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB103speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB103speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB103speed {
        SwPadCtlPadGpioSpiB103speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB103speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB103speed) -> u8 {
        SwPadCtlPadGpioSpiB103speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB104dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB104dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB104dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB104dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB104dse {
        SwPadCtlPadGpioSpiB104dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB104dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB104dse) -> u8 {
        SwPadCtlPadGpioSpiB104dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB104pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB104pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB104pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB104pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB104pus {
        SwPadCtlPadGpioSpiB104pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB104pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB104pus) -> u8 {
        SwPadCtlPadGpioSpiB104pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB104speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB104speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB104speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB104speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB104speed {
        SwPadCtlPadGpioSpiB104speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB104speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB104speed) -> u8 {
        SwPadCtlPadGpioSpiB104speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB105dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB105dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB105dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB105dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB105dse {
        SwPadCtlPadGpioSpiB105dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB105dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB105dse) -> u8 {
        SwPadCtlPadGpioSpiB105dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB105pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB105pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB105pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB105pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB105pus {
        SwPadCtlPadGpioSpiB105pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB105pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB105pus) -> u8 {
        SwPadCtlPadGpioSpiB105pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB105speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB105speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB105speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB105speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB105speed {
        SwPadCtlPadGpioSpiB105speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB105speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB105speed) -> u8 {
        SwPadCtlPadGpioSpiB105speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB106dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB106dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB106dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB106dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB106dse {
        SwPadCtlPadGpioSpiB106dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB106dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB106dse) -> u8 {
        SwPadCtlPadGpioSpiB106dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB106pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB106pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB106pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB106pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB106pus {
        SwPadCtlPadGpioSpiB106pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB106pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB106pus) -> u8 {
        SwPadCtlPadGpioSpiB106pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB106speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB106speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB106speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB106speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB106speed {
        SwPadCtlPadGpioSpiB106speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB106speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB106speed) -> u8 {
        SwPadCtlPadGpioSpiB106speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB107dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V_ = 0x01,
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
impl SwPadCtlPadGpioSpiB107dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB107dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB107dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB107dse {
        SwPadCtlPadGpioSpiB107dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB107dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB107dse) -> u8 {
        SwPadCtlPadGpioSpiB107dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB107pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadGpioSpiB107pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB107pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB107pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB107pus {
        SwPadCtlPadGpioSpiB107pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB107pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB107pus) -> u8 {
        SwPadCtlPadGpioSpiB107pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioSpiB107speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl SwPadCtlPadGpioSpiB107speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioSpiB107speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioSpiB107speed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioSpiB107speed {
        SwPadCtlPadGpioSpiB107speed::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioSpiB107speed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioSpiB107speed) -> u8 {
        SwPadCtlPadGpioSpiB107speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1cdBselectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT6"]
    GPIO_EMC_35_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT6"]
    GPIO_AD_B1_02_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT6"]
    GPIO_B1_12_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usdhc1cdBselectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1cdBselectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1cdBselectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1cdBselectInputDaisy {
        Usdhc1cdBselectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc1cdBselectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1cdBselectInputDaisy) -> u8 {
        Usdhc1cdBselectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1wpSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT3"]
    GPIO_EMC_12_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_36for Mode: ALT6"]
    GPIO_EMC_36_ALT6 = 0x01,
    #[doc = "Selecting Pad:GPIO_AD_B1_00 for Mode: ALT6"]
    GPIO_AD_B1_00_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT6"]
    GPIO_B1_13_ALT6 = 0x03,
}
impl Usdhc1wpSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1wpSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1wpSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1wpSelectInputDaisy {
        Usdhc1wpSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc1wpSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1wpSelectInputDaisy) -> u8 {
        Usdhc1wpSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1in17selectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_08 for Mode: ALT3"]
    GPIO_EMC_08_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1"]
    GPIO_AD_B0_03_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6"]
    GPIO_AD_B0_05_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT1"]
    GPIO_B1_03_ALT1 = 0x03,
}
impl Xbar1in17selectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1in17selectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1in17selectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1in17selectInputDaisy {
        Xbar1in17selectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1in17selectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1in17selectInputDaisy) -> u8 {
        Xbar1in17selectInputDaisy::to_bits(val)
    }
}
