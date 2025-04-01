#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Operating in Normal mode (NRM)"]
    NORMAL = 0,
    #[doc = "Unused (reserved)"]
    RES_01 = 0x01,
    #[doc = "Operating in Security Violation Mode (SVM)"]
    RES_10_SVM = 0x02,
    #[doc = "Operating in Logically Disabled Mode (LDM)"]
    LDM = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
