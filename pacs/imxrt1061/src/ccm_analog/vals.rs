#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0clkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0clkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0clkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0clkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0clkgateDelay {
        Misc0clkgateDelay::from_bits(val)
    }
}
impl From<Misc0clkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0clkgateDelay) -> u8 {
        Misc0clkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0clrClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0clrClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0clrClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0clrClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0clrClkgateDelay {
        Misc0clrClkgateDelay::from_bits(val)
    }
}
impl From<Misc0clrClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0clrClkgateDelay) -> u8 {
        Misc0clrClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0clrOscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0clrOscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0clrOscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0clrOscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0clrOscI {
        Misc0clrOscI::from_bits(val)
    }
}
impl From<Misc0clrOscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0clrOscI) -> u8 {
        Misc0clrOscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0clrReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0clrReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0clrReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0clrReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0clrReftopVbgadj {
        Misc0clrReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0clrReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0clrReftopVbgadj) -> u8 {
        Misc0clrReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0clrStopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0clrStopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0clrStopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0clrStopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0clrStopModeConfig {
        Misc0clrStopModeConfig::from_bits(val)
    }
}
impl From<Misc0clrStopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0clrStopModeConfig) -> u8 {
        Misc0clrStopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0oscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0oscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0oscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0oscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0oscI {
        Misc0oscI::from_bits(val)
    }
}
impl From<Misc0oscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0oscI) -> u8 {
        Misc0oscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0reftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0reftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0reftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0reftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0reftopVbgadj {
        Misc0reftopVbgadj::from_bits(val)
    }
}
impl From<Misc0reftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0reftopVbgadj) -> u8 {
        Misc0reftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0setClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0setClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0setClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0setClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0setClkgateDelay {
        Misc0setClkgateDelay::from_bits(val)
    }
}
impl From<Misc0setClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0setClkgateDelay) -> u8 {
        Misc0setClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0setOscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0setOscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0setOscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0setOscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0setOscI {
        Misc0setOscI::from_bits(val)
    }
}
impl From<Misc0setOscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0setOscI) -> u8 {
        Misc0setOscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0setReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0setReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0setReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0setReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0setReftopVbgadj {
        Misc0setReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0setReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0setReftopVbgadj) -> u8 {
        Misc0setReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0setStopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0setStopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0setStopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0setStopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0setStopModeConfig {
        Misc0setStopModeConfig::from_bits(val)
    }
}
impl From<Misc0setStopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0setStopModeConfig) -> u8 {
        Misc0setStopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0stopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0stopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0stopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0stopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0stopModeConfig {
        Misc0stopModeConfig::from_bits(val)
    }
}
impl From<Misc0stopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0stopModeConfig) -> u8 {
        Misc0stopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0togClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0togClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0togClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0togClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0togClkgateDelay {
        Misc0togClkgateDelay::from_bits(val)
    }
}
impl From<Misc0togClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0togClkgateDelay) -> u8 {
        Misc0togClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0togOscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0togOscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0togOscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0togOscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0togOscI {
        Misc0togOscI::from_bits(val)
    }
}
impl From<Misc0togOscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0togOscI) -> u8 {
        Misc0togOscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0togReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0togReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0togReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0togReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0togReftopVbgadj {
        Misc0togReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0togReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0togReftopVbgadj) -> u8 {
        Misc0togReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0togStopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0togStopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0togStopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0togStopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0togStopModeConfig {
        Misc0togStopModeConfig::from_bits(val)
    }
}
impl From<Misc0togStopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0togStopModeConfig) -> u8 {
        Misc0togStopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc1clrLvds1clkSel {
    #[doc = "Arm PLL"]
    ARM_PLL = 0x0,
    #[doc = "System PLL"]
    SYS_PLL = 0x01,
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4 = 0x02,
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5 = 0x03,
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6 = 0x04,
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7 = 0x05,
    #[doc = "Audio PLL"]
    AUDIO_PLL = 0x06,
    #[doc = "Video PLL"]
    VIDEO_PLL = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "USB1 PLL clock"]
    USB1_PLL = 0x0c,
    #[doc = "USB2 PLL clock"]
    USB2_PLL = 0x0d,
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0 = 0x0e,
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1 = 0x0f,
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2 = 0x10,
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3 = 0x11,
    #[doc = "xtal (24M)"]
    XTAL = 0x12,
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
}
impl Misc1clrLvds1clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc1clrLvds1clkSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc1clrLvds1clkSel {
    #[inline(always)]
    fn from(val: u8) -> Misc1clrLvds1clkSel {
        Misc1clrLvds1clkSel::from_bits(val)
    }
}
impl From<Misc1clrLvds1clkSel> for u8 {
    #[inline(always)]
    fn from(val: Misc1clrLvds1clkSel) -> u8 {
        Misc1clrLvds1clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc1lvds1clkSel {
    #[doc = "Arm PLL"]
    ARM_PLL = 0x0,
    #[doc = "System PLL"]
    SYS_PLL = 0x01,
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4 = 0x02,
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5 = 0x03,
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6 = 0x04,
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7 = 0x05,
    #[doc = "Audio PLL"]
    AUDIO_PLL = 0x06,
    #[doc = "Video PLL"]
    VIDEO_PLL = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "USB1 PLL clock"]
    USB1_PLL = 0x0c,
    #[doc = "USB2 PLL clock"]
    USB2_PLL = 0x0d,
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0 = 0x0e,
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1 = 0x0f,
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2 = 0x10,
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3 = 0x11,
    #[doc = "xtal (24M)"]
    XTAL = 0x12,
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
}
impl Misc1lvds1clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc1lvds1clkSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc1lvds1clkSel {
    #[inline(always)]
    fn from(val: u8) -> Misc1lvds1clkSel {
        Misc1lvds1clkSel::from_bits(val)
    }
}
impl From<Misc1lvds1clkSel> for u8 {
    #[inline(always)]
    fn from(val: Misc1lvds1clkSel) -> u8 {
        Misc1lvds1clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc1setLvds1clkSel {
    #[doc = "Arm PLL"]
    ARM_PLL = 0x0,
    #[doc = "System PLL"]
    SYS_PLL = 0x01,
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4 = 0x02,
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5 = 0x03,
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6 = 0x04,
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7 = 0x05,
    #[doc = "Audio PLL"]
    AUDIO_PLL = 0x06,
    #[doc = "Video PLL"]
    VIDEO_PLL = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "USB1 PLL clock"]
    USB1_PLL = 0x0c,
    #[doc = "USB2 PLL clock"]
    USB2_PLL = 0x0d,
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0 = 0x0e,
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1 = 0x0f,
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2 = 0x10,
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3 = 0x11,
    #[doc = "xtal (24M)"]
    XTAL = 0x12,
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
}
impl Misc1setLvds1clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc1setLvds1clkSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc1setLvds1clkSel {
    #[inline(always)]
    fn from(val: u8) -> Misc1setLvds1clkSel {
        Misc1setLvds1clkSel::from_bits(val)
    }
}
impl From<Misc1setLvds1clkSel> for u8 {
    #[inline(always)]
    fn from(val: Misc1setLvds1clkSel) -> u8 {
        Misc1setLvds1clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc1togLvds1clkSel {
    #[doc = "Arm PLL"]
    ARM_PLL = 0x0,
    #[doc = "System PLL"]
    SYS_PLL = 0x01,
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4 = 0x02,
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5 = 0x03,
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6 = 0x04,
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7 = 0x05,
    #[doc = "Audio PLL"]
    AUDIO_PLL = 0x06,
    #[doc = "Video PLL"]
    VIDEO_PLL = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "USB1 PLL clock"]
    USB1_PLL = 0x0c,
    #[doc = "USB2 PLL clock"]
    USB2_PLL = 0x0d,
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0 = 0x0e,
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1 = 0x0f,
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2 = 0x10,
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3 = 0x11,
    #[doc = "xtal (24M)"]
    XTAL = 0x12,
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
}
impl Misc1togLvds1clkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc1togLvds1clkSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc1togLvds1clkSel {
    #[inline(always)]
    fn from(val: u8) -> Misc1togLvds1clkSel {
        Misc1togLvds1clkSel::from_bits(val)
    }
}
impl From<Misc1togLvds1clkSel> for u8 {
    #[inline(always)]
    fn from(val: Misc1togLvds1clkSel) -> u8 {
        Misc1togLvds1clkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrReg0boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2clrReg0boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrReg0boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrReg0boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrReg0boOffset {
        Misc2clrReg0boOffset::from_bits(val)
    }
}
impl From<Misc2clrReg0boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrReg0boOffset) -> u8 {
        Misc2clrReg0boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrReg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2clrReg0stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrReg0stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrReg0stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrReg0stepTime {
        Misc2clrReg0stepTime::from_bits(val)
    }
}
impl From<Misc2clrReg0stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrReg0stepTime) -> u8 {
        Misc2clrReg0stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrReg1boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2clrReg1boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrReg1boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrReg1boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrReg1boOffset {
        Misc2clrReg1boOffset::from_bits(val)
    }
}
impl From<Misc2clrReg1boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrReg1boOffset) -> u8 {
        Misc2clrReg1boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrReg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2clrReg1stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrReg1stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrReg1stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrReg1stepTime {
        Misc2clrReg1stepTime::from_bits(val)
    }
}
impl From<Misc2clrReg1stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrReg1stepTime) -> u8 {
        Misc2clrReg1stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrReg2boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2clrReg2boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrReg2boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrReg2boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrReg2boOffset {
        Misc2clrReg2boOffset::from_bits(val)
    }
}
impl From<Misc2clrReg2boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrReg2boOffset) -> u8 {
        Misc2clrReg2boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrReg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2clrReg2stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrReg2stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrReg2stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrReg2stepTime {
        Misc2clrReg2stepTime::from_bits(val)
    }
}
impl From<Misc2clrReg2stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrReg2stepTime) -> u8 {
        Misc2clrReg2stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2clrVideoDiv {
    #[doc = "divide by 1 (Default)"]
    VIDEO_DIV_0 = 0x0,
    #[doc = "divide by 2"]
    VIDEO_DIV_1 = 0x01,
    #[doc = "divide by 1"]
    VIDEO_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    VIDEO_DIV_3 = 0x03,
}
impl Misc2clrVideoDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2clrVideoDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2clrVideoDiv {
    #[inline(always)]
    fn from(val: u8) -> Misc2clrVideoDiv {
        Misc2clrVideoDiv::from_bits(val)
    }
}
impl From<Misc2clrVideoDiv> for u8 {
    #[inline(always)]
    fn from(val: Misc2clrVideoDiv) -> u8 {
        Misc2clrVideoDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2reg0boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2reg0boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2reg0boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2reg0boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2reg0boOffset {
        Misc2reg0boOffset::from_bits(val)
    }
}
impl From<Misc2reg0boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2reg0boOffset) -> u8 {
        Misc2reg0boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2reg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2reg0stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2reg0stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2reg0stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2reg0stepTime {
        Misc2reg0stepTime::from_bits(val)
    }
}
impl From<Misc2reg0stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2reg0stepTime) -> u8 {
        Misc2reg0stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2reg1boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2reg1boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2reg1boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2reg1boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2reg1boOffset {
        Misc2reg1boOffset::from_bits(val)
    }
}
impl From<Misc2reg1boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2reg1boOffset) -> u8 {
        Misc2reg1boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2reg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2reg1stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2reg1stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2reg1stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2reg1stepTime {
        Misc2reg1stepTime::from_bits(val)
    }
}
impl From<Misc2reg1stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2reg1stepTime) -> u8 {
        Misc2reg1stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2reg2boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2reg2boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2reg2boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2reg2boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2reg2boOffset {
        Misc2reg2boOffset::from_bits(val)
    }
}
impl From<Misc2reg2boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2reg2boOffset) -> u8 {
        Misc2reg2boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2reg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2reg2stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2reg2stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2reg2stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2reg2stepTime {
        Misc2reg2stepTime::from_bits(val)
    }
}
impl From<Misc2reg2stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2reg2stepTime) -> u8 {
        Misc2reg2stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setReg0boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2setReg0boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setReg0boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setReg0boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2setReg0boOffset {
        Misc2setReg0boOffset::from_bits(val)
    }
}
impl From<Misc2setReg0boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2setReg0boOffset) -> u8 {
        Misc2setReg0boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setReg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2setReg0stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setReg0stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setReg0stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2setReg0stepTime {
        Misc2setReg0stepTime::from_bits(val)
    }
}
impl From<Misc2setReg0stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2setReg0stepTime) -> u8 {
        Misc2setReg0stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setReg1boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2setReg1boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setReg1boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setReg1boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2setReg1boOffset {
        Misc2setReg1boOffset::from_bits(val)
    }
}
impl From<Misc2setReg1boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2setReg1boOffset) -> u8 {
        Misc2setReg1boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setReg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2setReg1stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setReg1stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setReg1stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2setReg1stepTime {
        Misc2setReg1stepTime::from_bits(val)
    }
}
impl From<Misc2setReg1stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2setReg1stepTime) -> u8 {
        Misc2setReg1stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setReg2boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2setReg2boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setReg2boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setReg2boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2setReg2boOffset {
        Misc2setReg2boOffset::from_bits(val)
    }
}
impl From<Misc2setReg2boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2setReg2boOffset) -> u8 {
        Misc2setReg2boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setReg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2setReg2stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setReg2stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setReg2stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2setReg2stepTime {
        Misc2setReg2stepTime::from_bits(val)
    }
}
impl From<Misc2setReg2stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2setReg2stepTime) -> u8 {
        Misc2setReg2stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2setVideoDiv {
    #[doc = "divide by 1 (Default)"]
    VIDEO_DIV_0 = 0x0,
    #[doc = "divide by 2"]
    VIDEO_DIV_1 = 0x01,
    #[doc = "divide by 1"]
    VIDEO_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    VIDEO_DIV_3 = 0x03,
}
impl Misc2setVideoDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2setVideoDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2setVideoDiv {
    #[inline(always)]
    fn from(val: u8) -> Misc2setVideoDiv {
        Misc2setVideoDiv::from_bits(val)
    }
}
impl From<Misc2setVideoDiv> for u8 {
    #[inline(always)]
    fn from(val: Misc2setVideoDiv) -> u8 {
        Misc2setVideoDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togReg0boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2togReg0boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togReg0boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togReg0boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2togReg0boOffset {
        Misc2togReg0boOffset::from_bits(val)
    }
}
impl From<Misc2togReg0boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2togReg0boOffset) -> u8 {
        Misc2togReg0boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togReg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2togReg0stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togReg0stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togReg0stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2togReg0stepTime {
        Misc2togReg0stepTime::from_bits(val)
    }
}
impl From<Misc2togReg0stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2togReg0stepTime) -> u8 {
        Misc2togReg0stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togReg1boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2togReg1boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togReg1boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togReg1boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2togReg1boOffset {
        Misc2togReg1boOffset::from_bits(val)
    }
}
impl From<Misc2togReg1boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2togReg1boOffset) -> u8 {
        Misc2togReg1boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togReg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2togReg1stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togReg1stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togReg1stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2togReg1stepTime {
        Misc2togReg1stepTime::from_bits(val)
    }
}
impl From<Misc2togReg1stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2togReg1stepTime) -> u8 {
        Misc2togReg1stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togReg2boOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2togReg2boOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togReg2boOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togReg2boOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2togReg2boOffset {
        Misc2togReg2boOffset::from_bits(val)
    }
}
impl From<Misc2togReg2boOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2togReg2boOffset) -> u8 {
        Misc2togReg2boOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togReg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2togReg2stepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togReg2stepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togReg2stepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2togReg2stepTime {
        Misc2togReg2stepTime::from_bits(val)
    }
}
impl From<Misc2togReg2stepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2togReg2stepTime) -> u8 {
        Misc2togReg2stepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2togVideoDiv {
    #[doc = "divide by 1 (Default)"]
    VIDEO_DIV_0 = 0x0,
    #[doc = "divide by 2"]
    VIDEO_DIV_1 = 0x01,
    #[doc = "divide by 1"]
    VIDEO_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    VIDEO_DIV_3 = 0x03,
}
impl Misc2togVideoDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2togVideoDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2togVideoDiv {
    #[inline(always)]
    fn from(val: u8) -> Misc2togVideoDiv {
        Misc2togVideoDiv::from_bits(val)
    }
}
impl From<Misc2togVideoDiv> for u8 {
    #[inline(always)]
    fn from(val: Misc2togVideoDiv) -> u8 {
        Misc2togVideoDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2videoDiv {
    #[doc = "divide by 1 (Default)"]
    VIDEO_DIV_0 = 0x0,
    #[doc = "divide by 2"]
    VIDEO_DIV_1 = 0x01,
    #[doc = "divide by 1"]
    VIDEO_DIV_2 = 0x02,
    #[doc = "divide by 4"]
    VIDEO_DIV_3 = 0x03,
}
impl Misc2videoDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2videoDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2videoDiv {
    #[inline(always)]
    fn from(val: u8) -> Misc2videoDiv {
        Misc2videoDiv::from_bits(val)
    }
}
impl From<Misc2videoDiv> for u8 {
    #[inline(always)]
    fn from(val: Misc2videoDiv) -> u8 {
        Misc2videoDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllArmBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllArmBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllArmBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllArmBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllArmBypassClkSrc {
        PllArmBypassClkSrc::from_bits(val)
    }
}
impl From<PllArmBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllArmBypassClkSrc) -> u8 {
        PllArmBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllArmClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllArmClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllArmClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllArmClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllArmClrBypassClkSrc {
        PllArmClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllArmClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllArmClrBypassClkSrc) -> u8 {
        PllArmClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllArmSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllArmSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllArmSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllArmSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllArmSetBypassClkSrc {
        PllArmSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllArmSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllArmSetBypassClkSrc) -> u8 {
        PllArmSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllArmTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllArmTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllArmTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllArmTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllArmTogBypassClkSrc {
        PllArmTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllArmTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllArmTogBypassClkSrc) -> u8 {
        PllArmTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioBypassClkSrc {
        PllAudioBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioBypassClkSrc) -> u8 {
        PllAudioBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioClrBypassClkSrc {
        PllAudioClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioClrBypassClkSrc) -> u8 {
        PllAudioClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioClrPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioClrPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioClrPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioClrPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioClrPostDivSelect {
        PllAudioClrPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioClrPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioClrPostDivSelect) -> u8 {
        PllAudioClrPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioPostDivSelect {
        PllAudioPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioPostDivSelect) -> u8 {
        PllAudioPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioSetBypassClkSrc {
        PllAudioSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioSetBypassClkSrc) -> u8 {
        PllAudioSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioSetPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioSetPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioSetPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioSetPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioSetPostDivSelect {
        PllAudioSetPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioSetPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioSetPostDivSelect) -> u8 {
        PllAudioSetPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioTogBypassClkSrc {
        PllAudioTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioTogBypassClkSrc) -> u8 {
        PllAudioTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioTogPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioTogPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioTogPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioTogPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioTogPostDivSelect {
        PllAudioTogPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioTogPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioTogPostDivSelect) -> u8 {
        PllAudioTogPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetBypassClkSrc {
        PllEnetBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetBypassClkSrc) -> u8 {
        PllEnetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetClrBypassClkSrc {
        PllEnetClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetClrBypassClkSrc) -> u8 {
        PllEnetClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetClrEnet2divSelect {
    #[doc = "25MHz"]
    ENET2_DIV_SELECT_0 = 0x0,
    #[doc = "50MHz"]
    ENET2_DIV_SELECT_1 = 0x01,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET2_DIV_SELECT_2 = 0x02,
    #[doc = "125MHz"]
    ENET2_DIV_SELECT_3 = 0x03,
}
impl PllEnetClrEnet2divSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetClrEnet2divSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetClrEnet2divSelect {
    #[inline(always)]
    fn from(val: u8) -> PllEnetClrEnet2divSelect {
        PllEnetClrEnet2divSelect::from_bits(val)
    }
}
impl From<PllEnetClrEnet2divSelect> for u8 {
    #[inline(always)]
    fn from(val: PllEnetClrEnet2divSelect) -> u8 {
        PllEnetClrEnet2divSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetEnet2divSelect {
    #[doc = "25MHz"]
    ENET2_DIV_SELECT_0 = 0x0,
    #[doc = "50MHz"]
    ENET2_DIV_SELECT_1 = 0x01,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET2_DIV_SELECT_2 = 0x02,
    #[doc = "125MHz"]
    ENET2_DIV_SELECT_3 = 0x03,
}
impl PllEnetEnet2divSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetEnet2divSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetEnet2divSelect {
    #[inline(always)]
    fn from(val: u8) -> PllEnetEnet2divSelect {
        PllEnetEnet2divSelect::from_bits(val)
    }
}
impl From<PllEnetEnet2divSelect> for u8 {
    #[inline(always)]
    fn from(val: PllEnetEnet2divSelect) -> u8 {
        PllEnetEnet2divSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetSetBypassClkSrc {
        PllEnetSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetSetBypassClkSrc) -> u8 {
        PllEnetSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetSetEnet2divSelect {
    #[doc = "25MHz"]
    ENET2_DIV_SELECT_0 = 0x0,
    #[doc = "50MHz"]
    ENET2_DIV_SELECT_1 = 0x01,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET2_DIV_SELECT_2 = 0x02,
    #[doc = "125MHz"]
    ENET2_DIV_SELECT_3 = 0x03,
}
impl PllEnetSetEnet2divSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetSetEnet2divSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetSetEnet2divSelect {
    #[inline(always)]
    fn from(val: u8) -> PllEnetSetEnet2divSelect {
        PllEnetSetEnet2divSelect::from_bits(val)
    }
}
impl From<PllEnetSetEnet2divSelect> for u8 {
    #[inline(always)]
    fn from(val: PllEnetSetEnet2divSelect) -> u8 {
        PllEnetSetEnet2divSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetTogBypassClkSrc {
        PllEnetTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetTogBypassClkSrc) -> u8 {
        PllEnetTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetTogEnet2divSelect {
    #[doc = "25MHz"]
    ENET2_DIV_SELECT_0 = 0x0,
    #[doc = "50MHz"]
    ENET2_DIV_SELECT_1 = 0x01,
    #[doc = "100MHz (not 50% duty cycle)"]
    ENET2_DIV_SELECT_2 = 0x02,
    #[doc = "125MHz"]
    ENET2_DIV_SELECT_3 = 0x03,
}
impl PllEnetTogEnet2divSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetTogEnet2divSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetTogEnet2divSelect {
    #[inline(always)]
    fn from(val: u8) -> PllEnetTogEnet2divSelect {
        PllEnetTogEnet2divSelect::from_bits(val)
    }
}
impl From<PllEnetTogEnet2divSelect> for u8 {
    #[inline(always)]
    fn from(val: PllEnetTogEnet2divSelect) -> u8 {
        PllEnetTogEnet2divSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysBypassClkSrc {
        PllSysBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysBypassClkSrc) -> u8 {
        PllSysBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysClrBypassClkSrc {
        PllSysClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysClrBypassClkSrc) -> u8 {
        PllSysClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysSetBypassClkSrc {
        PllSysSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysSetBypassClkSrc) -> u8 {
        PllSysSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysTogBypassClkSrc {
        PllSysTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysTogBypassClkSrc) -> u8 {
        PllSysTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1bypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1bypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1bypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1bypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1bypassClkSrc {
        PllUsb1bypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1bypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1bypassClkSrc) -> u8 {
        PllUsb1bypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1clrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1clrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1clrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1clrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1clrBypassClkSrc {
        PllUsb1clrBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1clrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1clrBypassClkSrc) -> u8 {
        PllUsb1clrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1setBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1setBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1setBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1setBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1setBypassClkSrc {
        PllUsb1setBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1setBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1setBypassClkSrc) -> u8 {
        PllUsb1setBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1togBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1togBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1togBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1togBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1togBypassClkSrc {
        PllUsb1togBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1togBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1togBypassClkSrc) -> u8 {
        PllUsb1togBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb2bypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb2bypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb2bypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb2bypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb2bypassClkSrc {
        PllUsb2bypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb2bypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb2bypassClkSrc) -> u8 {
        PllUsb2bypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb2clrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb2clrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb2clrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb2clrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb2clrBypassClkSrc {
        PllUsb2clrBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb2clrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb2clrBypassClkSrc) -> u8 {
        PllUsb2clrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb2setBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb2setBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb2setBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb2setBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb2setBypassClkSrc {
        PllUsb2setBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb2setBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb2setBypassClkSrc) -> u8 {
        PllUsb2setBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb2togBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb2togBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb2togBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb2togBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb2togBypassClkSrc {
        PllUsb2togBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb2togBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb2togBypassClkSrc) -> u8 {
        PllUsb2togBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllVideoBypassClkSrc {
        PllVideoBypassClkSrc::from_bits(val)
    }
}
impl From<PllVideoBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllVideoBypassClkSrc) -> u8 {
        PllVideoBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllVideoClrBypassClkSrc {
        PllVideoClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllVideoClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllVideoClrBypassClkSrc) -> u8 {
        PllVideoClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoClrPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoClrPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoClrPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoClrPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllVideoClrPostDivSelect {
        PllVideoClrPostDivSelect::from_bits(val)
    }
}
impl From<PllVideoClrPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllVideoClrPostDivSelect) -> u8 {
        PllVideoClrPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllVideoPostDivSelect {
        PllVideoPostDivSelect::from_bits(val)
    }
}
impl From<PllVideoPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllVideoPostDivSelect) -> u8 {
        PllVideoPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllVideoSetBypassClkSrc {
        PllVideoSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllVideoSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllVideoSetBypassClkSrc) -> u8 {
        PllVideoSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoSetPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoSetPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoSetPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoSetPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllVideoSetPostDivSelect {
        PllVideoSetPostDivSelect::from_bits(val)
    }
}
impl From<PllVideoSetPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllVideoSetPostDivSelect) -> u8 {
        PllVideoSetPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    CLK1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllVideoTogBypassClkSrc {
        PllVideoTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllVideoTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllVideoTogBypassClkSrc) -> u8 {
        PllVideoTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllVideoTogPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllVideoTogPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllVideoTogPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllVideoTogPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllVideoTogPostDivSelect {
        PllVideoTogPostDivSelect::from_bits(val)
    }
}
impl From<PllVideoTogPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllVideoTogPostDivSelect) -> u8 {
        PllVideoTogPostDivSelect::to_bits(val)
    }
}
