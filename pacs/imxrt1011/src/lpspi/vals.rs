#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(pub u16);
impl Feature {
    #[doc = "Standard feature set supporting a 32-bit shift register."]
    pub const STANDARD: Self = Self(0x04);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Matcfg {
    #[doc = "Match is disabled"]
    DISABLED = 0,
    _RESERVED_1 = 0x01,
    #[doc = "Match is enabled is 1st data word is MATCH0 or MATCH1"]
    ENABLED_FIRSTDATAMATCH = 0x02,
    #[doc = "Match is enabled on any data word equal MATCH0 or MATCH1"]
    ENABLED_ANYDATAMATCH = 0x03,
    #[doc = "Match is enabled on data match sequence"]
    ENABLED_DATAMATCH_100 = 0x04,
    #[doc = "Match is enabled on data match sequence"]
    ENABLED_DATAMATCH_101 = 0x05,
    #[doc = "Match is enabled"]
    ENABLED_DATAMATCH_110 = 0x06,
    #[doc = "Match is enabled"]
    ENABLED_DATAMATCH_111 = 0x07,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pcs {
    #[doc = "Transfer using PCS\\[0\\]"]
    TX_PCS0 = 0,
    #[doc = "Transfer using PCS\\[1\\]"]
    TX_PCS1 = 0x01,
    #[doc = "Transfer using PCS\\[2\\]"]
    TX_PCS2 = 0x02,
    #[doc = "Transfer using PCS\\[3\\]"]
    TX_PCS3 = 0x03,
}
impl Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcs {
    #[inline(always)]
    fn from(val: u8) -> Pcs {
        Pcs::from_bits(val)
    }
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(val: Pcs) -> u8 {
        Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pincfg {
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    SIN_IN_SOUT_OUT = 0,
    #[doc = "SIN is used for both input and output data, only half-duplex serial transfers are supported"]
    SIN_BOTH_IN_OUT = 0x01,
    #[doc = "SOUT is used for both input and output data, only half-duplex serial transfers are supported"]
    SOUT_BOTH_IN_OUT = 0x02,
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    SOUT_IN_SIN_OUT = 0x03,
}
impl Pincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pincfg {
    #[inline(always)]
    fn from(val: u8) -> Pincfg {
        Pincfg::from_bits(val)
    }
}
impl From<Pincfg> for u8 {
    #[inline(always)]
    fn from(val: Pincfg) -> u8 {
        Pincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Prescale {
    #[doc = "Divide by 1"]
    DIVIDEBY1 = 0,
    #[doc = "Divide by 2"]
    DIVIDEBY2 = 0x01,
    #[doc = "Divide by 4"]
    DIVIDEBY4 = 0x02,
    #[doc = "Divide by 8"]
    DIVIDEBY8 = 0x03,
    #[doc = "Divide by 16"]
    DIVIDEBY16 = 0x04,
    #[doc = "Divide by 32"]
    DIVIDEBY32 = 0x05,
    #[doc = "Divide by 64"]
    DIVIDEBY64 = 0x06,
    #[doc = "Divide by 128"]
    DIVIDEBY128 = 0x07,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Width {
    #[doc = "1 bit transfer"]
    ONEBIT = 0,
    #[doc = "2 bit transfer"]
    TWOBIT = 0x01,
    #[doc = "4 bit transfer"]
    FOURBIT = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
