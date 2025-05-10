#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Buf31to8i(u32);
impl Buf31to8i {
    #[doc = "No such occurrence"]
    pub const BUF31TO8I_0: Self = Self(0x0);
    #[doc = "The corresponding MB has successfully completed transmission or reception"]
    pub const BUF31TO8I_1: Self = Self(0x01);
}
impl Buf31to8i {
    pub const fn from_bits(val: u32) -> Buf31to8i {
        Self(val & 0x00ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Buf31to8i {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BUF31TO8I_0"),
            0x01 => f.write_str("BUF31TO8I_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf31to8i {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BUF31TO8I_0"),
            0x01 => defmt::write!(f, "BUF31TO8I_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Buf31to8i {
    #[inline(always)]
    fn from(val: u32) -> Buf31to8i {
        Buf31to8i::from_bits(val)
    }
}
impl From<Buf31to8i> for u32 {
    #[inline(always)]
    fn from(val: Buf31to8i) -> u32 {
        Buf31to8i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf4to0i {
    #[doc = "No such occurrence"]
    BUF4TO0I_0 = 0x0,
    #[doc = "Corresponding MB completed transmission/reception"]
    BUF4TO0I_1 = 0x01,
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
}
impl Buf4to0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf4to0i {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf4to0i {
    #[inline(always)]
    fn from(val: u8) -> Buf4to0i {
        Buf4to0i::from_bits(val)
    }
}
impl From<Buf4to0i> for u8 {
    #[inline(always)]
    fn from(val: Buf4to0i) -> u8 {
        Buf4to0i::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bufhi(u32);
impl Bufhi {
    #[doc = "No such occurrence"]
    pub const BUFHI_0: Self = Self(0x0);
    #[doc = "The corresponding buffer has successfully completed transmission or reception"]
    pub const BUFHI_1: Self = Self(0x01);
}
impl Bufhi {
    pub const fn from_bits(val: u32) -> Bufhi {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Bufhi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BUFHI_0"),
            0x01 => f.write_str("BUFHI_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bufhi {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BUFHI_0"),
            0x01 => defmt::write!(f, "BUFHI_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Bufhi {
    #[inline(always)]
    fn from(val: u32) -> Bufhi {
        Bufhi::from_bits(val)
    }
}
impl From<Bufhi> for u32 {
    #[inline(always)]
    fn from(val: Bufhi) -> u32 {
        Bufhi::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bufhm(u32);
impl Bufhm {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    pub const BUFHM_0: Self = Self(0x0);
    #[doc = "The corresponding buffer Interrupt is enabled"]
    pub const BUFHM_1: Self = Self(0x01);
}
impl Bufhm {
    pub const fn from_bits(val: u32) -> Bufhm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Bufhm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BUFHM_0"),
            0x01 => f.write_str("BUFHM_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bufhm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BUFHM_0"),
            0x01 => defmt::write!(f, "BUFHM_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Bufhm {
    #[inline(always)]
    fn from(val: u32) -> Bufhm {
        Bufhm::from_bits(val)
    }
}
impl From<Bufhm> for u32 {
    #[inline(always)]
    fn from(val: Bufhm) -> u32 {
        Bufhm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Buflm(u32);
impl Buflm {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    pub const BUFLM_0: Self = Self(0x0);
    #[doc = "The corresponding buffer Interrupt is enabled"]
    pub const BUFLM_1: Self = Self(0x01);
}
impl Buflm {
    pub const fn from_bits(val: u32) -> Buflm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Buflm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BUFLM_0"),
            0x01 => f.write_str("BUFLM_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buflm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BUFLM_0"),
            0x01 => defmt::write!(f, "BUFLM_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Buflm {
    #[inline(always)]
    fn from(val: u32) -> Buflm {
        Buflm::from_bits(val)
    }
}
impl From<Buflm> for u32 {
    #[inline(always)]
    fn from(val: Buflm) -> u32 {
        Buflm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fgm(u32);
impl Fgm {
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    pub const FGM_0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked"]
    pub const FGM_1: Self = Self(0x01);
}
impl Fgm {
    pub const fn from_bits(val: u32) -> Fgm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Fgm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FGM_0"),
            0x01 => f.write_str("FGM_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fgm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FGM_0"),
            0x01 => defmt::write!(f, "FGM_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Fgm {
    #[inline(always)]
    fn from(val: u32) -> Fgm {
        Fgm::from_bits(val)
    }
}
impl From<Fgm> for u32 {
    #[inline(always)]
    fn from(val: Fgm) -> u32 {
        Fgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fltconf {
    #[doc = "Error Active"]
    FLTCONF_0 = 0x0,
    #[doc = "Error Passive"]
    FLTCONF_1 = 0x01,
    #[doc = "Bus off"]
    FLTCONF_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fltconf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fltconf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fltconf {
    #[inline(always)]
    fn from(val: u8) -> Fltconf {
        Fltconf::from_bits(val)
    }
}
impl From<Fltconf> for u8 {
    #[inline(always)]
    fn from(val: Fltconf) -> u8 {
        Fltconf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idam {
    #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
    IDAM_0 = 0x0,
    #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    IDAM_1 = 0x01,
    #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    IDAM_2 = 0x02,
    #[doc = "Format D All frames rejected."]
    IDAM_3 = 0x03,
}
impl Idam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idam {
    #[inline(always)]
    fn from(val: u8) -> Idam {
        Idam::from_bits(val)
    }
}
impl From<Idam> for u8 {
    #[inline(always)]
    fn from(val: Idam) -> u8 {
        Idam::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mg(u32);
impl Mg {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    pub const MG_0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked against the one received"]
    pub const MG_1: Self = Self(0x01);
}
impl Mg {
    pub const fn from_bits(val: u32) -> Mg {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Mg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MG_0"),
            0x01 => f.write_str("MG_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mg {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MG_0"),
            0x01 => defmt::write!(f, "MG_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Mg {
    #[inline(always)]
    fn from(val: u32) -> Mg {
        Mg::from_bits(val)
    }
}
impl From<Mg> for u32 {
    #[inline(always)]
    fn from(val: Mg) -> u32 {
        Mg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mi(u32);
impl Mi {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    pub const MI_0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked"]
    pub const MI_1: Self = Self(0x01);
}
impl Mi {
    pub const fn from_bits(val: u32) -> Mi {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Mi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MI_0"),
            0x01 => f.write_str("MI_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mi {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MI_0"),
            0x01 => defmt::write!(f, "MI_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Mi {
    #[inline(always)]
    fn from(val: u32) -> Mi {
        Mi::from_bits(val)
    }
}
impl From<Mi> for u32 {
    #[inline(always)]
    fn from(val: Mi) -> u32 {
        Mi::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rx14m(u32);
impl Rx14m {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    pub const RX14M_0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked"]
    pub const RX14M_1: Self = Self(0x01);
}
impl Rx14m {
    pub const fn from_bits(val: u32) -> Rx14m {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rx14m {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RX14M_0"),
            0x01 => f.write_str("RX14M_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx14m {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RX14M_0"),
            0x01 => defmt::write!(f, "RX14M_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rx14m {
    #[inline(always)]
    fn from(val: u32) -> Rx14m {
        Rx14m::from_bits(val)
    }
}
impl From<Rx14m> for u32 {
    #[inline(always)]
    fn from(val: Rx14m) -> u32 {
        Rx14m::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rx15m(u32);
impl Rx15m {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    pub const RX15M_0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked"]
    pub const RX15M_1: Self = Self(0x01);
}
impl Rx15m {
    pub const fn from_bits(val: u32) -> Rx15m {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rx15m {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RX15M_0"),
            0x01 => f.write_str("RX15M_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx15m {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RX15M_0"),
            0x01 => defmt::write!(f, "RX15M_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rx15m {
    #[inline(always)]
    fn from(val: u32) -> Rx15m {
        Rx15m::from_bits(val)
    }
}
impl From<Rx15m> for u32 {
    #[inline(always)]
    fn from(val: Rx15m) -> u32 {
        Rx15m::to_bits(val)
    }
}
