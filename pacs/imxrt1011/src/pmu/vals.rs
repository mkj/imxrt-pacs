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
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
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
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
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
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
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
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
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
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg1p1clrOutputTrg(pub u8);
impl Reg1p1clrOutputTrg {
    #[doc = "0.8V"]
    pub const OUTPUT_TRG_4: Self = Self(0x04);
    #[doc = "1.1V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
}
impl Reg1p1clrOutputTrg {
    pub const fn from_bits(val: u8) -> Reg1p1clrOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg1p1clrOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1clrOutputTrg {
        Reg1p1clrOutputTrg::from_bits(val)
    }
}
impl From<Reg1p1clrOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1clrOutputTrg) -> u8 {
        Reg1p1clrOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg1p1outputTrg(pub u8);
impl Reg1p1outputTrg {
    #[doc = "0.8V"]
    pub const OUTPUT_TRG_4: Self = Self(0x04);
    #[doc = "1.1V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
}
impl Reg1p1outputTrg {
    pub const fn from_bits(val: u8) -> Reg1p1outputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg1p1outputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1outputTrg {
        Reg1p1outputTrg::from_bits(val)
    }
}
impl From<Reg1p1outputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1outputTrg) -> u8 {
        Reg1p1outputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg1p1setOutputTrg(pub u8);
impl Reg1p1setOutputTrg {
    #[doc = "0.8V"]
    pub const OUTPUT_TRG_4: Self = Self(0x04);
    #[doc = "1.1V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
}
impl Reg1p1setOutputTrg {
    pub const fn from_bits(val: u8) -> Reg1p1setOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg1p1setOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1setOutputTrg {
        Reg1p1setOutputTrg::from_bits(val)
    }
}
impl From<Reg1p1setOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1setOutputTrg) -> u8 {
        Reg1p1setOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg1p1togOutputTrg(pub u8);
impl Reg1p1togOutputTrg {
    #[doc = "0.8V"]
    pub const OUTPUT_TRG_4: Self = Self(0x04);
    #[doc = "1.1V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
}
impl Reg1p1togOutputTrg {
    pub const fn from_bits(val: u8) -> Reg1p1togOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg1p1togOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1togOutputTrg {
        Reg1p1togOutputTrg::from_bits(val)
    }
}
impl From<Reg1p1togOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1togOutputTrg) -> u8 {
        Reg1p1togOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg2p5clrOutputTrg(pub u8);
impl Reg2p5clrOutputTrg {
    #[doc = "2.10V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "2.50V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
    #[doc = "2.875V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg2p5clrOutputTrg {
    pub const fn from_bits(val: u8) -> Reg2p5clrOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg2p5clrOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5clrOutputTrg {
        Reg2p5clrOutputTrg::from_bits(val)
    }
}
impl From<Reg2p5clrOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5clrOutputTrg) -> u8 {
        Reg2p5clrOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg2p5outputTrg(pub u8);
impl Reg2p5outputTrg {
    #[doc = "2.10V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "2.50V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
    #[doc = "2.875V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg2p5outputTrg {
    pub const fn from_bits(val: u8) -> Reg2p5outputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg2p5outputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5outputTrg {
        Reg2p5outputTrg::from_bits(val)
    }
}
impl From<Reg2p5outputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5outputTrg) -> u8 {
        Reg2p5outputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg2p5setOutputTrg(pub u8);
impl Reg2p5setOutputTrg {
    #[doc = "2.10V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "2.50V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
    #[doc = "2.875V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg2p5setOutputTrg {
    pub const fn from_bits(val: u8) -> Reg2p5setOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg2p5setOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5setOutputTrg {
        Reg2p5setOutputTrg::from_bits(val)
    }
}
impl From<Reg2p5setOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5setOutputTrg) -> u8 {
        Reg2p5setOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg2p5togOutputTrg(pub u8);
impl Reg2p5togOutputTrg {
    #[doc = "2.10V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "2.50V"]
    pub const OUTPUT_TRG_16: Self = Self(0x10);
    #[doc = "2.875V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg2p5togOutputTrg {
    pub const fn from_bits(val: u8) -> Reg2p5togOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg2p5togOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5togOutputTrg {
        Reg2p5togOutputTrg::from_bits(val)
    }
}
impl From<Reg2p5togOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5togOutputTrg) -> u8 {
        Reg2p5togOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg3p0clrOutputTrg(pub u8);
impl Reg3p0clrOutputTrg {
    #[doc = "2.625V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "3.000V"]
    pub const OUTPUT_TRG_15: Self = Self(0x0f);
    #[doc = "3.400V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg3p0clrOutputTrg {
    pub const fn from_bits(val: u8) -> Reg3p0clrOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg3p0clrOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0clrOutputTrg {
        Reg3p0clrOutputTrg::from_bits(val)
    }
}
impl From<Reg3p0clrOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0clrOutputTrg) -> u8 {
        Reg3p0clrOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg3p0outputTrg(pub u8);
impl Reg3p0outputTrg {
    #[doc = "2.625V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "3.000V"]
    pub const OUTPUT_TRG_15: Self = Self(0x0f);
    #[doc = "3.400V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg3p0outputTrg {
    pub const fn from_bits(val: u8) -> Reg3p0outputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg3p0outputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0outputTrg {
        Reg3p0outputTrg::from_bits(val)
    }
}
impl From<Reg3p0outputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0outputTrg) -> u8 {
        Reg3p0outputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg3p0setOutputTrg(pub u8);
impl Reg3p0setOutputTrg {
    #[doc = "2.625V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "3.000V"]
    pub const OUTPUT_TRG_15: Self = Self(0x0f);
    #[doc = "3.400V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg3p0setOutputTrg {
    pub const fn from_bits(val: u8) -> Reg3p0setOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg3p0setOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0setOutputTrg {
        Reg3p0setOutputTrg::from_bits(val)
    }
}
impl From<Reg3p0setOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0setOutputTrg) -> u8 {
        Reg3p0setOutputTrg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg3p0togOutputTrg(pub u8);
impl Reg3p0togOutputTrg {
    #[doc = "2.625V"]
    pub const OUTPUT_TRG_0: Self = Self(0);
    #[doc = "3.000V"]
    pub const OUTPUT_TRG_15: Self = Self(0x0f);
    #[doc = "3.400V"]
    pub const OUTPUT_TRG_31: Self = Self(0x1f);
}
impl Reg3p0togOutputTrg {
    pub const fn from_bits(val: u8) -> Reg3p0togOutputTrg {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Reg3p0togOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0togOutputTrg {
        Reg3p0togOutputTrg::from_bits(val)
    }
}
impl From<Reg3p0togOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0togOutputTrg) -> u8 {
        Reg3p0togOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreClrRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrRampRate {
        RegCoreClrRampRate::from_bits(val)
    }
}
impl From<RegCoreClrRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrRampRate) -> u8 {
        RegCoreClrRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrReg0adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreClrReg0adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg0adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg0adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg0adj {
        RegCoreClrReg0adj::from_bits(val)
    }
}
impl From<RegCoreClrReg0adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg0adj) -> u8 {
        RegCoreClrReg0adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrReg0targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreClrReg0targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg0targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg0targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg0targ {
        RegCoreClrReg0targ::from_bits(val)
    }
}
impl From<RegCoreClrReg0targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg0targ) -> u8 {
        RegCoreClrReg0targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrReg1adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreClrReg1adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg1adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg1adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg1adj {
        RegCoreClrReg1adj::from_bits(val)
    }
}
impl From<RegCoreClrReg1adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg1adj) -> u8 {
        RegCoreClrReg1adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrReg1targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreClrReg1targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg1targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg1targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg1targ {
        RegCoreClrReg1targ::from_bits(val)
    }
}
impl From<RegCoreClrReg1targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg1targ) -> u8 {
        RegCoreClrReg1targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrReg2adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreClrReg2adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg2adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg2adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg2adj {
        RegCoreClrReg2adj::from_bits(val)
    }
}
impl From<RegCoreClrReg2adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg2adj) -> u8 {
        RegCoreClrReg2adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreClrReg2targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreClrReg2targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg2targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg2targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg2targ {
        RegCoreClrReg2targ::from_bits(val)
    }
}
impl From<RegCoreClrReg2targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg2targ) -> u8 {
        RegCoreClrReg2targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreRampRate {
        RegCoreRampRate::from_bits(val)
    }
}
impl From<RegCoreRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreRampRate) -> u8 {
        RegCoreRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreReg0adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreReg0adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg0adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg0adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg0adj {
        RegCoreReg0adj::from_bits(val)
    }
}
impl From<RegCoreReg0adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg0adj) -> u8 {
        RegCoreReg0adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreReg0targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreReg0targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg0targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg0targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg0targ {
        RegCoreReg0targ::from_bits(val)
    }
}
impl From<RegCoreReg0targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg0targ) -> u8 {
        RegCoreReg0targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreReg1adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreReg1adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg1adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg1adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg1adj {
        RegCoreReg1adj::from_bits(val)
    }
}
impl From<RegCoreReg1adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg1adj) -> u8 {
        RegCoreReg1adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreReg1targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreReg1targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg1targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg1targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg1targ {
        RegCoreReg1targ::from_bits(val)
    }
}
impl From<RegCoreReg1targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg1targ) -> u8 {
        RegCoreReg1targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreReg2adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreReg2adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg2adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg2adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg2adj {
        RegCoreReg2adj::from_bits(val)
    }
}
impl From<RegCoreReg2adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg2adj) -> u8 {
        RegCoreReg2adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreReg2targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreReg2targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg2targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg2targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg2targ {
        RegCoreReg2targ::from_bits(val)
    }
}
impl From<RegCoreReg2targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg2targ) -> u8 {
        RegCoreReg2targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreSetRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetRampRate {
        RegCoreSetRampRate::from_bits(val)
    }
}
impl From<RegCoreSetRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetRampRate) -> u8 {
        RegCoreSetRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetReg0adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreSetReg0adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg0adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg0adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg0adj {
        RegCoreSetReg0adj::from_bits(val)
    }
}
impl From<RegCoreSetReg0adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg0adj) -> u8 {
        RegCoreSetReg0adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetReg0targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreSetReg0targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg0targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg0targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg0targ {
        RegCoreSetReg0targ::from_bits(val)
    }
}
impl From<RegCoreSetReg0targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg0targ) -> u8 {
        RegCoreSetReg0targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetReg1adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreSetReg1adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg1adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg1adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg1adj {
        RegCoreSetReg1adj::from_bits(val)
    }
}
impl From<RegCoreSetReg1adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg1adj) -> u8 {
        RegCoreSetReg1adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetReg1targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreSetReg1targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg1targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg1targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg1targ {
        RegCoreSetReg1targ::from_bits(val)
    }
}
impl From<RegCoreSetReg1targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg1targ) -> u8 {
        RegCoreSetReg1targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetReg2adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreSetReg2adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg2adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg2adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg2adj {
        RegCoreSetReg2adj::from_bits(val)
    }
}
impl From<RegCoreSetReg2adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg2adj) -> u8 {
        RegCoreSetReg2adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreSetReg2targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreSetReg2targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg2targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg2targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg2targ {
        RegCoreSetReg2targ::from_bits(val)
    }
}
impl From<RegCoreSetReg2targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg2targ) -> u8 {
        RegCoreSetReg2targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreTogRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogRampRate {
        RegCoreTogRampRate::from_bits(val)
    }
}
impl From<RegCoreTogRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogRampRate) -> u8 {
        RegCoreTogRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogReg0adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreTogReg0adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg0adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg0adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg0adj {
        RegCoreTogReg0adj::from_bits(val)
    }
}
impl From<RegCoreTogReg0adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg0adj) -> u8 {
        RegCoreTogReg0adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogReg0targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreTogReg0targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg0targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg0targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg0targ {
        RegCoreTogReg0targ::from_bits(val)
    }
}
impl From<RegCoreTogReg0targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg0targ) -> u8 {
        RegCoreTogReg0targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogReg1adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreTogReg1adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg1adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg1adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg1adj {
        RegCoreTogReg1adj::from_bits(val)
    }
}
impl From<RegCoreTogReg1adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg1adj) -> u8 {
        RegCoreTogReg1adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogReg1targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreTogReg1targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg1targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg1targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg1targ {
        RegCoreTogReg1targ::from_bits(val)
    }
}
impl From<RegCoreTogReg1targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg1targ) -> u8 {
        RegCoreTogReg1targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogReg2adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreTogReg2adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg2adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg2adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg2adj {
        RegCoreTogReg2adj::from_bits(val)
    }
}
impl From<RegCoreTogReg2adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg2adj) -> u8 {
        RegCoreTogReg2adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegCoreTogReg2targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
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
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreTogReg2targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg2targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg2targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg2targ {
        RegCoreTogReg2targ::from_bits(val)
    }
}
impl From<RegCoreTogReg2targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg2targ) -> u8 {
        RegCoreTogReg2targ::to_bits(val)
    }
}
