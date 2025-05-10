#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl0(pub u16);
impl Csctrl0 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl0cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl0cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl0cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl0cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl0cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl0cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl0dbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl0dbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl0dbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl0 {
    #[inline(always)]
    fn default() -> Csctrl0 {
        Csctrl0(0)
    }
}
impl core::fmt::Debug for Csctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl0")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Csctrl0 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {=bool:?}, tci: {=bool:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}" , self . cl1 () , self . cl2 () , self . tcf1 () , self . tcf2 () , self . tcf1en () , self . tcf2en () , self . up () , self . tci () , self . roc () , self . alt_load () , self . fault () , self . dbg_en ())
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl1(pub u16);
impl Csctrl1 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl1cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl1cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl1cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl1cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl1cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl1cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl1dbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl1dbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl1dbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl1 {
    #[inline(always)]
    fn default() -> Csctrl1 {
        Csctrl1(0)
    }
}
impl core::fmt::Debug for Csctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl1")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Csctrl1 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {=bool:?}, tci: {=bool:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}" , self . cl1 () , self . cl2 () , self . tcf1 () , self . tcf2 () , self . tcf1en () , self . tcf2en () , self . up () , self . tci () , self . roc () , self . alt_load () , self . fault () , self . dbg_en ())
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl2(pub u16);
impl Csctrl2 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl2cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl2cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl2cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl2cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl2cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl2cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl2dbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl2dbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl2dbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl2 {
    #[inline(always)]
    fn default() -> Csctrl2 {
        Csctrl2(0)
    }
}
impl core::fmt::Debug for Csctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl2")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Csctrl2 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {=bool:?}, tci: {=bool:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}" , self . cl1 () , self . cl2 () , self . tcf1 () , self . tcf2 () , self . tcf1en () , self . tcf2en () , self . up () , self . tci () , self . roc () , self . alt_load () , self . fault () , self . dbg_en ())
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csctrl3(pub u16);
impl Csctrl3 {
    #[doc = "Compare Load Control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Csctrl3cl1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Csctrl3cl1::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 1"]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Csctrl3cl1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Compare Load Control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Csctrl3cl2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Csctrl3cl2::from_bits(val as u8)
    }
    #[doc = "Compare Load Control 2"]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Csctrl3cl2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tcf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf1en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf2en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcf2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Counting Direction Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counting Direction Indicator"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Triggered Count Initialization Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tci(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered Count Initialization Control"]
    #[inline(always)]
    pub const fn set_tci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Reload on Capture"]
    #[must_use]
    #[inline(always)]
    pub const fn roc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reload on Capture"]
    #[inline(always)]
    pub const fn set_roc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Alternative Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alt_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alternative Load Enable"]
    #[inline(always)]
    pub const fn set_alt_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Fault Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Enable"]
    #[inline(always)]
    pub const fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Actions Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::Csctrl3dbgEn {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Csctrl3dbgEn::from_bits(val as u8)
    }
    #[doc = "Debug Actions Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::Csctrl3dbgEn) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Csctrl3 {
    #[inline(always)]
    fn default() -> Csctrl3 {
        Csctrl3(0)
    }
}
impl core::fmt::Debug for Csctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csctrl3")
            .field("cl1", &self.cl1())
            .field("cl2", &self.cl2())
            .field("tcf1", &self.tcf1())
            .field("tcf2", &self.tcf2())
            .field("tcf1en", &self.tcf1en())
            .field("tcf2en", &self.tcf2en())
            .field("up", &self.up())
            .field("tci", &self.tci())
            .field("roc", &self.roc())
            .field("alt_load", &self.alt_load())
            .field("fault", &self.fault())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Csctrl3 {{ cl1: {:?}, cl2: {:?}, tcf1: {=bool:?}, tcf2: {=bool:?}, tcf1en: {=bool:?}, tcf2en: {=bool:?}, up: {=bool:?}, tci: {=bool:?}, roc: {=bool:?}, alt_load: {=bool:?}, fault: {=bool:?}, dbg_en: {:?} }}" , self . cl1 () , self . cl2 () , self . tcf1 () , self . tcf2 () , self . tcf1en () , self . tcf2en () , self . up () , self . tci () , self . roc () , self . alt_load () , self . fault () , self . dbg_en ())
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u16);
impl Ctrl0 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl0outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl0outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl0outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl0scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl0scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl0scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl0pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl0pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl0pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl0cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl0cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl0cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl0 {{ outmode: {:?}, coinit: {=bool:?}, dir: {=bool:?}, length: {=bool:?}, once: {=bool:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}" , self . outmode () , self . coinit () , self . dir () , self . length () , self . once () , self . scs () , self . pcs () , self . cm ())
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u16);
impl Ctrl1 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl1outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl1outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl1outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl1scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl1scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl1scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl1pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl1pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl1pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl1cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl1cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl1cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl1 {{ outmode: {:?}, coinit: {=bool:?}, dir: {=bool:?}, length: {=bool:?}, once: {=bool:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}" , self . outmode () , self . coinit () , self . dir () , self . length () , self . once () , self . scs () , self . pcs () , self . cm ())
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u16);
impl Ctrl2 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl2outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl2outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl2outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl2scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl2scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl2scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl2pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl2pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl2pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl2cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl2cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl2cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl2 {{ outmode: {:?}, coinit: {=bool:?}, dir: {=bool:?}, length: {=bool:?}, once: {=bool:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}" , self . outmode () , self . coinit () , self . dir () , self . length () , self . once () , self . scs () , self . pcs () , self . cm ())
    }
}
#[doc = "Timer Channel Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3(pub u16);
impl Ctrl3 {
    #[doc = "Output Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn outmode(&self) -> super::vals::Ctrl3outmode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctrl3outmode::from_bits(val as u8)
    }
    #[doc = "Output Mode"]
    #[inline(always)]
    pub const fn set_outmode(&mut self, val: super::vals::Ctrl3outmode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Co-Channel Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn coinit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Co-Channel Initialization"]
    #[inline(always)]
    pub const fn set_coinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Count Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Count Length"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Count Length"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Secondary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Ctrl3scs {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::Ctrl3scs::from_bits(val as u8)
    }
    #[doc = "Secondary Count Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Ctrl3scs) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u16) & 0x03) << 7usize);
    }
    #[doc = "Primary Count Source"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Ctrl3pcs {
        let val = (self.0 >> 9usize) & 0x0f;
        super::vals::Ctrl3pcs::from_bits(val as u8)
    }
    #[doc = "Primary Count Source"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Ctrl3pcs) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val.to_bits() as u16) & 0x0f) << 9usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Ctrl3cm {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Ctrl3cm::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Ctrl3cm) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Ctrl3 {
    #[inline(always)]
    fn default() -> Ctrl3 {
        Ctrl3(0)
    }
}
impl core::fmt::Debug for Ctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3")
            .field("outmode", &self.outmode())
            .field("coinit", &self.coinit())
            .field("dir", &self.dir())
            .field("length", &self.length())
            .field("once", &self.once())
            .field("scs", &self.scs())
            .field("pcs", &self.pcs())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl3 {{ outmode: {:?}, coinit: {=bool:?}, dir: {=bool:?}, length: {=bool:?}, once: {=bool:?}, scs: {:?}, pcs: {:?}, cm: {:?} }}" , self . outmode () , self . coinit () , self . dir () , self . length () , self . once () , self . scs () , self . pcs () , self . cm ())
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0(pub u16);
impl Dma0 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma0 {
    #[inline(always)]
    fn default() -> Dma0 {
        Dma0(0)
    }
}
impl core::fmt::Debug for Dma0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1(pub u16);
impl Dma1 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma1 {
    #[inline(always)]
    fn default() -> Dma1 {
        Dma1(0)
    }
}
impl core::fmt::Debug for Dma1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2(pub u16);
impl Dma2 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma2 {
    #[inline(always)]
    fn default() -> Dma2 {
        Dma2(0)
    }
}
impl core::fmt::Debug for Dma2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma2")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma2 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3(pub u16);
impl Dma3 {
    #[doc = "Input Edge Flag DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub const fn set_iefde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpld2de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub const fn set_cmpld2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Dma3 {
    #[inline(always)]
    fn default() -> Dma3 {
        Dma3(0)
    }
}
impl core::fmt::Debug for Dma3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma3")
            .field("iefde", &self.iefde())
            .field("cmpld1de", &self.cmpld1de())
            .field("cmpld2de", &self.cmpld2de())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma3 {{ iefde: {=bool:?}, cmpld1de: {=bool:?}, cmpld2de: {=bool:?} }}",
            self.iefde(),
            self.cmpld1de(),
            self.cmpld2de()
        )
    }
}
#[doc = "Timer Channel Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enbl(pub u16);
impl Enbl {
    #[doc = "Timer Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enbl(&self) -> super::vals::Enbl {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enbl::from_bits(val as u8)
    }
    #[doc = "Timer Channel Enable"]
    #[inline(always)]
    pub const fn set_enbl(&mut self, val: super::vals::Enbl) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Enbl {
    #[inline(always)]
    fn default() -> Enbl {
        Enbl(0)
    }
}
impl core::fmt::Debug for Enbl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enbl").field("enbl", &self.enbl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enbl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enbl {{ enbl: {:?} }}", self.enbl())
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt0(pub u16);
impl Filt0 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt0 {
    #[inline(always)]
    fn default() -> Filt0 {
        Filt0(0)
    }
}
impl core::fmt::Debug for Filt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt0")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt0 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt1(pub u16);
impl Filt1 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt1 {
    #[inline(always)]
    fn default() -> Filt1 {
        Filt1(0)
    }
}
impl core::fmt::Debug for Filt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt1")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt1 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt2(pub u16);
impl Filt2 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt2 {
    #[inline(always)]
    fn default() -> Filt2 {
        Filt2(0)
    }
}
impl core::fmt::Debug for Filt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt2")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt2 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt3(pub u16);
impl Filt3 {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Filt3 {
    #[inline(always)]
    fn default() -> Filt3 {
        Filt3(0)
    }
}
impl core::fmt::Debug for Filt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt3")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt3 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl0(pub u16);
impl Sctrl0 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl0captureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl0captureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl0captureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl0 {
    #[inline(always)]
    fn default() -> Sctrl0 {
        Sctrl0(0)
    }
}
impl core::fmt::Debug for Sctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl0")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sctrl0 {{ oen: {=bool:?}, ops: {=bool:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}" , self . oen () , self . ops () , self . force () , self . val () , self . eeof () , self . mstr () , self . capture_mode () , self . input () , self . ips () , self . iefie () , self . ief () , self . tofie () , self . tof () , self . tcfie () , self . tcf ())
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl1(pub u16);
impl Sctrl1 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl1captureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl1captureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl1captureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl1 {
    #[inline(always)]
    fn default() -> Sctrl1 {
        Sctrl1(0)
    }
}
impl core::fmt::Debug for Sctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl1")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sctrl1 {{ oen: {=bool:?}, ops: {=bool:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}" , self . oen () , self . ops () , self . force () , self . val () , self . eeof () , self . mstr () , self . capture_mode () , self . input () , self . ips () , self . iefie () , self . ief () , self . tofie () , self . tof () , self . tcfie () , self . tcf ())
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl2(pub u16);
impl Sctrl2 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl2captureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl2captureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl2captureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl2 {
    #[inline(always)]
    fn default() -> Sctrl2 {
        Sctrl2(0)
    }
}
impl core::fmt::Debug for Sctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl2")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sctrl2 {{ oen: {=bool:?}, ops: {=bool:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}" , self . oen () , self . ops () , self . force () , self . val () , self . eeof () , self . mstr () , self . capture_mode () , self . input () , self . ips () , self . iefie () , self . ief () , self . tofie () , self . tof () , self . tcfie () , self . tcf ())
    }
}
#[doc = "Timer Channel Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl3(pub u16);
impl Sctrl3 {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Output Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ops(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output Polarity Select"]
    #[inline(always)]
    pub const fn set_ops(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Force OFLAG Output"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force OFLAG Output"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Forced OFLAG Value"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forced OFLAG Value"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable External OFLAG Force"]
    #[must_use]
    #[inline(always)]
    pub const fn eeof(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External OFLAG Force"]
    #[inline(always)]
    pub const fn set_eeof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Master Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub const fn set_mstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Input Capture Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn capture_mode(&self) -> super::vals::Sctrl3captureMode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sctrl3captureMode::from_bits(val as u8)
    }
    #[doc = "Input Capture Mode"]
    #[inline(always)]
    pub const fn set_capture_mode(&mut self, val: super::vals::Sctrl3captureMode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "External Input Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External Input Signal"]
    #[inline(always)]
    pub const fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Input Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ips(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Polarity Select"]
    #[inline(always)]
    pub const fn set_ips(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iefie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iefie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Input Edge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input Edge Flag"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tofie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sctrl3 {
    #[inline(always)]
    fn default() -> Sctrl3 {
        Sctrl3(0)
    }
}
impl core::fmt::Debug for Sctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl3")
            .field("oen", &self.oen())
            .field("ops", &self.ops())
            .field("force", &self.force())
            .field("val", &self.val())
            .field("eeof", &self.eeof())
            .field("mstr", &self.mstr())
            .field("capture_mode", &self.capture_mode())
            .field("input", &self.input())
            .field("ips", &self.ips())
            .field("iefie", &self.iefie())
            .field("ief", &self.ief())
            .field("tofie", &self.tofie())
            .field("tof", &self.tof())
            .field("tcfie", &self.tcfie())
            .field("tcf", &self.tcf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sctrl3 {{ oen: {=bool:?}, ops: {=bool:?}, force: {=bool:?}, val: {=bool:?}, eeof: {=bool:?}, mstr: {=bool:?}, capture_mode: {:?}, input: {=bool:?}, ips: {=bool:?}, iefie: {=bool:?}, ief: {=bool:?}, tofie: {=bool:?}, tof: {=bool:?}, tcfie: {=bool:?}, tcf: {=bool:?} }}" , self . oen () , self . ops () , self . force () , self . val () , self . eeof () , self . mstr () , self . capture_mode () , self . input () , self . ips () , self . iefie () , self . ief () , self . tofie () , self . tof () , self . tcfie () , self . tcf ())
    }
}
