#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadPmicOnReqMuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SNVS_PMIC_ON_REQ of instance: snvs"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO01 of instance: gpio5"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadPmicOnReqMuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadPmicOnReqMuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadPmicOnReqMuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadPmicOnReqMuxMode {
        SwMuxCtlPadPmicOnReqMuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadPmicOnReqMuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadPmicOnReqMuxMode) -> u8 {
        SwMuxCtlPadPmicOnReqMuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadPmicStbyReqMuxMode {
    #[doc = "Select mux mode: ALT0 mux port: CCM_PMIC_STBY_REQ of instance: ccm"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO02 of instance: gpio5"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadPmicStbyReqMuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadPmicStbyReqMuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadPmicStbyReqMuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadPmicStbyReqMuxMode {
        SwMuxCtlPadPmicStbyReqMuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadPmicStbyReqMuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadPmicStbyReqMuxMode) -> u8 {
        SwMuxCtlPadPmicStbyReqMuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadWakeupMuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO00 of instance: gpio5"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NMI of instance: CM7"]
    ALT7 = 0x07,
}
impl SwMuxCtlPadWakeupMuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadWakeupMuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadWakeupMuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadWakeupMuxMode {
        SwMuxCtlPadWakeupMuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadWakeupMuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadWakeupMuxMode) -> u8 {
        SwMuxCtlPadWakeupMuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadOnoffDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffDse {
        SwPadCtlPadOnoffDse::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffDse) -> u8 {
        SwPadCtlPadOnoffDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadOnoffPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffPus {
        SwPadCtlPadOnoffPus::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffPus) -> u8 {
        SwPadCtlPadOnoffPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadOnoffSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffSpeed {
        SwPadCtlPadOnoffSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffSpeed) -> u8 {
        SwPadCtlPadOnoffSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadPmicOnReqDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqDse {
        SwPadCtlPadPmicOnReqDse::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqDse) -> u8 {
        SwPadCtlPadPmicOnReqDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadPmicOnReqPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqPus {
        SwPadCtlPadPmicOnReqPus::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqPus) -> u8 {
        SwPadCtlPadPmicOnReqPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadPmicOnReqSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqSpeed {
        SwPadCtlPadPmicOnReqSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqSpeed) -> u8 {
        SwPadCtlPadPmicOnReqSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadPmicStbyReqDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqDse {
        SwPadCtlPadPmicStbyReqDse::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqDse) -> u8 {
        SwPadCtlPadPmicStbyReqDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadPmicStbyReqPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqPus {
        SwPadCtlPadPmicStbyReqPus::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqPus) -> u8 {
        SwPadCtlPadPmicStbyReqPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadPmicStbyReqSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqSpeed {
        SwPadCtlPadPmicStbyReqSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqSpeed) -> u8 {
        SwPadCtlPadPmicStbyReqSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBdse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadPorBdse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBdse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBdse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBdse {
        SwPadCtlPadPorBdse::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBdse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBdse) -> u8 {
        SwPadCtlPadPorBdse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBpus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadPorBpus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBpus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBpus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBpus {
        SwPadCtlPadPorBpus::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBpus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBpus) -> u8 {
        SwPadCtlPadPorBpus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBspeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadPorBspeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBspeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBspeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBspeed {
        SwPadCtlPadPorBspeed::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBspeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBspeed) -> u8 {
        SwPadCtlPadPorBspeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadTestModeDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeDse {
        SwPadCtlPadTestModeDse::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeDse) -> u8 {
        SwPadCtlPadTestModeDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModePus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadTestModePus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModePus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModePus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModePus {
        SwPadCtlPadTestModePus::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModePus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModePus) -> u8 {
        SwPadCtlPadTestModePus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadTestModeSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeSpeed {
        SwPadCtlPadTestModeSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeSpeed) -> u8 {
        SwPadCtlPadTestModeSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadWakeupDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupDse {
        SwPadCtlPadWakeupDse::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupDse) -> u8 {
        SwPadCtlPadWakeupDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadWakeupPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupPus {
        SwPadCtlPadWakeupPus::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupPus) -> u8 {
        SwPadCtlPadWakeupPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadWakeupSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupSpeed {
        SwPadCtlPadWakeupSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupSpeed) -> u8 {
        SwPadCtlPadWakeupSpeed::to_bits(val)
    }
}
