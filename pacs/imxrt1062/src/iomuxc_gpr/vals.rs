#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LockOcram2tzAddr(u8);
impl LockOcram2tzAddr {
    #[doc = "Field is not locked"]
    pub const LOCK_OCRAM2_TZ_ADDR_0: Self = Self(0x0);
    #[doc = "Field is locked (read access only)"]
    pub const LOCK_OCRAM2_TZ_ADDR_1: Self = Self(0x01);
}
impl LockOcram2tzAddr {
    pub const fn from_bits(val: u8) -> LockOcram2tzAddr {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LockOcram2tzAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("LOCK_OCRAM2_TZ_ADDR_0"),
            0x01 => f.write_str("LOCK_OCRAM2_TZ_ADDR_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LockOcram2tzAddr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LOCK_OCRAM2_TZ_ADDR_0"),
            0x01 => defmt::write!(f, "LOCK_OCRAM2_TZ_ADDR_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LockOcram2tzAddr {
    #[inline(always)]
    fn from(val: u8) -> LockOcram2tzAddr {
        LockOcram2tzAddr::from_bits(val)
    }
}
impl From<LockOcram2tzAddr> for u8 {
    #[inline(always)]
    fn from(val: LockOcram2tzAddr) -> u8 {
        LockOcram2tzAddr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LockOcramTzAddr(u8);
impl LockOcramTzAddr {
    #[doc = "Field is not locked"]
    pub const LOCK_OCRAM_TZ_ADDR_0: Self = Self(0x0);
    #[doc = "Field is locked (read access only)"]
    pub const LOCK_OCRAM_TZ_ADDR_1: Self = Self(0x01);
}
impl LockOcramTzAddr {
    pub const fn from_bits(val: u8) -> LockOcramTzAddr {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LockOcramTzAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("LOCK_OCRAM_TZ_ADDR_0"),
            0x01 => f.write_str("LOCK_OCRAM_TZ_ADDR_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LockOcramTzAddr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LOCK_OCRAM_TZ_ADDR_0"),
            0x01 => defmt::write!(f, "LOCK_OCRAM_TZ_ADDR_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LockOcramTzAddr {
    #[inline(always)]
    fn from(val: u8) -> LockOcramTzAddr {
        LockOcramTzAddr::from_bits(val)
    }
}
impl From<LockOcramTzAddr> for u8 {
    #[inline(always)]
    fn from(val: LockOcramTzAddr) -> u8 {
        LockOcramTzAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7apcAcR0ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R0_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R0_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7apcAcR0ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7apcAcR0ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7apcAcR0ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7apcAcR0ctrl {
        M7apcAcR0ctrl::from_bits(val)
    }
}
impl From<M7apcAcR0ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7apcAcR0ctrl) -> u8 {
        M7apcAcR0ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7apcAcR1ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R1_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R1_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7apcAcR1ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7apcAcR1ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7apcAcR1ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7apcAcR1ctrl {
        M7apcAcR1ctrl::from_bits(val)
    }
}
impl From<M7apcAcR1ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7apcAcR1ctrl) -> u8 {
        M7apcAcR1ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7apcAcR2ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R2_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R2_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7apcAcR2ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7apcAcR2ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7apcAcR2ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7apcAcR2ctrl {
        M7apcAcR2ctrl::from_bits(val)
    }
}
impl From<M7apcAcR2ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7apcAcR2ctrl) -> u8 {
        M7apcAcR2ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7apcAcR3ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R3_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R3_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7apcAcR3ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7apcAcR3ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7apcAcR3ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7apcAcR3ctrl {
        M7apcAcR3ctrl::from_bits(val)
    }
}
impl From<M7apcAcR3ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7apcAcR3ctrl) -> u8 {
        M7apcAcR3ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MqsClkDiv {
    #[doc = "mclk frequency = 1/1 * hmclk frequency"]
    DIVIDE_1 = 0x0,
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    DIVIDE_2 = 0x01,
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    DIVIDE_3 = 0x02,
    #[doc = "mclk frequency = 1/4 * hmclk frequency"]
    DIVIDE_4 = 0x03,
    #[doc = "mclk frequency = 1/5 * hmclk frequency"]
    DIVIDE_5 = 0x04,
    #[doc = "mclk frequency = 1/6 * hmclk frequency"]
    DIVIDE_6 = 0x05,
    #[doc = "mclk frequency = 1/7 * hmclk frequency"]
    DIVIDE_7 = 0x06,
    #[doc = "mclk frequency = 1/8 * hmclk frequency"]
    DIVIDE_8 = 0x07,
    #[doc = "mclk frequency = 1/9 * hmclk frequency"]
    DIVIDE_9 = 0x08,
    #[doc = "mclk frequency = 1/10 * hmclk frequency"]
    DIVIDE_10 = 0x09,
    #[doc = "mclk frequency = 1/11 * hmclk frequency"]
    DIVIDE_11 = 0x0a,
    #[doc = "mclk frequency = 1/12 * hmclk frequency"]
    DIVIDE_12 = 0x0b,
    #[doc = "mclk frequency = 1/13 * hmclk frequency"]
    DIVIDE_13 = 0x0c,
    #[doc = "mclk frequency = 1/14 * hmclk frequency"]
    DIVIDE_14 = 0x0d,
    #[doc = "mclk frequency = 1/15 * hmclk frequency"]
    DIVIDE_15 = 0x0e,
    #[doc = "mclk frequency = 1/16 * hmclk frequency"]
    DIVIDE_16 = 0x0f,
    #[doc = "mclk frequency = 1/17 * hmclk frequency"]
    DIVIDE_17 = 0x10,
    #[doc = "mclk frequency = 1/18 * hmclk frequency"]
    DIVIDE_18 = 0x11,
    #[doc = "mclk frequency = 1/19 * hmclk frequency"]
    DIVIDE_19 = 0x12,
    #[doc = "mclk frequency = 1/20 * hmclk frequency"]
    DIVIDE_20 = 0x13,
    #[doc = "mclk frequency = 1/21 * hmclk frequency"]
    DIVIDE_21 = 0x14,
    #[doc = "mclk frequency = 1/22 * hmclk frequency"]
    DIVIDE_22 = 0x15,
    #[doc = "mclk frequency = 1/23 * hmclk frequency"]
    DIVIDE_23 = 0x16,
    #[doc = "mclk frequency = 1/24 * hmclk frequency"]
    DIVIDE_24 = 0x17,
    #[doc = "mclk frequency = 1/25 * hmclk frequency"]
    DIVIDE_25 = 0x18,
    #[doc = "mclk frequency = 1/26 * hmclk frequency"]
    DIVIDE_26 = 0x19,
    #[doc = "mclk frequency = 1/27 * hmclk frequency"]
    DIVIDE_27 = 0x1a,
    #[doc = "mclk frequency = 1/28 * hmclk frequency"]
    DIVIDE_28 = 0x1b,
    #[doc = "mclk frequency = 1/29 * hmclk frequency"]
    DIVIDE_29 = 0x1c,
    #[doc = "mclk frequency = 1/30 * hmclk frequency"]
    DIVIDE_30 = 0x1d,
    #[doc = "mclk frequency = 1/31 * hmclk frequency"]
    DIVIDE_31 = 0x1e,
    #[doc = "mclk frequency = 1/32 * hmclk frequency"]
    DIVIDE_32 = 0x1f,
    #[doc = "mclk frequency = 1/33 * hmclk frequency"]
    DIVIDE_33 = 0x20,
    #[doc = "mclk frequency = 1/34 * hmclk frequency"]
    DIVIDE_34 = 0x21,
    #[doc = "mclk frequency = 1/35 * hmclk frequency"]
    DIVIDE_35 = 0x22,
    #[doc = "mclk frequency = 1/36 * hmclk frequency"]
    DIVIDE_36 = 0x23,
    #[doc = "mclk frequency = 1/37 * hmclk frequency"]
    DIVIDE_37 = 0x24,
    #[doc = "mclk frequency = 1/38 * hmclk frequency"]
    DIVIDE_38 = 0x25,
    #[doc = "mclk frequency = 1/39 * hmclk frequency"]
    DIVIDE_39 = 0x26,
    #[doc = "mclk frequency = 1/40 * hmclk frequency"]
    DIVIDE_40 = 0x27,
    #[doc = "mclk frequency = 1/41 * hmclk frequency"]
    DIVIDE_41 = 0x28,
    #[doc = "mclk frequency = 1/42 * hmclk frequency"]
    DIVIDE_42 = 0x29,
    #[doc = "mclk frequency = 1/43 * hmclk frequency"]
    DIVIDE_43 = 0x2a,
    #[doc = "mclk frequency = 1/44 * hmclk frequency"]
    DIVIDE_44 = 0x2b,
    #[doc = "mclk frequency = 1/45 * hmclk frequency"]
    DIVIDE_45 = 0x2c,
    #[doc = "mclk frequency = 1/46 * hmclk frequency"]
    DIVIDE_46 = 0x2d,
    #[doc = "mclk frequency = 1/47 * hmclk frequency"]
    DIVIDE_47 = 0x2e,
    #[doc = "mclk frequency = 1/48 * hmclk frequency"]
    DIVIDE_48 = 0x2f,
    #[doc = "mclk frequency = 1/49 * hmclk frequency"]
    DIVIDE_49 = 0x30,
    #[doc = "mclk frequency = 1/50 * hmclk frequency"]
    DIVIDE_50 = 0x31,
    #[doc = "mclk frequency = 1/51 * hmclk frequency"]
    DIVIDE_51 = 0x32,
    #[doc = "mclk frequency = 1/52 * hmclk frequency"]
    DIVIDE_52 = 0x33,
    #[doc = "mclk frequency = 1/53 * hmclk frequency"]
    DIVIDE_53 = 0x34,
    #[doc = "mclk frequency = 1/54 * hmclk frequency"]
    DIVIDE_54 = 0x35,
    #[doc = "mclk frequency = 1/55 * hmclk frequency"]
    DIVIDE_55 = 0x36,
    #[doc = "mclk frequency = 1/56 * hmclk frequency"]
    DIVIDE_56 = 0x37,
    #[doc = "mclk frequency = 1/57 * hmclk frequency"]
    DIVIDE_57 = 0x38,
    #[doc = "mclk frequency = 1/58 * hmclk frequency"]
    DIVIDE_58 = 0x39,
    #[doc = "mclk frequency = 1/59 * hmclk frequency"]
    DIVIDE_59 = 0x3a,
    #[doc = "mclk frequency = 1/60 * hmclk frequency"]
    DIVIDE_60 = 0x3b,
    #[doc = "mclk frequency = 1/61 * hmclk frequency"]
    DIVIDE_61 = 0x3c,
    #[doc = "mclk frequency = 1/62 * hmclk frequency"]
    DIVIDE_62 = 0x3d,
    #[doc = "mclk frequency = 1/63 * hmclk frequency"]
    DIVIDE_63 = 0x3e,
    #[doc = "mclk frequency = 1/64 * hmclk frequency"]
    DIVIDE_64 = 0x3f,
    #[doc = "mclk frequency = 1/65 * hmclk frequency"]
    DIVIDE_65 = 0x40,
    #[doc = "mclk frequency = 1/66 * hmclk frequency"]
    DIVIDE_66 = 0x41,
    #[doc = "mclk frequency = 1/67 * hmclk frequency"]
    DIVIDE_67 = 0x42,
    #[doc = "mclk frequency = 1/68 * hmclk frequency"]
    DIVIDE_68 = 0x43,
    #[doc = "mclk frequency = 1/69 * hmclk frequency"]
    DIVIDE_69 = 0x44,
    #[doc = "mclk frequency = 1/70 * hmclk frequency"]
    DIVIDE_70 = 0x45,
    #[doc = "mclk frequency = 1/71 * hmclk frequency"]
    DIVIDE_71 = 0x46,
    #[doc = "mclk frequency = 1/72 * hmclk frequency"]
    DIVIDE_72 = 0x47,
    #[doc = "mclk frequency = 1/73 * hmclk frequency"]
    DIVIDE_73 = 0x48,
    #[doc = "mclk frequency = 1/74 * hmclk frequency"]
    DIVIDE_74 = 0x49,
    #[doc = "mclk frequency = 1/75 * hmclk frequency"]
    DIVIDE_75 = 0x4a,
    #[doc = "mclk frequency = 1/76 * hmclk frequency"]
    DIVIDE_76 = 0x4b,
    #[doc = "mclk frequency = 1/77 * hmclk frequency"]
    DIVIDE_77 = 0x4c,
    #[doc = "mclk frequency = 1/78 * hmclk frequency"]
    DIVIDE_78 = 0x4d,
    #[doc = "mclk frequency = 1/79 * hmclk frequency"]
    DIVIDE_79 = 0x4e,
    #[doc = "mclk frequency = 1/80 * hmclk frequency"]
    DIVIDE_80 = 0x4f,
    #[doc = "mclk frequency = 1/81 * hmclk frequency"]
    DIVIDE_81 = 0x50,
    #[doc = "mclk frequency = 1/82 * hmclk frequency"]
    DIVIDE_82 = 0x51,
    #[doc = "mclk frequency = 1/83 * hmclk frequency"]
    DIVIDE_83 = 0x52,
    #[doc = "mclk frequency = 1/84 * hmclk frequency"]
    DIVIDE_84 = 0x53,
    #[doc = "mclk frequency = 1/85 * hmclk frequency"]
    DIVIDE_85 = 0x54,
    #[doc = "mclk frequency = 1/86 * hmclk frequency"]
    DIVIDE_86 = 0x55,
    #[doc = "mclk frequency = 1/87 * hmclk frequency"]
    DIVIDE_87 = 0x56,
    #[doc = "mclk frequency = 1/88 * hmclk frequency"]
    DIVIDE_88 = 0x57,
    #[doc = "mclk frequency = 1/89 * hmclk frequency"]
    DIVIDE_89 = 0x58,
    #[doc = "mclk frequency = 1/90 * hmclk frequency"]
    DIVIDE_90 = 0x59,
    #[doc = "mclk frequency = 1/91 * hmclk frequency"]
    DIVIDE_91 = 0x5a,
    #[doc = "mclk frequency = 1/92 * hmclk frequency"]
    DIVIDE_92 = 0x5b,
    #[doc = "mclk frequency = 1/93 * hmclk frequency"]
    DIVIDE_93 = 0x5c,
    #[doc = "mclk frequency = 1/94 * hmclk frequency"]
    DIVIDE_94 = 0x5d,
    #[doc = "mclk frequency = 1/95 * hmclk frequency"]
    DIVIDE_95 = 0x5e,
    #[doc = "mclk frequency = 1/96 * hmclk frequency"]
    DIVIDE_96 = 0x5f,
    #[doc = "mclk frequency = 1/97 * hmclk frequency"]
    DIVIDE_97 = 0x60,
    #[doc = "mclk frequency = 1/98 * hmclk frequency"]
    DIVIDE_98 = 0x61,
    #[doc = "mclk frequency = 1/99 * hmclk frequency"]
    DIVIDE_99 = 0x62,
    #[doc = "mclk frequency = 1/100 * hmclk frequency"]
    DIVIDE_100 = 0x63,
    #[doc = "mclk frequency = 1/101 * hmclk frequency"]
    DIVIDE_101 = 0x64,
    #[doc = "mclk frequency = 1/102 * hmclk frequency"]
    DIVIDE_102 = 0x65,
    #[doc = "mclk frequency = 1/103 * hmclk frequency"]
    DIVIDE_103 = 0x66,
    #[doc = "mclk frequency = 1/104 * hmclk frequency"]
    DIVIDE_104 = 0x67,
    #[doc = "mclk frequency = 1/105 * hmclk frequency"]
    DIVIDE_105 = 0x68,
    #[doc = "mclk frequency = 1/106 * hmclk frequency"]
    DIVIDE_106 = 0x69,
    #[doc = "mclk frequency = 1/107 * hmclk frequency"]
    DIVIDE_107 = 0x6a,
    #[doc = "mclk frequency = 1/108 * hmclk frequency"]
    DIVIDE_108 = 0x6b,
    #[doc = "mclk frequency = 1/109 * hmclk frequency"]
    DIVIDE_109 = 0x6c,
    #[doc = "mclk frequency = 1/110 * hmclk frequency"]
    DIVIDE_110 = 0x6d,
    #[doc = "mclk frequency = 1/111 * hmclk frequency"]
    DIVIDE_111 = 0x6e,
    #[doc = "mclk frequency = 1/112 * hmclk frequency"]
    DIVIDE_112 = 0x6f,
    #[doc = "mclk frequency = 1/113 * hmclk frequency"]
    DIVIDE_113 = 0x70,
    #[doc = "mclk frequency = 1/114 * hmclk frequency"]
    DIVIDE_114 = 0x71,
    #[doc = "mclk frequency = 1/115 * hmclk frequency"]
    DIVIDE_115 = 0x72,
    #[doc = "mclk frequency = 1/116 * hmclk frequency"]
    DIVIDE_116 = 0x73,
    #[doc = "mclk frequency = 1/117 * hmclk frequency"]
    DIVIDE_117 = 0x74,
    #[doc = "mclk frequency = 1/118 * hmclk frequency"]
    DIVIDE_118 = 0x75,
    #[doc = "mclk frequency = 1/119 * hmclk frequency"]
    DIVIDE_119 = 0x76,
    #[doc = "mclk frequency = 1/120 * hmclk frequency"]
    DIVIDE_120 = 0x77,
    #[doc = "mclk frequency = 1/121 * hmclk frequency"]
    DIVIDE_121 = 0x78,
    #[doc = "mclk frequency = 1/122 * hmclk frequency"]
    DIVIDE_122 = 0x79,
    #[doc = "mclk frequency = 1/123 * hmclk frequency"]
    DIVIDE_123 = 0x7a,
    #[doc = "mclk frequency = 1/124 * hmclk frequency"]
    DIVIDE_124 = 0x7b,
    #[doc = "mclk frequency = 1/125 * hmclk frequency"]
    DIVIDE_125 = 0x7c,
    #[doc = "mclk frequency = 1/126 * hmclk frequency"]
    DIVIDE_126 = 0x7d,
    #[doc = "mclk frequency = 1/127 * hmclk frequency"]
    DIVIDE_127 = 0x7e,
    #[doc = "mclk frequency = 1/128 * hmclk frequency"]
    DIVIDE_128 = 0x7f,
    #[doc = "mclk frequency = 1/129 * hmclk frequency"]
    DIVIDE_129 = 0x80,
    #[doc = "mclk frequency = 1/130 * hmclk frequency"]
    DIVIDE_130 = 0x81,
    #[doc = "mclk frequency = 1/131 * hmclk frequency"]
    DIVIDE_131 = 0x82,
    #[doc = "mclk frequency = 1/132 * hmclk frequency"]
    DIVIDE_132 = 0x83,
    #[doc = "mclk frequency = 1/133 * hmclk frequency"]
    DIVIDE_133 = 0x84,
    #[doc = "mclk frequency = 1/134 * hmclk frequency"]
    DIVIDE_134 = 0x85,
    #[doc = "mclk frequency = 1/135 * hmclk frequency"]
    DIVIDE_135 = 0x86,
    #[doc = "mclk frequency = 1/136 * hmclk frequency"]
    DIVIDE_136 = 0x87,
    #[doc = "mclk frequency = 1/137 * hmclk frequency"]
    DIVIDE_137 = 0x88,
    #[doc = "mclk frequency = 1/138 * hmclk frequency"]
    DIVIDE_138 = 0x89,
    #[doc = "mclk frequency = 1/139 * hmclk frequency"]
    DIVIDE_139 = 0x8a,
    #[doc = "mclk frequency = 1/140 * hmclk frequency"]
    DIVIDE_140 = 0x8b,
    #[doc = "mclk frequency = 1/141 * hmclk frequency"]
    DIVIDE_141 = 0x8c,
    #[doc = "mclk frequency = 1/142 * hmclk frequency"]
    DIVIDE_142 = 0x8d,
    #[doc = "mclk frequency = 1/143 * hmclk frequency"]
    DIVIDE_143 = 0x8e,
    #[doc = "mclk frequency = 1/144 * hmclk frequency"]
    DIVIDE_144 = 0x8f,
    #[doc = "mclk frequency = 1/145 * hmclk frequency"]
    DIVIDE_145 = 0x90,
    #[doc = "mclk frequency = 1/146 * hmclk frequency"]
    DIVIDE_146 = 0x91,
    #[doc = "mclk frequency = 1/147 * hmclk frequency"]
    DIVIDE_147 = 0x92,
    #[doc = "mclk frequency = 1/148 * hmclk frequency"]
    DIVIDE_148 = 0x93,
    #[doc = "mclk frequency = 1/149 * hmclk frequency"]
    DIVIDE_149 = 0x94,
    #[doc = "mclk frequency = 1/150 * hmclk frequency"]
    DIVIDE_150 = 0x95,
    #[doc = "mclk frequency = 1/151 * hmclk frequency"]
    DIVIDE_151 = 0x96,
    #[doc = "mclk frequency = 1/152 * hmclk frequency"]
    DIVIDE_152 = 0x97,
    #[doc = "mclk frequency = 1/153 * hmclk frequency"]
    DIVIDE_153 = 0x98,
    #[doc = "mclk frequency = 1/154 * hmclk frequency"]
    DIVIDE_154 = 0x99,
    #[doc = "mclk frequency = 1/155 * hmclk frequency"]
    DIVIDE_155 = 0x9a,
    #[doc = "mclk frequency = 1/156 * hmclk frequency"]
    DIVIDE_156 = 0x9b,
    #[doc = "mclk frequency = 1/157 * hmclk frequency"]
    DIVIDE_157 = 0x9c,
    #[doc = "mclk frequency = 1/158 * hmclk frequency"]
    DIVIDE_158 = 0x9d,
    #[doc = "mclk frequency = 1/159 * hmclk frequency"]
    DIVIDE_159 = 0x9e,
    #[doc = "mclk frequency = 1/160 * hmclk frequency"]
    DIVIDE_160 = 0x9f,
    #[doc = "mclk frequency = 1/161 * hmclk frequency"]
    DIVIDE_161 = 0xa0,
    #[doc = "mclk frequency = 1/162 * hmclk frequency"]
    DIVIDE_162 = 0xa1,
    #[doc = "mclk frequency = 1/163 * hmclk frequency"]
    DIVIDE_163 = 0xa2,
    #[doc = "mclk frequency = 1/164 * hmclk frequency"]
    DIVIDE_164 = 0xa3,
    #[doc = "mclk frequency = 1/165 * hmclk frequency"]
    DIVIDE_165 = 0xa4,
    #[doc = "mclk frequency = 1/166 * hmclk frequency"]
    DIVIDE_166 = 0xa5,
    #[doc = "mclk frequency = 1/167 * hmclk frequency"]
    DIVIDE_167 = 0xa6,
    #[doc = "mclk frequency = 1/168 * hmclk frequency"]
    DIVIDE_168 = 0xa7,
    #[doc = "mclk frequency = 1/169 * hmclk frequency"]
    DIVIDE_169 = 0xa8,
    #[doc = "mclk frequency = 1/170 * hmclk frequency"]
    DIVIDE_170 = 0xa9,
    #[doc = "mclk frequency = 1/171 * hmclk frequency"]
    DIVIDE_171 = 0xaa,
    #[doc = "mclk frequency = 1/172 * hmclk frequency"]
    DIVIDE_172 = 0xab,
    #[doc = "mclk frequency = 1/173 * hmclk frequency"]
    DIVIDE_173 = 0xac,
    #[doc = "mclk frequency = 1/174 * hmclk frequency"]
    DIVIDE_174 = 0xad,
    #[doc = "mclk frequency = 1/175 * hmclk frequency"]
    DIVIDE_175 = 0xae,
    #[doc = "mclk frequency = 1/176 * hmclk frequency"]
    DIVIDE_176 = 0xaf,
    #[doc = "mclk frequency = 1/177 * hmclk frequency"]
    DIVIDE_177 = 0xb0,
    #[doc = "mclk frequency = 1/178 * hmclk frequency"]
    DIVIDE_178 = 0xb1,
    #[doc = "mclk frequency = 1/179 * hmclk frequency"]
    DIVIDE_179 = 0xb2,
    #[doc = "mclk frequency = 1/180 * hmclk frequency"]
    DIVIDE_180 = 0xb3,
    #[doc = "mclk frequency = 1/181 * hmclk frequency"]
    DIVIDE_181 = 0xb4,
    #[doc = "mclk frequency = 1/182 * hmclk frequency"]
    DIVIDE_182 = 0xb5,
    #[doc = "mclk frequency = 1/183 * hmclk frequency"]
    DIVIDE_183 = 0xb6,
    #[doc = "mclk frequency = 1/184 * hmclk frequency"]
    DIVIDE_184 = 0xb7,
    #[doc = "mclk frequency = 1/185 * hmclk frequency"]
    DIVIDE_185 = 0xb8,
    #[doc = "mclk frequency = 1/186 * hmclk frequency"]
    DIVIDE_186 = 0xb9,
    #[doc = "mclk frequency = 1/187 * hmclk frequency"]
    DIVIDE_187 = 0xba,
    #[doc = "mclk frequency = 1/188 * hmclk frequency"]
    DIVIDE_188 = 0xbb,
    #[doc = "mclk frequency = 1/189 * hmclk frequency"]
    DIVIDE_189 = 0xbc,
    #[doc = "mclk frequency = 1/190 * hmclk frequency"]
    DIVIDE_190 = 0xbd,
    #[doc = "mclk frequency = 1/191 * hmclk frequency"]
    DIVIDE_191 = 0xbe,
    #[doc = "mclk frequency = 1/192 * hmclk frequency"]
    DIVIDE_192 = 0xbf,
    #[doc = "mclk frequency = 1/193 * hmclk frequency"]
    DIVIDE_193 = 0xc0,
    #[doc = "mclk frequency = 1/194 * hmclk frequency"]
    DIVIDE_194 = 0xc1,
    #[doc = "mclk frequency = 1/195 * hmclk frequency"]
    DIVIDE_195 = 0xc2,
    #[doc = "mclk frequency = 1/196 * hmclk frequency"]
    DIVIDE_196 = 0xc3,
    #[doc = "mclk frequency = 1/197 * hmclk frequency"]
    DIVIDE_197 = 0xc4,
    #[doc = "mclk frequency = 1/198 * hmclk frequency"]
    DIVIDE_198 = 0xc5,
    #[doc = "mclk frequency = 1/199 * hmclk frequency"]
    DIVIDE_199 = 0xc6,
    #[doc = "mclk frequency = 1/200 * hmclk frequency"]
    DIVIDE_200 = 0xc7,
    #[doc = "mclk frequency = 1/201 * hmclk frequency"]
    DIVIDE_201 = 0xc8,
    #[doc = "mclk frequency = 1/202 * hmclk frequency"]
    DIVIDE_202 = 0xc9,
    #[doc = "mclk frequency = 1/203 * hmclk frequency"]
    DIVIDE_203 = 0xca,
    #[doc = "mclk frequency = 1/204 * hmclk frequency"]
    DIVIDE_204 = 0xcb,
    #[doc = "mclk frequency = 1/205 * hmclk frequency"]
    DIVIDE_205 = 0xcc,
    #[doc = "mclk frequency = 1/206 * hmclk frequency"]
    DIVIDE_206 = 0xcd,
    #[doc = "mclk frequency = 1/207 * hmclk frequency"]
    DIVIDE_207 = 0xce,
    #[doc = "mclk frequency = 1/208 * hmclk frequency"]
    DIVIDE_208 = 0xcf,
    #[doc = "mclk frequency = 1/209 * hmclk frequency"]
    DIVIDE_209 = 0xd0,
    #[doc = "mclk frequency = 1/210 * hmclk frequency"]
    DIVIDE_210 = 0xd1,
    #[doc = "mclk frequency = 1/211 * hmclk frequency"]
    DIVIDE_211 = 0xd2,
    #[doc = "mclk frequency = 1/212 * hmclk frequency"]
    DIVIDE_212 = 0xd3,
    #[doc = "mclk frequency = 1/213 * hmclk frequency"]
    DIVIDE_213 = 0xd4,
    #[doc = "mclk frequency = 1/214 * hmclk frequency"]
    DIVIDE_214 = 0xd5,
    #[doc = "mclk frequency = 1/215 * hmclk frequency"]
    DIVIDE_215 = 0xd6,
    #[doc = "mclk frequency = 1/216 * hmclk frequency"]
    DIVIDE_216 = 0xd7,
    #[doc = "mclk frequency = 1/217 * hmclk frequency"]
    DIVIDE_217 = 0xd8,
    #[doc = "mclk frequency = 1/218 * hmclk frequency"]
    DIVIDE_218 = 0xd9,
    #[doc = "mclk frequency = 1/219 * hmclk frequency"]
    DIVIDE_219 = 0xda,
    #[doc = "mclk frequency = 1/220 * hmclk frequency"]
    DIVIDE_220 = 0xdb,
    #[doc = "mclk frequency = 1/221 * hmclk frequency"]
    DIVIDE_221 = 0xdc,
    #[doc = "mclk frequency = 1/222 * hmclk frequency"]
    DIVIDE_222 = 0xdd,
    #[doc = "mclk frequency = 1/223 * hmclk frequency"]
    DIVIDE_223 = 0xde,
    #[doc = "mclk frequency = 1/224 * hmclk frequency"]
    DIVIDE_224 = 0xdf,
    #[doc = "mclk frequency = 1/225 * hmclk frequency"]
    DIVIDE_225 = 0xe0,
    #[doc = "mclk frequency = 1/226 * hmclk frequency"]
    DIVIDE_226 = 0xe1,
    #[doc = "mclk frequency = 1/227 * hmclk frequency"]
    DIVIDE_227 = 0xe2,
    #[doc = "mclk frequency = 1/228 * hmclk frequency"]
    DIVIDE_228 = 0xe3,
    #[doc = "mclk frequency = 1/229 * hmclk frequency"]
    DIVIDE_229 = 0xe4,
    #[doc = "mclk frequency = 1/230 * hmclk frequency"]
    DIVIDE_230 = 0xe5,
    #[doc = "mclk frequency = 1/231 * hmclk frequency"]
    DIVIDE_231 = 0xe6,
    #[doc = "mclk frequency = 1/232 * hmclk frequency"]
    DIVIDE_232 = 0xe7,
    #[doc = "mclk frequency = 1/233 * hmclk frequency"]
    DIVIDE_233 = 0xe8,
    #[doc = "mclk frequency = 1/234 * hmclk frequency"]
    DIVIDE_234 = 0xe9,
    #[doc = "mclk frequency = 1/235 * hmclk frequency"]
    DIVIDE_235 = 0xea,
    #[doc = "mclk frequency = 1/236 * hmclk frequency"]
    DIVIDE_236 = 0xeb,
    #[doc = "mclk frequency = 1/237 * hmclk frequency"]
    DIVIDE_237 = 0xec,
    #[doc = "mclk frequency = 1/238 * hmclk frequency"]
    DIVIDE_238 = 0xed,
    #[doc = "mclk frequency = 1/239 * hmclk frequency"]
    DIVIDE_239 = 0xee,
    #[doc = "mclk frequency = 1/240 * hmclk frequency"]
    DIVIDE_240 = 0xef,
    #[doc = "mclk frequency = 1/241 * hmclk frequency"]
    DIVIDE_241 = 0xf0,
    #[doc = "mclk frequency = 1/242 * hmclk frequency"]
    DIVIDE_242 = 0xf1,
    #[doc = "mclk frequency = 1/243 * hmclk frequency"]
    DIVIDE_243 = 0xf2,
    #[doc = "mclk frequency = 1/244 * hmclk frequency"]
    DIVIDE_244 = 0xf3,
    #[doc = "mclk frequency = 1/245 * hmclk frequency"]
    DIVIDE_245 = 0xf4,
    #[doc = "mclk frequency = 1/246 * hmclk frequency"]
    DIVIDE_246 = 0xf5,
    #[doc = "mclk frequency = 1/247 * hmclk frequency"]
    DIVIDE_247 = 0xf6,
    #[doc = "mclk frequency = 1/248 * hmclk frequency"]
    DIVIDE_248 = 0xf7,
    #[doc = "mclk frequency = 1/249 * hmclk frequency"]
    DIVIDE_249 = 0xf8,
    #[doc = "mclk frequency = 1/250 * hmclk frequency"]
    DIVIDE_250 = 0xf9,
    #[doc = "mclk frequency = 1/251 * hmclk frequency"]
    DIVIDE_251 = 0xfa,
    #[doc = "mclk frequency = 1/252 * hmclk frequency"]
    DIVIDE_252 = 0xfb,
    #[doc = "mclk frequency = 1/253 * hmclk frequency"]
    DIVIDE_253 = 0xfc,
    #[doc = "mclk frequency = 1/254 * hmclk frequency"]
    DIVIDE_254 = 0xfd,
    #[doc = "mclk frequency = 1/255 * hmclk frequency"]
    DIVIDE_255 = 0xfe,
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    DIVIDE_256 = 0xff,
}
impl MqsClkDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MqsClkDiv {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MqsClkDiv {
    #[inline(always)]
    fn from(val: u8) -> MqsClkDiv {
        MqsClkDiv::from_bits(val)
    }
}
impl From<MqsClkDiv> for u8 {
    #[inline(always)]
    fn from(val: MqsClkDiv) -> u8 {
        MqsClkDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1mclk1sel {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0 = 0x0,
    #[doc = "ccm.ssi2_clk_root"]
    SAI1_MCLK1_SEL_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_3 = 0x03,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai1mclk1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1mclk1sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1mclk1sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1mclk1sel {
        Sai1mclk1sel::from_bits(val)
    }
}
impl From<Sai1mclk1sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1mclk1sel) -> u8 {
        Sai1mclk1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1mclk2sel {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0 = 0x0,
    #[doc = "ccm.ssi2_clk_root"]
    SAI1_MCLK2_SEL_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_3 = 0x03,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai1mclk2sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1mclk2sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1mclk2sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1mclk2sel {
        Sai1mclk2sel::from_bits(val)
    }
}
impl From<Sai1mclk2sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1mclk2sel) -> u8 {
        Sai1mclk2sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1mclk3sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0 = 0x0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI1_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3 = 0x03,
}
impl Sai1mclk3sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1mclk3sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1mclk3sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1mclk3sel {
        Sai1mclk3sel::from_bits(val)
    }
}
impl From<Sai1mclk3sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1mclk3sel) -> u8 {
        Sai1mclk3sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2mclk3sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI2_MCLK3_SEL_0 = 0x0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI2_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI2_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI2_MCLK3_SEL_3 = 0x03,
}
impl Sai2mclk3sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2mclk3sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2mclk3sel {
    #[inline(always)]
    fn from(val: u8) -> Sai2mclk3sel {
        Sai2mclk3sel::from_bits(val)
    }
}
impl From<Sai2mclk3sel> for u8 {
    #[inline(always)]
    fn from(val: Sai2mclk3sel) -> u8 {
        Sai2mclk3sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3mclk3sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0 = 0x0,
    #[doc = "iomux.spdif_tx_clk2"]
    SAI3_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3 = 0x03,
}
impl Sai3mclk3sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3mclk3sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3mclk3sel {
    #[inline(always)]
    fn from(val: u8) -> Sai3mclk3sel {
        Sai3mclk3sel::from_bits(val)
    }
}
impl From<Sai3mclk3sel> for u8 {
    #[inline(always)]
    fn from(val: Sai3mclk3sel) -> u8 {
        Sai3mclk3sel::to_bits(val)
    }
}
