#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl0cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0cl1 {
        Csctrl0cl1::from_bits(val)
    }
}
impl From<Csctrl0cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0cl1) -> u8 {
        Csctrl0cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl0cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0cl2 {
        Csctrl0cl2::from_bits(val)
    }
}
impl From<Csctrl0cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0cl2) -> u8 {
        Csctrl0cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0dbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl0dbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0dbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0dbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0dbgEn {
        Csctrl0dbgEn::from_bits(val)
    }
}
impl From<Csctrl0dbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0dbgEn) -> u8 {
        Csctrl0dbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl1cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1cl1 {
        Csctrl1cl1::from_bits(val)
    }
}
impl From<Csctrl1cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1cl1) -> u8 {
        Csctrl1cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl1cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1cl2 {
        Csctrl1cl2::from_bits(val)
    }
}
impl From<Csctrl1cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1cl2) -> u8 {
        Csctrl1cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1dbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl1dbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1dbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1dbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1dbgEn {
        Csctrl1dbgEn::from_bits(val)
    }
}
impl From<Csctrl1dbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1dbgEn) -> u8 {
        Csctrl1dbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl2cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2cl1 {
        Csctrl2cl1::from_bits(val)
    }
}
impl From<Csctrl2cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2cl1) -> u8 {
        Csctrl2cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl2cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2cl2 {
        Csctrl2cl2::from_bits(val)
    }
}
impl From<Csctrl2cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2cl2) -> u8 {
        Csctrl2cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2dbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl2dbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2dbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2dbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2dbgEn {
        Csctrl2dbgEn::from_bits(val)
    }
}
impl From<Csctrl2dbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2dbgEn) -> u8 {
        Csctrl2dbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl3cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3cl1 {
        Csctrl3cl1::from_bits(val)
    }
}
impl From<Csctrl3cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3cl1) -> u8 {
        Csctrl3cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl3cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3cl2 {
        Csctrl3cl2::from_bits(val)
    }
}
impl From<Csctrl3cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3cl2) -> u8 {
        Csctrl3cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3dbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl3dbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3dbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3dbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3dbgEn {
        Csctrl3dbgEn::from_bits(val)
    }
}
impl From<Csctrl3dbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3dbgEn) -> u8 {
        Csctrl3dbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl0cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0cm {
        Ctrl0cm::from_bits(val)
    }
}
impl From<Ctrl0cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0cm) -> u8 {
        Ctrl0cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl0outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0outmode {
        Ctrl0outmode::from_bits(val)
    }
}
impl From<Ctrl0outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0outmode) -> u8 {
        Ctrl0outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl0pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0pcs {
        Ctrl0pcs::from_bits(val)
    }
}
impl From<Ctrl0pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0pcs) -> u8 {
        Ctrl0pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl0scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0scs {
        Ctrl0scs::from_bits(val)
    }
}
impl From<Ctrl0scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0scs) -> u8 {
        Ctrl0scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl1cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1cm {
        Ctrl1cm::from_bits(val)
    }
}
impl From<Ctrl1cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1cm) -> u8 {
        Ctrl1cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl1outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1outmode {
        Ctrl1outmode::from_bits(val)
    }
}
impl From<Ctrl1outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1outmode) -> u8 {
        Ctrl1outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl1pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1pcs {
        Ctrl1pcs::from_bits(val)
    }
}
impl From<Ctrl1pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1pcs) -> u8 {
        Ctrl1pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl1scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1scs {
        Ctrl1scs::from_bits(val)
    }
}
impl From<Ctrl1scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1scs) -> u8 {
        Ctrl1scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl2cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2cm {
        Ctrl2cm::from_bits(val)
    }
}
impl From<Ctrl2cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2cm) -> u8 {
        Ctrl2cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl2outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2outmode {
        Ctrl2outmode::from_bits(val)
    }
}
impl From<Ctrl2outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2outmode) -> u8 {
        Ctrl2outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl2pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2pcs {
        Ctrl2pcs::from_bits(val)
    }
}
impl From<Ctrl2pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2pcs) -> u8 {
        Ctrl2pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl2scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2scs {
        Ctrl2scs::from_bits(val)
    }
}
impl From<Ctrl2scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2scs) -> u8 {
        Ctrl2scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl3cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3cm {
        Ctrl3cm::from_bits(val)
    }
}
impl From<Ctrl3cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3cm) -> u8 {
        Ctrl3cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl3outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3outmode {
        Ctrl3outmode::from_bits(val)
    }
}
impl From<Ctrl3outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3outmode) -> u8 {
        Ctrl3outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl3pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3pcs {
        Ctrl3pcs::from_bits(val)
    }
}
impl From<Ctrl3pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3pcs) -> u8 {
        Ctrl3pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl3scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3scs {
        Ctrl3scs::from_bits(val)
    }
}
impl From<Ctrl3scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3scs) -> u8 {
        Ctrl3scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enbl {
    #[doc = "Timer channel is disabled."]
    DISABLE = 0x0,
    #[doc = "Timer channel is enabled. (default)"]
    ENABLE = 0x01,
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
}
impl Enbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enbl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enbl {
    #[inline(always)]
    fn from(val: u8) -> Enbl {
        Enbl::from_bits(val)
    }
}
impl From<Enbl> for u8 {
    #[inline(always)]
    fn from(val: Enbl) -> u8 {
        Enbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl0captureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl0captureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl0captureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl0captureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl0captureMode {
        Sctrl0captureMode::from_bits(val)
    }
}
impl From<Sctrl0captureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl0captureMode) -> u8 {
        Sctrl0captureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl1captureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl1captureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl1captureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl1captureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl1captureMode {
        Sctrl1captureMode::from_bits(val)
    }
}
impl From<Sctrl1captureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl1captureMode) -> u8 {
        Sctrl1captureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl2captureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl2captureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl2captureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl2captureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl2captureMode {
        Sctrl2captureMode::from_bits(val)
    }
}
impl From<Sctrl2captureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl2captureMode) -> u8 {
        Sctrl2captureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl3captureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl3captureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl3captureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl3captureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl3captureMode {
        Sctrl3captureMode::from_bits(val)
    }
}
impl From<Sctrl3captureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl3captureMode) -> u8 {
        Sctrl3captureMode::to_bits(val)
    }
}
