#[doc = "Chip Silicon Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Digprog(pub u32);
impl Digprog {
    #[doc = "Chip silicon revision"]
    #[must_use]
    #[inline(always)]
    pub const fn silicon_revision(&self) -> super::vals::SiliconRevision {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SiliconRevision::from_bits(val as u32)
    }
    #[doc = "Chip silicon revision"]
    #[inline(always)]
    pub const fn set_silicon_revision(&mut self, val: super::vals::SiliconRevision) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Digprog {
    #[inline(always)]
    fn default() -> Digprog {
        Digprog(0)
    }
}
impl core::fmt::Debug for Digprog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Digprog")
            .field("silicon_revision", &self.silicon_revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Digprog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Digprog {{ silicon_revision: {:?} }}",
            self.silicon_revision()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1chrgDetect(pub u32);
impl Usb1chrgDetect {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1chrgDetect {
    #[inline(always)]
    fn default() -> Usb1chrgDetect {
        Usb1chrgDetect(0)
    }
}
impl core::fmt::Debug for Usb1chrgDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1chrgDetect")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1chrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1chrgDetect {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1chrgDetectClr(pub u32);
impl Usb1chrgDetectClr {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1chrgDetectClr {
    #[inline(always)]
    fn default() -> Usb1chrgDetectClr {
        Usb1chrgDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1chrgDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1chrgDetectClr")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1chrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1chrgDetectClr {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}" , self . chk_contact () , self . chk_chrg_b () , self . en_b ())
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1chrgDetectSet(pub u32);
impl Usb1chrgDetectSet {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1chrgDetectSet {
    #[inline(always)]
    fn default() -> Usb1chrgDetectSet {
        Usb1chrgDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1chrgDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1chrgDetectSet")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1chrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1chrgDetectSet {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}" , self . chk_contact () , self . chk_chrg_b () , self . en_b ())
    }
}
#[doc = "USB Charger Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1chrgDetectStat(pub u32);
impl Usb1chrgDetectStat {
    #[doc = "State of the USB plug contact detector."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "State of the USB plug contact detector."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb1chrgDetectStat {
    #[inline(always)]
    fn default() -> Usb1chrgDetectStat {
        Usb1chrgDetectStat(0)
    }
}
impl core::fmt::Debug for Usb1chrgDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1chrgDetectStat")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1chrgDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1chrgDetectStat {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?} }}" , self . plug_contact () , self . chrg_detected () , self . dm_state () , self . dp_state ())
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1chrgDetectTog(pub u32);
impl Usb1chrgDetectTog {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb1chrgDetectTog {
    #[inline(always)]
    fn default() -> Usb1chrgDetectTog {
        Usb1chrgDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1chrgDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1chrgDetectTog")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1chrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1chrgDetectTog {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}" , self . chk_contact () , self . chk_chrg_b () , self . en_b ())
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1loopback(pub u32);
impl Usb1loopback {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1loopback {
    #[inline(always)]
    fn default() -> Usb1loopback {
        Usb1loopback(0)
    }
}
impl core::fmt::Debug for Usb1loopback {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1loopback")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1loopback {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1loopback {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1loopbackClr(pub u32);
impl Usb1loopbackClr {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1loopbackClr {
    #[inline(always)]
    fn default() -> Usb1loopbackClr {
        Usb1loopbackClr(0)
    }
}
impl core::fmt::Debug for Usb1loopbackClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1loopbackClr")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1loopbackClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1loopbackClr {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1loopbackSet(pub u32);
impl Usb1loopbackSet {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1loopbackSet {
    #[inline(always)]
    fn default() -> Usb1loopbackSet {
        Usb1loopbackSet(0)
    }
}
impl core::fmt::Debug for Usb1loopbackSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1loopbackSet")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1loopbackSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1loopbackSet {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1loopbackTog(pub u32);
impl Usb1loopbackTog {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb1loopbackTog {
    #[inline(always)]
    fn default() -> Usb1loopbackTog {
        Usb1loopbackTog(0)
    }
}
impl core::fmt::Debug for Usb1loopbackTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1loopbackTog")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1loopbackTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1loopbackTog {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1misc(pub u32);
impl Usb1misc {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1misc {
    #[inline(always)]
    fn default() -> Usb1misc {
        Usb1misc(0)
    }
}
impl core::fmt::Debug for Usb1misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1misc")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1misc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1misc {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1miscClr(pub u32);
impl Usb1miscClr {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1miscClr {
    #[inline(always)]
    fn default() -> Usb1miscClr {
        Usb1miscClr(0)
    }
}
impl core::fmt::Debug for Usb1miscClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1miscClr")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1miscClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1miscClr {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1miscSet(pub u32);
impl Usb1miscSet {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1miscSet {
    #[inline(always)]
    fn default() -> Usb1miscSet {
        Usb1miscSet(0)
    }
}
impl core::fmt::Debug for Usb1miscSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1miscSet")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1miscSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1miscSet {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1miscTog(pub u32);
impl Usb1miscTog {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb1miscTog {
    #[inline(always)]
    fn default() -> Usb1miscTog {
        Usb1miscTog(0)
    }
}
impl core::fmt::Debug for Usb1miscTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1miscTog")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1miscTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1miscTog {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1vbusDetect(pub u32);
impl Usb1vbusDetect {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1vbusDetectVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1vbusDetectVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: super::vals::Usb1vbusDetectVbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1vbusDetect {
    #[inline(always)]
    fn default() -> Usb1vbusDetect {
        Usb1vbusDetect(0)
    }
}
impl core::fmt::Debug for Usb1vbusDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1vbusDetect")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1vbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1vbusDetect {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1vbusDetectClr(pub u32);
impl Usb1vbusDetectClr {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1vbusDetectClrVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1vbusDetectClrVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1vbusDetectClrVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1vbusDetectClr {
    #[inline(always)]
    fn default() -> Usb1vbusDetectClr {
        Usb1vbusDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1vbusDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1vbusDetectClr")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1vbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1vbusDetectClr {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1vbusDetectSet(pub u32);
impl Usb1vbusDetectSet {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1vbusDetectSetVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1vbusDetectSetVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1vbusDetectSetVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1vbusDetectSet {
    #[inline(always)]
    fn default() -> Usb1vbusDetectSet {
        Usb1vbusDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1vbusDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1vbusDetectSet")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1vbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1vbusDetectSet {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB VBUS Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1vbusDetectStat(pub u32);
impl Usb1vbusDetectStat {
    #[doc = "Session End for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End for USB OTG"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBus valid for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBus valid for USB OTG"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb1vbusDetectStat {
    #[inline(always)]
    fn default() -> Usb1vbusDetectStat {
        Usb1vbusDetectStat(0)
    }
}
impl core::fmt::Debug for Usb1vbusDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1vbusDetectStat")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1vbusDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1vbusDetectStat {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?} }}" , self . sessend () , self . bvalid () , self . avalid () , self . vbus_valid ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1vbusDetectTog(pub u32);
impl Usb1vbusDetectTog {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1vbusDetectTogVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1vbusDetectTogVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1vbusDetectTogVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb1vbusDetectTog {
    #[inline(always)]
    fn default() -> Usb1vbusDetectTog {
        Usb1vbusDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1vbusDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1vbusDetectTog")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1vbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb1vbusDetectTog {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2chrgDetect(pub u32);
impl Usb2chrgDetect {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2chrgDetect {
    #[inline(always)]
    fn default() -> Usb2chrgDetect {
        Usb2chrgDetect(0)
    }
}
impl core::fmt::Debug for Usb2chrgDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2chrgDetect")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2chrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2chrgDetect {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}",
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b()
        )
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2chrgDetectClr(pub u32);
impl Usb2chrgDetectClr {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2chrgDetectClr {
    #[inline(always)]
    fn default() -> Usb2chrgDetectClr {
        Usb2chrgDetectClr(0)
    }
}
impl core::fmt::Debug for Usb2chrgDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2chrgDetectClr")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2chrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2chrgDetectClr {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}" , self . chk_contact () , self . chk_chrg_b () , self . en_b ())
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2chrgDetectSet(pub u32);
impl Usb2chrgDetectSet {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2chrgDetectSet {
    #[inline(always)]
    fn default() -> Usb2chrgDetectSet {
        Usb2chrgDetectSet(0)
    }
}
impl core::fmt::Debug for Usb2chrgDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2chrgDetectSet")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2chrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2chrgDetectSet {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}" , self . chk_contact () , self . chk_chrg_b () , self . en_b ())
    }
}
#[doc = "USB Charger Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2chrgDetectStat(pub u32);
impl Usb2chrgDetectStat {
    #[doc = "State of the USB plug contact detector."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "State of the USB plug contact detector."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP line state output of the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP line state output of the charger detector."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb2chrgDetectStat {
    #[inline(always)]
    fn default() -> Usb2chrgDetectStat {
        Usb2chrgDetectStat(0)
    }
}
impl core::fmt::Debug for Usb2chrgDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2chrgDetectStat")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2chrgDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2chrgDetectStat {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?} }}" , self . plug_contact () , self . chrg_detected () , self . dm_state () , self . dp_state ())
    }
}
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2chrgDetectTog(pub u32);
impl Usb2chrgDetectTog {
    #[doc = "Check the contact of USB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Check the contact of USB plug"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Check the charger connection"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Check the charger connection"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Control the charger detector."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control the charger detector."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Usb2chrgDetectTog {
    #[inline(always)]
    fn default() -> Usb2chrgDetectTog {
        Usb2chrgDetectTog(0)
    }
}
impl core::fmt::Debug for Usb2chrgDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2chrgDetectTog")
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2chrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2chrgDetectTog {{ chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?} }}" , self . chk_contact () , self . chk_chrg_b () , self . en_b ())
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2loopback(pub u32);
impl Usb2loopback {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2loopback {
    #[inline(always)]
    fn default() -> Usb2loopback {
        Usb2loopback(0)
    }
}
impl core::fmt::Debug for Usb2loopback {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2loopback")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2loopback {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2loopback {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2loopbackClr(pub u32);
impl Usb2loopbackClr {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2loopbackClr {
    #[inline(always)]
    fn default() -> Usb2loopbackClr {
        Usb2loopbackClr(0)
    }
}
impl core::fmt::Debug for Usb2loopbackClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2loopbackClr")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2loopbackClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2loopbackClr {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2loopbackSet(pub u32);
impl Usb2loopbackSet {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2loopbackSet {
    #[inline(always)]
    fn default() -> Usb2loopbackSet {
        Usb2loopbackSet(0)
    }
}
impl core::fmt::Debug for Usb2loopbackSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2loopbackSet")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2loopbackSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2loopbackSet {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2loopbackTog(pub u32);
impl Usb2loopbackTog {
    #[doc = "Setting this bit can enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit can enable 1"]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Usb2loopbackTog {
    #[inline(always)]
    fn default() -> Usb2loopbackTog {
        Usb2loopbackTog(0)
    }
}
impl core::fmt::Debug for Usb2loopbackTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2loopbackTog")
            .field("utmi_teststart", &self.utmi_teststart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2loopbackTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb2loopbackTog {{ utmi_teststart: {=bool:?} }}",
            self.utmi_teststart()
        )
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2misc(pub u32);
impl Usb2misc {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2misc {
    #[inline(always)]
    fn default() -> Usb2misc {
        Usb2misc(0)
    }
}
impl core::fmt::Debug for Usb2misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2misc")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2misc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2misc {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2miscClr(pub u32);
impl Usb2miscClr {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2miscClr {
    #[inline(always)]
    fn default() -> Usb2miscClr {
        Usb2miscClr(0)
    }
}
impl core::fmt::Debug for Usb2miscClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2miscClr")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2miscClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2miscClr {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2miscSet(pub u32);
impl Usb2miscSet {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2miscSet {
    #[inline(always)]
    fn default() -> Usb2miscSet {
        Usb2miscSet(0)
    }
}
impl core::fmt::Debug for Usb2miscSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2miscSet")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2miscSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2miscSet {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2miscTog(pub u32);
impl Usb2miscTog {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_use_external_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub const fn set_hs_use_external_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[must_use]
    #[inline(always)]
    pub const fn en_deglitch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub const fn set_en_deglitch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_utmi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk to the UTMI block."]
    #[inline(always)]
    pub const fn set_en_clk_utmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Usb2miscTog {
    #[inline(always)]
    fn default() -> Usb2miscTog {
        Usb2miscTog(0)
    }
}
impl core::fmt::Debug for Usb2miscTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2miscTog")
            .field("hs_use_external_r", &self.hs_use_external_r())
            .field("en_deglitch", &self.en_deglitch())
            .field("en_clk_utmi", &self.en_clk_utmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2miscTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2miscTog {{ hs_use_external_r: {=bool:?}, en_deglitch: {=bool:?}, en_clk_utmi: {=bool:?} }}" , self . hs_use_external_r () , self . en_deglitch () , self . en_clk_utmi ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2vbusDetect(pub u32);
impl Usb2vbusDetect {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2vbusDetectVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2vbusDetectVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: super::vals::Usb2vbusDetectVbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2vbusDetect {
    #[inline(always)]
    fn default() -> Usb2vbusDetect {
        Usb2vbusDetect(0)
    }
}
impl core::fmt::Debug for Usb2vbusDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2vbusDetect")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2vbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2vbusDetect {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2vbusDetectClr(pub u32);
impl Usb2vbusDetectClr {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2vbusDetectClrVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2vbusDetectClrVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb2vbusDetectClrVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2vbusDetectClr {
    #[inline(always)]
    fn default() -> Usb2vbusDetectClr {
        Usb2vbusDetectClr(0)
    }
}
impl core::fmt::Debug for Usb2vbusDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2vbusDetectClr")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2vbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2vbusDetectClr {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2vbusDetectSet(pub u32);
impl Usb2vbusDetectSet {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2vbusDetectSetVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2vbusDetectSetVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb2vbusDetectSetVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2vbusDetectSet {
    #[inline(always)]
    fn default() -> Usb2vbusDetectSet {
        Usb2vbusDetectSet(0)
    }
}
impl core::fmt::Debug for Usb2vbusDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2vbusDetectSet")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2vbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2vbusDetectSet {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
#[doc = "USB VBUS Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2vbusDetectStat(pub u32);
impl Usb2vbusDetectStat {
    #[doc = "Session End for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End for USB OTG"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBus valid for USB OTG"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBus valid for USB OTG"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb2vbusDetectStat {
    #[inline(always)]
    fn default() -> Usb2vbusDetectStat {
        Usb2vbusDetectStat(0)
    }
}
impl core::fmt::Debug for Usb2vbusDetectStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2vbusDetectStat")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2vbusDetectStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2vbusDetectStat {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?} }}" , self . sessend () , self . bvalid () , self . avalid () , self . vbus_valid ())
    }
}
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb2vbusDetectTog(pub u32);
impl Usb2vbusDetectTog {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb2vbusDetectTogVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb2vbusDetectTogVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb2vbusDetectTogVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "USB OTG discharge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG discharge VBUS."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB OTG charge VBUS."]
    #[must_use]
    #[inline(always)]
    pub const fn charge_vbus(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB OTG charge VBUS."]
    #[inline(always)]
    pub const fn set_charge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Usb2vbusDetectTog {
    #[inline(always)]
    fn default() -> Usb2vbusDetectTog {
        Usb2vbusDetectTog(0)
    }
}
impl core::fmt::Debug for Usb2vbusDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb2vbusDetectTog")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("charge_vbus", &self.charge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb2vbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usb2vbusDetectTog {{ vbusvalid_thresh: {:?}, vbusvalid_pwrup_cmps: {=bool:?}, discharge_vbus: {=bool:?}, charge_vbus: {=bool:?} }}" , self . vbusvalid_thresh () , self . vbusvalid_pwrup_cmps () , self . discharge_vbus () , self . charge_vbus ())
    }
}
