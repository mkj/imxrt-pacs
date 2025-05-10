#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlClrXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlClrXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlClrXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlClrXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlClrXtaloscPwrupDelay {
        LowpwrCtrlClrXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlClrXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlClrXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlClrXtaloscPwrupDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlSetXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlSetXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlSetXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlSetXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlSetXtaloscPwrupDelay {
        LowpwrCtrlSetXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlSetXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlSetXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlSetXtaloscPwrupDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlTogXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlTogXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlTogXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlTogXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlTogXtaloscPwrupDelay {
        LowpwrCtrlTogXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlTogXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlTogXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlTogXtaloscPwrupDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlXtaloscPwrupDelay {
        LowpwrCtrlXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlXtaloscPwrupDelay::to_bits(val)
    }
}
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
