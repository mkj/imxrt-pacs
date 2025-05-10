#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2clrEvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2clrEvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2clrEvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2clrEvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2clrEvenLinePattern {
        Ctrl2clrEvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2clrEvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2clrEvenLinePattern) -> u8 {
        Ctrl2clrEvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2clrOddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2clrOddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2clrOddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2clrOddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2clrOddLinePattern {
        Ctrl2clrOddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2clrOddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2clrOddLinePattern) -> u8 {
        Ctrl2clrOddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2clrOutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2clrOutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2clrOutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2clrOutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2clrOutstandingReqs {
        Ctrl2clrOutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2clrOutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2clrOutstandingReqs) -> u8 {
        Ctrl2clrOutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2evenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2evenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2evenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2evenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2evenLinePattern {
        Ctrl2evenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2evenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2evenLinePattern) -> u8 {
        Ctrl2evenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2oddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2oddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2oddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2oddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2oddLinePattern {
        Ctrl2oddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2oddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2oddLinePattern) -> u8 {
        Ctrl2oddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2outstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2outstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2outstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2outstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2outstandingReqs {
        Ctrl2outstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2outstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2outstandingReqs) -> u8 {
        Ctrl2outstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2setEvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2setEvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2setEvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2setEvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2setEvenLinePattern {
        Ctrl2setEvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2setEvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2setEvenLinePattern) -> u8 {
        Ctrl2setEvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2setOddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2setOddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2setOddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2setOddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2setOddLinePattern {
        Ctrl2setOddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2setOddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2setOddLinePattern) -> u8 {
        Ctrl2setOddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2setOutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2setOutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2setOutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2setOutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2setOutstandingReqs {
        Ctrl2setOutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2setOutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2setOutstandingReqs) -> u8 {
        Ctrl2setOutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2togEvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2togEvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2togEvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2togEvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2togEvenLinePattern {
        Ctrl2togEvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2togEvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2togEvenLinePattern) -> u8 {
        Ctrl2togEvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2togOddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2togOddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2togOddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2togOddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2togOddLinePattern {
        Ctrl2togOddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2togOddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2togOddLinePattern) -> u8 {
        Ctrl2togOddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2togOutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2togOutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2togOutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2togOutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2togOutstandingReqs {
        Ctrl2togOutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2togOutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2togOutstandingReqs) -> u8 {
        Ctrl2togOutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlClrCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrCscDataSwizzle {
        CtrlClrCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlClrCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrCscDataSwizzle) -> u8 {
        CtrlClrCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlClrInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrInputDataSwizzle {
        CtrlClrInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlClrInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrInputDataSwizzle) -> u8 {
        CtrlClrInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlClrLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrLcdDatabusWidth {
        CtrlClrLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlClrLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrLcdDatabusWidth) -> u8 {
        CtrlClrLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlClrWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrWordLength {
        CtrlClrWordLength::from_bits(val)
    }
}
impl From<CtrlClrWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrWordLength) -> u8 {
        CtrlClrWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlCscDataSwizzle {
        CtrlCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlCscDataSwizzle) -> u8 {
        CtrlCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlInputDataSwizzle {
        CtrlInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlInputDataSwizzle) -> u8 {
        CtrlInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlLcdDatabusWidth {
        CtrlLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlLcdDatabusWidth) -> u8 {
        CtrlLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlSetCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetCscDataSwizzle {
        CtrlSetCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlSetCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetCscDataSwizzle) -> u8 {
        CtrlSetCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlSetInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetInputDataSwizzle {
        CtrlSetInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlSetInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetInputDataSwizzle) -> u8 {
        CtrlSetInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlSetLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetLcdDatabusWidth {
        CtrlSetLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlSetLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetLcdDatabusWidth) -> u8 {
        CtrlSetLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlSetWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetWordLength {
        CtrlSetWordLength::from_bits(val)
    }
}
impl From<CtrlSetWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetWordLength) -> u8 {
        CtrlSetWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlTogCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogCscDataSwizzle {
        CtrlTogCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlTogCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogCscDataSwizzle) -> u8 {
        CtrlTogCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlTogInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogInputDataSwizzle {
        CtrlTogInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlTogInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogInputDataSwizzle) -> u8 {
        CtrlTogInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlTogLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogLcdDatabusWidth {
        CtrlTogLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlTogLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogLcdDatabusWidth) -> u8 {
        CtrlTogLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlTogWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogWordLength {
        CtrlTogWordLength::from_bits(val)
    }
}
impl From<CtrlTogWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogWordLength) -> u8 {
        CtrlTogWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlWordLength {
        CtrlWordLength::from_bits(val)
    }
}
impl From<CtrlWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlWordLength) -> u8 {
        CtrlWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon00incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon00incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon00incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon00incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00incSel {
        Pigeon00incSel::from_bits(val)
    }
}
impl From<Pigeon00incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00incSel) -> u8 {
        Pigeon00incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon00maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon00maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon00maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon00maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00maskCntSel {
        Pigeon00maskCntSel::from_bits(val)
    }
}
impl From<Pigeon00maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00maskCntSel) -> u8 {
        Pigeon00maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon00stateMask(u8);
impl Pigeon00stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon00stateMask {
    pub const fn from_bits(val: u8) -> Pigeon00stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon00stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon00stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon00stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00stateMask {
        Pigeon00stateMask::from_bits(val)
    }
}
impl From<Pigeon00stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00stateMask) -> u8 {
        Pigeon00stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon01clrCnt(u16);
impl Pigeon01clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon01clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon01clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon01clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon01clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon01clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon01clrCnt {
        Pigeon01clrCnt::from_bits(val)
    }
}
impl From<Pigeon01clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon01clrCnt) -> u16 {
        Pigeon01clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon01setCnt(u16);
impl Pigeon01setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon01setCnt {
    pub const fn from_bits(val: u16) -> Pigeon01setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon01setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon01setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon01setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon01setCnt {
        Pigeon01setCnt::from_bits(val)
    }
}
impl From<Pigeon01setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon01setCnt) -> u16 {
        Pigeon01setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon02sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon02sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon02sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon02sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon02sigAnother {
        Pigeon02sigAnother::from_bits(val)
    }
}
impl From<Pigeon02sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon02sigAnother) -> u8 {
        Pigeon02sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon02sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon02sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon02sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon02sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon02sigLogic {
        Pigeon02sigLogic::from_bits(val)
    }
}
impl From<Pigeon02sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon02sigLogic) -> u8 {
        Pigeon02sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon100incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon100incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon100incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon100incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100incSel {
        Pigeon100incSel::from_bits(val)
    }
}
impl From<Pigeon100incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100incSel) -> u8 {
        Pigeon100incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon100maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon100maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon100maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon100maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100maskCntSel {
        Pigeon100maskCntSel::from_bits(val)
    }
}
impl From<Pigeon100maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100maskCntSel) -> u8 {
        Pigeon100maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon100stateMask(u8);
impl Pigeon100stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon100stateMask {
    pub const fn from_bits(val: u8) -> Pigeon100stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon100stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon100stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon100stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100stateMask {
        Pigeon100stateMask::from_bits(val)
    }
}
impl From<Pigeon100stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100stateMask) -> u8 {
        Pigeon100stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon101clrCnt(u16);
impl Pigeon101clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon101clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon101clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon101clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon101clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon101clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon101clrCnt {
        Pigeon101clrCnt::from_bits(val)
    }
}
impl From<Pigeon101clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon101clrCnt) -> u16 {
        Pigeon101clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon101setCnt(u16);
impl Pigeon101setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon101setCnt {
    pub const fn from_bits(val: u16) -> Pigeon101setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon101setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon101setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon101setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon101setCnt {
        Pigeon101setCnt::from_bits(val)
    }
}
impl From<Pigeon101setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon101setCnt) -> u16 {
        Pigeon101setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon102sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon102sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon102sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon102sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon102sigAnother {
        Pigeon102sigAnother::from_bits(val)
    }
}
impl From<Pigeon102sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon102sigAnother) -> u8 {
        Pigeon102sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon102sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon102sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon102sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon102sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon102sigLogic {
        Pigeon102sigLogic::from_bits(val)
    }
}
impl From<Pigeon102sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon102sigLogic) -> u8 {
        Pigeon102sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon10incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon10incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon10incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon10incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10incSel {
        Pigeon10incSel::from_bits(val)
    }
}
impl From<Pigeon10incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10incSel) -> u8 {
        Pigeon10incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon10maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon10maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon10maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon10maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10maskCntSel {
        Pigeon10maskCntSel::from_bits(val)
    }
}
impl From<Pigeon10maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10maskCntSel) -> u8 {
        Pigeon10maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon10stateMask(u8);
impl Pigeon10stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon10stateMask {
    pub const fn from_bits(val: u8) -> Pigeon10stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon10stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon10stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon10stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10stateMask {
        Pigeon10stateMask::from_bits(val)
    }
}
impl From<Pigeon10stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10stateMask) -> u8 {
        Pigeon10stateMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon110incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon110incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon110incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon110incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110incSel {
        Pigeon110incSel::from_bits(val)
    }
}
impl From<Pigeon110incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110incSel) -> u8 {
        Pigeon110incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon110maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon110maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon110maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon110maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110maskCntSel {
        Pigeon110maskCntSel::from_bits(val)
    }
}
impl From<Pigeon110maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110maskCntSel) -> u8 {
        Pigeon110maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon110stateMask(u8);
impl Pigeon110stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon110stateMask {
    pub const fn from_bits(val: u8) -> Pigeon110stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon110stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon110stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon110stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110stateMask {
        Pigeon110stateMask::from_bits(val)
    }
}
impl From<Pigeon110stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110stateMask) -> u8 {
        Pigeon110stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon111clrCnt(u16);
impl Pigeon111clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon111clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon111clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon111clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon111clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon111clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon111clrCnt {
        Pigeon111clrCnt::from_bits(val)
    }
}
impl From<Pigeon111clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon111clrCnt) -> u16 {
        Pigeon111clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon111setCnt(u16);
impl Pigeon111setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon111setCnt {
    pub const fn from_bits(val: u16) -> Pigeon111setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon111setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon111setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon111setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon111setCnt {
        Pigeon111setCnt::from_bits(val)
    }
}
impl From<Pigeon111setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon111setCnt) -> u16 {
        Pigeon111setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon112sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon112sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon112sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon112sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon112sigAnother {
        Pigeon112sigAnother::from_bits(val)
    }
}
impl From<Pigeon112sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon112sigAnother) -> u8 {
        Pigeon112sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon112sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon112sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon112sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon112sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon112sigLogic {
        Pigeon112sigLogic::from_bits(val)
    }
}
impl From<Pigeon112sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon112sigLogic) -> u8 {
        Pigeon112sigLogic::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon11clrCnt(u16);
impl Pigeon11clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon11clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon11clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon11clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon11clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon11clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon11clrCnt {
        Pigeon11clrCnt::from_bits(val)
    }
}
impl From<Pigeon11clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon11clrCnt) -> u16 {
        Pigeon11clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon11setCnt(u16);
impl Pigeon11setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon11setCnt {
    pub const fn from_bits(val: u16) -> Pigeon11setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon11setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon11setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon11setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon11setCnt {
        Pigeon11setCnt::from_bits(val)
    }
}
impl From<Pigeon11setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon11setCnt) -> u16 {
        Pigeon11setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon12sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon12sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon12sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon12sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon12sigAnother {
        Pigeon12sigAnother::from_bits(val)
    }
}
impl From<Pigeon12sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon12sigAnother) -> u8 {
        Pigeon12sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon12sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon12sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon12sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon12sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon12sigLogic {
        Pigeon12sigLogic::from_bits(val)
    }
}
impl From<Pigeon12sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon12sigLogic) -> u8 {
        Pigeon12sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon20incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon20incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon20incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon20incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20incSel {
        Pigeon20incSel::from_bits(val)
    }
}
impl From<Pigeon20incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20incSel) -> u8 {
        Pigeon20incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon20maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon20maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon20maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon20maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20maskCntSel {
        Pigeon20maskCntSel::from_bits(val)
    }
}
impl From<Pigeon20maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20maskCntSel) -> u8 {
        Pigeon20maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon20stateMask(u8);
impl Pigeon20stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon20stateMask {
    pub const fn from_bits(val: u8) -> Pigeon20stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon20stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon20stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon20stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20stateMask {
        Pigeon20stateMask::from_bits(val)
    }
}
impl From<Pigeon20stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20stateMask) -> u8 {
        Pigeon20stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon21clrCnt(u16);
impl Pigeon21clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon21clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon21clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon21clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon21clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon21clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon21clrCnt {
        Pigeon21clrCnt::from_bits(val)
    }
}
impl From<Pigeon21clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon21clrCnt) -> u16 {
        Pigeon21clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon21setCnt(u16);
impl Pigeon21setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon21setCnt {
    pub const fn from_bits(val: u16) -> Pigeon21setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon21setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon21setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon21setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon21setCnt {
        Pigeon21setCnt::from_bits(val)
    }
}
impl From<Pigeon21setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon21setCnt) -> u16 {
        Pigeon21setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon22sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon22sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon22sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon22sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon22sigAnother {
        Pigeon22sigAnother::from_bits(val)
    }
}
impl From<Pigeon22sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon22sigAnother) -> u8 {
        Pigeon22sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon22sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon22sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon22sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon22sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon22sigLogic {
        Pigeon22sigLogic::from_bits(val)
    }
}
impl From<Pigeon22sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon22sigLogic) -> u8 {
        Pigeon22sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon30incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon30incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon30incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon30incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30incSel {
        Pigeon30incSel::from_bits(val)
    }
}
impl From<Pigeon30incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30incSel) -> u8 {
        Pigeon30incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon30maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon30maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon30maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon30maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30maskCntSel {
        Pigeon30maskCntSel::from_bits(val)
    }
}
impl From<Pigeon30maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30maskCntSel) -> u8 {
        Pigeon30maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon30stateMask(u8);
impl Pigeon30stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon30stateMask {
    pub const fn from_bits(val: u8) -> Pigeon30stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon30stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon30stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon30stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30stateMask {
        Pigeon30stateMask::from_bits(val)
    }
}
impl From<Pigeon30stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30stateMask) -> u8 {
        Pigeon30stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon31clrCnt(u16);
impl Pigeon31clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon31clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon31clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon31clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon31clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon31clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon31clrCnt {
        Pigeon31clrCnt::from_bits(val)
    }
}
impl From<Pigeon31clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon31clrCnt) -> u16 {
        Pigeon31clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon31setCnt(u16);
impl Pigeon31setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon31setCnt {
    pub const fn from_bits(val: u16) -> Pigeon31setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon31setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon31setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon31setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon31setCnt {
        Pigeon31setCnt::from_bits(val)
    }
}
impl From<Pigeon31setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon31setCnt) -> u16 {
        Pigeon31setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon32sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon32sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon32sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon32sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon32sigAnother {
        Pigeon32sigAnother::from_bits(val)
    }
}
impl From<Pigeon32sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon32sigAnother) -> u8 {
        Pigeon32sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon32sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon32sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon32sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon32sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon32sigLogic {
        Pigeon32sigLogic::from_bits(val)
    }
}
impl From<Pigeon32sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon32sigLogic) -> u8 {
        Pigeon32sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon40incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon40incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon40incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon40incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40incSel {
        Pigeon40incSel::from_bits(val)
    }
}
impl From<Pigeon40incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40incSel) -> u8 {
        Pigeon40incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon40maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon40maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon40maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon40maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40maskCntSel {
        Pigeon40maskCntSel::from_bits(val)
    }
}
impl From<Pigeon40maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40maskCntSel) -> u8 {
        Pigeon40maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon40stateMask(u8);
impl Pigeon40stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon40stateMask {
    pub const fn from_bits(val: u8) -> Pigeon40stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon40stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon40stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon40stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40stateMask {
        Pigeon40stateMask::from_bits(val)
    }
}
impl From<Pigeon40stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40stateMask) -> u8 {
        Pigeon40stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon41clrCnt(u16);
impl Pigeon41clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon41clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon41clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon41clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon41clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon41clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon41clrCnt {
        Pigeon41clrCnt::from_bits(val)
    }
}
impl From<Pigeon41clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon41clrCnt) -> u16 {
        Pigeon41clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon41setCnt(u16);
impl Pigeon41setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon41setCnt {
    pub const fn from_bits(val: u16) -> Pigeon41setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon41setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon41setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon41setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon41setCnt {
        Pigeon41setCnt::from_bits(val)
    }
}
impl From<Pigeon41setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon41setCnt) -> u16 {
        Pigeon41setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon42sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon42sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon42sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon42sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon42sigAnother {
        Pigeon42sigAnother::from_bits(val)
    }
}
impl From<Pigeon42sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon42sigAnother) -> u8 {
        Pigeon42sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon42sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon42sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon42sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon42sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon42sigLogic {
        Pigeon42sigLogic::from_bits(val)
    }
}
impl From<Pigeon42sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon42sigLogic) -> u8 {
        Pigeon42sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon50incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon50incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon50incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon50incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50incSel {
        Pigeon50incSel::from_bits(val)
    }
}
impl From<Pigeon50incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50incSel) -> u8 {
        Pigeon50incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon50maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon50maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon50maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon50maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50maskCntSel {
        Pigeon50maskCntSel::from_bits(val)
    }
}
impl From<Pigeon50maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50maskCntSel) -> u8 {
        Pigeon50maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon50stateMask(u8);
impl Pigeon50stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon50stateMask {
    pub const fn from_bits(val: u8) -> Pigeon50stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon50stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon50stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon50stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50stateMask {
        Pigeon50stateMask::from_bits(val)
    }
}
impl From<Pigeon50stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50stateMask) -> u8 {
        Pigeon50stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon51clrCnt(u16);
impl Pigeon51clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon51clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon51clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon51clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon51clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon51clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon51clrCnt {
        Pigeon51clrCnt::from_bits(val)
    }
}
impl From<Pigeon51clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon51clrCnt) -> u16 {
        Pigeon51clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon51setCnt(u16);
impl Pigeon51setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon51setCnt {
    pub const fn from_bits(val: u16) -> Pigeon51setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon51setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon51setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon51setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon51setCnt {
        Pigeon51setCnt::from_bits(val)
    }
}
impl From<Pigeon51setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon51setCnt) -> u16 {
        Pigeon51setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon52sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon52sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon52sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon52sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon52sigAnother {
        Pigeon52sigAnother::from_bits(val)
    }
}
impl From<Pigeon52sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon52sigAnother) -> u8 {
        Pigeon52sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon52sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon52sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon52sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon52sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon52sigLogic {
        Pigeon52sigLogic::from_bits(val)
    }
}
impl From<Pigeon52sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon52sigLogic) -> u8 {
        Pigeon52sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon60incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon60incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon60incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon60incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60incSel {
        Pigeon60incSel::from_bits(val)
    }
}
impl From<Pigeon60incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60incSel) -> u8 {
        Pigeon60incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon60maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon60maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon60maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon60maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60maskCntSel {
        Pigeon60maskCntSel::from_bits(val)
    }
}
impl From<Pigeon60maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60maskCntSel) -> u8 {
        Pigeon60maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon60stateMask(u8);
impl Pigeon60stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon60stateMask {
    pub const fn from_bits(val: u8) -> Pigeon60stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon60stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon60stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon60stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60stateMask {
        Pigeon60stateMask::from_bits(val)
    }
}
impl From<Pigeon60stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60stateMask) -> u8 {
        Pigeon60stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon61clrCnt(u16);
impl Pigeon61clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon61clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon61clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon61clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon61clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon61clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon61clrCnt {
        Pigeon61clrCnt::from_bits(val)
    }
}
impl From<Pigeon61clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon61clrCnt) -> u16 {
        Pigeon61clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon61setCnt(u16);
impl Pigeon61setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon61setCnt {
    pub const fn from_bits(val: u16) -> Pigeon61setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon61setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon61setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon61setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon61setCnt {
        Pigeon61setCnt::from_bits(val)
    }
}
impl From<Pigeon61setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon61setCnt) -> u16 {
        Pigeon61setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon62sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon62sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon62sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon62sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon62sigAnother {
        Pigeon62sigAnother::from_bits(val)
    }
}
impl From<Pigeon62sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon62sigAnother) -> u8 {
        Pigeon62sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon62sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon62sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon62sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon62sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon62sigLogic {
        Pigeon62sigLogic::from_bits(val)
    }
}
impl From<Pigeon62sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon62sigLogic) -> u8 {
        Pigeon62sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon70incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon70incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon70incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon70incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70incSel {
        Pigeon70incSel::from_bits(val)
    }
}
impl From<Pigeon70incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70incSel) -> u8 {
        Pigeon70incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon70maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon70maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon70maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon70maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70maskCntSel {
        Pigeon70maskCntSel::from_bits(val)
    }
}
impl From<Pigeon70maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70maskCntSel) -> u8 {
        Pigeon70maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon70stateMask(u8);
impl Pigeon70stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon70stateMask {
    pub const fn from_bits(val: u8) -> Pigeon70stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon70stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon70stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon70stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70stateMask {
        Pigeon70stateMask::from_bits(val)
    }
}
impl From<Pigeon70stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70stateMask) -> u8 {
        Pigeon70stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon71clrCnt(u16);
impl Pigeon71clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon71clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon71clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon71clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon71clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon71clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon71clrCnt {
        Pigeon71clrCnt::from_bits(val)
    }
}
impl From<Pigeon71clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon71clrCnt) -> u16 {
        Pigeon71clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon71setCnt(u16);
impl Pigeon71setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon71setCnt {
    pub const fn from_bits(val: u16) -> Pigeon71setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon71setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon71setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon71setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon71setCnt {
        Pigeon71setCnt::from_bits(val)
    }
}
impl From<Pigeon71setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon71setCnt) -> u16 {
        Pigeon71setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon72sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon72sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon72sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon72sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon72sigAnother {
        Pigeon72sigAnother::from_bits(val)
    }
}
impl From<Pigeon72sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon72sigAnother) -> u8 {
        Pigeon72sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon72sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon72sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon72sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon72sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon72sigLogic {
        Pigeon72sigLogic::from_bits(val)
    }
}
impl From<Pigeon72sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon72sigLogic) -> u8 {
        Pigeon72sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon80incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon80incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon80incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon80incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80incSel {
        Pigeon80incSel::from_bits(val)
    }
}
impl From<Pigeon80incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80incSel) -> u8 {
        Pigeon80incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon80maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon80maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon80maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon80maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80maskCntSel {
        Pigeon80maskCntSel::from_bits(val)
    }
}
impl From<Pigeon80maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80maskCntSel) -> u8 {
        Pigeon80maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon80stateMask(u8);
impl Pigeon80stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon80stateMask {
    pub const fn from_bits(val: u8) -> Pigeon80stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon80stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon80stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon80stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80stateMask {
        Pigeon80stateMask::from_bits(val)
    }
}
impl From<Pigeon80stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80stateMask) -> u8 {
        Pigeon80stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon81clrCnt(u16);
impl Pigeon81clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon81clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon81clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon81clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon81clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon81clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon81clrCnt {
        Pigeon81clrCnt::from_bits(val)
    }
}
impl From<Pigeon81clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon81clrCnt) -> u16 {
        Pigeon81clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon81setCnt(u16);
impl Pigeon81setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon81setCnt {
    pub const fn from_bits(val: u16) -> Pigeon81setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon81setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon81setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon81setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon81setCnt {
        Pigeon81setCnt::from_bits(val)
    }
}
impl From<Pigeon81setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon81setCnt) -> u16 {
        Pigeon81setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon82sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon82sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon82sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon82sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon82sigAnother {
        Pigeon82sigAnother::from_bits(val)
    }
}
impl From<Pigeon82sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon82sigAnother) -> u8 {
        Pigeon82sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon82sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon82sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon82sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon82sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon82sigLogic {
        Pigeon82sigLogic::from_bits(val)
    }
}
impl From<Pigeon82sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon82sigLogic) -> u8 {
        Pigeon82sigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon90incSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon90incSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon90incSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon90incSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90incSel {
        Pigeon90incSel::from_bits(val)
    }
}
impl From<Pigeon90incSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90incSel) -> u8 {
        Pigeon90incSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon90maskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon90maskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon90maskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon90maskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90maskCntSel {
        Pigeon90maskCntSel::from_bits(val)
    }
}
impl From<Pigeon90maskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90maskCntSel) -> u8 {
        Pigeon90maskCntSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon90stateMask(u8);
impl Pigeon90stateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon90stateMask {
    pub const fn from_bits(val: u8) -> Pigeon90stateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon90stateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon90stateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon90stateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90stateMask {
        Pigeon90stateMask::from_bits(val)
    }
}
impl From<Pigeon90stateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90stateMask) -> u8 {
        Pigeon90stateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon91clrCnt(u16);
impl Pigeon91clrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon91clrCnt {
    pub const fn from_bits(val: u16) -> Pigeon91clrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon91clrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon91clrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon91clrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon91clrCnt {
        Pigeon91clrCnt::from_bits(val)
    }
}
impl From<Pigeon91clrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon91clrCnt) -> u16 {
        Pigeon91clrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon91setCnt(u16);
impl Pigeon91setCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon91setCnt {
    pub const fn from_bits(val: u16) -> Pigeon91setCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon91setCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon91setCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon91setCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon91setCnt {
        Pigeon91setCnt::from_bits(val)
    }
}
impl From<Pigeon91setCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon91setCnt) -> u16 {
        Pigeon91setCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon92sigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Pigeon92sigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon92sigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon92sigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon92sigAnother {
        Pigeon92sigAnother::from_bits(val)
    }
}
impl From<Pigeon92sigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon92sigAnother) -> u8 {
        Pigeon92sigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon92sigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon92sigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon92sigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon92sigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon92sigLogic {
        Pigeon92sigLogic::from_bits(val)
    }
}
impl From<Pigeon92sigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon92sigLogic) -> u8 {
        Pigeon92sigLogic::to_bits(val)
    }
}
