#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WrUnlock(u16);
impl WrUnlock {
    #[doc = "OTP write access is locked."]
    pub const NO_KEY: Self = Self(0x0);
    #[doc = "OTP write access is unlocked."]
    pub const KEY: Self = Self(0x3e77);
}
impl WrUnlock {
    pub const fn from_bits(val: u16) -> WrUnlock {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for WrUnlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_KEY"),
            0x3e77 => f.write_str("KEY"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WrUnlock {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_KEY"),
            0x3e77 => defmt::write!(f, "KEY"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
