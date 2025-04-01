#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WrUnlock(pub u16);
impl WrUnlock {
    #[doc = "OTP write access is locked."]
    pub const OTP_W_LOCKED: Self = Self(0);
    #[doc = "OTP write access is unlocked."]
    pub const OTP_W_UNLOCKED: Self = Self(0x3e77);
}
impl WrUnlock {
    pub const fn from_bits(val: u16) -> WrUnlock {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for WrUnlock {
    #[inline(always)]
    fn from(val: u16) -> WrUnlock {
        WrUnlock::from_bits(val)
    }
}
impl From<WrUnlock> for u16 {
    #[inline(always)]
    fn from(val: WrUnlock) -> u16 {
        WrUnlock::to_bits(val)
    }
}
