#[doc = "Watchdog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wcr(pub u32);
impl Wcr {
    #[doc = "WDZST"]
    #[inline(always)]
    pub const fn wdzst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WDZST"]
    #[inline(always)]
    pub const fn set_wdzst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WDBG"]
    #[inline(always)]
    pub const fn wdbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WDBG"]
    #[inline(always)]
    pub const fn set_wdbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WDE"]
    #[inline(always)]
    pub const fn wde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WDE"]
    #[inline(always)]
    pub const fn set_wde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub const fn wdt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub const fn set_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SRS"]
    #[inline(always)]
    pub const fn srs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SRS"]
    #[inline(always)]
    pub const fn set_srs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "WDA"]
    #[inline(always)]
    pub const fn wda(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "WDA"]
    #[inline(always)]
    pub const fn set_wda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "software reset extension, an option way to generate software reset"]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "software reset extension, an option way to generate software reset"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "WDW"]
    #[inline(always)]
    pub const fn wdw(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "WDW"]
    #[inline(always)]
    pub const fn set_wdw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "WT"]
    #[inline(always)]
    pub const fn wt(&self) -> super::vals::Wt {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Wt::from_bits(val as u8)
    }
    #[doc = "WT"]
    #[inline(always)]
    pub const fn set_wt(&mut self, val: super::vals::Wt) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
}
impl Default for Wcr {
    #[inline(always)]
    fn default() -> Wcr {
        Wcr(0)
    }
}
#[doc = "Watchdog Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wicr(pub u32);
impl Wicr {
    #[doc = "WICT"]
    #[inline(always)]
    pub const fn wict(&self) -> super::vals::Wict {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Wict::from_bits(val as u8)
    }
    #[doc = "WICT"]
    #[inline(always)]
    pub const fn set_wict(&mut self, val: super::vals::Wict) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "WTIS"]
    #[inline(always)]
    pub const fn wtis(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "WTIS"]
    #[inline(always)]
    pub const fn set_wtis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "WIE"]
    #[inline(always)]
    pub const fn wie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "WIE"]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Wicr {
    #[inline(always)]
    fn default() -> Wicr {
        Wicr(0)
    }
}
#[doc = "Watchdog Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wmcr(pub u32);
impl Wmcr {
    #[doc = "PDE"]
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PDE"]
    #[inline(always)]
    pub const fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Wmcr {
    #[inline(always)]
    fn default() -> Wmcr {
        Wmcr(0)
    }
}
#[doc = "Watchdog Reset Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wrsr(pub u32);
impl Wrsr {
    #[doc = "SFTW"]
    #[inline(always)]
    pub const fn sftw(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SFTW"]
    #[inline(always)]
    pub const fn set_sftw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TOUT"]
    #[inline(always)]
    pub const fn tout(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TOUT"]
    #[inline(always)]
    pub const fn set_tout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "POR"]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "POR"]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Wrsr {
    #[inline(always)]
    fn default() -> Wrsr {
        Wrsr(0)
    }
}
#[doc = "Watchdog Service Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wsr(pub u32);
impl Wsr {
    #[doc = "WSR"]
    #[inline(always)]
    pub const fn wsr(&self) -> super::vals::Wsr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Wsr::from_bits(val as u16)
    }
    #[doc = "WSR"]
    #[inline(always)]
    pub const fn set_wsr(&mut self, val: super::vals::Wsr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Wsr {
    #[inline(always)]
    fn default() -> Wsr {
        Wsr(0)
    }
}
