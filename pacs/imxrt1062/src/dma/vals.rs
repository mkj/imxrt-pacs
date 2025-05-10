#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdAttrSsize {
    #[doc = "8-bit"]
    EIGHT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TcdAttrSsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdAttrSsize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdAttrSsize {
    #[inline(always)]
    fn from(val: u8) -> TcdAttrSsize {
        TcdAttrSsize::from_bits(val)
    }
}
impl From<TcdAttrSsize> for u8 {
    #[inline(always)]
    fn from(val: TcdAttrSsize) -> u8 {
        TcdAttrSsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdCsrBwc {
    #[doc = "No eDMA engine stalls"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    STALL4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    STALL8 = 0x03,
}
impl TcdCsrBwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdCsrBwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdCsrBwc {
    #[inline(always)]
    fn from(val: u8) -> TcdCsrBwc {
        TcdCsrBwc::from_bits(val)
    }
}
impl From<TcdCsrBwc> for u8 {
    #[inline(always)]
    fn from(val: TcdCsrBwc) -> u8 {
        TcdCsrBwc::to_bits(val)
    }
}
