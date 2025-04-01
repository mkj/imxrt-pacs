#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Captde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Captde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Captde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Captde {
    #[inline(always)]
    fn from(val: u8) -> Captde {
        Captde::from_bits(val)
    }
}
impl From<Captde> for u8 {
    #[inline(always)]
    fn from(val: Captde) -> u8 {
        Captde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel {
    #[inline(always)]
    fn from(val: u8) -> ClkSel {
        ClkSel::from_bits(val)
    }
}
impl From<ClkSel> for u8 {
    #[inline(always)]
    fn from(val: ClkSel) -> u8 {
        ClkSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cmpf(pub u8);
impl Cmpf {
    #[doc = "No compare event has occurred for a particular VALx value."]
    pub const NO_EVENT: Self = Self(0);
    #[doc = "A compare event has occurred for a particular VALx value."]
    pub const EVENT: Self = Self(0x01);
}
impl Cmpf {
    pub const fn from_bits(val: u8) -> Cmpf {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Cmpf {
    #[inline(always)]
    fn from(val: u8) -> Cmpf {
        Cmpf::from_bits(val)
    }
}
impl From<Cmpf> for u8 {
    #[inline(always)]
    fn from(val: Cmpf) -> u8 {
        Cmpf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cmpie(pub u8);
impl Cmpie {
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    pub const DISABLED: Self = Self(0);
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
    pub const ENABLED: Self = Self(0x01);
}
impl Cmpie {
    pub const fn from_bits(val: u8) -> Cmpie {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Cmpie {
    #[inline(always)]
    fn from(val: u8) -> Cmpie {
        Cmpie::from_bits(val)
    }
}
impl From<Cmpie> for u8 {
    #[inline(always)]
    fn from(val: Cmpie) -> u8 {
        Cmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edga0 {
    #[doc = "Disabled"]
    DISABLED = 0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Edga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edga0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edga0 {
    #[inline(always)]
    fn from(val: u8) -> Edga0 {
        Edga0::from_bits(val)
    }
}
impl From<Edga0> for u8 {
    #[inline(always)]
    fn from(val: Edga0) -> u8 {
        Edga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edga1 {
    #[doc = "Disabled"]
    DISABLED = 0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Edga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edga1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edga1 {
    #[inline(always)]
    fn from(val: u8) -> Edga1 {
        Edga1::from_bits(val)
    }
}
impl From<Edga1> for u8 {
    #[inline(always)]
    fn from(val: Edga1) -> u8 {
        Edga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edgb0 {
    #[doc = "Disabled"]
    DISABLED = 0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Edgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edgb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edgb0 {
    #[inline(always)]
    fn from(val: u8) -> Edgb0 {
        Edgb0::from_bits(val)
    }
}
impl From<Edgb0> for u8 {
    #[inline(always)]
    fn from(val: Edgb0) -> u8 {
        Edgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edgb1 {
    #[doc = "Disabled"]
    DISABLED = 0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Edgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edgb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edgb1 {
    #[inline(always)]
    fn from(val: u8) -> Edgb1 {
        Edgb1::from_bits(val)
    }
}
impl From<Edgb1> for u8 {
    #[inline(always)]
    fn from(val: Edgb1) -> u8 {
        Edgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edgx0 {
    #[doc = "Disabled"]
    DISABLED = 0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Edgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edgx0 {
    #[inline(always)]
    fn from(val: u8) -> Edgx0 {
        Edgx0::from_bits(val)
    }
}
impl From<Edgx0> for u8 {
    #[inline(always)]
    fn from(val: Edgx0) -> u8 {
        Edgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edgx1 {
    #[doc = "Disabled"]
    DISABLED = 0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Edgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edgx1 {
    #[inline(always)]
    fn from(val: u8) -> Edgx1 {
        Edgx1::from_bits(val)
    }
}
impl From<Edgx1> for u8 {
    #[inline(always)]
    fn from(val: Edgx1) -> u8 {
        Edgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fauto {
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
    MANUAL = 0,
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
    AUTOMATIC = 0x01,
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
impl Fauto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fauto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fauto {
    #[inline(always)]
    fn from(val: u8) -> Fauto {
        Fauto::from_bits(val)
    }
}
impl From<Fauto> for u8 {
    #[inline(always)]
    fn from(val: Fauto) -> u8 {
        Fauto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fflag {
    #[doc = "No fault on the FAULTx pin."]
    NO_FLAG = 0,
    #[doc = "Fault on the FAULTx pin."]
    FLAG = 0x01,
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
impl Fflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fflag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fflag {
    #[inline(always)]
    fn from(val: u8) -> Fflag {
        Fflag::from_bits(val)
    }
}
impl From<Fflag> for u8 {
    #[inline(always)]
    fn from(val: Fflag) -> u8 {
        Fflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ffull {
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    PWM_OUTPUTS_NOT_REENABLED = 0,
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    PWM_OUTPUTS_REENABLED = 0x01,
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
impl Ffull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ffull {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ffull {
    #[inline(always)]
    fn from(val: u8) -> Ffull {
        Ffull::from_bits(val)
    }
}
impl From<Ffull> for u8 {
    #[inline(always)]
    fn from(val: Ffull) -> u8 {
        Ffull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fhalf {
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    PWM_OUTPUTS_NOT_REENABLED = 0,
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    PWM_OUTPUTS_REENABLED = 0x01,
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
impl Fhalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fhalf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fhalf {
    #[inline(always)]
    fn from(val: u8) -> Fhalf {
        Fhalf::from_bits(val)
    }
}
impl From<Fhalf> for u8 {
    #[inline(always)]
    fn from(val: Fhalf) -> u8 {
        Fhalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fie {
    #[doc = "FAULTx CPU interrupt requests disabled."]
    DISABLED = 0,
    #[doc = "FAULTx CPU interrupt requests enabled."]
    ENABLED = 0x01,
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
impl Fie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fie {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fie {
    #[inline(always)]
    fn from(val: u8) -> Fie {
        Fie::from_bits(val)
    }
}
impl From<Fie> for u8 {
    #[inline(always)]
    fn from(val: Fie) -> u8 {
        Fie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Flvl {
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    LOGIC_0 = 0,
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    LOGIC_1 = 0x01,
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
impl Flvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flvl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flvl {
    #[inline(always)]
    fn from(val: u8) -> Flvl {
        Flvl::from_bits(val)
    }
}
impl From<Flvl> for u8 {
    #[inline(always)]
    fn from(val: Flvl) -> u8 {
        Flvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    MASTER = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 0x07,
}
impl ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceSel {
    #[inline(always)]
    fn from(val: u8) -> ForceSel {
        ForceSel::from_bits(val)
    }
}
impl From<ForceSel> for u8 {
    #[inline(always)]
    fn from(val: ForceSel) -> u8 {
        ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fsafe {
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is set then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    NORMAL = 0,
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
    SAFE = 0x01,
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
impl Fsafe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsafe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsafe {
    #[inline(always)]
    fn from(val: u8) -> Fsafe {
        Fsafe::from_bits(val)
    }
}
impl From<Fsafe> for u8 {
    #[inline(always)]
    fn from(val: Fsafe) -> u8 {
        Fsafe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitSel {
    #[inline(always)]
    fn from(val: u8) -> InitSel {
        InitSel::from_bits(val)
    }
}
impl From<InitSel> for u8 {
    #[inline(always)]
    fn from(val: InitSel) -> u8 {
        InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ipol {
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM23 = 0,
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM45 = 0x01,
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
impl Ipol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipol {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipol {
    #[inline(always)]
    fn from(val: u8) -> Ipol {
        Ipol::from_bits(val)
    }
}
impl From<Ipol> for u8 {
    #[inline(always)]
    fn from(val: Ipol) -> u8 {
        Ipol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ldfq {
    #[doc = "Every PWM opportunity"]
    EVERYPWM = 0,
    #[doc = "Every 2 PWM opportunities"]
    EVERY2PWM = 0x01,
    #[doc = "Every 3 PWM opportunities"]
    EVERY3PWM = 0x02,
    #[doc = "Every 4 PWM opportunities"]
    EVERY4PWM = 0x03,
    #[doc = "Every 5 PWM opportunities"]
    EVERY5PWM = 0x04,
    #[doc = "Every 6 PWM opportunities"]
    EVERY6PWM = 0x05,
    #[doc = "Every 7 PWM opportunities"]
    EVERY7PWM = 0x06,
    #[doc = "Every 8 PWM opportunities"]
    EVERY8PWM = 0x07,
    #[doc = "Every 9 PWM opportunities"]
    EVERY9PWM = 0x08,
    #[doc = "Every 10 PWM opportunities"]
    EVERY10PWM = 0x09,
    #[doc = "Every 11 PWM opportunities"]
    EVERY11PWM = 0x0a,
    #[doc = "Every 12 PWM opportunities"]
    EVERY12PWM = 0x0b,
    #[doc = "Every 13 PWM opportunities"]
    EVERY13PWM = 0x0c,
    #[doc = "Every 14 PWM opportunities"]
    EVERY14PWM = 0x0d,
    #[doc = "Every 15 PWM opportunities"]
    EVERY15PWM = 0x0e,
    #[doc = "Every 16 PWM opportunities"]
    EVERY16PWM = 0x0f,
}
impl Ldfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldfq {
    #[inline(always)]
    fn from(val: u8) -> Ldfq {
        Ldfq::from_bits(val)
    }
}
impl From<Ldfq> for u8 {
    #[inline(always)]
    fn from(val: Ldfq) -> u8 {
        Ldfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ldok {
    #[doc = "Do not load new values."]
    DISABLED = 0,
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    ENABLED = 0x01,
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
impl Ldok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldok {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldok {
    #[inline(always)]
    fn from(val: u8) -> Ldok {
        Ldok::from_bits(val)
    }
}
impl From<Ldok> for u8 {
    #[inline(always)]
    fn from(val: Ldok) -> u8 {
        Ldok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Maska {
    #[doc = "PWM_A output normal."]
    NORMAL = 0,
    #[doc = "PWM_A output masked."]
    MASKED = 0x01,
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
impl Maska {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maska {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maska {
    #[inline(always)]
    fn from(val: u8) -> Maska {
        Maska::from_bits(val)
    }
}
impl From<Maska> for u8 {
    #[inline(always)]
    fn from(val: Maska) -> u8 {
        Maska::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Maskb {
    #[doc = "PWM_B output normal."]
    NORMAL = 0,
    #[doc = "PWM_B output masked."]
    MASKED = 0x01,
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
impl Maskb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maskb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maskb {
    #[inline(always)]
    fn from(val: u8) -> Maskb {
        Maskb::from_bits(val)
    }
}
impl From<Maskb> for u8 {
    #[inline(always)]
    fn from(val: Maskb) -> u8 {
        Maskb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Maskx {
    #[doc = "PWM_X output normal."]
    NORMAL = 0,
    #[doc = "PWM_X output masked."]
    MASKED = 0x01,
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
impl Maskx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maskx {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maskx {
    #[inline(always)]
    fn from(val: u8) -> Maskx {
        Maskx::from_bits(val)
    }
}
impl From<Maskx> for u8 {
    #[inline(always)]
    fn from(val: Maskx) -> u8 {
        Maskx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Monpll {
    #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    NOTLOCKED_DO_NOT_MON_PLL = 0,
    #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    NOTLOCKED_MON_PLL = 0x01,
    #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    LOCKED_DO_NOT_MON_PLL = 0x02,
    #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    LOCKED_MON_PLL = 0x03,
}
impl Monpll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Monpll {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Monpll {
    #[inline(always)]
    fn from(val: u8) -> Monpll {
        Monpll::from_bits(val)
    }
}
impl From<Monpll> for u8 {
    #[inline(always)]
    fn from(val: Monpll) -> u8 {
        Monpll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Nocomb {
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    ENABLED = 0,
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    DISABLED = 0x01,
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
impl Nocomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nocomb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nocomb {
    #[inline(always)]
    fn from(val: u8) -> Nocomb {
        Nocomb::from_bits(val)
    }
}
impl From<Nocomb> for u8 {
    #[inline(always)]
    fn from(val: Nocomb) -> u8 {
        Nocomb::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct OutTrigEn(pub u8);
impl OutTrigEn {
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    pub const VAL0: Self = Self(0x01);
}
impl OutTrigEn {
    pub const fn from_bits(val: u8) -> OutTrigEn {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for OutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> OutTrigEn {
        OutTrigEn::from_bits(val)
    }
}
impl From<OutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: OutTrigEn) -> u8 {
        OutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Prsc {
    #[doc = "Prescaler 1"]
    ONE = 0,
    #[doc = "Prescaler 2"]
    TWO = 0x01,
    #[doc = "Prescaler 4"]
    FOUR = 0x02,
    #[doc = "Prescaler 8"]
    EIGHT = 0x03,
    #[doc = "Prescaler 16"]
    SIXTEEN = 0x04,
    #[doc = "Prescaler 32"]
    THIRTYTWO = 0x05,
    #[doc = "Prescaler 64"]
    SIXTYFOUR = 0x06,
    #[doc = "Prescaler 128"]
    HUNDREDTWENTYEIGHT = 0x07,
}
impl Prsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prsc {
    #[inline(always)]
    fn from(val: u8) -> Prsc {
        Prsc::from_bits(val)
    }
}
impl From<Prsc> for u8 {
    #[inline(always)]
    fn from(val: Prsc) -> u8 {
        Prsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwmaEn {
    #[doc = "PWM_A output disabled."]
    DISABLED = 0,
    #[doc = "PWM_A output enabled."]
    ENABLED = 0x01,
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
impl PwmaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwmaEn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwmaEn {
    #[inline(always)]
    fn from(val: u8) -> PwmaEn {
        PwmaEn::from_bits(val)
    }
}
impl From<PwmaEn> for u8 {
    #[inline(always)]
    fn from(val: PwmaEn) -> u8 {
        PwmaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is tristated."]
    TRISTATED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmafs {
    #[inline(always)]
    fn from(val: u8) -> Pwmafs {
        Pwmafs::from_bits(val)
    }
}
impl From<Pwmafs> for u8 {
    #[inline(always)]
    fn from(val: Pwmafs) -> u8 {
        Pwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwmbEn {
    #[doc = "PWM_B output disabled."]
    DISABLED = 0,
    #[doc = "PWM_B output enabled."]
    ENABLED = 0x01,
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
impl PwmbEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwmbEn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwmbEn {
    #[inline(always)]
    fn from(val: u8) -> PwmbEn {
        PwmbEn::from_bits(val)
    }
}
impl From<PwmbEn> for u8 {
    #[inline(always)]
    fn from(val: PwmbEn) -> u8 {
        PwmbEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is tristated."]
    TRISTATED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Pwmbfs {
        Pwmbfs::from_bits(val)
    }
}
impl From<Pwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Pwmbfs) -> u8 {
        Pwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwmxEn {
    #[doc = "PWM_X output disabled."]
    DISABLED = 0,
    #[doc = "PWM_X output enabled."]
    ENABLED = 0x01,
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
impl PwmxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwmxEn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwmxEn {
    #[inline(always)]
    fn from(val: u8) -> PwmxEn {
        PwmxEn::from_bits(val)
    }
}
impl From<PwmxEn> for u8 {
    #[inline(always)]
    fn from(val: PwmxEn) -> u8 {
        PwmxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is tristated."]
    TRISTATED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Pwmxfs {
        Pwmxfs::from_bits(val)
    }
}
impl From<Pwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Pwmxfs) -> u8 {
        Pwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Run {
    #[doc = "PWM counter is stopped, but PWM outputs will hold the current state."]
    DISABLED = 0,
    #[doc = "PWM counter is started in the corresponding submodule."]
    ENABLED = 0x01,
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
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm0sel23 {
    #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
    SM0PWM23 = 0,
    #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    INVERTED_SM0PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM0OUT23\\] is used by the deadtime logic."]
    SM0OUT23 = 0x02,
    #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
    PWM0_EXTA = 0x03,
}
impl Sm0sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm0sel23 {
        Sm0sel23::from_bits(val)
    }
}
impl From<Sm0sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm0sel23) -> u8 {
        Sm0sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm0sel45 {
    #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
    SM0PWM45 = 0,
    #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    INVERTED_SM0PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM0OUT45\\] is used by the deadtime logic."]
    SM0OUT45 = 0x02,
    #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
    PWM0_EXTB = 0x03,
}
impl Sm0sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm0sel45 {
        Sm0sel45::from_bits(val)
    }
}
impl From<Sm0sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm0sel45) -> u8 {
        Sm0sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm1sel23 {
    #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
    SM1PWM23 = 0,
    #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    INVERTED_SM1PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM1OUT23\\] is used by the deadtime logic."]
    SM1OUT23 = 0x02,
    #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
    PWM1_EXTA = 0x03,
}
impl Sm1sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm1sel23 {
        Sm1sel23::from_bits(val)
    }
}
impl From<Sm1sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm1sel23) -> u8 {
        Sm1sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm1sel45 {
    #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
    SM1PWM45 = 0,
    #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    INVERTED_SM1PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM1OUT45\\] is used by the deadtime logic."]
    SM1OUT45 = 0x02,
    #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
    PWM1_EXTB = 0x03,
}
impl Sm1sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm1sel45 {
        Sm1sel45::from_bits(val)
    }
}
impl From<Sm1sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm1sel45) -> u8 {
        Sm1sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm2sel23 {
    #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
    SM2PWM23 = 0,
    #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    INVERTED_SM2PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM2OUT23\\] is used by the deadtime logic."]
    SM2OUT23 = 0x02,
    #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
    PWM2_EXTA = 0x03,
}
impl Sm2sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm2sel23 {
        Sm2sel23::from_bits(val)
    }
}
impl From<Sm2sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm2sel23) -> u8 {
        Sm2sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm2sel45 {
    #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
    SM2PWM45 = 0,
    #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    INVERTED_SM2PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM2OUT45\\] is used by the deadtime logic."]
    SM2OUT45 = 0x02,
    #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
    PWM2_EXTB = 0x03,
}
impl Sm2sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm2sel45 {
        Sm2sel45::from_bits(val)
    }
}
impl From<Sm2sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm2sel45) -> u8 {
        Sm2sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm3sel23 {
    #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
    SM3PWM23 = 0,
    #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    INVERTED_SM3PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM3OUT23\\] is used by the deadtime logic."]
    SM3OUT23 = 0x02,
    #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
    PWM3_EXTA = 0x03,
}
impl Sm3sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm3sel23 {
        Sm3sel23::from_bits(val)
    }
}
impl From<Sm3sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm3sel23) -> u8 {
        Sm3sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sm3sel45 {
    #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
    SM3PWM45 = 0,
    #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    INVERTED_SM3PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM3OUT45\\] is used by the deadtime logic."]
    SM3OUT45 = 0x02,
    #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
    PWM3_EXTB = 0x03,
}
impl Sm3sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm3sel45 {
        Sm3sel45::from_bits(val)
    }
}
impl From<Sm3sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm3sel45) -> u8 {
        Sm3sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UpdateMask {
    #[doc = "Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    NORMAL = 0,
    #[doc = "Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    MASK_IMMEDIATE = 0x01,
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
impl UpdateMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UpdateMask {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UpdateMask {
    #[inline(always)]
    fn from(val: u8) -> UpdateMask {
        UpdateMask::from_bits(val)
    }
}
impl From<UpdateMask> for u8 {
    #[inline(always)]
    fn from(val: UpdateMask) -> u8 {
        UpdateMask::to_bits(val)
    }
}
