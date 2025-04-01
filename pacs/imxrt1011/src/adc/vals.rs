#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adiclk {
    #[doc = "IPG clock"]
    ADICLK_0 = 0,
    #[doc = "IPG clock divided by 2"]
    ADICLK_1 = 0x01,
    #[doc = "Alternate clock (ALTCLK)"]
    ADICLK_2 = 0x02,
    #[doc = "Asynchronous clock (ADACK)"]
    ADICLK_3 = 0x03,
}
impl Adiclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adiclk {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adiclk {
    #[inline(always)]
    fn from(val: u8) -> Adiclk {
        Adiclk::from_bits(val)
    }
}
impl From<Adiclk> for u8 {
    #[inline(always)]
    fn from(val: Adiclk) -> u8 {
        Adiclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adiv {
    #[doc = "Input clock"]
    ADIV_0 = 0,
    #[doc = "Input clock / 2"]
    ADIV_1 = 0x01,
    #[doc = "Input clock / 4"]
    ADIV_2 = 0x02,
    #[doc = "Input clock / 8"]
    ADIV_3 = 0x03,
}
impl Adiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adiv {
    #[inline(always)]
    fn from(val: u8) -> Adiv {
        Adiv::from_bits(val)
    }
}
impl From<Adiv> for u8 {
    #[inline(always)]
    fn from(val: Adiv) -> u8 {
        Adiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adsts {
    #[doc = "Sample period (ADC clocks) = 3 if ADLSMP=0b Sample period (ADC clocks) = 13 if ADLSMP=1b"]
    ADSTS_0 = 0,
    #[doc = "Sample period (ADC clocks) = 5 if ADLSMP=0b Sample period (ADC clocks) = 17 if ADLSMP=1b"]
    ADSTS_1 = 0x01,
    #[doc = "Sample period (ADC clocks) = 7 if ADLSMP=0b Sample period (ADC clocks) = 21 if ADLSMP=1b"]
    ADSTS_2 = 0x02,
    #[doc = "Sample period (ADC clocks) = 9 if ADLSMP=0b Sample period (ADC clocks) = 25 if ADLSMP=1b"]
    ADSTS_3 = 0x03,
}
impl Adsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adsts {
    #[inline(always)]
    fn from(val: u8) -> Adsts {
        Adsts::from_bits(val)
    }
}
impl From<Adsts> for u8 {
    #[inline(always)]
    fn from(val: Adsts) -> u8 {
        Adsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Avgs {
    #[doc = "4 samples averaged"]
    AVGS_0 = 0,
    #[doc = "8 samples averaged"]
    AVGS_1 = 0x01,
    #[doc = "16 samples averaged"]
    AVGS_2 = 0x02,
    #[doc = "32 samples averaged"]
    AVGS_3 = 0x03,
}
impl Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avgs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avgs {
    #[inline(always)]
    fn from(val: u8) -> Avgs {
        Avgs::from_bits(val)
    }
}
impl From<Avgs> for u8 {
    #[inline(always)]
    fn from(val: Avgs) -> u8 {
        Avgs::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hc0adch(pub u8);
impl Hc0adch {
    #[doc = "External channel selection from ADC_ETC"]
    pub const ADCH_16: Self = Self(0x10);
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    pub const ADCH_25: Self = Self(0x19);
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    pub const ADCH_31: Self = Self(0x1f);
}
impl Hc0adch {
    pub const fn from_bits(val: u8) -> Hc0adch {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Hc0adch {
    #[inline(always)]
    fn from(val: u8) -> Hc0adch {
        Hc0adch::from_bits(val)
    }
}
impl From<Hc0adch> for u8 {
    #[inline(always)]
    fn from(val: Hc0adch) -> u8 {
        Hc0adch::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HcAdch(pub u8);
impl HcAdch {
    #[doc = "External channel selection from ADC_ETC"]
    pub const ADCH_16: Self = Self(0x10);
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    pub const ADCH_25: Self = Self(0x19);
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    pub const ADCH_31: Self = Self(0x1f);
}
impl HcAdch {
    pub const fn from_bits(val: u8) -> HcAdch {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for HcAdch {
    #[inline(always)]
    fn from(val: u8) -> HcAdch {
        HcAdch::from_bits(val)
    }
}
impl From<HcAdch> for u8 {
    #[inline(always)]
    fn from(val: HcAdch) -> u8 {
        HcAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "8-bit conversion"]
    MODE_0 = 0,
    #[doc = "10-bit conversion"]
    MODE_1 = 0x01,
    #[doc = "12-bit conversion"]
    MODE_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Refsel {
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    REFSEL_0 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
