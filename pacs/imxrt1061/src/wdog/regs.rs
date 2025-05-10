#[doc = "Watchdog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wcr(pub u16);
impl Wcr {
    #[doc = "WDZST"]
    #[must_use]
    #[inline(always)]
    pub const fn wdzst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WDZST"]
    #[inline(always)]
    pub const fn set_wdzst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "WDBG"]
    #[must_use]
    #[inline(always)]
    pub const fn wdbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WDBG"]
    #[inline(always)]
    pub const fn set_wdbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "WDE"]
    #[must_use]
    #[inline(always)]
    pub const fn wde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WDE"]
    #[inline(always)]
    pub const fn set_wde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "WDT"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub const fn set_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "SRS"]
    #[must_use]
    #[inline(always)]
    pub const fn srs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SRS"]
    #[inline(always)]
    pub const fn set_srs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "WDA"]
    #[must_use]
    #[inline(always)]
    pub const fn wda(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "WDA"]
    #[inline(always)]
    pub const fn set_wda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Software Reset Extension, an optional way to generate software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset Extension, an optional way to generate software reset"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "WDW"]
    #[must_use]
    #[inline(always)]
    pub const fn wdw(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "WDW"]
    #[inline(always)]
    pub const fn set_wdw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "WT"]
    #[must_use]
    #[inline(always)]
    pub const fn wt(&self) -> super::vals::Wt {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Wt::from_bits(val as u8)
    }
    #[doc = "WT"]
    #[inline(always)]
    pub const fn set_wt(&mut self, val: super::vals::Wt) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Wcr {
    #[inline(always)]
    fn default() -> Wcr {
        Wcr(0)
    }
}
impl core::fmt::Debug for Wcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wcr")
            .field("wdzst", &self.wdzst())
            .field("wdbg", &self.wdbg())
            .field("wde", &self.wde())
            .field("wdt", &self.wdt())
            .field("srs", &self.srs())
            .field("wda", &self.wda())
            .field("sre", &self.sre())
            .field("wdw", &self.wdw())
            .field("wt", &self.wt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wcr {{ wdzst: {=bool:?}, wdbg: {=bool:?}, wde: {=bool:?}, wdt: {=bool:?}, srs: {=bool:?}, wda: {=bool:?}, sre: {=bool:?}, wdw: {=bool:?}, wt: {:?} }}" , self . wdzst () , self . wdbg () , self . wde () , self . wdt () , self . srs () , self . wda () , self . sre () , self . wdw () , self . wt ())
    }
}
#[doc = "Watchdog Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wicr(pub u16);
impl Wicr {
    #[doc = "WICT"]
    #[must_use]
    #[inline(always)]
    pub const fn wict(&self) -> super::vals::Wict {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Wict::from_bits(val as u8)
    }
    #[doc = "WICT"]
    #[inline(always)]
    pub const fn set_wict(&mut self, val: super::vals::Wict) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "WTIS"]
    #[must_use]
    #[inline(always)]
    pub const fn wtis(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "WTIS"]
    #[inline(always)]
    pub const fn set_wtis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "WIE"]
    #[must_use]
    #[inline(always)]
    pub const fn wie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "WIE"]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Wicr {
    #[inline(always)]
    fn default() -> Wicr {
        Wicr(0)
    }
}
impl core::fmt::Debug for Wicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wicr")
            .field("wict", &self.wict())
            .field("wtis", &self.wtis())
            .field("wie", &self.wie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wicr {{ wict: {:?}, wtis: {=bool:?}, wie: {=bool:?} }}",
            self.wict(),
            self.wtis(),
            self.wie()
        )
    }
}
#[doc = "Watchdog Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wmcr(pub u16);
impl Wmcr {
    #[doc = "PDE"]
    #[must_use]
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PDE"]
    #[inline(always)]
    pub const fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
}
impl Default for Wmcr {
    #[inline(always)]
    fn default() -> Wmcr {
        Wmcr(0)
    }
}
impl core::fmt::Debug for Wmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wmcr").field("pde", &self.pde()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wmcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wmcr {{ pde: {=bool:?} }}", self.pde())
    }
}
#[doc = "Watchdog Reset Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrsr(pub u16);
impl Wrsr {
    #[doc = "SFTW"]
    #[must_use]
    #[inline(always)]
    pub const fn sftw(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SFTW"]
    #[inline(always)]
    pub const fn set_sftw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "TOUT"]
    #[must_use]
    #[inline(always)]
    pub const fn tout(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TOUT"]
    #[inline(always)]
    pub const fn set_tout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "POR"]
    #[must_use]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "POR"]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for Wrsr {
    #[inline(always)]
    fn default() -> Wrsr {
        Wrsr(0)
    }
}
impl core::fmt::Debug for Wrsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wrsr")
            .field("sftw", &self.sftw())
            .field("tout", &self.tout())
            .field("por", &self.por())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wrsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wrsr {{ sftw: {=bool:?}, tout: {=bool:?}, por: {=bool:?} }}",
            self.sftw(),
            self.tout(),
            self.por()
        )
    }
}
#[doc = "Watchdog Service Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wsr(pub u16);
impl Wsr {
    #[doc = "WSR"]
    #[must_use]
    #[inline(always)]
    pub const fn wsr(&self) -> super::vals::Wsr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Wsr::from_bits(val as u16)
    }
    #[doc = "WSR"]
    #[inline(always)]
    pub const fn set_wsr(&mut self, val: super::vals::Wsr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for Wsr {
    #[inline(always)]
    fn default() -> Wsr {
        Wsr(0)
    }
}
impl core::fmt::Debug for Wsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wsr").field("wsr", &self.wsr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wsr {{ wsr: {:?} }}", self.wsr())
    }
}
