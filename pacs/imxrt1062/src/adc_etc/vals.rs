#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig0chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain10csel0 {
        Trig0chain10csel0::from_bits(val)
    }
}
impl From<Trig0chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain10csel0) -> u8 {
        Trig0chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig0chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain10csel1 {
        Trig0chain10csel1::from_bits(val)
    }
}
impl From<Trig0chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain10csel1) -> u8 {
        Trig0chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain10hwts0(u8);
impl Trig0chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig0chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig0chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain10hwts0 {
        Trig0chain10hwts0::from_bits(val)
    }
}
impl From<Trig0chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain10hwts0) -> u8 {
        Trig0chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain10hwts1(u8);
impl Trig0chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig0chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig0chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain10hwts1 {
        Trig0chain10hwts1::from_bits(val)
    }
}
impl From<Trig0chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain10hwts1) -> u8 {
        Trig0chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig0chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain10ie0 {
        Trig0chain10ie0::from_bits(val)
    }
}
impl From<Trig0chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain10ie0) -> u8 {
        Trig0chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig0chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain10ie1 {
        Trig0chain10ie1::from_bits(val)
    }
}
impl From<Trig0chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain10ie1) -> u8 {
        Trig0chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig0chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain32csel2 {
        Trig0chain32csel2::from_bits(val)
    }
}
impl From<Trig0chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain32csel2) -> u8 {
        Trig0chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig0chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain32csel3 {
        Trig0chain32csel3::from_bits(val)
    }
}
impl From<Trig0chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain32csel3) -> u8 {
        Trig0chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain32hwts2(u8);
impl Trig0chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig0chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig0chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain32hwts2 {
        Trig0chain32hwts2::from_bits(val)
    }
}
impl From<Trig0chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain32hwts2) -> u8 {
        Trig0chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain32hwts3(u8);
impl Trig0chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig0chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig0chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain32hwts3 {
        Trig0chain32hwts3::from_bits(val)
    }
}
impl From<Trig0chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain32hwts3) -> u8 {
        Trig0chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig0chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain32ie2 {
        Trig0chain32ie2::from_bits(val)
    }
}
impl From<Trig0chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain32ie2) -> u8 {
        Trig0chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig0chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain32ie3 {
        Trig0chain32ie3::from_bits(val)
    }
}
impl From<Trig0chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain32ie3) -> u8 {
        Trig0chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig0chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain54csel4 {
        Trig0chain54csel4::from_bits(val)
    }
}
impl From<Trig0chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain54csel4) -> u8 {
        Trig0chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig0chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain54csel5 {
        Trig0chain54csel5::from_bits(val)
    }
}
impl From<Trig0chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain54csel5) -> u8 {
        Trig0chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain54hwts4(u8);
impl Trig0chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig0chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig0chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain54hwts4 {
        Trig0chain54hwts4::from_bits(val)
    }
}
impl From<Trig0chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain54hwts4) -> u8 {
        Trig0chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain54hwts5(u8);
impl Trig0chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig0chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig0chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain54hwts5 {
        Trig0chain54hwts5::from_bits(val)
    }
}
impl From<Trig0chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain54hwts5) -> u8 {
        Trig0chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig0chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain54ie4 {
        Trig0chain54ie4::from_bits(val)
    }
}
impl From<Trig0chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain54ie4) -> u8 {
        Trig0chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig0chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain54ie5 {
        Trig0chain54ie5::from_bits(val)
    }
}
impl From<Trig0chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain54ie5) -> u8 {
        Trig0chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig0chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain76csel6 {
        Trig0chain76csel6::from_bits(val)
    }
}
impl From<Trig0chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain76csel6) -> u8 {
        Trig0chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig0chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain76csel7 {
        Trig0chain76csel7::from_bits(val)
    }
}
impl From<Trig0chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain76csel7) -> u8 {
        Trig0chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain76hwts6(u8);
impl Trig0chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig0chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig0chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain76hwts6 {
        Trig0chain76hwts6::from_bits(val)
    }
}
impl From<Trig0chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain76hwts6) -> u8 {
        Trig0chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig0chain76hwts7(u8);
impl Trig0chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig0chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig0chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig0chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig0chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain76hwts7 {
        Trig0chain76hwts7::from_bits(val)
    }
}
impl From<Trig0chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain76hwts7) -> u8 {
        Trig0chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig0chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain76ie6 {
        Trig0chain76ie6::from_bits(val)
    }
}
impl From<Trig0chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain76ie6) -> u8 {
        Trig0chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig0chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig0chain76ie7 {
        Trig0chain76ie7::from_bits(val)
    }
}
impl From<Trig0chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig0chain76ie7) -> u8 {
        Trig0chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig0ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig0ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig0ctrlTrigChain {
        Trig0ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig0ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig0ctrlTrigChain) -> u8 {
        Trig0ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig1chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain10csel0 {
        Trig1chain10csel0::from_bits(val)
    }
}
impl From<Trig1chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain10csel0) -> u8 {
        Trig1chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig1chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain10csel1 {
        Trig1chain10csel1::from_bits(val)
    }
}
impl From<Trig1chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain10csel1) -> u8 {
        Trig1chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain10hwts0(u8);
impl Trig1chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig1chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig1chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain10hwts0 {
        Trig1chain10hwts0::from_bits(val)
    }
}
impl From<Trig1chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain10hwts0) -> u8 {
        Trig1chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain10hwts1(u8);
impl Trig1chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig1chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig1chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain10hwts1 {
        Trig1chain10hwts1::from_bits(val)
    }
}
impl From<Trig1chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain10hwts1) -> u8 {
        Trig1chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig1chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain10ie0 {
        Trig1chain10ie0::from_bits(val)
    }
}
impl From<Trig1chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain10ie0) -> u8 {
        Trig1chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig1chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain10ie1 {
        Trig1chain10ie1::from_bits(val)
    }
}
impl From<Trig1chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain10ie1) -> u8 {
        Trig1chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig1chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain32csel2 {
        Trig1chain32csel2::from_bits(val)
    }
}
impl From<Trig1chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain32csel2) -> u8 {
        Trig1chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig1chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain32csel3 {
        Trig1chain32csel3::from_bits(val)
    }
}
impl From<Trig1chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain32csel3) -> u8 {
        Trig1chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain32hwts2(u8);
impl Trig1chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig1chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig1chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain32hwts2 {
        Trig1chain32hwts2::from_bits(val)
    }
}
impl From<Trig1chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain32hwts2) -> u8 {
        Trig1chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain32hwts3(u8);
impl Trig1chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig1chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig1chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain32hwts3 {
        Trig1chain32hwts3::from_bits(val)
    }
}
impl From<Trig1chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain32hwts3) -> u8 {
        Trig1chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig1chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain32ie2 {
        Trig1chain32ie2::from_bits(val)
    }
}
impl From<Trig1chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain32ie2) -> u8 {
        Trig1chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig1chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain32ie3 {
        Trig1chain32ie3::from_bits(val)
    }
}
impl From<Trig1chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain32ie3) -> u8 {
        Trig1chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig1chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain54csel4 {
        Trig1chain54csel4::from_bits(val)
    }
}
impl From<Trig1chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain54csel4) -> u8 {
        Trig1chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig1chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain54csel5 {
        Trig1chain54csel5::from_bits(val)
    }
}
impl From<Trig1chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain54csel5) -> u8 {
        Trig1chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain54hwts4(u8);
impl Trig1chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig1chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig1chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain54hwts4 {
        Trig1chain54hwts4::from_bits(val)
    }
}
impl From<Trig1chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain54hwts4) -> u8 {
        Trig1chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain54hwts5(u8);
impl Trig1chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig1chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig1chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain54hwts5 {
        Trig1chain54hwts5::from_bits(val)
    }
}
impl From<Trig1chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain54hwts5) -> u8 {
        Trig1chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig1chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain54ie4 {
        Trig1chain54ie4::from_bits(val)
    }
}
impl From<Trig1chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain54ie4) -> u8 {
        Trig1chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig1chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain54ie5 {
        Trig1chain54ie5::from_bits(val)
    }
}
impl From<Trig1chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain54ie5) -> u8 {
        Trig1chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig1chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain76csel6 {
        Trig1chain76csel6::from_bits(val)
    }
}
impl From<Trig1chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain76csel6) -> u8 {
        Trig1chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig1chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain76csel7 {
        Trig1chain76csel7::from_bits(val)
    }
}
impl From<Trig1chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain76csel7) -> u8 {
        Trig1chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain76hwts6(u8);
impl Trig1chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig1chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig1chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain76hwts6 {
        Trig1chain76hwts6::from_bits(val)
    }
}
impl From<Trig1chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain76hwts6) -> u8 {
        Trig1chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig1chain76hwts7(u8);
impl Trig1chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig1chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig1chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig1chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig1chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain76hwts7 {
        Trig1chain76hwts7::from_bits(val)
    }
}
impl From<Trig1chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain76hwts7) -> u8 {
        Trig1chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig1chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain76ie6 {
        Trig1chain76ie6::from_bits(val)
    }
}
impl From<Trig1chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain76ie6) -> u8 {
        Trig1chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig1chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig1chain76ie7 {
        Trig1chain76ie7::from_bits(val)
    }
}
impl From<Trig1chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig1chain76ie7) -> u8 {
        Trig1chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig1ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig1ctrlTrigChain {
        Trig1ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig1ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig1ctrlTrigChain) -> u8 {
        Trig1ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig2chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain10csel0 {
        Trig2chain10csel0::from_bits(val)
    }
}
impl From<Trig2chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain10csel0) -> u8 {
        Trig2chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig2chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain10csel1 {
        Trig2chain10csel1::from_bits(val)
    }
}
impl From<Trig2chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain10csel1) -> u8 {
        Trig2chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain10hwts0(u8);
impl Trig2chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig2chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig2chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain10hwts0 {
        Trig2chain10hwts0::from_bits(val)
    }
}
impl From<Trig2chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain10hwts0) -> u8 {
        Trig2chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain10hwts1(u8);
impl Trig2chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig2chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig2chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain10hwts1 {
        Trig2chain10hwts1::from_bits(val)
    }
}
impl From<Trig2chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain10hwts1) -> u8 {
        Trig2chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig2chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain10ie0 {
        Trig2chain10ie0::from_bits(val)
    }
}
impl From<Trig2chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain10ie0) -> u8 {
        Trig2chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig2chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain10ie1 {
        Trig2chain10ie1::from_bits(val)
    }
}
impl From<Trig2chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain10ie1) -> u8 {
        Trig2chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig2chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain32csel2 {
        Trig2chain32csel2::from_bits(val)
    }
}
impl From<Trig2chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain32csel2) -> u8 {
        Trig2chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig2chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain32csel3 {
        Trig2chain32csel3::from_bits(val)
    }
}
impl From<Trig2chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain32csel3) -> u8 {
        Trig2chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain32hwts2(u8);
impl Trig2chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig2chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig2chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain32hwts2 {
        Trig2chain32hwts2::from_bits(val)
    }
}
impl From<Trig2chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain32hwts2) -> u8 {
        Trig2chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain32hwts3(u8);
impl Trig2chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig2chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig2chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain32hwts3 {
        Trig2chain32hwts3::from_bits(val)
    }
}
impl From<Trig2chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain32hwts3) -> u8 {
        Trig2chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig2chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain32ie2 {
        Trig2chain32ie2::from_bits(val)
    }
}
impl From<Trig2chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain32ie2) -> u8 {
        Trig2chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig2chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain32ie3 {
        Trig2chain32ie3::from_bits(val)
    }
}
impl From<Trig2chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain32ie3) -> u8 {
        Trig2chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig2chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain54csel4 {
        Trig2chain54csel4::from_bits(val)
    }
}
impl From<Trig2chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain54csel4) -> u8 {
        Trig2chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig2chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain54csel5 {
        Trig2chain54csel5::from_bits(val)
    }
}
impl From<Trig2chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain54csel5) -> u8 {
        Trig2chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain54hwts4(u8);
impl Trig2chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig2chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig2chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain54hwts4 {
        Trig2chain54hwts4::from_bits(val)
    }
}
impl From<Trig2chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain54hwts4) -> u8 {
        Trig2chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain54hwts5(u8);
impl Trig2chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig2chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig2chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain54hwts5 {
        Trig2chain54hwts5::from_bits(val)
    }
}
impl From<Trig2chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain54hwts5) -> u8 {
        Trig2chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig2chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain54ie4 {
        Trig2chain54ie4::from_bits(val)
    }
}
impl From<Trig2chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain54ie4) -> u8 {
        Trig2chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig2chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain54ie5 {
        Trig2chain54ie5::from_bits(val)
    }
}
impl From<Trig2chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain54ie5) -> u8 {
        Trig2chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig2chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain76csel6 {
        Trig2chain76csel6::from_bits(val)
    }
}
impl From<Trig2chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain76csel6) -> u8 {
        Trig2chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig2chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain76csel7 {
        Trig2chain76csel7::from_bits(val)
    }
}
impl From<Trig2chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain76csel7) -> u8 {
        Trig2chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain76hwts6(u8);
impl Trig2chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig2chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig2chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain76hwts6 {
        Trig2chain76hwts6::from_bits(val)
    }
}
impl From<Trig2chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain76hwts6) -> u8 {
        Trig2chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig2chain76hwts7(u8);
impl Trig2chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig2chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig2chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig2chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig2chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain76hwts7 {
        Trig2chain76hwts7::from_bits(val)
    }
}
impl From<Trig2chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain76hwts7) -> u8 {
        Trig2chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig2chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain76ie6 {
        Trig2chain76ie6::from_bits(val)
    }
}
impl From<Trig2chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain76ie6) -> u8 {
        Trig2chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig2chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig2chain76ie7 {
        Trig2chain76ie7::from_bits(val)
    }
}
impl From<Trig2chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig2chain76ie7) -> u8 {
        Trig2chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig2ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig2ctrlTrigChain {
        Trig2ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig2ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig2ctrlTrigChain) -> u8 {
        Trig2ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig3chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain10csel0 {
        Trig3chain10csel0::from_bits(val)
    }
}
impl From<Trig3chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain10csel0) -> u8 {
        Trig3chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig3chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain10csel1 {
        Trig3chain10csel1::from_bits(val)
    }
}
impl From<Trig3chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain10csel1) -> u8 {
        Trig3chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain10hwts0(u8);
impl Trig3chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig3chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig3chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain10hwts0 {
        Trig3chain10hwts0::from_bits(val)
    }
}
impl From<Trig3chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain10hwts0) -> u8 {
        Trig3chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain10hwts1(u8);
impl Trig3chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig3chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig3chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain10hwts1 {
        Trig3chain10hwts1::from_bits(val)
    }
}
impl From<Trig3chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain10hwts1) -> u8 {
        Trig3chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig3chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain10ie0 {
        Trig3chain10ie0::from_bits(val)
    }
}
impl From<Trig3chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain10ie0) -> u8 {
        Trig3chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig3chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain10ie1 {
        Trig3chain10ie1::from_bits(val)
    }
}
impl From<Trig3chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain10ie1) -> u8 {
        Trig3chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig3chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain32csel2 {
        Trig3chain32csel2::from_bits(val)
    }
}
impl From<Trig3chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain32csel2) -> u8 {
        Trig3chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig3chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain32csel3 {
        Trig3chain32csel3::from_bits(val)
    }
}
impl From<Trig3chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain32csel3) -> u8 {
        Trig3chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain32hwts2(u8);
impl Trig3chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig3chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig3chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain32hwts2 {
        Trig3chain32hwts2::from_bits(val)
    }
}
impl From<Trig3chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain32hwts2) -> u8 {
        Trig3chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain32hwts3(u8);
impl Trig3chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig3chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig3chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain32hwts3 {
        Trig3chain32hwts3::from_bits(val)
    }
}
impl From<Trig3chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain32hwts3) -> u8 {
        Trig3chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig3chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain32ie2 {
        Trig3chain32ie2::from_bits(val)
    }
}
impl From<Trig3chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain32ie2) -> u8 {
        Trig3chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig3chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain32ie3 {
        Trig3chain32ie3::from_bits(val)
    }
}
impl From<Trig3chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain32ie3) -> u8 {
        Trig3chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig3chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain54csel4 {
        Trig3chain54csel4::from_bits(val)
    }
}
impl From<Trig3chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain54csel4) -> u8 {
        Trig3chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig3chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain54csel5 {
        Trig3chain54csel5::from_bits(val)
    }
}
impl From<Trig3chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain54csel5) -> u8 {
        Trig3chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain54hwts4(u8);
impl Trig3chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig3chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig3chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain54hwts4 {
        Trig3chain54hwts4::from_bits(val)
    }
}
impl From<Trig3chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain54hwts4) -> u8 {
        Trig3chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain54hwts5(u8);
impl Trig3chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig3chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig3chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain54hwts5 {
        Trig3chain54hwts5::from_bits(val)
    }
}
impl From<Trig3chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain54hwts5) -> u8 {
        Trig3chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig3chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain54ie4 {
        Trig3chain54ie4::from_bits(val)
    }
}
impl From<Trig3chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain54ie4) -> u8 {
        Trig3chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig3chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain54ie5 {
        Trig3chain54ie5::from_bits(val)
    }
}
impl From<Trig3chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain54ie5) -> u8 {
        Trig3chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig3chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain76csel6 {
        Trig3chain76csel6::from_bits(val)
    }
}
impl From<Trig3chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain76csel6) -> u8 {
        Trig3chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig3chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain76csel7 {
        Trig3chain76csel7::from_bits(val)
    }
}
impl From<Trig3chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain76csel7) -> u8 {
        Trig3chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain76hwts6(u8);
impl Trig3chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig3chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig3chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain76hwts6 {
        Trig3chain76hwts6::from_bits(val)
    }
}
impl From<Trig3chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain76hwts6) -> u8 {
        Trig3chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig3chain76hwts7(u8);
impl Trig3chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig3chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig3chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig3chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig3chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain76hwts7 {
        Trig3chain76hwts7::from_bits(val)
    }
}
impl From<Trig3chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain76hwts7) -> u8 {
        Trig3chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig3chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain76ie6 {
        Trig3chain76ie6::from_bits(val)
    }
}
impl From<Trig3chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain76ie6) -> u8 {
        Trig3chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig3chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig3chain76ie7 {
        Trig3chain76ie7::from_bits(val)
    }
}
impl From<Trig3chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig3chain76ie7) -> u8 {
        Trig3chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig3ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig3ctrlTrigChain {
        Trig3ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig3ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig3ctrlTrigChain) -> u8 {
        Trig3ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig4chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain10csel0 {
        Trig4chain10csel0::from_bits(val)
    }
}
impl From<Trig4chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain10csel0) -> u8 {
        Trig4chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig4chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain10csel1 {
        Trig4chain10csel1::from_bits(val)
    }
}
impl From<Trig4chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain10csel1) -> u8 {
        Trig4chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain10hwts0(u8);
impl Trig4chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig4chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig4chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain10hwts0 {
        Trig4chain10hwts0::from_bits(val)
    }
}
impl From<Trig4chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain10hwts0) -> u8 {
        Trig4chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain10hwts1(u8);
impl Trig4chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig4chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig4chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain10hwts1 {
        Trig4chain10hwts1::from_bits(val)
    }
}
impl From<Trig4chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain10hwts1) -> u8 {
        Trig4chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig4chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain10ie0 {
        Trig4chain10ie0::from_bits(val)
    }
}
impl From<Trig4chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain10ie0) -> u8 {
        Trig4chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig4chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain10ie1 {
        Trig4chain10ie1::from_bits(val)
    }
}
impl From<Trig4chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain10ie1) -> u8 {
        Trig4chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig4chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain32csel2 {
        Trig4chain32csel2::from_bits(val)
    }
}
impl From<Trig4chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain32csel2) -> u8 {
        Trig4chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig4chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain32csel3 {
        Trig4chain32csel3::from_bits(val)
    }
}
impl From<Trig4chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain32csel3) -> u8 {
        Trig4chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain32hwts2(u8);
impl Trig4chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig4chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig4chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain32hwts2 {
        Trig4chain32hwts2::from_bits(val)
    }
}
impl From<Trig4chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain32hwts2) -> u8 {
        Trig4chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain32hwts3(u8);
impl Trig4chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig4chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig4chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain32hwts3 {
        Trig4chain32hwts3::from_bits(val)
    }
}
impl From<Trig4chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain32hwts3) -> u8 {
        Trig4chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig4chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain32ie2 {
        Trig4chain32ie2::from_bits(val)
    }
}
impl From<Trig4chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain32ie2) -> u8 {
        Trig4chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig4chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain32ie3 {
        Trig4chain32ie3::from_bits(val)
    }
}
impl From<Trig4chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain32ie3) -> u8 {
        Trig4chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig4chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain54csel4 {
        Trig4chain54csel4::from_bits(val)
    }
}
impl From<Trig4chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain54csel4) -> u8 {
        Trig4chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig4chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain54csel5 {
        Trig4chain54csel5::from_bits(val)
    }
}
impl From<Trig4chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain54csel5) -> u8 {
        Trig4chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain54hwts4(u8);
impl Trig4chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig4chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig4chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain54hwts4 {
        Trig4chain54hwts4::from_bits(val)
    }
}
impl From<Trig4chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain54hwts4) -> u8 {
        Trig4chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain54hwts5(u8);
impl Trig4chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig4chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig4chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain54hwts5 {
        Trig4chain54hwts5::from_bits(val)
    }
}
impl From<Trig4chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain54hwts5) -> u8 {
        Trig4chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig4chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain54ie4 {
        Trig4chain54ie4::from_bits(val)
    }
}
impl From<Trig4chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain54ie4) -> u8 {
        Trig4chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig4chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain54ie5 {
        Trig4chain54ie5::from_bits(val)
    }
}
impl From<Trig4chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain54ie5) -> u8 {
        Trig4chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig4chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain76csel6 {
        Trig4chain76csel6::from_bits(val)
    }
}
impl From<Trig4chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain76csel6) -> u8 {
        Trig4chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig4chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain76csel7 {
        Trig4chain76csel7::from_bits(val)
    }
}
impl From<Trig4chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain76csel7) -> u8 {
        Trig4chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain76hwts6(u8);
impl Trig4chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig4chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig4chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain76hwts6 {
        Trig4chain76hwts6::from_bits(val)
    }
}
impl From<Trig4chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain76hwts6) -> u8 {
        Trig4chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig4chain76hwts7(u8);
impl Trig4chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig4chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig4chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig4chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig4chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain76hwts7 {
        Trig4chain76hwts7::from_bits(val)
    }
}
impl From<Trig4chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain76hwts7) -> u8 {
        Trig4chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig4chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain76ie6 {
        Trig4chain76ie6::from_bits(val)
    }
}
impl From<Trig4chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain76ie6) -> u8 {
        Trig4chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig4chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig4chain76ie7 {
        Trig4chain76ie7::from_bits(val)
    }
}
impl From<Trig4chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig4chain76ie7) -> u8 {
        Trig4chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig4ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig4ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig4ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig4ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig4ctrlTrigChain {
        Trig4ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig4ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig4ctrlTrigChain) -> u8 {
        Trig4ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig5chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain10csel0 {
        Trig5chain10csel0::from_bits(val)
    }
}
impl From<Trig5chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain10csel0) -> u8 {
        Trig5chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig5chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain10csel1 {
        Trig5chain10csel1::from_bits(val)
    }
}
impl From<Trig5chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain10csel1) -> u8 {
        Trig5chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain10hwts0(u8);
impl Trig5chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig5chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig5chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain10hwts0 {
        Trig5chain10hwts0::from_bits(val)
    }
}
impl From<Trig5chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain10hwts0) -> u8 {
        Trig5chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain10hwts1(u8);
impl Trig5chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig5chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig5chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain10hwts1 {
        Trig5chain10hwts1::from_bits(val)
    }
}
impl From<Trig5chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain10hwts1) -> u8 {
        Trig5chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig5chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain10ie0 {
        Trig5chain10ie0::from_bits(val)
    }
}
impl From<Trig5chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain10ie0) -> u8 {
        Trig5chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig5chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain10ie1 {
        Trig5chain10ie1::from_bits(val)
    }
}
impl From<Trig5chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain10ie1) -> u8 {
        Trig5chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig5chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain32csel2 {
        Trig5chain32csel2::from_bits(val)
    }
}
impl From<Trig5chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain32csel2) -> u8 {
        Trig5chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig5chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain32csel3 {
        Trig5chain32csel3::from_bits(val)
    }
}
impl From<Trig5chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain32csel3) -> u8 {
        Trig5chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain32hwts2(u8);
impl Trig5chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig5chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig5chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain32hwts2 {
        Trig5chain32hwts2::from_bits(val)
    }
}
impl From<Trig5chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain32hwts2) -> u8 {
        Trig5chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain32hwts3(u8);
impl Trig5chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig5chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig5chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain32hwts3 {
        Trig5chain32hwts3::from_bits(val)
    }
}
impl From<Trig5chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain32hwts3) -> u8 {
        Trig5chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig5chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain32ie2 {
        Trig5chain32ie2::from_bits(val)
    }
}
impl From<Trig5chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain32ie2) -> u8 {
        Trig5chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig5chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain32ie3 {
        Trig5chain32ie3::from_bits(val)
    }
}
impl From<Trig5chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain32ie3) -> u8 {
        Trig5chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig5chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain54csel4 {
        Trig5chain54csel4::from_bits(val)
    }
}
impl From<Trig5chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain54csel4) -> u8 {
        Trig5chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig5chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain54csel5 {
        Trig5chain54csel5::from_bits(val)
    }
}
impl From<Trig5chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain54csel5) -> u8 {
        Trig5chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain54hwts4(u8);
impl Trig5chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig5chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig5chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain54hwts4 {
        Trig5chain54hwts4::from_bits(val)
    }
}
impl From<Trig5chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain54hwts4) -> u8 {
        Trig5chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain54hwts5(u8);
impl Trig5chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig5chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig5chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain54hwts5 {
        Trig5chain54hwts5::from_bits(val)
    }
}
impl From<Trig5chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain54hwts5) -> u8 {
        Trig5chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig5chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain54ie4 {
        Trig5chain54ie4::from_bits(val)
    }
}
impl From<Trig5chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain54ie4) -> u8 {
        Trig5chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig5chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain54ie5 {
        Trig5chain54ie5::from_bits(val)
    }
}
impl From<Trig5chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain54ie5) -> u8 {
        Trig5chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig5chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain76csel6 {
        Trig5chain76csel6::from_bits(val)
    }
}
impl From<Trig5chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain76csel6) -> u8 {
        Trig5chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig5chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain76csel7 {
        Trig5chain76csel7::from_bits(val)
    }
}
impl From<Trig5chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain76csel7) -> u8 {
        Trig5chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain76hwts6(u8);
impl Trig5chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig5chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig5chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain76hwts6 {
        Trig5chain76hwts6::from_bits(val)
    }
}
impl From<Trig5chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain76hwts6) -> u8 {
        Trig5chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig5chain76hwts7(u8);
impl Trig5chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig5chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig5chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig5chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig5chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain76hwts7 {
        Trig5chain76hwts7::from_bits(val)
    }
}
impl From<Trig5chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain76hwts7) -> u8 {
        Trig5chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig5chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain76ie6 {
        Trig5chain76ie6::from_bits(val)
    }
}
impl From<Trig5chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain76ie6) -> u8 {
        Trig5chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig5chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig5chain76ie7 {
        Trig5chain76ie7::from_bits(val)
    }
}
impl From<Trig5chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig5chain76ie7) -> u8 {
        Trig5chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig5ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig5ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig5ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig5ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig5ctrlTrigChain {
        Trig5ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig5ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig5ctrlTrigChain) -> u8 {
        Trig5ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig6chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain10csel0 {
        Trig6chain10csel0::from_bits(val)
    }
}
impl From<Trig6chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain10csel0) -> u8 {
        Trig6chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig6chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain10csel1 {
        Trig6chain10csel1::from_bits(val)
    }
}
impl From<Trig6chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain10csel1) -> u8 {
        Trig6chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain10hwts0(u8);
impl Trig6chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig6chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig6chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain10hwts0 {
        Trig6chain10hwts0::from_bits(val)
    }
}
impl From<Trig6chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain10hwts0) -> u8 {
        Trig6chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain10hwts1(u8);
impl Trig6chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig6chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig6chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain10hwts1 {
        Trig6chain10hwts1::from_bits(val)
    }
}
impl From<Trig6chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain10hwts1) -> u8 {
        Trig6chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig6chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain10ie0 {
        Trig6chain10ie0::from_bits(val)
    }
}
impl From<Trig6chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain10ie0) -> u8 {
        Trig6chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig6chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain10ie1 {
        Trig6chain10ie1::from_bits(val)
    }
}
impl From<Trig6chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain10ie1) -> u8 {
        Trig6chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig6chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain32csel2 {
        Trig6chain32csel2::from_bits(val)
    }
}
impl From<Trig6chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain32csel2) -> u8 {
        Trig6chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig6chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain32csel3 {
        Trig6chain32csel3::from_bits(val)
    }
}
impl From<Trig6chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain32csel3) -> u8 {
        Trig6chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain32hwts2(u8);
impl Trig6chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig6chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig6chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain32hwts2 {
        Trig6chain32hwts2::from_bits(val)
    }
}
impl From<Trig6chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain32hwts2) -> u8 {
        Trig6chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain32hwts3(u8);
impl Trig6chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig6chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig6chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain32hwts3 {
        Trig6chain32hwts3::from_bits(val)
    }
}
impl From<Trig6chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain32hwts3) -> u8 {
        Trig6chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig6chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain32ie2 {
        Trig6chain32ie2::from_bits(val)
    }
}
impl From<Trig6chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain32ie2) -> u8 {
        Trig6chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig6chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain32ie3 {
        Trig6chain32ie3::from_bits(val)
    }
}
impl From<Trig6chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain32ie3) -> u8 {
        Trig6chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig6chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain54csel4 {
        Trig6chain54csel4::from_bits(val)
    }
}
impl From<Trig6chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain54csel4) -> u8 {
        Trig6chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig6chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain54csel5 {
        Trig6chain54csel5::from_bits(val)
    }
}
impl From<Trig6chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain54csel5) -> u8 {
        Trig6chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain54hwts4(u8);
impl Trig6chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig6chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig6chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain54hwts4 {
        Trig6chain54hwts4::from_bits(val)
    }
}
impl From<Trig6chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain54hwts4) -> u8 {
        Trig6chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain54hwts5(u8);
impl Trig6chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig6chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig6chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain54hwts5 {
        Trig6chain54hwts5::from_bits(val)
    }
}
impl From<Trig6chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain54hwts5) -> u8 {
        Trig6chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig6chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain54ie4 {
        Trig6chain54ie4::from_bits(val)
    }
}
impl From<Trig6chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain54ie4) -> u8 {
        Trig6chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig6chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain54ie5 {
        Trig6chain54ie5::from_bits(val)
    }
}
impl From<Trig6chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain54ie5) -> u8 {
        Trig6chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig6chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain76csel6 {
        Trig6chain76csel6::from_bits(val)
    }
}
impl From<Trig6chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain76csel6) -> u8 {
        Trig6chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig6chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain76csel7 {
        Trig6chain76csel7::from_bits(val)
    }
}
impl From<Trig6chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain76csel7) -> u8 {
        Trig6chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain76hwts6(u8);
impl Trig6chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig6chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig6chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain76hwts6 {
        Trig6chain76hwts6::from_bits(val)
    }
}
impl From<Trig6chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain76hwts6) -> u8 {
        Trig6chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig6chain76hwts7(u8);
impl Trig6chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig6chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig6chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig6chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig6chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain76hwts7 {
        Trig6chain76hwts7::from_bits(val)
    }
}
impl From<Trig6chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain76hwts7) -> u8 {
        Trig6chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig6chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain76ie6 {
        Trig6chain76ie6::from_bits(val)
    }
}
impl From<Trig6chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain76ie6) -> u8 {
        Trig6chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig6chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig6chain76ie7 {
        Trig6chain76ie7::from_bits(val)
    }
}
impl From<Trig6chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig6chain76ie7) -> u8 {
        Trig6chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig6ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig6ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig6ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig6ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig6ctrlTrigChain {
        Trig6ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig6ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig6ctrlTrigChain) -> u8 {
        Trig6ctrlTrigChain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain10csel0 {
    #[doc = "ADC Channel 0 selected"]
    CSEL0_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL0_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL0_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL0_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL0_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL0_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL0_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL0_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL0_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL0_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL0_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL0_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL0_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL0_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL0_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL0_15 = 0x0f,
}
impl Trig7chain10csel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain10csel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain10csel0 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain10csel0 {
        Trig7chain10csel0::from_bits(val)
    }
}
impl From<Trig7chain10csel0> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain10csel0) -> u8 {
        Trig7chain10csel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain10csel1 {
    #[doc = "ADC Channel 0 selected"]
    CSEL1_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL1_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL1_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL1_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL1_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL1_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL1_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL1_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL1_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL1_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL1_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL1_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL1_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL1_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL1_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL1_15 = 0x0f,
}
impl Trig7chain10csel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain10csel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain10csel1 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain10csel1 {
        Trig7chain10csel1::from_bits(val)
    }
}
impl From<Trig7chain10csel1> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain10csel1) -> u8 {
        Trig7chain10csel1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain10hwts0(u8);
impl Trig7chain10hwts0 {
    #[doc = "no trigger selected"]
    pub const HWTS0_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS0_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS0_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS0_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS0_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS0_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS0_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS0_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS0_128: Self = Self(0x80);
}
impl Trig7chain10hwts0 {
    pub const fn from_bits(val: u8) -> Trig7chain10hwts0 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain10hwts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS0_0"),
            0x01 => f.write_str("HWTS0_1"),
            0x02 => f.write_str("HWTS0_2"),
            0x04 => f.write_str("HWTS0_4"),
            0x08 => f.write_str("HWTS0_8"),
            0x10 => f.write_str("HWTS0_16"),
            0x20 => f.write_str("HWTS0_32"),
            0x40 => f.write_str("HWTS0_64"),
            0x80 => f.write_str("HWTS0_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain10hwts0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS0_0"),
            0x01 => defmt::write!(f, "HWTS0_1"),
            0x02 => defmt::write!(f, "HWTS0_2"),
            0x04 => defmt::write!(f, "HWTS0_4"),
            0x08 => defmt::write!(f, "HWTS0_8"),
            0x10 => defmt::write!(f, "HWTS0_16"),
            0x20 => defmt::write!(f, "HWTS0_32"),
            0x40 => defmt::write!(f, "HWTS0_64"),
            0x80 => defmt::write!(f, "HWTS0_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain10hwts0 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain10hwts0 {
        Trig7chain10hwts0::from_bits(val)
    }
}
impl From<Trig7chain10hwts0> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain10hwts0) -> u8 {
        Trig7chain10hwts0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain10hwts1(u8);
impl Trig7chain10hwts1 {
    #[doc = "no trigger selected"]
    pub const HWTS1_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS1_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS1_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS1_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS1_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS1_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS1_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS1_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS1_128: Self = Self(0x80);
}
impl Trig7chain10hwts1 {
    pub const fn from_bits(val: u8) -> Trig7chain10hwts1 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain10hwts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS1_0"),
            0x01 => f.write_str("HWTS1_1"),
            0x02 => f.write_str("HWTS1_2"),
            0x04 => f.write_str("HWTS1_4"),
            0x08 => f.write_str("HWTS1_8"),
            0x10 => f.write_str("HWTS1_16"),
            0x20 => f.write_str("HWTS1_32"),
            0x40 => f.write_str("HWTS1_64"),
            0x80 => f.write_str("HWTS1_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain10hwts1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS1_0"),
            0x01 => defmt::write!(f, "HWTS1_1"),
            0x02 => defmt::write!(f, "HWTS1_2"),
            0x04 => defmt::write!(f, "HWTS1_4"),
            0x08 => defmt::write!(f, "HWTS1_8"),
            0x10 => defmt::write!(f, "HWTS1_16"),
            0x20 => defmt::write!(f, "HWTS1_32"),
            0x40 => defmt::write!(f, "HWTS1_64"),
            0x80 => defmt::write!(f, "HWTS1_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain10hwts1 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain10hwts1 {
        Trig7chain10hwts1::from_bits(val)
    }
}
impl From<Trig7chain10hwts1> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain10hwts1) -> u8 {
        Trig7chain10hwts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain10ie0 {
    #[doc = "No interrupt when finished"]
    IE0_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 0x03,
}
impl Trig7chain10ie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain10ie0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain10ie0 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain10ie0 {
        Trig7chain10ie0::from_bits(val)
    }
}
impl From<Trig7chain10ie0> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain10ie0) -> u8 {
        Trig7chain10ie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain10ie1 {
    #[doc = "No interrupt when finished"]
    IE1_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 0x03,
}
impl Trig7chain10ie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain10ie1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain10ie1 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain10ie1 {
        Trig7chain10ie1::from_bits(val)
    }
}
impl From<Trig7chain10ie1> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain10ie1) -> u8 {
        Trig7chain10ie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain32csel2 {
    #[doc = "ADC Channel 0 selected"]
    CSEL2_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL2_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL2_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL2_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL2_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL2_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL2_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL2_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL2_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL2_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL2_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL2_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL2_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL2_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL2_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL2_15 = 0x0f,
}
impl Trig7chain32csel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain32csel2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain32csel2 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain32csel2 {
        Trig7chain32csel2::from_bits(val)
    }
}
impl From<Trig7chain32csel2> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain32csel2) -> u8 {
        Trig7chain32csel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain32csel3 {
    #[doc = "ADC Channel 0 selected"]
    CSEL3_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL3_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL3_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL3_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL3_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL3_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL3_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL3_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL3_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL3_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL3_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL3_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL3_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL3_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL3_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL3_15 = 0x0f,
}
impl Trig7chain32csel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain32csel3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain32csel3 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain32csel3 {
        Trig7chain32csel3::from_bits(val)
    }
}
impl From<Trig7chain32csel3> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain32csel3) -> u8 {
        Trig7chain32csel3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain32hwts2(u8);
impl Trig7chain32hwts2 {
    #[doc = "no trigger selected"]
    pub const HWTS2_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS2_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS2_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS2_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS2_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS2_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS2_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS2_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS2_128: Self = Self(0x80);
}
impl Trig7chain32hwts2 {
    pub const fn from_bits(val: u8) -> Trig7chain32hwts2 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain32hwts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS2_0"),
            0x01 => f.write_str("HWTS2_1"),
            0x02 => f.write_str("HWTS2_2"),
            0x04 => f.write_str("HWTS2_4"),
            0x08 => f.write_str("HWTS2_8"),
            0x10 => f.write_str("HWTS2_16"),
            0x20 => f.write_str("HWTS2_32"),
            0x40 => f.write_str("HWTS2_64"),
            0x80 => f.write_str("HWTS2_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain32hwts2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS2_0"),
            0x01 => defmt::write!(f, "HWTS2_1"),
            0x02 => defmt::write!(f, "HWTS2_2"),
            0x04 => defmt::write!(f, "HWTS2_4"),
            0x08 => defmt::write!(f, "HWTS2_8"),
            0x10 => defmt::write!(f, "HWTS2_16"),
            0x20 => defmt::write!(f, "HWTS2_32"),
            0x40 => defmt::write!(f, "HWTS2_64"),
            0x80 => defmt::write!(f, "HWTS2_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain32hwts2 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain32hwts2 {
        Trig7chain32hwts2::from_bits(val)
    }
}
impl From<Trig7chain32hwts2> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain32hwts2) -> u8 {
        Trig7chain32hwts2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain32hwts3(u8);
impl Trig7chain32hwts3 {
    #[doc = "no trigger selected"]
    pub const HWTS3_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS3_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS3_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS3_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS3_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS3_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS3_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS3_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS3_128: Self = Self(0x80);
}
impl Trig7chain32hwts3 {
    pub const fn from_bits(val: u8) -> Trig7chain32hwts3 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain32hwts3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS3_0"),
            0x01 => f.write_str("HWTS3_1"),
            0x02 => f.write_str("HWTS3_2"),
            0x04 => f.write_str("HWTS3_4"),
            0x08 => f.write_str("HWTS3_8"),
            0x10 => f.write_str("HWTS3_16"),
            0x20 => f.write_str("HWTS3_32"),
            0x40 => f.write_str("HWTS3_64"),
            0x80 => f.write_str("HWTS3_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain32hwts3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS3_0"),
            0x01 => defmt::write!(f, "HWTS3_1"),
            0x02 => defmt::write!(f, "HWTS3_2"),
            0x04 => defmt::write!(f, "HWTS3_4"),
            0x08 => defmt::write!(f, "HWTS3_8"),
            0x10 => defmt::write!(f, "HWTS3_16"),
            0x20 => defmt::write!(f, "HWTS3_32"),
            0x40 => defmt::write!(f, "HWTS3_64"),
            0x80 => defmt::write!(f, "HWTS3_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain32hwts3 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain32hwts3 {
        Trig7chain32hwts3::from_bits(val)
    }
}
impl From<Trig7chain32hwts3> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain32hwts3) -> u8 {
        Trig7chain32hwts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain32ie2 {
    #[doc = "No interrupt when finished"]
    IE2_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 0x03,
}
impl Trig7chain32ie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain32ie2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain32ie2 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain32ie2 {
        Trig7chain32ie2::from_bits(val)
    }
}
impl From<Trig7chain32ie2> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain32ie2) -> u8 {
        Trig7chain32ie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain32ie3 {
    #[doc = "No interrupt when finished"]
    IE3_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 0x03,
}
impl Trig7chain32ie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain32ie3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain32ie3 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain32ie3 {
        Trig7chain32ie3::from_bits(val)
    }
}
impl From<Trig7chain32ie3> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain32ie3) -> u8 {
        Trig7chain32ie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain54csel4 {
    #[doc = "ADC Channel 0 selected"]
    CSEL4_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL4_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL4_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL4_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL4_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL4_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL4_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL4_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL4_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL4_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL4_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL4_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL4_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL4_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL4_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL4_15 = 0x0f,
}
impl Trig7chain54csel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain54csel4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain54csel4 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain54csel4 {
        Trig7chain54csel4::from_bits(val)
    }
}
impl From<Trig7chain54csel4> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain54csel4) -> u8 {
        Trig7chain54csel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain54csel5 {
    #[doc = "ADC Channel 0 selected"]
    CSEL5_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL5_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL5_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL5_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL5_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL5_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL5_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL5_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL5_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL5_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL5_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL5_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL5_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL5_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL5_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL5_15 = 0x0f,
}
impl Trig7chain54csel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain54csel5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain54csel5 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain54csel5 {
        Trig7chain54csel5::from_bits(val)
    }
}
impl From<Trig7chain54csel5> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain54csel5) -> u8 {
        Trig7chain54csel5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain54hwts4(u8);
impl Trig7chain54hwts4 {
    #[doc = "no trigger selected"]
    pub const HWTS4_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS4_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS4_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS4_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS4_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS4_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS4_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS4_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS4_128: Self = Self(0x80);
}
impl Trig7chain54hwts4 {
    pub const fn from_bits(val: u8) -> Trig7chain54hwts4 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain54hwts4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS4_0"),
            0x01 => f.write_str("HWTS4_1"),
            0x02 => f.write_str("HWTS4_2"),
            0x04 => f.write_str("HWTS4_4"),
            0x08 => f.write_str("HWTS4_8"),
            0x10 => f.write_str("HWTS4_16"),
            0x20 => f.write_str("HWTS4_32"),
            0x40 => f.write_str("HWTS4_64"),
            0x80 => f.write_str("HWTS4_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain54hwts4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS4_0"),
            0x01 => defmt::write!(f, "HWTS4_1"),
            0x02 => defmt::write!(f, "HWTS4_2"),
            0x04 => defmt::write!(f, "HWTS4_4"),
            0x08 => defmt::write!(f, "HWTS4_8"),
            0x10 => defmt::write!(f, "HWTS4_16"),
            0x20 => defmt::write!(f, "HWTS4_32"),
            0x40 => defmt::write!(f, "HWTS4_64"),
            0x80 => defmt::write!(f, "HWTS4_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain54hwts4 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain54hwts4 {
        Trig7chain54hwts4::from_bits(val)
    }
}
impl From<Trig7chain54hwts4> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain54hwts4) -> u8 {
        Trig7chain54hwts4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain54hwts5(u8);
impl Trig7chain54hwts5 {
    #[doc = "no trigger selected"]
    pub const HWTS5_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS5_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS5_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS5_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS5_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS5_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS5_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS5_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS5_128: Self = Self(0x80);
}
impl Trig7chain54hwts5 {
    pub const fn from_bits(val: u8) -> Trig7chain54hwts5 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain54hwts5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS5_0"),
            0x01 => f.write_str("HWTS5_1"),
            0x02 => f.write_str("HWTS5_2"),
            0x04 => f.write_str("HWTS5_4"),
            0x08 => f.write_str("HWTS5_8"),
            0x10 => f.write_str("HWTS5_16"),
            0x20 => f.write_str("HWTS5_32"),
            0x40 => f.write_str("HWTS5_64"),
            0x80 => f.write_str("HWTS5_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain54hwts5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS5_0"),
            0x01 => defmt::write!(f, "HWTS5_1"),
            0x02 => defmt::write!(f, "HWTS5_2"),
            0x04 => defmt::write!(f, "HWTS5_4"),
            0x08 => defmt::write!(f, "HWTS5_8"),
            0x10 => defmt::write!(f, "HWTS5_16"),
            0x20 => defmt::write!(f, "HWTS5_32"),
            0x40 => defmt::write!(f, "HWTS5_64"),
            0x80 => defmt::write!(f, "HWTS5_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain54hwts5 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain54hwts5 {
        Trig7chain54hwts5::from_bits(val)
    }
}
impl From<Trig7chain54hwts5> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain54hwts5) -> u8 {
        Trig7chain54hwts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain54ie4 {
    #[doc = "No interrupt when finished"]
    IE4_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 0x03,
}
impl Trig7chain54ie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain54ie4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain54ie4 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain54ie4 {
        Trig7chain54ie4::from_bits(val)
    }
}
impl From<Trig7chain54ie4> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain54ie4) -> u8 {
        Trig7chain54ie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain54ie5 {
    #[doc = "No interrupt when finished"]
    IE5_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 0x03,
}
impl Trig7chain54ie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain54ie5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain54ie5 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain54ie5 {
        Trig7chain54ie5::from_bits(val)
    }
}
impl From<Trig7chain54ie5> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain54ie5) -> u8 {
        Trig7chain54ie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain76csel6 {
    #[doc = "ADC Channel 0 selected"]
    CSEL6_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL6_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL6_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL6_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL6_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL6_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL6_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL6_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL6_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL6_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL6_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL6_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL6_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL6_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL6_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL6_15 = 0x0f,
}
impl Trig7chain76csel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain76csel6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain76csel6 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain76csel6 {
        Trig7chain76csel6::from_bits(val)
    }
}
impl From<Trig7chain76csel6> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain76csel6) -> u8 {
        Trig7chain76csel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain76csel7 {
    #[doc = "ADC Channel 0 selected."]
    CSEL7_0 = 0x0,
    #[doc = "ADC Channel 1 selected."]
    CSEL7_1 = 0x01,
    #[doc = "ADC Channel 2 selected."]
    CSEL7_2 = 0x02,
    #[doc = "ADC Channel 3 selected."]
    CSEL7_3 = 0x03,
    #[doc = "ADC Channel 4 selected."]
    CSEL7_4 = 0x04,
    #[doc = "ADC Channel 5 selected."]
    CSEL7_5 = 0x05,
    #[doc = "ADC Channel 6 selected."]
    CSEL7_6 = 0x06,
    #[doc = "ADC Channel 7 selected."]
    CSEL7_7 = 0x07,
    #[doc = "ADC Channel 8 selected."]
    CSEL7_8 = 0x08,
    #[doc = "ADC Channel 9 selected."]
    CSEL7_9 = 0x09,
    #[doc = "ADC Channel 10 selected."]
    CSEL7_10 = 0x0a,
    #[doc = "ADC Channel 11 selected."]
    CSEL7_11 = 0x0b,
    #[doc = "ADC Channel 12 selected."]
    CSEL7_12 = 0x0c,
    #[doc = "ADC Channel 13 selected."]
    CSEL7_13 = 0x0d,
    #[doc = "ADC Channel 14 selected."]
    CSEL7_14 = 0x0e,
    #[doc = "ADC Channel 15 selected."]
    CSEL7_15 = 0x0f,
}
impl Trig7chain76csel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain76csel7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain76csel7 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain76csel7 {
        Trig7chain76csel7::from_bits(val)
    }
}
impl From<Trig7chain76csel7> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain76csel7) -> u8 {
        Trig7chain76csel7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain76hwts6(u8);
impl Trig7chain76hwts6 {
    #[doc = "no trigger selected"]
    pub const HWTS6_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS6_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS6_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS6_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS6_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS6_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS6_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS6_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS6_128: Self = Self(0x80);
}
impl Trig7chain76hwts6 {
    pub const fn from_bits(val: u8) -> Trig7chain76hwts6 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain76hwts6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS6_0"),
            0x01 => f.write_str("HWTS6_1"),
            0x02 => f.write_str("HWTS6_2"),
            0x04 => f.write_str("HWTS6_4"),
            0x08 => f.write_str("HWTS6_8"),
            0x10 => f.write_str("HWTS6_16"),
            0x20 => f.write_str("HWTS6_32"),
            0x40 => f.write_str("HWTS6_64"),
            0x80 => f.write_str("HWTS6_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain76hwts6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS6_0"),
            0x01 => defmt::write!(f, "HWTS6_1"),
            0x02 => defmt::write!(f, "HWTS6_2"),
            0x04 => defmt::write!(f, "HWTS6_4"),
            0x08 => defmt::write!(f, "HWTS6_8"),
            0x10 => defmt::write!(f, "HWTS6_16"),
            0x20 => defmt::write!(f, "HWTS6_32"),
            0x40 => defmt::write!(f, "HWTS6_64"),
            0x80 => defmt::write!(f, "HWTS6_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain76hwts6 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain76hwts6 {
        Trig7chain76hwts6::from_bits(val)
    }
}
impl From<Trig7chain76hwts6> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain76hwts6) -> u8 {
        Trig7chain76hwts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trig7chain76hwts7(u8);
impl Trig7chain76hwts7 {
    #[doc = "no trigger selected"]
    pub const HWTS7_0: Self = Self(0x0);
    #[doc = "ADC TRIG0 selected"]
    pub const HWTS7_1: Self = Self(0x01);
    #[doc = "ADC TRIG1 selected"]
    pub const HWTS7_2: Self = Self(0x02);
    #[doc = "ADC TRIG2 selected"]
    pub const HWTS7_4: Self = Self(0x04);
    #[doc = "ADC TRIG3 selected"]
    pub const HWTS7_8: Self = Self(0x08);
    #[doc = "ADC TRIG4 selected"]
    pub const HWTS7_16: Self = Self(0x10);
    #[doc = "ADC TRIG5 selected"]
    pub const HWTS7_32: Self = Self(0x20);
    #[doc = "ADC TRIG6 selected"]
    pub const HWTS7_64: Self = Self(0x40);
    #[doc = "ADC TRIG7 selected"]
    pub const HWTS7_128: Self = Self(0x80);
}
impl Trig7chain76hwts7 {
    pub const fn from_bits(val: u8) -> Trig7chain76hwts7 {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Trig7chain76hwts7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("HWTS7_0"),
            0x01 => f.write_str("HWTS7_1"),
            0x02 => f.write_str("HWTS7_2"),
            0x04 => f.write_str("HWTS7_4"),
            0x08 => f.write_str("HWTS7_8"),
            0x10 => f.write_str("HWTS7_16"),
            0x20 => f.write_str("HWTS7_32"),
            0x40 => f.write_str("HWTS7_64"),
            0x80 => f.write_str("HWTS7_128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain76hwts7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HWTS7_0"),
            0x01 => defmt::write!(f, "HWTS7_1"),
            0x02 => defmt::write!(f, "HWTS7_2"),
            0x04 => defmt::write!(f, "HWTS7_4"),
            0x08 => defmt::write!(f, "HWTS7_8"),
            0x10 => defmt::write!(f, "HWTS7_16"),
            0x20 => defmt::write!(f, "HWTS7_32"),
            0x40 => defmt::write!(f, "HWTS7_64"),
            0x80 => defmt::write!(f, "HWTS7_128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Trig7chain76hwts7 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain76hwts7 {
        Trig7chain76hwts7::from_bits(val)
    }
}
impl From<Trig7chain76hwts7> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain76hwts7) -> u8 {
        Trig7chain76hwts7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain76ie6 {
    #[doc = "No interrupt when finished"]
    IE6_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 0x03,
}
impl Trig7chain76ie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain76ie6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain76ie6 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain76ie6 {
        Trig7chain76ie6::from_bits(val)
    }
}
impl From<Trig7chain76ie6> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain76ie6) -> u8 {
        Trig7chain76ie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7chain76ie7 {
    #[doc = "No interrupt when finished"]
    IE7_0 = 0x0,
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 0x01,
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 0x02,
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 0x03,
}
impl Trig7chain76ie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7chain76ie7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7chain76ie7 {
    #[inline(always)]
    fn from(val: u8) -> Trig7chain76ie7 {
        Trig7chain76ie7::from_bits(val)
    }
}
impl From<Trig7chain76ie7> for u8 {
    #[inline(always)]
    fn from(val: Trig7chain76ie7) -> u8 {
        Trig7chain76ie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig7ctrlTrigChain {
    #[doc = "Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0x0,
    #[doc = "Trigger chain length is 2"]
    TRIG_CHAIN_1 = 0x01,
    #[doc = "Trigger chain length is 3"]
    TRIG_CHAIN_2 = 0x02,
    #[doc = "Trigger chain length is 4"]
    TRIG_CHAIN_3 = 0x03,
    #[doc = "Trigger chain length is 5"]
    TRIG_CHAIN_4 = 0x04,
    #[doc = "Trigger chain length is 6"]
    TRIG_CHAIN_5 = 0x05,
    #[doc = "Trigger chain length is 7"]
    TRIG_CHAIN_6 = 0x06,
    #[doc = "Trigger chain length is 8"]
    TRIG_CHAIN_7 = 0x07,
}
impl Trig7ctrlTrigChain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig7ctrlTrigChain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig7ctrlTrigChain {
    #[inline(always)]
    fn from(val: u8) -> Trig7ctrlTrigChain {
        Trig7ctrlTrigChain::from_bits(val)
    }
}
impl From<Trig7ctrlTrigChain> for u8 {
    #[inline(always)]
    fn from(val: Trig7ctrlTrigChain) -> u8 {
        Trig7ctrlTrigChain::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TrigEnable(u8);
impl TrigEnable {
    #[doc = "disable all 8 external XBAR triggers."]
    pub const TRIG_ENABLE_0: Self = Self(0x0);
    #[doc = "enable external XBAR trigger0."]
    pub const TRIG_ENABLE_1: Self = Self(0x01);
    #[doc = "enable external XBAR trigger1."]
    pub const TRIG_ENABLE_2: Self = Self(0x02);
    #[doc = "enable external XBAR trigger0 and trigger1."]
    pub const TRIG_ENABLE_3: Self = Self(0x03);
    #[doc = "enable all 8 external XBAR triggers."]
    pub const TRIG_ENABLE_255: Self = Self(0xff);
}
impl TrigEnable {
    pub const fn from_bits(val: u8) -> TrigEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for TrigEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TRIG_ENABLE_0"),
            0x01 => f.write_str("TRIG_ENABLE_1"),
            0x02 => f.write_str("TRIG_ENABLE_2"),
            0x03 => f.write_str("TRIG_ENABLE_3"),
            0xff => f.write_str("TRIG_ENABLE_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TRIG_ENABLE_0"),
            0x01 => defmt::write!(f, "TRIG_ENABLE_1"),
            0x02 => defmt::write!(f, "TRIG_ENABLE_2"),
            0x03 => defmt::write!(f, "TRIG_ENABLE_3"),
            0xff => defmt::write!(f, "TRIG_ENABLE_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for TrigEnable {
    #[inline(always)]
    fn from(val: u8) -> TrigEnable {
        TrigEnable::from_bits(val)
    }
}
impl From<TrigEnable> for u8 {
    #[inline(always)]
    fn from(val: TrigEnable) -> u8 {
        TrigEnable::to_bits(val)
    }
}
