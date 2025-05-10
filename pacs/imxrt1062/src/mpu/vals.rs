#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RasrA1ap {
    #[doc = "Any access generates a permission fault."]
    NO_ACCESS = 0x0,
    #[doc = "Privileged access only."]
    PRIVILEGED_ACCESS = 0x01,
    #[doc = "Any unprivileged write generates a permission fault (Priviledge read/write, unprivileged read-only)."]
    UNPRIVILEGED_READ_ACCESS = 0x02,
    #[doc = "Full access"]
    FULL_ACCESS = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Privileged read-only access."]
    PRIVILEGED_READ_ONLY = 0x05,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY = 0x06,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY_2 = 0x07,
}
impl RasrA1ap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RasrA1ap {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RasrA1ap {
    #[inline(always)]
    fn from(val: u8) -> RasrA1ap {
        RasrA1ap::from_bits(val)
    }
}
impl From<RasrA1ap> for u8 {
    #[inline(always)]
    fn from(val: RasrA1ap) -> u8 {
        RasrA1ap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RasrA2ap {
    #[doc = "Any access generates a permission fault."]
    NO_ACCESS = 0x0,
    #[doc = "Privileged access only."]
    PRIVILEGED_ACCESS = 0x01,
    #[doc = "Any unprivileged write generates a permission fault (Priviledge read/write, unprivileged read-only)."]
    UNPRIVILEGED_READ_ACCESS = 0x02,
    #[doc = "Full access"]
    FULL_ACCESS = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Privileged read-only access."]
    PRIVILEGED_READ_ONLY = 0x05,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY = 0x06,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY_2 = 0x07,
}
impl RasrA2ap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RasrA2ap {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RasrA2ap {
    #[inline(always)]
    fn from(val: u8) -> RasrA2ap {
        RasrA2ap::from_bits(val)
    }
}
impl From<RasrA2ap> for u8 {
    #[inline(always)]
    fn from(val: RasrA2ap) -> u8 {
        RasrA2ap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RasrA3ap {
    #[doc = "Any access generates a permission fault."]
    NO_ACCESS = 0x0,
    #[doc = "Privileged access only."]
    PRIVILEGED_ACCESS = 0x01,
    #[doc = "Any unprivileged write generates a permission fault (Priviledge read/write, unprivileged read-only)."]
    UNPRIVILEGED_READ_ACCESS = 0x02,
    #[doc = "Full access"]
    FULL_ACCESS = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Privileged read-only access."]
    PRIVILEGED_READ_ONLY = 0x05,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY = 0x06,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY_2 = 0x07,
}
impl RasrA3ap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RasrA3ap {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RasrA3ap {
    #[inline(always)]
    fn from(val: u8) -> RasrA3ap {
        RasrA3ap::from_bits(val)
    }
}
impl From<RasrA3ap> for u8 {
    #[inline(always)]
    fn from(val: RasrA3ap) -> u8 {
        RasrA3ap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RasrAp {
    #[doc = "Any access generates a permission fault."]
    NO_ACCESS = 0x0,
    #[doc = "Privileged access only."]
    PRIVILEGED_ACCESS = 0x01,
    #[doc = "Any unprivileged write generates a permission fault (Priviledge read/write, unprivileged read-only)."]
    UNPRIVILEGED_READ_ACCESS = 0x02,
    #[doc = "Full access"]
    FULL_ACCESS = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Privileged read-only access."]
    PRIVILEGED_READ_ONLY = 0x05,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY = 0x06,
    #[doc = "Privileged and unprivileged read-only access."]
    READ_ONLY_2 = 0x07,
}
impl RasrAp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RasrAp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RasrAp {
    #[inline(always)]
    fn from(val: u8) -> RasrAp {
        RasrAp::from_bits(val)
    }
}
impl From<RasrAp> for u8 {
    #[inline(always)]
    fn from(val: RasrAp) -> u8 {
        RasrAp::to_bits(val)
    }
}
