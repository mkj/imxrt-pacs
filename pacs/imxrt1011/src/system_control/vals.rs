#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AuxiliaryRegisters {
    #[doc = "Not supported"]
    AUXILIARY_REGISTERS_0 = 0,
    #[doc = "Support for Auxiliary Control Register only."]
    AUXILIARY_REGISTERS_1 = 0x01,
    #[doc = "ARMv7-M unused"]
    AUXILIARY_REGISTERS_2 = 0x02,
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
}
impl AuxiliaryRegisters {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AuxiliaryRegisters {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AuxiliaryRegisters {
    #[inline(always)]
    fn from(val: u8) -> AuxiliaryRegisters {
        AuxiliaryRegisters::from_bits(val)
    }
}
impl From<AuxiliaryRegisters> for u8 {
    #[inline(always)]
    fn from(val: AuxiliaryRegisters) -> u8 {
        AuxiliaryRegisters::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Aximtype {
    #[doc = "OKAY."]
    AXIMTYPE_0 = 0,
    #[doc = "EXOKAY."]
    AXIMTYPE_1 = 0x01,
    #[doc = "SLVERR."]
    AXIMTYPE_2 = 0x02,
    #[doc = "DECERR."]
    AXIMTYPE_3 = 0x03,
}
impl Aximtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aximtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aximtype {
    #[inline(always)]
    fn from(val: u8) -> Aximtype {
        Aximtype::from_bits(val)
    }
}
impl From<Aximtype> for u8 {
    #[inline(always)]
    fn from(val: Aximtype) -> u8 {
        Aximtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BarrierInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    BARRIER_INSTRS_0 = 0,
    #[doc = "Adds support for the DMB, DSB, and ISB barrier instructions."]
    BARRIER_INSTRS_1 = 0x01,
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
}
impl BarrierInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BarrierInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BarrierInstrs {
    #[inline(always)]
    fn from(val: u8) -> BarrierInstrs {
        BarrierInstrs::from_bits(val)
    }
}
impl From<BarrierInstrs> for u8 {
    #[inline(always)]
    fn from(val: BarrierInstrs) -> u8 {
        BarrierInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BitcountInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    BITCOUNT_INSTRS_0 = 0,
    #[doc = "Adds support for the CLZ instruction"]
    BITCOUNT_INSTRS_1 = 0x01,
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
}
impl BitcountInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BitcountInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BitcountInstrs {
    #[inline(always)]
    fn from(val: u8) -> BitcountInstrs {
        BitcountInstrs::from_bits(val)
    }
}
impl From<BitcountInstrs> for u8 {
    #[inline(always)]
    fn from(val: BitcountInstrs) -> u8 {
        BitcountInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BitfieldInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    BITFIELD_INSTRS_0 = 0,
    #[doc = "Adds support for the BFC, BFI, SBFX, and UBFX instructions"]
    BITFIELD_INSTRS_1 = 0x01,
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
}
impl BitfieldInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BitfieldInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BitfieldInstrs {
    #[inline(always)]
    fn from(val: u8) -> BitfieldInstrs {
        BitfieldInstrs::from_bits(val)
    }
}
impl From<BitfieldInstrs> for u8 {
    #[inline(always)]
    fn from(val: BitfieldInstrs) -> u8 {
        BitfieldInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl1 {
    #[doc = "No cache"]
    CL1_0 = 0,
    #[doc = "Instruction cache only"]
    CL1_1 = 0x01,
    #[doc = "Data cache only"]
    CL1_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL1_3 = 0x03,
    #[doc = "Unified cache"]
    CL1_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl1 {
    #[inline(always)]
    fn from(val: u8) -> Cl1 {
        Cl1::from_bits(val)
    }
}
impl From<Cl1> for u8 {
    #[inline(always)]
    fn from(val: Cl1) -> u8 {
        Cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl2 {
    #[doc = "No cache"]
    CL2_0 = 0,
    #[doc = "Instruction cache only"]
    CL2_1 = 0x01,
    #[doc = "Data cache only"]
    CL2_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL2_3 = 0x03,
    #[doc = "Unified cache"]
    CL2_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl2 {
    #[inline(always)]
    fn from(val: u8) -> Cl2 {
        Cl2::from_bits(val)
    }
}
impl From<Cl2> for u8 {
    #[inline(always)]
    fn from(val: Cl2) -> u8 {
        Cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl3 {
    #[doc = "No cache"]
    CL3_0 = 0,
    #[doc = "Instruction cache only"]
    CL3_1 = 0x01,
    #[doc = "Data cache only"]
    CL3_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL3_3 = 0x03,
    #[doc = "Unified cache"]
    CL3_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl3 {
    #[inline(always)]
    fn from(val: u8) -> Cl3 {
        Cl3::from_bits(val)
    }
}
impl From<Cl3> for u8 {
    #[inline(always)]
    fn from(val: Cl3) -> u8 {
        Cl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl4 {
    #[doc = "No cache"]
    CL4_0 = 0,
    #[doc = "Instruction cache only"]
    CL4_1 = 0x01,
    #[doc = "Data cache only"]
    CL4_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL4_3 = 0x03,
    #[doc = "Unified cache"]
    CL4_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl4 {
    #[inline(always)]
    fn from(val: u8) -> Cl4 {
        Cl4::from_bits(val)
    }
}
impl From<Cl4> for u8 {
    #[inline(always)]
    fn from(val: Cl4) -> u8 {
        Cl4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl5 {
    #[doc = "No cache"]
    CL5_0 = 0,
    #[doc = "Instruction cache only"]
    CL5_1 = 0x01,
    #[doc = "Data cache only"]
    CL5_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL5_3 = 0x03,
    #[doc = "Unified cache"]
    CL5_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl5 {
    #[inline(always)]
    fn from(val: u8) -> Cl5 {
        Cl5::from_bits(val)
    }
}
impl From<Cl5> for u8 {
    #[inline(always)]
    fn from(val: Cl5) -> u8 {
        Cl5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl6 {
    #[doc = "No cache"]
    CL6_0 = 0,
    #[doc = "Instruction cache only"]
    CL6_1 = 0x01,
    #[doc = "Data cache only"]
    CL6_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL6_3 = 0x03,
    #[doc = "Unified cache"]
    CL6_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl6 {
    #[inline(always)]
    fn from(val: u8) -> Cl6 {
        Cl6::from_bits(val)
    }
}
impl From<Cl6> for u8 {
    #[inline(always)]
    fn from(val: Cl6) -> u8 {
        Cl6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cl7 {
    #[doc = "No cache"]
    CL7_0 = 0,
    #[doc = "Instruction cache only"]
    CL7_1 = 0x01,
    #[doc = "Data cache only"]
    CL7_2 = 0x02,
    #[doc = "Separate instruction and data caches"]
    CL7_3 = 0x03,
    #[doc = "Unified cache"]
    CL7_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cl7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl7 {
    #[inline(always)]
    fn from(val: u8) -> Cl7 {
        Cl7::from_bits(val)
    }
}
impl From<Cl7> for u8 {
    #[inline(always)]
    fn from(val: Cl7) -> u8 {
        Cl7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cm7ahbpcrSz {
    #[doc = "0MB. AHBP disabled."]
    SZ_0 = 0,
    #[doc = "64MB."]
    SZ_1 = 0x01,
    #[doc = "128MB."]
    SZ_2 = 0x02,
    #[doc = "256MB."]
    SZ_3 = 0x03,
    #[doc = "512MB."]
    SZ_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cm7ahbpcrSz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm7ahbpcrSz {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm7ahbpcrSz {
    #[inline(always)]
    fn from(val: u8) -> Cm7ahbpcrSz {
        Cm7ahbpcrSz::from_bits(val)
    }
}
impl From<Cm7ahbpcrSz> for u8 {
    #[inline(always)]
    fn from(val: Cm7ahbpcrSz) -> u8 {
        Cm7ahbpcrSz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cm7dtcmcrSz {
    #[doc = "No TCM implemented."]
    SZ_0 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4KB."]
    SZ_3 = 0x03,
    #[doc = "8KB."]
    SZ_4 = 0x04,
    #[doc = "16KB."]
    SZ_5 = 0x05,
    #[doc = "32KB."]
    SZ_6 = 0x06,
    #[doc = "64KB."]
    SZ_7 = 0x07,
    #[doc = "128KB."]
    SZ_8 = 0x08,
    #[doc = "256KB."]
    SZ_9 = 0x09,
    #[doc = "512KB."]
    SZ_10 = 0x0a,
    #[doc = "1MB."]
    SZ_11 = 0x0b,
    #[doc = "2MB."]
    SZ_12 = 0x0c,
    #[doc = "4MB."]
    SZ_13 = 0x0d,
    #[doc = "8MB."]
    SZ_14 = 0x0e,
    #[doc = "16MB."]
    SZ_15 = 0x0f,
}
impl Cm7dtcmcrSz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm7dtcmcrSz {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm7dtcmcrSz {
    #[inline(always)]
    fn from(val: u8) -> Cm7dtcmcrSz {
        Cm7dtcmcrSz::from_bits(val)
    }
}
impl From<Cm7dtcmcrSz> for u8 {
    #[inline(always)]
    fn from(val: Cm7dtcmcrSz) -> u8 {
        Cm7dtcmcrSz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cm7itcmcrSz {
    #[doc = "No TCM implemented."]
    SZ_0 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4KB."]
    SZ_3 = 0x03,
    #[doc = "8KB."]
    SZ_4 = 0x04,
    #[doc = "16KB."]
    SZ_5 = 0x05,
    #[doc = "32KB."]
    SZ_6 = 0x06,
    #[doc = "64KB."]
    SZ_7 = 0x07,
    #[doc = "128KB."]
    SZ_8 = 0x08,
    #[doc = "256KB."]
    SZ_9 = 0x09,
    #[doc = "512KB."]
    SZ_10 = 0x0a,
    #[doc = "1MB."]
    SZ_11 = 0x0b,
    #[doc = "2MB."]
    SZ_12 = 0x0c,
    #[doc = "4MB."]
    SZ_13 = 0x0d,
    #[doc = "8MB."]
    SZ_14 = 0x0e,
    #[doc = "16MB."]
    SZ_15 = 0x0f,
}
impl Cm7itcmcrSz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm7itcmcrSz {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm7itcmcrSz {
    #[inline(always)]
    fn from(val: u8) -> Cm7itcmcrSz {
        Cm7itcmcrSz::from_bits(val)
    }
}
impl From<Cm7itcmcrSz> for u8 {
    #[inline(always)]
    fn from(val: Cm7itcmcrSz) -> u8 {
        Cm7itcmcrSz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CmpbranchInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    CMPBRANCH_INSTRS_0 = 0,
    #[doc = "Adds support for the CBNZ and CBZ instructions"]
    CMPBRANCH_INSTRS_1 = 0x01,
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
}
impl CmpbranchInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpbranchInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpbranchInstrs {
    #[inline(always)]
    fn from(val: u8) -> CmpbranchInstrs {
        CmpbranchInstrs::from_bits(val)
    }
}
impl From<CmpbranchInstrs> for u8 {
    #[inline(always)]
    fn from(val: CmpbranchInstrs) -> u8 {
        CmpbranchInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CoprocInstrs {
    #[doc = "None supported, except for separately attributed architectures, for example the Floating-point extension"]
    COPROC_INSTRS_0 = 0,
    #[doc = "Adds support for generic CDP, LDC, MCR, MRC, and STC instructions"]
    COPROC_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for generic CDP2, LDC2, MCR2, MRC2, and STC2 instructions"]
    COPROC_INSTRS_2 = 0x02,
    #[doc = "As for 2, and adds support for generic MCRR and MRRC instructions"]
    COPROC_INSTRS_3 = 0x03,
    #[doc = "As for 3, and adds support for generic MCRR2 and MRRC2 instructions"]
    COPROC_INSTRS_4 = 0x04,
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
impl CoprocInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoprocInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoprocInstrs {
    #[inline(always)]
    fn from(val: u8) -> CoprocInstrs {
        CoprocInstrs::from_bits(val)
    }
}
impl From<CoprocInstrs> for u8 {
    #[inline(always)]
    fn from(val: CoprocInstrs) -> u8 {
        CoprocInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp0 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP0_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP0_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP0_3 = 0x03,
}
impl Cp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp0 {
    #[inline(always)]
    fn from(val: u8) -> Cp0 {
        Cp0::from_bits(val)
    }
}
impl From<Cp0> for u8 {
    #[inline(always)]
    fn from(val: Cp0) -> u8 {
        Cp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp1 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP1_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP1_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP1_3 = 0x03,
}
impl Cp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp1 {
    #[inline(always)]
    fn from(val: u8) -> Cp1 {
        Cp1::from_bits(val)
    }
}
impl From<Cp1> for u8 {
    #[inline(always)]
    fn from(val: Cp1) -> u8 {
        Cp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp10 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP10_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP10_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP10_3 = 0x03,
}
impl Cp10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp10 {
    #[inline(always)]
    fn from(val: u8) -> Cp10 {
        Cp10::from_bits(val)
    }
}
impl From<Cp10> for u8 {
    #[inline(always)]
    fn from(val: Cp10) -> u8 {
        Cp10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp11 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP11_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP11_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP11_3 = 0x03,
}
impl Cp11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp11 {
    #[inline(always)]
    fn from(val: u8) -> Cp11 {
        Cp11::from_bits(val)
    }
}
impl From<Cp11> for u8 {
    #[inline(always)]
    fn from(val: Cp11) -> u8 {
        Cp11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp2 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP2_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP2_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP2_3 = 0x03,
}
impl Cp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp2 {
    #[inline(always)]
    fn from(val: u8) -> Cp2 {
        Cp2::from_bits(val)
    }
}
impl From<Cp2> for u8 {
    #[inline(always)]
    fn from(val: Cp2) -> u8 {
        Cp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp3 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP3_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP3_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP3_3 = 0x03,
}
impl Cp3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp3 {
    #[inline(always)]
    fn from(val: u8) -> Cp3 {
        Cp3::from_bits(val)
    }
}
impl From<Cp3> for u8 {
    #[inline(always)]
    fn from(val: Cp3) -> u8 {
        Cp3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp4 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP4_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP4_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP4_3 = 0x03,
}
impl Cp4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp4 {
    #[inline(always)]
    fn from(val: u8) -> Cp4 {
        Cp4::from_bits(val)
    }
}
impl From<Cp4> for u8 {
    #[inline(always)]
    fn from(val: Cp4) -> u8 {
        Cp4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp5 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP5_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP5_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP5_3 = 0x03,
}
impl Cp5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp5 {
    #[inline(always)]
    fn from(val: u8) -> Cp5 {
        Cp5::from_bits(val)
    }
}
impl From<Cp5> for u8 {
    #[inline(always)]
    fn from(val: Cp5) -> u8 {
        Cp5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp6 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP6_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP6_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP6_3 = 0x03,
}
impl Cp6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp6 {
    #[inline(always)]
    fn from(val: u8) -> Cp6 {
        Cp6::from_bits(val)
    }
}
impl From<Cp6> for u8 {
    #[inline(always)]
    fn from(val: Cp6) -> u8 {
        Cp6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cp7 {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    CP7_0 = 0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP7_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access."]
    CP7_3 = 0x03,
}
impl Cp7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp7 {
    #[inline(always)]
    fn from(val: u8) -> Cp7 {
        Cp7::from_bits(val)
    }
}
impl From<Cp7> for u8 {
    #[inline(always)]
    fn from(val: Cp7) -> u8 {
        Cp7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctl {
    #[doc = "AHBS access priority demoted. This is the reset value."]
    CTL_0 = 0,
    #[doc = "Software access priority demoted."]
    CTL_1 = 0x01,
    #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\\[INITCOUNT\\] value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\\[TPRI\\]."]
    CTL_2 = 0x02,
    #[doc = "AHBSPRI signal has control of access priority."]
    CTL_3 = 0x03,
}
impl Ctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctl {
    #[inline(always)]
    fn from(val: u8) -> Ctl {
        Ctl::from_bits(val)
    }
}
impl From<Ctl> for u8 {
    #[inline(always)]
    fn from(val: Ctl) -> u8 {
        Ctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DebugInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    DEBUG_INSTRS_0 = 0,
    #[doc = "Adds support for the BKPT instruction"]
    DEBUG_INSTRS_1 = 0x01,
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
}
impl DebugInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugInstrs {
    #[inline(always)]
    fn from(val: u8) -> DebugInstrs {
        DebugInstrs::from_bits(val)
    }
}
impl From<DebugInstrs> for u8 {
    #[inline(always)]
    fn from(val: DebugInstrs) -> u8 {
        DebugInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Debugmodel {
    #[doc = "Not supported"]
    DEBUGMODEL_0 = 0,
    #[doc = "Support for M profile Debug architecture, with memory-mapped access."]
    DEBUGMODEL_1 = 0x01,
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
}
impl Debugmodel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debugmodel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debugmodel {
    #[inline(always)]
    fn from(val: u8) -> Debugmodel {
        Debugmodel::from_bits(val)
    }
}
impl From<Debugmodel> for u8 {
    #[inline(always)]
    fn from(val: Debugmodel) -> u8 {
        Debugmodel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Disdi(pub u8);
impl Disdi {
    #[doc = "Normal operation."]
    pub const DISDI_0: Self = Self(0);
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
    pub const DISDI_1: Self = Self(0x01);
}
impl Disdi {
    pub const fn from_bits(val: u8) -> Disdi {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Disdi {
    #[inline(always)]
    fn from(val: u8) -> Disdi {
        Disdi::from_bits(val)
    }
}
impl From<Disdi> for u8 {
    #[inline(always)]
    fn from(val: Disdi) -> u8 {
        Disdi::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Disissch1(pub u8);
impl Disissch1 {
    #[doc = "Normal operation."]
    pub const DISISSCH1_0: Self = Self(0);
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
    pub const DISISSCH1_1: Self = Self(0x01);
}
impl Disissch1 {
    pub const fn from_bits(val: u8) -> Disissch1 {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Disissch1 {
    #[inline(always)]
    fn from(val: u8) -> Disissch1 {
        Disissch1::from_bits(val)
    }
}
impl From<Disissch1> for u8 {
    #[inline(always)]
    fn from(val: Disissch1) -> u8 {
        Disissch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DivideInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    DIVIDE_INSTRS_0 = 0,
    #[doc = "Adds support for the SDIV and UDIV instructions"]
    DIVIDE_INSTRS_1 = 0x01,
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
}
impl DivideInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DivideInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DivideInstrs {
    #[inline(always)]
    fn from(val: u8) -> DivideInstrs {
        DivideInstrs::from_bits(val)
    }
}
impl From<DivideInstrs> for u8 {
    #[inline(always)]
    fn from(val: DivideInstrs) -> u8 {
        DivideInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtendInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    EXTEND_INSTRS_0 = 0,
    #[doc = "Adds support for the SXTB, SXTH, UXTB, and UXTH instructions"]
    EXTEND_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the SXTAB, SXTAB16, SXTAH, SXTB16, UXTAB, UXTAB16, UXTAH, and UXTB16 instructions"]
    EXTEND_INSTRS_2 = 0x02,
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
}
impl ExtendInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtendInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtendInstrs {
    #[inline(always)]
    fn from(val: u8) -> ExtendInstrs {
        ExtendInstrs::from_bits(val)
    }
}
impl From<ExtendInstrs> for u8 {
    #[inline(always)]
    fn from(val: ExtendInstrs) -> u8 {
        ExtendInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Format {
    _RESERVED_0 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "ARMv7 format."]
    FORMAT_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Format {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Format {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Format {
    #[inline(always)]
    fn from(val: u8) -> Format {
        Format::from_bits(val)
    }
}
impl From<Format> for u8 {
    #[inline(always)]
    fn from(val: Format) -> u8 {
        Format::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum IfthenInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    IFTHEN_INSTRS_0 = 0,
    #[doc = "Adds support for the IT instructions, and for the IT bits in the PSRs"]
    IFTHEN_INSTRS_1 = 0x01,
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
}
impl IfthenInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfthenInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfthenInstrs {
    #[inline(always)]
    fn from(val: u8) -> IfthenInstrs {
        IfthenInstrs::from_bits(val)
    }
}
impl From<IfthenInstrs> for u8 {
    #[inline(always)]
    fn from(val: IfthenInstrs) -> u8 {
        IfthenInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ImmediateInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    IMMEDIATE_INSTRS_0 = 0,
    #[doc = "Adds support for the ADDW, MOVW, MOVT, and SUBW instructions"]
    IMMEDIATE_INSTRS_1 = 0x01,
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
}
impl ImmediateInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ImmediateInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ImmediateInstrs {
    #[inline(always)]
    fn from(val: u8) -> ImmediateInstrs {
        ImmediateInstrs::from_bits(val)
    }
}
impl From<ImmediateInstrs> for u8 {
    #[inline(always)]
    fn from(val: ImmediateInstrs) -> u8 {
        ImmediateInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum InterworkInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    INTERWORK_INSTRS_0 = 0,
    #[doc = "Adds support for the BX instruction, and the T bit in the PSR"]
    INTERWORK_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the BLX instruction, and PC loads have BX-like behavior"]
    INTERWORK_INSTRS_2 = 0x02,
    #[doc = "ARMv7-M unused"]
    INTERWORK_INSTRS_3 = 0x03,
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
impl InterworkInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InterworkInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InterworkInstrs {
    #[inline(always)]
    fn from(val: u8) -> InterworkInstrs {
        InterworkInstrs::from_bits(val)
    }
}
impl From<InterworkInstrs> for u8 {
    #[inline(always)]
    fn from(val: InterworkInstrs) -> u8 {
        InterworkInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Level {
    #[doc = "Level 1 cache."]
    LEVEL_0 = 0,
    #[doc = "Level 2 cache."]
    LEVEL_1 = 0x01,
    #[doc = "Level 3 cache."]
    LEVEL_2 = 0x02,
    #[doc = "Level 4 cache."]
    LEVEL_3 = 0x03,
    #[doc = "Level 5 cache."]
    LEVEL_4 = 0x04,
    #[doc = "Level 6 cache."]
    LEVEL_5 = 0x05,
    #[doc = "Level 7 cache."]
    LEVEL_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Level {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Level {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Level {
    #[inline(always)]
    fn from(val: u8) -> Level {
        Level::from_bits(val)
    }
}
impl From<Level> for u8 {
    #[inline(always)]
    fn from(val: Level) -> u8 {
        Level::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Linesize {
    #[doc = "The line length of 4 words."]
    LINESIZE_0 = 0,
    #[doc = "The line length of 8 words."]
    LINESIZE_1 = 0x01,
    #[doc = "The line length of 16 words."]
    LINESIZE_2 = 0x02,
    #[doc = "The line length of 32 words."]
    LINESIZE_3 = 0x03,
    #[doc = "The line length of 64 words."]
    LINESIZE_4 = 0x04,
    #[doc = "The line length of 128 words."]
    LINESIZE_5 = 0x05,
    #[doc = "The line length of 256 words."]
    LINESIZE_6 = 0x06,
    #[doc = "The line length of 512 words."]
    LINESIZE_7 = 0x07,
}
impl Linesize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linesize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linesize {
    #[inline(always)]
    fn from(val: u8) -> Linesize {
        Linesize::from_bits(val)
    }
}
impl From<Linesize> for u8 {
    #[inline(always)]
    fn from(val: Linesize) -> u8 {
        Linesize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LoadstoreInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    LOADSTORE_INSTRS_0 = 0,
    #[doc = "Adds support for the LDRD and STRD instructions"]
    LOADSTORE_INSTRS_1 = 0x01,
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
}
impl LoadstoreInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LoadstoreInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LoadstoreInstrs {
    #[inline(always)]
    fn from(val: u8) -> LoadstoreInstrs {
        LoadstoreInstrs::from_bits(val)
    }
}
impl From<LoadstoreInstrs> for u8 {
    #[inline(always)]
    fn from(val: LoadstoreInstrs) -> u8 {
        LoadstoreInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Loc {
    #[doc = "0"]
    LOC_0 = 0,
    #[doc = "1"]
    LOC_1 = 0x01,
    #[doc = "2"]
    LOC_2 = 0x02,
    #[doc = "3"]
    LOC_3 = 0x03,
    #[doc = "4"]
    LOC_4 = 0x04,
    #[doc = "5"]
    LOC_5 = 0x05,
    #[doc = "6"]
    LOC_6 = 0x06,
    #[doc = "7"]
    LOC_7 = 0x07,
}
impl Loc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loc {
    #[inline(always)]
    fn from(val: u8) -> Loc {
        Loc::from_bits(val)
    }
}
impl From<Loc> for u8 {
    #[inline(always)]
    fn from(val: Loc) -> u8 {
        Loc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lou {
    #[doc = "0"]
    LOU_0 = 0,
    #[doc = "1"]
    LOU_1 = 0x01,
    #[doc = "2"]
    LOU_2 = 0x02,
    #[doc = "3"]
    LOU_3 = 0x03,
    #[doc = "4"]
    LOU_4 = 0x04,
    #[doc = "5"]
    LOU_5 = 0x05,
    #[doc = "6"]
    LOU_6 = 0x06,
    #[doc = "7"]
    LOU_7 = 0x07,
}
impl Lou {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lou {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lou {
    #[inline(always)]
    fn from(val: u8) -> Lou {
        Lou::from_bits(val)
    }
}
impl From<Lou> for u8 {
    #[inline(always)]
    fn from(val: Lou) -> u8 {
        Lou::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Louis {
    #[doc = "0"]
    LOUIS_0 = 0,
    #[doc = "1"]
    LOUIS_1 = 0x01,
    #[doc = "2"]
    LOUIS_2 = 0x02,
    #[doc = "3"]
    LOUIS_3 = 0x03,
    #[doc = "4"]
    LOUIS_4 = 0x04,
    #[doc = "5"]
    LOUIS_5 = 0x05,
    #[doc = "6"]
    LOUIS_6 = 0x06,
    #[doc = "7"]
    LOUIS_7 = 0x07,
}
impl Louis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Louis {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Louis {
    #[inline(always)]
    fn from(val: u8) -> Louis {
        Louis::from_bits(val)
    }
}
impl From<Louis> for u8 {
    #[inline(always)]
    fn from(val: Louis) -> u8 {
        Louis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MemhintInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    MEMHINT_INSTRS_0 = 0,
    #[doc = "Adds support for the PLD instruction, ARMv7-M unused."]
    MEMHINT_INSTRS_1 = 0x01,
    #[doc = "As for 1, ARMv7-M unused."]
    MEMHINT_INSTRS_2 = 0x02,
    #[doc = "As for 1 or 2, and adds support for the PLI instruction."]
    MEMHINT_INSTRS_3 = 0x03,
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
impl MemhintInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MemhintInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MemhintInstrs {
    #[inline(always)]
    fn from(val: u8) -> MemhintInstrs {
        MemhintInstrs::from_bits(val)
    }
}
impl From<MemhintInstrs> for u8 {
    #[inline(always)]
    fn from(val: MemhintInstrs) -> u8 {
        MemhintInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultInstrs {
    #[doc = "None supported. This means only MUL is supported. ARMv7-M unused."]
    MULT_INSTRS_0 = 0,
    #[doc = "Adds support for the MLA instruction, ARMv7-M unused."]
    MULT_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the MLS instruction."]
    MULT_INSTRS_2 = 0x02,
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
}
impl MultInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultInstrs {
    #[inline(always)]
    fn from(val: u8) -> MultInstrs {
        MultInstrs::from_bits(val)
    }
}
impl From<MultInstrs> for u8 {
    #[inline(always)]
    fn from(val: MultInstrs) -> u8 {
        MultInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultiaccessintInstrs {
    #[doc = "None supported. This means the LDM and STM instructions are not interruptible. ARMv7-M unused."]
    MULTIACCESSINT_INSTRS_0 = 0,
    #[doc = "LDM and STM instructions are restartable."]
    MULTIACCESSINT_INSTRS_1 = 0x01,
    #[doc = "LDM and STM instructions are continuable."]
    MULTIACCESSINT_INSTRS_2 = 0x02,
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
}
impl MultiaccessintInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiaccessintInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiaccessintInstrs {
    #[inline(always)]
    fn from(val: u8) -> MultiaccessintInstrs {
        MultiaccessintInstrs::from_bits(val)
    }
}
impl From<MultiaccessintInstrs> for u8 {
    #[inline(always)]
    fn from(val: MultiaccessintInstrs) -> u8 {
        MultiaccessintInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultsInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    MULTS_INSTRS_0 = 0,
    #[doc = "Adds support for the SMULL and SMLAL instructions"]
    MULTS_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the SMLABB, SMLABT, SMLALBB, SMLALBT, SMLALTB, SMLALTT, SMLATB, SMLATT, SMLAWB, SMLAWT, SMULBB, SMULBT, SMULTB, SMULTT, SMULWB, and SMULWT instructions."]
    MULTS_INSTRS_2 = 0x02,
    #[doc = "As for 2, and adds support for the SMLAD, SMLADX, SMLALD, SMLALDX, SMLSD, SMLSDX, SMLSLD, SMLSLDX, SMMLA, SMMLAR, SMMLS, SMMLSR, SMMUL, SMMULR, SMUAD, SMUADX, SMUSD, and SMUSDX instructions."]
    MULTS_INSTRS_3 = 0x03,
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
impl MultsInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultsInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultsInstrs {
    #[inline(always)]
    fn from(val: u8) -> MultsInstrs {
        MultsInstrs::from_bits(val)
    }
}
impl From<MultsInstrs> for u8 {
    #[inline(always)]
    fn from(val: MultsInstrs) -> u8 {
        MultsInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultuInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    MULTU_INSTRS_0 = 0,
    #[doc = "Adds support for the UMULL and UMLAL instructions."]
    MULTU_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the UMAAL instruction."]
    MULTU_INSTRS_2 = 0x02,
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
}
impl MultuInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultuInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultuInstrs {
    #[inline(always)]
    fn from(val: u8) -> MultuInstrs {
        MultuInstrs::from_bits(val)
    }
}
impl From<MultuInstrs> for u8 {
    #[inline(always)]
    fn from(val: MultuInstrs) -> u8 {
        MultuInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutermostShareability {
    #[doc = "Implemented as Non-cacheable"]
    OUTERMOST_SHAREABILITY_0 = 0,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_1 = 0x01,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_2 = 0x02,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_3 = 0x03,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_4 = 0x04,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_5 = 0x05,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_6 = 0x06,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_7 = 0x07,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_8 = 0x08,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_9 = 0x09,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_10 = 0x0a,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_11 = 0x0b,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_12 = 0x0c,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_13 = 0x0d,
    #[doc = "ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_14 = 0x0e,
    #[doc = "Shareability ignored."]
    OUTERMOST_SHAREABILITY_15 = 0x0f,
}
impl OutermostShareability {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutermostShareability {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutermostShareability {
    #[inline(always)]
    fn from(val: u8) -> OutermostShareability {
        OutermostShareability::from_bits(val)
    }
}
impl From<OutermostShareability> for u8 {
    #[inline(always)]
    fn from(val: OutermostShareability) -> u8 {
        OutermostShareability::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pmsasupport {
    #[doc = "Not supported"]
    PMSASUPPORT_0 = 0,
    #[doc = "ARMv7-M unused"]
    PMSASUPPORT_1 = 0x01,
    #[doc = "ARMv7-M unused"]
    PMSASUPPORT_2 = 0x02,
    #[doc = "PMSAv7, providing support for a base region and subregions."]
    PMSASUPPORT_3 = 0x03,
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
impl Pmsasupport {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmsasupport {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmsasupport {
    #[inline(always)]
    fn from(val: u8) -> Pmsasupport {
        Pmsasupport::from_bits(val)
    }
}
impl From<Pmsasupport> for u8 {
    #[inline(always)]
    fn from(val: Pmsasupport) -> u8 {
        Pmsasupport::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Progmodel {
    #[doc = "ARMv7-M unused"]
    PROGMODEL_0 = 0,
    _RESERVED_1 = 0x01,
    #[doc = "Two-stack programmers' model supported"]
    PROGMODEL_2 = 0x02,
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
}
impl Progmodel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Progmodel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Progmodel {
    #[inline(always)]
    fn from(val: u8) -> Progmodel {
        Progmodel::from_bits(val)
    }
}
impl From<Progmodel> for u8 {
    #[inline(always)]
    fn from(val: Progmodel) -> u8 {
        Progmodel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PsrMinstrs {
    #[doc = "None supported, ARMv7-M unused."]
    PSR_M_INSTRS_0 = 0,
    #[doc = "Adds support for the M-profile forms of the CPS, MRS, and MSR instructions, to access the PSRs."]
    PSR_M_INSTRS_1 = 0x01,
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
}
impl PsrMinstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsrMinstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsrMinstrs {
    #[inline(always)]
    fn from(val: u8) -> PsrMinstrs {
        PsrMinstrs::from_bits(val)
    }
}
impl From<PsrMinstrs> for u8 {
    #[inline(always)]
    fn from(val: PsrMinstrs) -> u8 {
        PsrMinstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReversalInstrs {
    #[doc = "None supported, ARMv7-M unused"]
    REVERSAL_INSTRS_0 = 0,
    #[doc = "Adds support for the REV, REV16, and REVSH instructions, ARMv7-M unused."]
    REVERSAL_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the RBIT instruction."]
    REVERSAL_INSTRS_2 = 0x02,
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
}
impl ReversalInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReversalInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReversalInstrs {
    #[inline(always)]
    fn from(val: u8) -> ReversalInstrs {
        ReversalInstrs::from_bits(val)
    }
}
impl From<ReversalInstrs> for u8 {
    #[inline(always)]
    fn from(val: ReversalInstrs) -> u8 {
        ReversalInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SaturateInstrs {
    #[doc = "None supported"]
    SATURATE_INSTRS_0 = 0,
    #[doc = "Adds support for the QADD, QDADD, QDSUB, and QSUB instructions, and for the Q bit in the PSRs."]
    SATURATE_INSTRS_1 = 0x01,
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
}
impl SaturateInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaturateInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaturateInstrs {
    #[inline(always)]
    fn from(val: u8) -> SaturateInstrs {
        SaturateInstrs::from_bits(val)
    }
}
impl From<SaturateInstrs> for u8 {
    #[inline(always)]
    fn from(val: SaturateInstrs) -> u8 {
        SaturateInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ShareabilityLevels {
    #[doc = "One level of shareability implemented"]
    SHAREABILITY_LEVELS_0 = 0,
    #[doc = "ARMv7-M unused"]
    SHAREABILITY_LEVELS_1 = 0x01,
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
}
impl ShareabilityLevels {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShareabilityLevels {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShareabilityLevels {
    #[inline(always)]
    fn from(val: u8) -> ShareabilityLevels {
        ShareabilityLevels::from_bits(val)
    }
}
impl From<ShareabilityLevels> for u8 {
    #[inline(always)]
    fn from(val: ShareabilityLevels) -> u8 {
        ShareabilityLevels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SimdInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    SIMD_INSTRS_0 = 0,
    #[doc = "Adds support for the SSAT and USAT instructions, and for the Q bit in the PSRs."]
    SIMD_INSTRS_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "As for 1, and adds support for the PKHBT, PKHTB, QADD16, QADD8, QASX, QSUB16, QSUB8, QSAX, SADD16, SADD8, SASX, SEL, SHADD16, SHADD8, SHASX, SHSUB16, SHSUB8, SHSAX, SSAT16, SSUB16, SSUB8, SSAX, SXTAB16, SXTB16, UADD16, UADD8, UASX, UHADD16, UHADD8, UHASX, UHSUB16, UHSUB8, UHSAX, UQADD16, UQADD8, UQASX, UQSUB16, UQSUB8, UQSAX, USAD8, USADA8, USAT16, USUB16, USUB8, USAX, UXTAB16, and UXTB16 instructions. Also adds support for the GE\\[3:0\\] bits in the PSRs."]
    SIMD_INSTRS_3 = 0x03,
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
impl SimdInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SimdInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SimdInstrs {
    #[inline(always)]
    fn from(val: u8) -> SimdInstrs {
        SimdInstrs::from_bits(val)
    }
}
impl From<SimdInstrs> for u8 {
    #[inline(always)]
    fn from(val: SimdInstrs) -> u8 {
        SimdInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum State0 {
    #[doc = "ARMv7-M unused"]
    STATE0_0 = 0,
    #[doc = "ARMv7-M unused"]
    STATE0_1 = 0x01,
    #[doc = "ARMv7-M unused"]
    STATE0_2 = 0x02,
    #[doc = "Support for Thumb encoding including Thumb-2 technology, with all basic 16-bit and 32-bit instructions."]
    STATE0_3 = 0x03,
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
impl State0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for State0 {
    #[inline(always)]
    fn from(val: u8) -> State0 {
        State0::from_bits(val)
    }
}
impl From<State0> for u8 {
    #[inline(always)]
    fn from(val: State0) -> u8 {
        State0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum State1 {
    #[doc = "The processor does not support the ARM instruction set."]
    STATE1_0 = 0,
    #[doc = "ARMv7-M unused"]
    STATE1_1 = 0x01,
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
}
impl State1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for State1 {
    #[inline(always)]
    fn from(val: u8) -> State1 {
        State1::from_bits(val)
    }
}
impl From<State1> for u8 {
    #[inline(always)]
    fn from(val: State1) -> u8 {
        State1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SvcInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    SVC_INSTRS_0 = 0,
    #[doc = "Adds support for the SVC instruction."]
    SVC_INSTRS_1 = 0x01,
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
}
impl SvcInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SvcInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SvcInstrs {
    #[inline(always)]
    fn from(val: u8) -> SvcInstrs {
        SvcInstrs::from_bits(val)
    }
}
impl From<SvcInstrs> for u8 {
    #[inline(always)]
    fn from(val: SvcInstrs) -> u8 {
        SvcInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TabbranchInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    TABBRANCH_INSTRS_0 = 0,
    #[doc = "Adds support for the TBB and TBH instructions."]
    TABBRANCH_INSTRS_1 = 0x01,
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
}
impl TabbranchInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TabbranchInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TabbranchInstrs {
    #[inline(always)]
    fn from(val: u8) -> TabbranchInstrs {
        TabbranchInstrs::from_bits(val)
    }
}
impl From<TabbranchInstrs> for u8 {
    #[inline(always)]
    fn from(val: TabbranchInstrs) -> u8 {
        TabbranchInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TcmSupport {
    #[doc = "No tightly coupled memories implemented."]
    TCM_SUPPORT_0 = 0,
    #[doc = "Tightly coupled memories implemented with IMPLEMENTATION DEFINED control."]
    TCM_SUPPORT_1 = 0x01,
    #[doc = "ARMv7-M unused"]
    TCM_SUPPORT_2 = 0x02,
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
}
impl TcmSupport {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcmSupport {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcmSupport {
    #[inline(always)]
    fn from(val: u8) -> TcmSupport {
        TcmSupport::from_bits(val)
    }
}
impl From<TcmSupport> for u8 {
    #[inline(always)]
    fn from(val: TcmSupport) -> u8 {
        TcmSupport::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ThumbcopyInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    THUMBCOPY_INSTRS_0 = 0,
    #[doc = "Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    THUMBCOPY_INSTRS_1 = 0x01,
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
}
impl ThumbcopyInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ThumbcopyInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ThumbcopyInstrs {
    #[inline(always)]
    fn from(val: u8) -> ThumbcopyInstrs {
        ThumbcopyInstrs::from_bits(val)
    }
}
impl From<ThumbcopyInstrs> for u8 {
    #[inline(always)]
    fn from(val: ThumbcopyInstrs) -> u8 {
        ThumbcopyInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TruenopInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    TRUENOP_INSTRS_0 = 0,
    #[doc = "Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    TRUENOP_INSTRS_1 = 0x01,
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
}
impl TruenopInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TruenopInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TruenopInstrs {
    #[inline(always)]
    fn from(val: u8) -> TruenopInstrs {
        TruenopInstrs::from_bits(val)
    }
}
impl From<TruenopInstrs> for u8 {
    #[inline(always)]
    fn from(val: TruenopInstrs) -> u8 {
        TruenopInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UnprivInstrs {
    #[doc = "None supported, ARMv7-M unused."]
    UNPRIV_INSTRS_0 = 0,
    #[doc = "Adds support for the LDRBT, LDRT, STRBT, and STRT instructions."]
    UNPRIV_INSTRS_1 = 0x01,
    #[doc = "As for 1, and adds support for the LDRHT, LDRSBT, LDRSHT, and STRHT instructions."]
    UNPRIV_INSTRS_2 = 0x02,
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
}
impl UnprivInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnprivInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnprivInstrs {
    #[inline(always)]
    fn from(val: u8) -> UnprivInstrs {
        UnprivInstrs::from_bits(val)
    }
}
impl From<UnprivInstrs> for u8 {
    #[inline(always)]
    fn from(val: UnprivInstrs) -> u8 {
        UnprivInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WfiStall {
    #[doc = "Not supported"]
    WFI_STALL_0 = 0,
    #[doc = "Support for WFI stalling"]
    WFI_STALL_1 = 0x01,
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
}
impl WfiStall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WfiStall {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WfiStall {
    #[inline(always)]
    fn from(val: u8) -> WfiStall {
        WfiStall::from_bits(val)
    }
}
impl From<WfiStall> for u8 {
    #[inline(always)]
    fn from(val: WfiStall) -> u8 {
        WfiStall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WithshiftsInstrs {
    #[doc = "Nonzero shifts supported only in MOV and shift instructions."]
    WITHSHIFTS_INSTRS_0 = 0,
    #[doc = "Adds support for shifts of loads and stores over the range LSL 0-3."]
    WITHSHIFTS_INSTRS_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "As for 1, and adds support for other constant shift options, on loads, stores, and other instructions."]
    WITHSHIFTS_INSTRS_3 = 0x03,
    #[doc = "ARMv7-M unused."]
    WITHSHIFTS_INSTRS_4 = 0x04,
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
impl WithshiftsInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WithshiftsInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WithshiftsInstrs {
    #[inline(always)]
    fn from(val: u8) -> WithshiftsInstrs {
        WithshiftsInstrs::from_bits(val)
    }
}
impl From<WithshiftsInstrs> for u8 {
    #[inline(always)]
    fn from(val: WithshiftsInstrs) -> u8 {
        WithshiftsInstrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WritebackInstrs {
    #[doc = "Basic support. Only the LDM, STM, PUSH, and POP instructions support writeback addressing modes. ARMv7-M unused."]
    WRITEBACK_INSTRS_0 = 0,
    #[doc = "Adds support for all of the writeback addressing modes defined in the ARMv7-M architecture."]
    WRITEBACK_INSTRS_1 = 0x01,
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
}
impl WritebackInstrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WritebackInstrs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WritebackInstrs {
    #[inline(always)]
    fn from(val: u8) -> WritebackInstrs {
        WritebackInstrs::from_bits(val)
    }
}
impl From<WritebackInstrs> for u8 {
    #[inline(always)]
    fn from(val: WritebackInstrs) -> u8 {
        WritebackInstrs::to_bits(val)
    }
}
