#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SiliconRevision(u32);
impl SiliconRevision {
    #[doc = "Silicon revision 1.0"]
    pub const SILICON_REVISION_7077888: Self = Self(0x006c_0000);
}
impl SiliconRevision {
    pub const fn from_bits(val: u32) -> SiliconRevision {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SiliconRevision {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x006c_0000 => f.write_str("SILICON_REVISION_7077888"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SiliconRevision {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x006c_0000 => defmt::write!(f, "SILICON_REVISION_7077888"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SiliconRevision {
    #[inline(always)]
    fn from(val: u32) -> SiliconRevision {
        SiliconRevision::from_bits(val)
    }
}
impl From<SiliconRevision> for u32 {
    #[inline(always)]
    fn from(val: SiliconRevision) -> u32 {
        SiliconRevision::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1vbusDetectClrVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1vbusDetectClrVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1vbusDetectClrVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1vbusDetectClrVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1vbusDetectClrVbusvalidThresh {
        Usb1vbusDetectClrVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1vbusDetectClrVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1vbusDetectClrVbusvalidThresh) -> u8 {
        Usb1vbusDetectClrVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1vbusDetectSetVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1vbusDetectSetVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1vbusDetectSetVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1vbusDetectSetVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1vbusDetectSetVbusvalidThresh {
        Usb1vbusDetectSetVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1vbusDetectSetVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1vbusDetectSetVbusvalidThresh) -> u8 {
        Usb1vbusDetectSetVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1vbusDetectTogVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1vbusDetectTogVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1vbusDetectTogVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1vbusDetectTogVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1vbusDetectTogVbusvalidThresh {
        Usb1vbusDetectTogVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1vbusDetectTogVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1vbusDetectTogVbusvalidThresh) -> u8 {
        Usb1vbusDetectTogVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1vbusDetectVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1vbusDetectVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1vbusDetectVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1vbusDetectVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1vbusDetectVbusvalidThresh {
        Usb1vbusDetectVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1vbusDetectVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1vbusDetectVbusvalidThresh) -> u8 {
        Usb1vbusDetectVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2vbusDetectClrVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2vbusDetectClrVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2vbusDetectClrVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2vbusDetectClrVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2vbusDetectClrVbusvalidThresh {
        Usb2vbusDetectClrVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2vbusDetectClrVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2vbusDetectClrVbusvalidThresh) -> u8 {
        Usb2vbusDetectClrVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2vbusDetectSetVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2vbusDetectSetVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2vbusDetectSetVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2vbusDetectSetVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2vbusDetectSetVbusvalidThresh {
        Usb2vbusDetectSetVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2vbusDetectSetVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2vbusDetectSetVbusvalidThresh) -> u8 {
        Usb2vbusDetectSetVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2vbusDetectTogVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2vbusDetectTogVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2vbusDetectTogVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2vbusDetectTogVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2vbusDetectTogVbusvalidThresh {
        Usb2vbusDetectTogVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2vbusDetectTogVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2vbusDetectTogVbusvalidThresh) -> u8 {
        Usb2vbusDetectTogVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2vbusDetectVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2vbusDetectVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2vbusDetectVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2vbusDetectVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2vbusDetectVbusvalidThresh {
        Usb2vbusDetectVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2vbusDetectVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2vbusDetectVbusvalidThresh) -> u8 {
        Usb2vbusDetectVbusvalidThresh::to_bits(val)
    }
}
