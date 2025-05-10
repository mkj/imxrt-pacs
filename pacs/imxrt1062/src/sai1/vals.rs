#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set."]
    pub const STD: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("STD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STD"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2msel {
    #[doc = "Bus Clock selected."]
    BUS_CLOCK = 0x0,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    MCLK1 = 0x01,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    MCLK2 = 0x02,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    MCLK3 = 0x03,
}
impl Rcr2msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2msel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2msel {
    #[inline(always)]
    fn from(val: u8) -> Rcr2msel {
        Rcr2msel::from_bits(val)
    }
}
impl From<Rcr2msel> for u8 {
    #[inline(always)]
    fn from(val: Rcr2msel) -> u8 {
        Rcr2msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4fcomb {
    #[doc = "FIFO combine mode disabled."]
    DISABLED = 0x0,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    ENA_ON_FIFO_WRITES = 0x01,
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    ENA_ON_FIFO_READS = 0x02,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    ENA_ON_FIFO_WRITES_READS = 0x03,
}
impl Rcr4fcomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4fcomb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4fcomb {
    #[inline(always)]
    fn from(val: u8) -> Rcr4fcomb {
        Rcr4fcomb::from_bits(val)
    }
}
impl From<Rcr4fcomb> for u8 {
    #[inline(always)]
    fn from(val: Rcr4fcomb) -> u8 {
        Rcr4fcomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4fpack {
    #[doc = "FIFO packing is disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit FIFO packing is enabled"]
    EIGHT_BIT_PACKING = 0x02,
    #[doc = "16-bit FIFO packing is enabled"]
    SIXTEEN_BIT_PACKING = 0x03,
}
impl Rcr4fpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4fpack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4fpack {
    #[inline(always)]
    fn from(val: u8) -> Rcr4fpack {
        Rcr4fpack::from_bits(val)
    }
}
impl From<Rcr4fpack> for u8 {
    #[inline(always)]
    fn from(val: Rcr4fpack) -> u8 {
        Rcr4fpack::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rwm(u32);
impl Rwm {
    #[doc = "Word N is enabled."]
    pub const WORD_N_ENABLED: Self = Self(0x0);
    #[doc = "Word N is masked."]
    pub const WORD_N_MASKED: Self = Self(0x01);
}
impl Rwm {
    pub const fn from_bits(val: u32) -> Rwm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rwm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WORD_N_ENABLED"),
            0x01 => f.write_str("WORD_N_MASKED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WORD_N_ENABLED"),
            0x01 => defmt::write!(f, "WORD_N_MASKED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rwm {
    #[inline(always)]
    fn from(val: u32) -> Rwm {
        Rwm::from_bits(val)
    }
}
impl From<Rwm> for u32 {
    #[inline(always)]
    fn from(val: Rwm) -> u32 {
        Rwm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2msel {
    #[doc = "Bus Clock selected."]
    BUS_CLOCK = 0x0,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    MCLK1 = 0x01,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    MCLK2 = 0x02,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    MCLK3 = 0x03,
}
impl Tcr2msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2msel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2msel {
    #[inline(always)]
    fn from(val: u8) -> Tcr2msel {
        Tcr2msel::from_bits(val)
    }
}
impl From<Tcr2msel> for u8 {
    #[inline(always)]
    fn from(val: Tcr2msel) -> u8 {
        Tcr2msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4fcomb {
    #[doc = "FIFO combine mode disabled."]
    DISABLED = 0x0,
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    ENABLED_ON_FIFO_READS = 0x01,
    #[doc = "FIFO combine mode enabled on FIFO writes (by software)."]
    ENABLED_ON_FIFO_WRITES = 0x02,
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    ENABLED_ON_FIFO_READS_WRITES = 0x03,
}
impl Tcr4fcomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4fcomb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4fcomb {
    #[inline(always)]
    fn from(val: u8) -> Tcr4fcomb {
        Tcr4fcomb::from_bits(val)
    }
}
impl From<Tcr4fcomb> for u8 {
    #[inline(always)]
    fn from(val: Tcr4fcomb) -> u8 {
        Tcr4fcomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4fpack {
    #[doc = "FIFO packing is disabled."]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit FIFO packing is enabled."]
    EIGHT_BIT_FIFO_PACKING = 0x02,
    #[doc = "16-bit FIFO packing is enabled."]
    SIXTEEN_BIT_FIFO_PACKING = 0x03,
}
impl Tcr4fpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4fpack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4fpack {
    #[inline(always)]
    fn from(val: u8) -> Tcr4fpack {
        Tcr4fpack::from_bits(val)
    }
}
impl From<Tcr4fpack> for u8 {
    #[inline(always)]
    fn from(val: Tcr4fpack) -> u8 {
        Tcr4fpack::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Twm(u32);
impl Twm {
    #[doc = "Word N is enabled."]
    pub const WORD_N_ENABLED: Self = Self(0x0);
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    pub const WORD_N_MASKED: Self = Self(0x01);
}
impl Twm {
    pub const fn from_bits(val: u32) -> Twm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Twm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WORD_N_ENABLED"),
            0x01 => f.write_str("WORD_N_MASKED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Twm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WORD_N_ENABLED"),
            0x01 => defmt::write!(f, "WORD_N_MASKED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Twm {
    #[inline(always)]
    fn from(val: u32) -> Twm {
        Twm::from_bits(val)
    }
}
impl From<Twm> for u32 {
    #[inline(always)]
    fn from(val: Twm) -> u32 {
        Twm::to_bits(val)
    }
}
