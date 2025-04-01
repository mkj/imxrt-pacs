#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LockOcramTzAddr(pub u8);
impl LockOcramTzAddr {
    #[doc = "Field is not locked"]
    pub const LOCK_OCRAM_TZ_ADDR_0: Self = Self(0);
    #[doc = "Field is locked (read access only)"]
    pub const LOCK_OCRAM_TZ_ADDR_1: Self = Self(0x01);
}
impl LockOcramTzAddr {
    pub const fn from_bits(val: u8) -> LockOcramTzAddr {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
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
pub enum M7apcAcR0ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R0_CTRL_0 = 0,
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
pub enum M7apcAcR1ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R1_CTRL_0 = 0,
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
pub enum M7apcAcR2ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R2_CTRL_0 = 0,
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
pub enum M7apcAcR3ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R3_CTRL_0 = 0,
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
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MqsClkDiv(pub u8);
impl MqsClkDiv {
    #[doc = "mclk frequency = hmclk frequency"]
    pub const MQS_CLK_DIV_0: Self = Self(0);
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    pub const MQS_CLK_DIV_1: Self = Self(0x01);
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    pub const MQS_CLK_DIV_2: Self = Self(0x02);
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    pub const MQS_CLK_DIV_255: Self = Self(0xff);
}
impl MqsClkDiv {
    pub const fn from_bits(val: u8) -> MqsClkDiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
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
pub enum Sai1mclk1sel {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0 = 0,
    _RESERVED_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
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
pub enum Sai1mclk2sel {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0 = 0,
    _RESERVED_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
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
pub enum Sai1mclk3sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0 = 0,
    #[doc = "SPDIF_EXT_CLK"]
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
pub enum Sai3mclk3sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0 = 0,
    #[doc = "SPDIF_EXT_CLK"]
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
