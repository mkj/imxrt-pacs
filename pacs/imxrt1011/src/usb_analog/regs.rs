#[doc = "Chip Silicon Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Digprog(pub u32);
impl Digprog {
    #[doc = "Chip silicon revision"]
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
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1chrgDetect(pub u32);
impl Usb1chrgDetect {
    #[doc = "Check the contact of USB plug"]
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
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1chrgDetectClr(pub u32);
impl Usb1chrgDetectClr {
    #[doc = "Check the contact of USB plug"]
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
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1chrgDetectSet(pub u32);
impl Usb1chrgDetectSet {
    #[doc = "Check the contact of USB plug"]
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
#[doc = "USB Charger Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1chrgDetectStat(pub u32);
impl Usb1chrgDetectStat {
    #[doc = "State of the USB plug contact detector."]
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
#[doc = "USB Charger Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1chrgDetectTog(pub u32);
impl Usb1chrgDetectTog {
    #[doc = "Check the contact of USB plug"]
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
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1loopback(pub u32);
impl Usb1loopback {
    #[doc = "Setting this bit can enable 1"]
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
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1loopbackClr(pub u32);
impl Usb1loopbackClr {
    #[doc = "Setting this bit can enable 1"]
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
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1loopbackSet(pub u32);
impl Usb1loopbackSet {
    #[doc = "Setting this bit can enable 1"]
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
#[doc = "USB Loopback Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1loopbackTog(pub u32);
impl Usb1loopbackTog {
    #[doc = "Setting this bit can enable 1"]
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
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1misc(pub u32);
impl Usb1misc {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
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
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1miscClr(pub u32);
impl Usb1miscClr {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
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
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1miscSet(pub u32);
impl Usb1miscSet {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
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
#[doc = "USB Misc Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1miscTog(pub u32);
impl Usb1miscTog {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
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
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1vbusDetect(pub u32);
impl Usb1vbusDetect {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
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
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1vbusDetectClr(pub u32);
impl Usb1vbusDetectClr {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
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
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1vbusDetectSet(pub u32);
impl Usb1vbusDetectSet {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
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
#[doc = "USB VBUS Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1vbusDetectStat(pub u32);
impl Usb1vbusDetectStat {
    #[doc = "Session End for USB OTG"]
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
#[doc = "USB VBUS Detect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Usb1vbusDetectTog(pub u32);
impl Usb1vbusDetectTog {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
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
