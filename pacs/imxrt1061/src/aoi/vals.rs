#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt0ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3 = 0x03,
}
impl Bfcrt010pt0ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt0ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt0ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt0ac {
        Bfcrt010pt0ac::from_bits(val)
    }
}
impl From<Bfcrt010pt0ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt0ac) -> u8 {
        Bfcrt010pt0ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt0bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3 = 0x03,
}
impl Bfcrt010pt0bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt0bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt0bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt0bc {
        Bfcrt010pt0bc::from_bits(val)
    }
}
impl From<Bfcrt010pt0bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt0bc) -> u8 {
        Bfcrt010pt0bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt0cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3 = 0x03,
}
impl Bfcrt010pt0cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt0cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt0cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt0cc {
        Bfcrt010pt0cc::from_bits(val)
    }
}
impl From<Bfcrt010pt0cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt0cc) -> u8 {
        Bfcrt010pt0cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt0dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3 = 0x03,
}
impl Bfcrt010pt0dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt0dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt0dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt0dc {
        Bfcrt010pt0dc::from_bits(val)
    }
}
impl From<Bfcrt010pt0dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt0dc) -> u8 {
        Bfcrt010pt0dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt1ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3 = 0x03,
}
impl Bfcrt010pt1ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt1ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt1ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt1ac {
        Bfcrt010pt1ac::from_bits(val)
    }
}
impl From<Bfcrt010pt1ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt1ac) -> u8 {
        Bfcrt010pt1ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt1bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3 = 0x03,
}
impl Bfcrt010pt1bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt1bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt1bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt1bc {
        Bfcrt010pt1bc::from_bits(val)
    }
}
impl From<Bfcrt010pt1bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt1bc) -> u8 {
        Bfcrt010pt1bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt1cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3 = 0x03,
}
impl Bfcrt010pt1cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt1cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt1cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt1cc {
        Bfcrt010pt1cc::from_bits(val)
    }
}
impl From<Bfcrt010pt1cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt1cc) -> u8 {
        Bfcrt010pt1cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010pt1dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3 = 0x03,
}
impl Bfcrt010pt1dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010pt1dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010pt1dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010pt1dc {
        Bfcrt010pt1dc::from_bits(val)
    }
}
impl From<Bfcrt010pt1dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010pt1dc) -> u8 {
        Bfcrt010pt1dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt0ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3 = 0x03,
}
impl Bfcrt011pt0ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt0ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt0ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt0ac {
        Bfcrt011pt0ac::from_bits(val)
    }
}
impl From<Bfcrt011pt0ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt0ac) -> u8 {
        Bfcrt011pt0ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt0bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3 = 0x03,
}
impl Bfcrt011pt0bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt0bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt0bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt0bc {
        Bfcrt011pt0bc::from_bits(val)
    }
}
impl From<Bfcrt011pt0bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt0bc) -> u8 {
        Bfcrt011pt0bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt0cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3 = 0x03,
}
impl Bfcrt011pt0cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt0cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt0cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt0cc {
        Bfcrt011pt0cc::from_bits(val)
    }
}
impl From<Bfcrt011pt0cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt0cc) -> u8 {
        Bfcrt011pt0cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt0dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3 = 0x03,
}
impl Bfcrt011pt0dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt0dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt0dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt0dc {
        Bfcrt011pt0dc::from_bits(val)
    }
}
impl From<Bfcrt011pt0dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt0dc) -> u8 {
        Bfcrt011pt0dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt1ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3 = 0x03,
}
impl Bfcrt011pt1ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt1ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt1ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt1ac {
        Bfcrt011pt1ac::from_bits(val)
    }
}
impl From<Bfcrt011pt1ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt1ac) -> u8 {
        Bfcrt011pt1ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt1bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3 = 0x03,
}
impl Bfcrt011pt1bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt1bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt1bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt1bc {
        Bfcrt011pt1bc::from_bits(val)
    }
}
impl From<Bfcrt011pt1bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt1bc) -> u8 {
        Bfcrt011pt1bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt1cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3 = 0x03,
}
impl Bfcrt011pt1cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt1cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt1cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt1cc {
        Bfcrt011pt1cc::from_bits(val)
    }
}
impl From<Bfcrt011pt1cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt1cc) -> u8 {
        Bfcrt011pt1cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011pt1dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3 = 0x03,
}
impl Bfcrt011pt1dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011pt1dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011pt1dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011pt1dc {
        Bfcrt011pt1dc::from_bits(val)
    }
}
impl From<Bfcrt011pt1dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011pt1dc) -> u8 {
        Bfcrt011pt1dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt0ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3 = 0x03,
}
impl Bfcrt012pt0ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt0ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt0ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt0ac {
        Bfcrt012pt0ac::from_bits(val)
    }
}
impl From<Bfcrt012pt0ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt0ac) -> u8 {
        Bfcrt012pt0ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt0bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3 = 0x03,
}
impl Bfcrt012pt0bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt0bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt0bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt0bc {
        Bfcrt012pt0bc::from_bits(val)
    }
}
impl From<Bfcrt012pt0bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt0bc) -> u8 {
        Bfcrt012pt0bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt0cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3 = 0x03,
}
impl Bfcrt012pt0cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt0cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt0cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt0cc {
        Bfcrt012pt0cc::from_bits(val)
    }
}
impl From<Bfcrt012pt0cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt0cc) -> u8 {
        Bfcrt012pt0cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt0dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3 = 0x03,
}
impl Bfcrt012pt0dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt0dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt0dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt0dc {
        Bfcrt012pt0dc::from_bits(val)
    }
}
impl From<Bfcrt012pt0dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt0dc) -> u8 {
        Bfcrt012pt0dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt1ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3 = 0x03,
}
impl Bfcrt012pt1ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt1ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt1ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt1ac {
        Bfcrt012pt1ac::from_bits(val)
    }
}
impl From<Bfcrt012pt1ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt1ac) -> u8 {
        Bfcrt012pt1ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt1bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3 = 0x03,
}
impl Bfcrt012pt1bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt1bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt1bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt1bc {
        Bfcrt012pt1bc::from_bits(val)
    }
}
impl From<Bfcrt012pt1bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt1bc) -> u8 {
        Bfcrt012pt1bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt1cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3 = 0x03,
}
impl Bfcrt012pt1cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt1cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt1cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt1cc {
        Bfcrt012pt1cc::from_bits(val)
    }
}
impl From<Bfcrt012pt1cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt1cc) -> u8 {
        Bfcrt012pt1cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012pt1dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3 = 0x03,
}
impl Bfcrt012pt1dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012pt1dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012pt1dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012pt1dc {
        Bfcrt012pt1dc::from_bits(val)
    }
}
impl From<Bfcrt012pt1dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012pt1dc) -> u8 {
        Bfcrt012pt1dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt0ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3 = 0x03,
}
impl Bfcrt013pt0ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt0ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt0ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt0ac {
        Bfcrt013pt0ac::from_bits(val)
    }
}
impl From<Bfcrt013pt0ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt0ac) -> u8 {
        Bfcrt013pt0ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt0bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3 = 0x03,
}
impl Bfcrt013pt0bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt0bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt0bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt0bc {
        Bfcrt013pt0bc::from_bits(val)
    }
}
impl From<Bfcrt013pt0bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt0bc) -> u8 {
        Bfcrt013pt0bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt0cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3 = 0x03,
}
impl Bfcrt013pt0cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt0cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt0cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt0cc {
        Bfcrt013pt0cc::from_bits(val)
    }
}
impl From<Bfcrt013pt0cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt0cc) -> u8 {
        Bfcrt013pt0cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt0dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3 = 0x03,
}
impl Bfcrt013pt0dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt0dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt0dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt0dc {
        Bfcrt013pt0dc::from_bits(val)
    }
}
impl From<Bfcrt013pt0dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt0dc) -> u8 {
        Bfcrt013pt0dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt1ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3 = 0x03,
}
impl Bfcrt013pt1ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt1ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt1ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt1ac {
        Bfcrt013pt1ac::from_bits(val)
    }
}
impl From<Bfcrt013pt1ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt1ac) -> u8 {
        Bfcrt013pt1ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt1bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3 = 0x03,
}
impl Bfcrt013pt1bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt1bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt1bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt1bc {
        Bfcrt013pt1bc::from_bits(val)
    }
}
impl From<Bfcrt013pt1bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt1bc) -> u8 {
        Bfcrt013pt1bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt1cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3 = 0x03,
}
impl Bfcrt013pt1cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt1cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt1cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt1cc {
        Bfcrt013pt1cc::from_bits(val)
    }
}
impl From<Bfcrt013pt1cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt1cc) -> u8 {
        Bfcrt013pt1cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013pt1dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3 = 0x03,
}
impl Bfcrt013pt1dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013pt1dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013pt1dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013pt1dc {
        Bfcrt013pt1dc::from_bits(val)
    }
}
impl From<Bfcrt013pt1dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013pt1dc) -> u8 {
        Bfcrt013pt1dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt2ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3 = 0x03,
}
impl Bfcrt230pt2ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt2ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt2ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt2ac {
        Bfcrt230pt2ac::from_bits(val)
    }
}
impl From<Bfcrt230pt2ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt2ac) -> u8 {
        Bfcrt230pt2ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt2bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3 = 0x03,
}
impl Bfcrt230pt2bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt2bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt2bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt2bc {
        Bfcrt230pt2bc::from_bits(val)
    }
}
impl From<Bfcrt230pt2bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt2bc) -> u8 {
        Bfcrt230pt2bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt2cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3 = 0x03,
}
impl Bfcrt230pt2cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt2cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt2cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt2cc {
        Bfcrt230pt2cc::from_bits(val)
    }
}
impl From<Bfcrt230pt2cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt2cc) -> u8 {
        Bfcrt230pt2cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt2dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3 = 0x03,
}
impl Bfcrt230pt2dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt2dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt2dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt2dc {
        Bfcrt230pt2dc::from_bits(val)
    }
}
impl From<Bfcrt230pt2dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt2dc) -> u8 {
        Bfcrt230pt2dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt3ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3 = 0x03,
}
impl Bfcrt230pt3ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt3ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt3ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt3ac {
        Bfcrt230pt3ac::from_bits(val)
    }
}
impl From<Bfcrt230pt3ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt3ac) -> u8 {
        Bfcrt230pt3ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt3bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3 = 0x03,
}
impl Bfcrt230pt3bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt3bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt3bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt3bc {
        Bfcrt230pt3bc::from_bits(val)
    }
}
impl From<Bfcrt230pt3bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt3bc) -> u8 {
        Bfcrt230pt3bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt3cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3 = 0x03,
}
impl Bfcrt230pt3cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt3cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt3cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt3cc {
        Bfcrt230pt3cc::from_bits(val)
    }
}
impl From<Bfcrt230pt3cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt3cc) -> u8 {
        Bfcrt230pt3cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230pt3dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3 = 0x03,
}
impl Bfcrt230pt3dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230pt3dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230pt3dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230pt3dc {
        Bfcrt230pt3dc::from_bits(val)
    }
}
impl From<Bfcrt230pt3dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230pt3dc) -> u8 {
        Bfcrt230pt3dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt2ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3 = 0x03,
}
impl Bfcrt231pt2ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt2ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt2ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt2ac {
        Bfcrt231pt2ac::from_bits(val)
    }
}
impl From<Bfcrt231pt2ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt2ac) -> u8 {
        Bfcrt231pt2ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt2bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3 = 0x03,
}
impl Bfcrt231pt2bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt2bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt2bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt2bc {
        Bfcrt231pt2bc::from_bits(val)
    }
}
impl From<Bfcrt231pt2bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt2bc) -> u8 {
        Bfcrt231pt2bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt2cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3 = 0x03,
}
impl Bfcrt231pt2cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt2cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt2cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt2cc {
        Bfcrt231pt2cc::from_bits(val)
    }
}
impl From<Bfcrt231pt2cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt2cc) -> u8 {
        Bfcrt231pt2cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt2dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3 = 0x03,
}
impl Bfcrt231pt2dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt2dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt2dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt2dc {
        Bfcrt231pt2dc::from_bits(val)
    }
}
impl From<Bfcrt231pt2dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt2dc) -> u8 {
        Bfcrt231pt2dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt3ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3 = 0x03,
}
impl Bfcrt231pt3ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt3ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt3ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt3ac {
        Bfcrt231pt3ac::from_bits(val)
    }
}
impl From<Bfcrt231pt3ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt3ac) -> u8 {
        Bfcrt231pt3ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt3bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3 = 0x03,
}
impl Bfcrt231pt3bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt3bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt3bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt3bc {
        Bfcrt231pt3bc::from_bits(val)
    }
}
impl From<Bfcrt231pt3bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt3bc) -> u8 {
        Bfcrt231pt3bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt3cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3 = 0x03,
}
impl Bfcrt231pt3cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt3cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt3cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt3cc {
        Bfcrt231pt3cc::from_bits(val)
    }
}
impl From<Bfcrt231pt3cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt3cc) -> u8 {
        Bfcrt231pt3cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231pt3dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3 = 0x03,
}
impl Bfcrt231pt3dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231pt3dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231pt3dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231pt3dc {
        Bfcrt231pt3dc::from_bits(val)
    }
}
impl From<Bfcrt231pt3dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231pt3dc) -> u8 {
        Bfcrt231pt3dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt2ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3 = 0x03,
}
impl Bfcrt232pt2ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt2ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt2ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt2ac {
        Bfcrt232pt2ac::from_bits(val)
    }
}
impl From<Bfcrt232pt2ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt2ac) -> u8 {
        Bfcrt232pt2ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt2bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3 = 0x03,
}
impl Bfcrt232pt2bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt2bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt2bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt2bc {
        Bfcrt232pt2bc::from_bits(val)
    }
}
impl From<Bfcrt232pt2bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt2bc) -> u8 {
        Bfcrt232pt2bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt2cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3 = 0x03,
}
impl Bfcrt232pt2cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt2cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt2cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt2cc {
        Bfcrt232pt2cc::from_bits(val)
    }
}
impl From<Bfcrt232pt2cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt2cc) -> u8 {
        Bfcrt232pt2cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt2dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3 = 0x03,
}
impl Bfcrt232pt2dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt2dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt2dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt2dc {
        Bfcrt232pt2dc::from_bits(val)
    }
}
impl From<Bfcrt232pt2dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt2dc) -> u8 {
        Bfcrt232pt2dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt3ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3 = 0x03,
}
impl Bfcrt232pt3ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt3ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt3ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt3ac {
        Bfcrt232pt3ac::from_bits(val)
    }
}
impl From<Bfcrt232pt3ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt3ac) -> u8 {
        Bfcrt232pt3ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt3bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3 = 0x03,
}
impl Bfcrt232pt3bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt3bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt3bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt3bc {
        Bfcrt232pt3bc::from_bits(val)
    }
}
impl From<Bfcrt232pt3bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt3bc) -> u8 {
        Bfcrt232pt3bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt3cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3 = 0x03,
}
impl Bfcrt232pt3cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt3cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt3cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt3cc {
        Bfcrt232pt3cc::from_bits(val)
    }
}
impl From<Bfcrt232pt3cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt3cc) -> u8 {
        Bfcrt232pt3cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232pt3dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3 = 0x03,
}
impl Bfcrt232pt3dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232pt3dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232pt3dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232pt3dc {
        Bfcrt232pt3dc::from_bits(val)
    }
}
impl From<Bfcrt232pt3dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232pt3dc) -> u8 {
        Bfcrt232pt3dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt2ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3 = 0x03,
}
impl Bfcrt233pt2ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt2ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt2ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt2ac {
        Bfcrt233pt2ac::from_bits(val)
    }
}
impl From<Bfcrt233pt2ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt2ac) -> u8 {
        Bfcrt233pt2ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt2bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3 = 0x03,
}
impl Bfcrt233pt2bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt2bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt2bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt2bc {
        Bfcrt233pt2bc::from_bits(val)
    }
}
impl From<Bfcrt233pt2bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt2bc) -> u8 {
        Bfcrt233pt2bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt2cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3 = 0x03,
}
impl Bfcrt233pt2cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt2cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt2cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt2cc {
        Bfcrt233pt2cc::from_bits(val)
    }
}
impl From<Bfcrt233pt2cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt2cc) -> u8 {
        Bfcrt233pt2cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt2dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3 = 0x03,
}
impl Bfcrt233pt2dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt2dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt2dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt2dc {
        Bfcrt233pt2dc::from_bits(val)
    }
}
impl From<Bfcrt233pt2dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt2dc) -> u8 {
        Bfcrt233pt2dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt3ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3 = 0x03,
}
impl Bfcrt233pt3ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt3ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt3ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt3ac {
        Bfcrt233pt3ac::from_bits(val)
    }
}
impl From<Bfcrt233pt3ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt3ac) -> u8 {
        Bfcrt233pt3ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt3bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3 = 0x03,
}
impl Bfcrt233pt3bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt3bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt3bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt3bc {
        Bfcrt233pt3bc::from_bits(val)
    }
}
impl From<Bfcrt233pt3bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt3bc) -> u8 {
        Bfcrt233pt3bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt3cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3 = 0x03,
}
impl Bfcrt233pt3cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt3cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt3cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt3cc {
        Bfcrt233pt3cc::from_bits(val)
    }
}
impl From<Bfcrt233pt3cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt3cc) -> u8 {
        Bfcrt233pt3cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233pt3dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3 = 0x03,
}
impl Bfcrt233pt3dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233pt3dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233pt3dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233pt3dc {
        Bfcrt233pt3dc::from_bits(val)
    }
}
impl From<Bfcrt233pt3dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233pt3dc) -> u8 {
        Bfcrt233pt3dc::to_bits(val)
    }
}
