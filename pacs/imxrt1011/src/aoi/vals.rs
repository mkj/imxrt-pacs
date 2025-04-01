#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt0ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3 = 0x03,
}
impl Pt0ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0ac {
    #[inline(always)]
    fn from(val: u8) -> Pt0ac {
        Pt0ac::from_bits(val)
    }
}
impl From<Pt0ac> for u8 {
    #[inline(always)]
    fn from(val: Pt0ac) -> u8 {
        Pt0ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt0bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3 = 0x03,
}
impl Pt0bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0bc {
    #[inline(always)]
    fn from(val: u8) -> Pt0bc {
        Pt0bc::from_bits(val)
    }
}
impl From<Pt0bc> for u8 {
    #[inline(always)]
    fn from(val: Pt0bc) -> u8 {
        Pt0bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt0cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3 = 0x03,
}
impl Pt0cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0cc {
    #[inline(always)]
    fn from(val: u8) -> Pt0cc {
        Pt0cc::from_bits(val)
    }
}
impl From<Pt0cc> for u8 {
    #[inline(always)]
    fn from(val: Pt0cc) -> u8 {
        Pt0cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt0dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3 = 0x03,
}
impl Pt0dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0dc {
    #[inline(always)]
    fn from(val: u8) -> Pt0dc {
        Pt0dc::from_bits(val)
    }
}
impl From<Pt0dc> for u8 {
    #[inline(always)]
    fn from(val: Pt0dc) -> u8 {
        Pt0dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt1ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3 = 0x03,
}
impl Pt1ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1ac {
    #[inline(always)]
    fn from(val: u8) -> Pt1ac {
        Pt1ac::from_bits(val)
    }
}
impl From<Pt1ac> for u8 {
    #[inline(always)]
    fn from(val: Pt1ac) -> u8 {
        Pt1ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt1bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3 = 0x03,
}
impl Pt1bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1bc {
    #[inline(always)]
    fn from(val: u8) -> Pt1bc {
        Pt1bc::from_bits(val)
    }
}
impl From<Pt1bc> for u8 {
    #[inline(always)]
    fn from(val: Pt1bc) -> u8 {
        Pt1bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt1cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3 = 0x03,
}
impl Pt1cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1cc {
    #[inline(always)]
    fn from(val: u8) -> Pt1cc {
        Pt1cc::from_bits(val)
    }
}
impl From<Pt1cc> for u8 {
    #[inline(always)]
    fn from(val: Pt1cc) -> u8 {
        Pt1cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt1dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3 = 0x03,
}
impl Pt1dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1dc {
    #[inline(always)]
    fn from(val: u8) -> Pt1dc {
        Pt1dc::from_bits(val)
    }
}
impl From<Pt1dc> for u8 {
    #[inline(always)]
    fn from(val: Pt1dc) -> u8 {
        Pt1dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt2ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3 = 0x03,
}
impl Pt2ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2ac {
    #[inline(always)]
    fn from(val: u8) -> Pt2ac {
        Pt2ac::from_bits(val)
    }
}
impl From<Pt2ac> for u8 {
    #[inline(always)]
    fn from(val: Pt2ac) -> u8 {
        Pt2ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt2bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3 = 0x03,
}
impl Pt2bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2bc {
    #[inline(always)]
    fn from(val: u8) -> Pt2bc {
        Pt2bc::from_bits(val)
    }
}
impl From<Pt2bc> for u8 {
    #[inline(always)]
    fn from(val: Pt2bc) -> u8 {
        Pt2bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt2cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3 = 0x03,
}
impl Pt2cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2cc {
    #[inline(always)]
    fn from(val: u8) -> Pt2cc {
        Pt2cc::from_bits(val)
    }
}
impl From<Pt2cc> for u8 {
    #[inline(always)]
    fn from(val: Pt2cc) -> u8 {
        Pt2cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt2dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3 = 0x03,
}
impl Pt2dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2dc {
    #[inline(always)]
    fn from(val: u8) -> Pt2dc {
        Pt2dc::from_bits(val)
    }
}
impl From<Pt2dc> for u8 {
    #[inline(always)]
    fn from(val: Pt2dc) -> u8 {
        Pt2dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt3ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3 = 0x03,
}
impl Pt3ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3ac {
    #[inline(always)]
    fn from(val: u8) -> Pt3ac {
        Pt3ac::from_bits(val)
    }
}
impl From<Pt3ac> for u8 {
    #[inline(always)]
    fn from(val: Pt3ac) -> u8 {
        Pt3ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt3bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3 = 0x03,
}
impl Pt3bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3bc {
    #[inline(always)]
    fn from(val: u8) -> Pt3bc {
        Pt3bc::from_bits(val)
    }
}
impl From<Pt3bc> for u8 {
    #[inline(always)]
    fn from(val: Pt3bc) -> u8 {
        Pt3bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt3cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3 = 0x03,
}
impl Pt3cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3cc {
    #[inline(always)]
    fn from(val: u8) -> Pt3cc {
        Pt3cc::from_bits(val)
    }
}
impl From<Pt3cc> for u8 {
    #[inline(always)]
    fn from(val: Pt3cc) -> u8 {
        Pt3cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pt3dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3 = 0x03,
}
impl Pt3dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3dc {
    #[inline(always)]
    fn from(val: u8) -> Pt3dc {
        Pt3dc::from_bits(val)
    }
}
impl From<Pt3dc> for u8 {
    #[inline(always)]
    fn from(val: Pt3dc) -> u8 {
        Pt3dc::to_bits(val)
    }
}
