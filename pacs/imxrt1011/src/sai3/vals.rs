#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(pub u16);
impl Feature {
    #[doc = "Standard feature set."]
    pub const STD: Self = Self(0);
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
pub enum Rcr2msel {
    #[doc = "Bus Clock selected."]
    BUS_CLOCK = 0,
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
pub enum Rcr4fpack {
    #[doc = "FIFO packing is disabled"]
    DISABLED = 0,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rwm(pub u32);
impl Rwm {
    #[doc = "Word N is enabled."]
    pub const WORD_N_ENABLED: Self = Self(0);
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
pub enum Tcr2msel {
    #[doc = "Bus Clock selected."]
    BUS_CLOCK = 0,
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
pub enum Tcr4fpack {
    #[doc = "FIFO packing is disabled."]
    DISABLED = 0,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Twm(pub u32);
impl Twm {
    #[doc = "Word N is enabled."]
    pub const WORD_N_ENABLED: Self = Self(0);
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
