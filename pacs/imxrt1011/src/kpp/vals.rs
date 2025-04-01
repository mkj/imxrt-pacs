#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kcdd(pub u8);
impl Kcdd {
    #[doc = "COLn pin is configured as an input."]
    pub const INPUT: Self = Self(0);
    #[doc = "COLn pin is configured as an output."]
    pub const OUTPUT: Self = Self(0x01);
}
impl Kcdd {
    pub const fn from_bits(val: u8) -> Kcdd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Kcdd {
    #[inline(always)]
    fn from(val: u8) -> Kcdd {
        Kcdd::from_bits(val)
    }
}
impl From<Kcdd> for u8 {
    #[inline(always)]
    fn from(val: Kcdd) -> u8 {
        Kcdd::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kco(pub u8);
impl Kco {
    #[doc = "Column strobe output is totem pole drive."]
    pub const TOTEM_POLE: Self = Self(0);
    #[doc = "Column strobe output is open drain."]
    pub const OPEN_DRAIN: Self = Self(0x01);
}
impl Kco {
    pub const fn from_bits(val: u8) -> Kco {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Kco {
    #[inline(always)]
    fn from(val: u8) -> Kco {
        Kco::from_bits(val)
    }
}
impl From<Kco> for u8 {
    #[inline(always)]
    fn from(val: Kco) -> u8 {
        Kco::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Krdd(pub u8);
impl Krdd {
    #[doc = "ROWn pin configured as an input."]
    pub const INPUT: Self = Self(0);
    #[doc = "ROWn pin configured as an output."]
    pub const OUTPUT: Self = Self(0x01);
}
impl Krdd {
    pub const fn from_bits(val: u8) -> Krdd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Krdd {
    #[inline(always)]
    fn from(val: u8) -> Krdd {
        Krdd::from_bits(val)
    }
}
impl From<Krdd> for u8 {
    #[inline(always)]
    fn from(val: Krdd) -> u8 {
        Krdd::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kre(pub u8);
impl Kre {
    #[doc = "Row is not included in the keypad key press detect."]
    pub const KRE_0: Self = Self(0);
    #[doc = "Row is included in the keypad key press detect."]
    pub const KRE_1: Self = Self(0x01);
}
impl Kre {
    pub const fn from_bits(val: u8) -> Kre {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Kre {
    #[inline(always)]
    fn from(val: u8) -> Kre {
        Kre::from_bits(val)
    }
}
impl From<Kre> for u8 {
    #[inline(always)]
    fn from(val: Kre) -> u8 {
        Kre::to_bits(val)
    }
}
