#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigOpt(pub u8);
impl ConfigOpt {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    pub const CONFIG_OPT_VAL: Self = Self(0);
}
impl ConfigOpt {
    pub const fn from_bits(val: u8) -> ConfigOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ConfigOpt {
    #[inline(always)]
    fn from(val: u8) -> ConfigOpt {
        ConfigOpt::from_bits(val)
    }
}
impl From<ConfigOpt> for u8 {
    #[inline(always)]
    fn from(val: ConfigOpt) -> u8 {
        ConfigOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EcoRev(pub u8);
impl EcoRev {
    #[doc = "TRNG_ECO_REV for TRNG."]
    pub const ECO_REV_VAL: Self = Self(0);
}
impl EcoRev {
    pub const fn from_bits(val: u8) -> EcoRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for EcoRev {
    #[inline(always)]
    fn from(val: u8) -> EcoRev {
        EcoRev::from_bits(val)
    }
}
impl From<EcoRev> for u8 {
    #[inline(always)]
    fn from(val: EcoRev) -> u8 {
        EcoRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Era(pub u8);
impl Era {
    #[doc = "COMPILE_OPT for TRNG."]
    pub const ERA_VAL: Self = Self(0);
}
impl Era {
    pub const fn from_bits(val: u8) -> Era {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Era {
    #[inline(always)]
    fn from(val: u8) -> Era {
        Era::from_bits(val)
    }
}
impl From<Era> for u8 {
    #[inline(always)]
    fn from(val: Era) -> u8 {
        Era::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntgOpt(pub u8);
impl IntgOpt {
    #[doc = "INTG_OPT for TRNG."]
    pub const INTG_OPT_VAL: Self = Self(0);
}
impl IntgOpt {
    pub const fn from_bits(val: u8) -> IntgOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for IntgOpt {
    #[inline(always)]
    fn from(val: u8) -> IntgOpt {
        IntgOpt::from_bits(val)
    }
}
impl From<IntgOpt> for u8 {
    #[inline(always)]
    fn from(val: IntgOpt) -> u8 {
        IntgOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IpId(pub u16);
impl IpId {
    #[doc = "ID for TRNG."]
    pub const IP_ID_VAL: Self = Self(0x30);
}
impl IpId {
    pub const fn from_bits(val: u16) -> IpId {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for IpId {
    #[inline(always)]
    fn from(val: u16) -> IpId {
        IpId::from_bits(val)
    }
}
impl From<IpId> for u16 {
    #[inline(always)]
    fn from(val: IpId) -> u16 {
        IpId::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MajRev(pub u8);
impl MajRev {
    #[doc = "Major revision number for TRNG."]
    pub const MAJ_REV_VAL: Self = Self(0x01);
}
impl MajRev {
    pub const fn from_bits(val: u8) -> MajRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for MajRev {
    #[inline(always)]
    fn from(val: u8) -> MajRev {
        MajRev::from_bits(val)
    }
}
impl From<MajRev> for u8 {
    #[inline(always)]
    fn from(val: MajRev) -> u8 {
        MajRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinRev(pub u8);
impl MinRev {
    #[doc = "Minor revision number for TRNG."]
    pub const MIN_REV_VAL: Self = Self(0);
}
impl MinRev {
    pub const fn from_bits(val: u8) -> MinRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for MinRev {
    #[inline(always)]
    fn from(val: u8) -> MinRev {
        MinRev::from_bits(val)
    }
}
impl From<MinRev> for u8 {
    #[inline(always)]
    fn from(val: MinRev) -> u8 {
        MinRev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum OscDiv {
    #[doc = "use ring oscillator with no divide"]
    OSC_DIV_DIV1 = 0,
    #[doc = "use ring oscillator divided-by-2"]
    OSC_DIV_DIV2 = 0x01,
    #[doc = "use ring oscillator divided-by-4"]
    OSC_DIV_DIV4 = 0x02,
    #[doc = "use ring oscillator divided-by-8"]
    OSC_DIV_DIV8 = 0x03,
}
impl OscDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscDiv {
    #[inline(always)]
    fn from(val: u8) -> OscDiv {
        OscDiv::from_bits(val)
    }
}
impl From<OscDiv> for u8 {
    #[inline(always)]
    fn from(val: OscDiv) -> u8 {
        OscDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SampMode {
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_VON_BOTH = 0,
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_RAW_BOTH = 0x01,
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SAMP_MODE_VON_ENT = 0x02,
    #[doc = "undefined/reserved."]
    SAMP_MODE_UNDEF = 0x03,
}
impl SampMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SampMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SampMode {
    #[inline(always)]
    fn from(val: u8) -> SampMode {
        SampMode::from_bits(val)
    }
}
impl From<SampMode> for u8 {
    #[inline(always)]
    fn from(val: SampMode) -> u8 {
        SampMode::to_bits(val)
    }
}
