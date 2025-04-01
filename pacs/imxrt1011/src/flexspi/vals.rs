#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ahbcmderrcode {
    #[doc = "No error."]
    AHBCMDERRCODE_0 = 0,
    _RESERVED_1 = 0x01,
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence."]
    AHBCMDERRCODE_2 = 0x02,
    #[doc = "There is unknown instruction opcode in the sequence."]
    AHBCMDERRCODE_3 = 0x03,
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    AHBCMDERRCODE_4 = 0x04,
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    AHBCMDERRCODE_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout."]
    AHBCMDERRCODE_14 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ahbcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderrcode {
        Ahbcmderrcode::from_bits(val)
    }
}
impl From<Ahbcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderrcode) -> u8 {
        Ahbcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Arbcmdsrc {
    #[doc = "Triggered by AHB read command (triggered by AHB read)."]
    ARBCMDSRC_0 = 0,
    #[doc = "Triggered by AHB write command (triggered by AHB Write)."]
    ARBCMDSRC_1 = 0x01,
    #[doc = "Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
    ARBCMDSRC_2 = 0x02,
    #[doc = "Triggered by suspended command (resumed)."]
    ARBCMDSRC_3 = 0x03,
}
impl Arbcmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbcmdsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbcmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Arbcmdsrc {
        Arbcmdsrc::from_bits(val)
    }
}
impl From<Arbcmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Arbcmdsrc) -> u8 {
        Arbcmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Awrwaitunit {
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    AWRWAITUNIT_0 = 0,
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    AWRWAITUNIT_1 = 0x01,
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    AWRWAITUNIT_2 = 0x02,
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    AWRWAITUNIT_3 = 0x03,
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    AWRWAITUNIT_4 = 0x04,
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    AWRWAITUNIT_5 = 0x05,
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    AWRWAITUNIT_6 = 0x06,
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    AWRWAITUNIT_7 = 0x07,
}
impl Awrwaitunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Awrwaitunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Awrwaitunit {
    #[inline(always)]
    fn from(val: u8) -> Awrwaitunit {
        Awrwaitunit::from_bits(val)
    }
}
impl From<Awrwaitunit> for u8 {
    #[inline(always)]
    fn from(val: Awrwaitunit) -> u8 {
        Awrwaitunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ipcmderrcode {
    #[doc = "No error."]
    IPCMDERRCODE_0 = 0,
    _RESERVED_1 = 0x01,
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence."]
    IPCMDERRCODE_2 = 0x02,
    #[doc = "There is unknown instruction opcode in the sequence."]
    IPCMDERRCODE_3 = 0x03,
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    IPCMDERRCODE_4 = 0x04,
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    IPCMDERRCODE_5 = 0x05,
    #[doc = "Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    IPCMDERRCODE_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout."]
    IPCMDERRCODE_14 = 0x0e,
    #[doc = "Flash boundary crossed."]
    IPCMDERRCODE_15 = 0x0f,
}
impl Ipcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderrcode {
        Ipcmderrcode::from_bits(val)
    }
}
impl From<Ipcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderrcode) -> u8 {
        Ipcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rxclksrc {
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
    RXCLKSRC_0 = 0,
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
    RXCLKSRC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Flash provided Read strobe and input from DQS pad"]
    RXCLKSRC_3 = 0x03,
}
impl Rxclksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxclksrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxclksrc {
    #[inline(always)]
    fn from(val: u8) -> Rxclksrc {
        Rxclksrc::from_bits(val)
    }
}
impl From<Rxclksrc> for u8 {
    #[inline(always)]
    fn from(val: Rxclksrc) -> u8 {
        Rxclksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Serclkdiv {
    #[doc = "Divided by 1"]
    SERCLKDIV_0 = 0,
    #[doc = "Divided by 2"]
    SERCLKDIV_1 = 0x01,
    #[doc = "Divided by 3"]
    SERCLKDIV_2 = 0x02,
    #[doc = "Divided by 4"]
    SERCLKDIV_3 = 0x03,
    #[doc = "Divided by 5"]
    SERCLKDIV_4 = 0x04,
    #[doc = "Divided by 6"]
    SERCLKDIV_5 = 0x05,
    #[doc = "Divided by 7"]
    SERCLKDIV_6 = 0x06,
    #[doc = "Divided by 8"]
    SERCLKDIV_7 = 0x07,
}
impl Serclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Serclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Serclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Serclkdiv {
        Serclkdiv::from_bits(val)
    }
}
impl From<Serclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Serclkdiv) -> u8 {
        Serclkdiv::to_bits(val)
    }
}
