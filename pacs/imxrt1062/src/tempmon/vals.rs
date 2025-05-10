#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1clrMeasureFreq(u16);
impl Tempsense1clrMeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1clrMeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1clrMeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1clrMeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1clrMeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1clrMeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1clrMeasureFreq {
        Tempsense1clrMeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1clrMeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1clrMeasureFreq) -> u16 {
        Tempsense1clrMeasureFreq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1measureFreq(u16);
impl Tempsense1measureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1measureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1measureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1measureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1measureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1measureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1measureFreq {
        Tempsense1measureFreq::from_bits(val)
    }
}
impl From<Tempsense1measureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1measureFreq) -> u16 {
        Tempsense1measureFreq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1setMeasureFreq(u16);
impl Tempsense1setMeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1setMeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1setMeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1setMeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1setMeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1setMeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1setMeasureFreq {
        Tempsense1setMeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1setMeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1setMeasureFreq) -> u16 {
        Tempsense1setMeasureFreq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1togMeasureFreq(u16);
impl Tempsense1togMeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1togMeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1togMeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1togMeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1togMeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1togMeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1togMeasureFreq {
        Tempsense1togMeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1togMeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1togMeasureFreq) -> u16 {
        Tempsense1togMeasureFreq::to_bits(val)
    }
}
