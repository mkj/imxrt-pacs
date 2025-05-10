#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeGlitch {
    #[doc = "Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles"]
    DE_GLITCH_0 = 0x0,
    #[doc = "Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles"]
    DE_GLITCH_1 = 0x01,
    #[doc = "Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles"]
    DE_GLITCH_2 = 0x02,
    #[doc = "Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles"]
    DE_GLITCH_3 = 0x03,
}
impl DeGlitch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeGlitch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeGlitch {
    #[inline(always)]
    fn from(val: u8) -> DeGlitch {
        DeGlitch::from_bits(val)
    }
}
impl From<DeGlitch> for u8 {
    #[inline(always)]
    fn from(val: DeGlitch) -> u8 {
        DeGlitch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StateMachine {
    #[doc = "Idle"]
    STATE_MACHINE_0 = 0x0,
    #[doc = "Pre-charge"]
    STATE_MACHINE_1 = 0x01,
    #[doc = "Detect"]
    STATE_MACHINE_2 = 0x02,
    #[doc = "X-measure"]
    STATE_MACHINE_3 = 0x03,
    #[doc = "Y-measure"]
    STATE_MACHINE_4 = 0x04,
    #[doc = "Pre-charge"]
    STATE_MACHINE_5 = 0x05,
    #[doc = "Detect"]
    STATE_MACHINE_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl StateMachine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StateMachine {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StateMachine {
    #[inline(always)]
    fn from(val: u8) -> StateMachine {
        StateMachine::from_bits(val)
    }
}
impl From<StateMachine> for u8 {
    #[inline(always)]
    fn from(val: StateMachine) -> u8 {
        StateMachine::to_bits(val)
    }
}
