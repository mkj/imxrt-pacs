#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Misc0clkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0,
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
pub enum Misc0clrClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0,
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
pub enum Misc0clrOscI {
    #[doc = "Nominal"]
    NOMINAL = 0,
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
pub enum Misc0clrReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0,
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
pub enum Misc0clrStopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0,
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
pub enum Misc0oscI {
    #[doc = "Nominal"]
    NOMINAL = 0,
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
pub enum Misc0reftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0,
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
pub enum Misc0setClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0,
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
pub enum Misc0setOscI {
    #[doc = "Nominal"]
    NOMINAL = 0,
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
pub enum Misc0setReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0,
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
pub enum Misc0setStopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0,
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
pub enum Misc0stopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0,
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
pub enum Misc0togClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0,
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
pub enum Misc0togOscI {
    #[doc = "Nominal"]
    NOMINAL = 0,
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
pub enum Misc0togReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0,
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
pub enum Misc0togStopModeConfig {
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0,
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
pub enum Misc2clrReg0boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2clrReg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2clrReg1boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2clrReg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2clrReg2boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2clrReg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2reg0boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2reg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2reg1boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2reg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2reg2boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2reg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2setReg0boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2setReg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2setReg1boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2setReg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2setReg2boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2setReg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2togReg0boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2togReg0stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2togReg1boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2togReg1stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum Misc2togReg2boOffset {
    _RESERVED_0 = 0,
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
pub enum Misc2togReg2stepTime {
    #[doc = "64"]
    _64_CLOCKS = 0,
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
pub enum PllAudioBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllAudioClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllAudioClrPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0,
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
pub enum PllAudioPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0,
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
pub enum PllAudioSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllAudioSetPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0,
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
pub enum PllAudioTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllAudioTogPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0,
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
pub enum PllEnetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllEnetClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllEnetSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllEnetTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllSysBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllSysClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllSysSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllSysTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllUsb1bypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllUsb1clrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllUsb1setBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
pub enum PllUsb1togBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
    _RESERVED_1 = 0x01,
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
