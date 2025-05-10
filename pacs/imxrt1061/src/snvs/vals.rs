#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcalbVal {
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    ADD_0_PER_32768_TICKS = 0x0,
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    ADD_1_PER_32768_TICKS = 0x01,
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    ADD_2_PER_32768_TICKS = 0x02,
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
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    ADD_15_PER_32768_TICKS = 0x0f,
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    SUB_16_PER_32768_TICKS = 0x10,
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    SUB_15_PER_32768_TICKS = 0x11,
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
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    SUB_2_PER_32768_TICKS = 0x1e,
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    SUB_1_PER_32768_TICKS = 0x1f,
}
impl HpcalbVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcalbVal {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcalbVal {
    #[inline(always)]
    fn from(val: u8) -> HpcalbVal {
        HpcalbVal::from_bits(val)
    }
}
impl From<HpcalbVal> for u8 {
    #[inline(always)]
    fn from(val: HpcalbVal) -> u8 {
        HpcalbVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcalbVal {
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    ADD_0_PER_32768_TICKS = 0x0,
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    ADD_1_PER_32768_TICKS = 0x01,
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    ADD_2_PER_32768_TICKS = 0x02,
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
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    ADD_15_PER_32768_TICKS = 0x0f,
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    SUB_16_PER_32768_TICKS = 0x10,
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    SUB_15_PER_32768_TICKS = 0x11,
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
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    SUB_2_PER_32768_TICKS = 0x1e,
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    SUB_1_PER_32768_TICKS = 0x1f,
}
impl LpcalbVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcalbVal {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcalbVal {
    #[inline(always)]
    fn from(val: u8) -> LpcalbVal {
        LpcalbVal::from_bits(val)
    }
}
impl From<LpcalbVal> for u8 {
    #[inline(always)]
    fn from(val: LpcalbVal) -> u8 {
        LpcalbVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpsvCfg {
    #[doc = "LP security violation is disabled"]
    DISABLED = 0x0,
    #[doc = "LP security violation is a non-fatal violation"]
    NON_FATAL = 0x01,
    #[doc = "LP security violation is a fatal violation"]
    FATAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpsvCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpsvCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpsvCfg {
    #[inline(always)]
    fn from(val: u8) -> LpsvCfg {
        LpsvCfg::from_bits(val)
    }
}
impl From<LpsvCfg> for u8 {
    #[inline(always)]
    fn from(val: LpsvCfg) -> u8 {
        LpsvCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterKeySel {
    #[doc = "Select one time programmable master key."]
    SELECT_OTPMK = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select zeroizable master key when MKS_EN bit is set ."]
    SELECT_ZMK = 0x02,
    #[doc = "Select combined master key when MKS_EN bit is set ."]
    SELECT_COMBO = 0x03,
}
impl MasterKeySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterKeySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterKeySel {
    #[inline(always)]
    fn from(val: u8) -> MasterKeySel {
        MasterKeySel::from_bits(val)
    }
}
impl From<MasterKeySel> for u8 {
    #[inline(always)]
    fn from(val: MasterKeySel) -> u8 {
        MasterKeySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiFreq {
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_0 = 0x0,
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_1 = 0x01,
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_2 = 0x02,
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_3 = 0x03,
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_4 = 0x04,
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_5 = 0x05,
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_6 = 0x06,
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_7 = 0x07,
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_8 = 0x08,
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_9 = 0x09,
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_10 = 0x0a,
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_11 = 0x0b,
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_12 = 0x0c,
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_13 = 0x0d,
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_14 = 0x0e,
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_1R5 = 0x0f,
}
impl PiFreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiFreq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiFreq {
    #[inline(always)]
    fn from(val: u8) -> PiFreq {
        PiFreq::from_bits(val)
    }
}
impl From<PiFreq> for u8 {
    #[inline(always)]
    fn from(val: PiFreq) -> u8 {
        PiFreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsmState {
    #[doc = "Init"]
    INIT = 0x0,
    #[doc = "Hard Fail"]
    HARD_FAIL = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Soft Fail"]
    SOFT_FAIL = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)"]
    INTERMEDIATE = 0x08,
    #[doc = "Check"]
    CHECK = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "Non-Secure"]
    NON_SECURE = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "Trusted"]
    TRUSTED = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Secure"]
    SECURE = 0x0f,
}
impl SsmState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsmState {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsmState {
    #[inline(always)]
    fn from(val: u8) -> SsmState {
        SsmState::from_bits(val)
    }
}
impl From<SsmState> for u8 {
    #[inline(always)]
    fn from(val: SsmState) -> u8 {
        SsmState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv5cfg {
    #[doc = "Security Violation 5 is disabled"]
    DISABLED = 0x0,
    #[doc = "Security Violation 5 is a non-fatal violation"]
    NON_FATAL = 0x01,
    #[doc = "Security Violation 5 is a fatal violation"]
    FATAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sv5cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv5cfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv5cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv5cfg {
        Sv5cfg::from_bits(val)
    }
}
impl From<Sv5cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv5cfg) -> u8 {
        Sv5cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysSecurityCfg {
    #[doc = "Fab Configuration - the default configuration of newly fabricated chips"]
    FAB_CONFIG = 0x0,
    #[doc = "Open Configuration - the configuration after NXP-programmable fuses have been blown"]
    OPEN_CONFIG = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Closed Configuration - the configuration after OEM-programmable fuses have been blown"]
    CLOSED_CONFIG = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Field Return Configuration - the configuration of chips that are returned to NXP for analysis"]
    FIELD_RETURN_CONFIG = 0x07,
}
impl SysSecurityCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysSecurityCfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysSecurityCfg {
    #[inline(always)]
    fn from(val: u8) -> SysSecurityCfg {
        SysSecurityCfg::from_bits(val)
    }
}
impl From<SysSecurityCfg> for u8 {
    #[inline(always)]
    fn from(val: SysSecurityCfg) -> u8 {
        SysSecurityCfg::to_bits(val)
    }
}
