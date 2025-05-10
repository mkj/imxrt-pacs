#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fltconf {
    #[doc = "Error Active"]
    ERROR_ACTIVE = 0x0,
    #[doc = "Error Passive"]
    ERROR_PASSIVE = 0x01,
    #[doc = "Bus Off"]
    BUS_OFF = 0x02,
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
    #[doc = "Format A: One full ID (standard and extended) per ID filter table element."]
    ONE_FULL_ID = 0x0,
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID filter table element."]
    TWO_FULL_ID = 0x01,
    #[doc = "Format C: Four partial 8-bit standard IDs per ID filter table element."]
    FOUR_PARTIAL_ID = 0x02,
    #[doc = "Format D: All frames rejected."]
    ALL_FRAMES_REJECTED = 0x03,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbdsr0 {
    #[doc = "Selects 8 bytes per message buffer."]
    R0_8_BYTES = 0x0,
    #[doc = "Selects 16 bytes per message buffer."]
    R0_16_BYTES = 0x01,
    #[doc = "Selects 32 bytes per message buffer."]
    R0_32_BYTES = 0x02,
    #[doc = "Selects 64 bytes per message buffer."]
    R0_64_BYTES = 0x03,
}
impl Mbdsr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbdsr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbdsr0 {
    #[inline(always)]
    fn from(val: u8) -> Mbdsr0 {
        Mbdsr0::from_bits(val)
    }
}
impl From<Mbdsr0> for u8 {
    #[inline(always)]
    fn from(val: Mbdsr0) -> u8 {
        Mbdsr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbdsr1 {
    #[doc = "Selects 8 bytes per message buffer."]
    R1_8_BYTES = 0x0,
    #[doc = "Selects 16 bytes per message buffer."]
    R1_16_BYTES = 0x01,
    #[doc = "Selects 32 bytes per message buffer."]
    R1_32_BYTES = 0x02,
    #[doc = "Selects 64 bytes per message buffer."]
    R1_64_BYTES = 0x03,
}
impl Mbdsr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbdsr1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbdsr1 {
    #[inline(always)]
    fn from(val: u8) -> Mbdsr1 {
        Mbdsr1::from_bits(val)
    }
}
impl From<Mbdsr1> for u8 {
    #[inline(always)]
    fn from(val: Mbdsr1) -> u8 {
        Mbdsr1::to_bits(val)
    }
}
