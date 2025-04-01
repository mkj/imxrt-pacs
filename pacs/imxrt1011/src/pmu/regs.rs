#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc0(pub u32);
impl Misc0 {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0reftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0reftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0reftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0stopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0stopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0stopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0oscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0oscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0oscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0clkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0clkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0clkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0 {
    #[inline(always)]
    fn default() -> Misc0 {
        Misc0(0)
    }
}
#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc0clr(pub u32);
impl Misc0clr {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0clrReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0clrReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0clrReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0clrStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0clrStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0clrStopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0clrOscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0clrOscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0clrOscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0clrClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0clrClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0clrClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0clr {
    #[inline(always)]
    fn default() -> Misc0clr {
        Misc0clr(0)
    }
}
#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc0set(pub u32);
impl Misc0set {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0setReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0setReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0setReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0setStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0setStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0setStopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0setOscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0setOscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0setOscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0setClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0setClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0setClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0set {
    #[inline(always)]
    fn default() -> Misc0set {
        Misc0set(0)
    }
}
#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc0tog(pub u32);
impl Misc0tog {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0togReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0togReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0togReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0togStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0togStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0togStopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0togOscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0togOscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0togOscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0togClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0togClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0togClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0tog {
    #[inline(always)]
    fn default() -> Misc0tog {
        Misc0tog(0)
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc1(pub u32);
impl Misc1 {
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1 {
    #[inline(always)]
    fn default() -> Misc1 {
        Misc1(0)
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc1clr(pub u32);
impl Misc1clr {
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1clr {
    #[inline(always)]
    fn default() -> Misc1clr {
        Misc1clr(0)
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc1set(pub u32);
impl Misc1set {
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1set {
    #[inline(always)]
    fn default() -> Misc1set {
        Misc1set(0)
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc1tog(pub u32);
impl Misc1tog {
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1tog {
    #[inline(always)]
    fn default() -> Misc1tog {
        Misc1tog(0)
    }
}
#[doc = "Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc2(pub u32);
impl Misc2 {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2reg0boOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2reg0boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2reg0boOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2reg1boOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2reg1boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2reg1boOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2reg2boOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2reg2boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2reg2boOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2reg0stepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2reg0stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2reg0stepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2reg1stepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2reg1stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2reg1stepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2reg2stepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2reg2stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2reg2stepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Misc2 {
    #[inline(always)]
    fn default() -> Misc2 {
        Misc2(0)
    }
}
#[doc = "Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc2clr(pub u32);
impl Misc2clr {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2clrReg0boOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2clrReg0boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2clrReg0boOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2clrReg1boOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2clrReg1boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2clrReg1boOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2clrReg2boOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2clrReg2boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2clrReg2boOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2clrReg0stepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2clrReg0stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2clrReg0stepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2clrReg1stepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2clrReg1stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2clrReg1stepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2clrReg2stepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2clrReg2stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2clrReg2stepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Misc2clr {
    #[inline(always)]
    fn default() -> Misc2clr {
        Misc2clr(0)
    }
}
#[doc = "Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc2set(pub u32);
impl Misc2set {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2setReg0boOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2setReg0boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2setReg0boOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2setReg1boOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2setReg1boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2setReg1boOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2setReg2boOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2setReg2boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2setReg2boOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2setReg0stepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2setReg0stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2setReg0stepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2setReg1stepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2setReg1stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2setReg1stepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2setReg2stepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2setReg2stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2setReg2stepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Misc2set {
    #[inline(always)]
    fn default() -> Misc2set {
        Misc2set(0)
    }
}
#[doc = "Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Misc2tog(pub u32);
impl Misc2tog {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2togReg0boOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2togReg0boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2togReg0boOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2togReg1boOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2togReg1boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2togReg1boOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2togReg2boOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2togReg2boOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2togReg2boOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2togReg0stepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2togReg0stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2togReg0stepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2togReg1stepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2togReg1stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2togReg1stepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2togReg2stepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2togReg2stepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2togReg2stepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Misc2tog {
    #[inline(always)]
    fn default() -> Misc2tog {
        Misc2tog(0)
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg1p1(pub u32);
impl Reg1p1 {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1outputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1outputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1outputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1 {
    #[inline(always)]
    fn default() -> Reg1p1 {
        Reg1p1(0)
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg1p1clr(pub u32);
impl Reg1p1clr {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1clrOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1clrOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1clrOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1clr {
    #[inline(always)]
    fn default() -> Reg1p1clr {
        Reg1p1clr(0)
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg1p1set(pub u32);
impl Reg1p1set {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1setOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1setOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1setOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1set {
    #[inline(always)]
    fn default() -> Reg1p1set {
        Reg1p1set(0)
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg1p1tog(pub u32);
impl Reg1p1tog {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1togOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1togOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1togOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1tog {
    #[inline(always)]
    fn default() -> Reg1p1tog {
        Reg1p1tog(0)
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg2p5(pub u32);
impl Reg2p5 {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5outputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5outputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5outputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5 {
    #[inline(always)]
    fn default() -> Reg2p5 {
        Reg2p5(0)
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg2p5clr(pub u32);
impl Reg2p5clr {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5clrOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5clrOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5clrOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5clr {
    #[inline(always)]
    fn default() -> Reg2p5clr {
        Reg2p5clr(0)
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg2p5set(pub u32);
impl Reg2p5set {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5setOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5setOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5setOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5set {
    #[inline(always)]
    fn default() -> Reg2p5set {
        Reg2p5set(0)
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg2p5tog(pub u32);
impl Reg2p5tog {
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5togOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5togOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5togOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5tog {
    #[inline(always)]
    fn default() -> Reg2p5tog {
        Reg2p5tog(0)
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg3p0(pub u32);
impl Reg3p0 {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0outputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0outputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0outputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0 {
    #[inline(always)]
    fn default() -> Reg3p0 {
        Reg3p0(0)
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg3p0clr(pub u32);
impl Reg3p0clr {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0clrOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0clrOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0clrOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0clr {
    #[inline(always)]
    fn default() -> Reg3p0clr {
        Reg3p0clr(0)
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg3p0set(pub u32);
impl Reg3p0set {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0setOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0setOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0setOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0set {
    #[inline(always)]
    fn default() -> Reg3p0set {
        Reg3p0set(0)
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Reg3p0tog(pub u32);
impl Reg3p0tog {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0togOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0togOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0togOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0tog {
    #[inline(always)]
    fn default() -> Reg3p0tog {
        Reg3p0tog(0)
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RegCore(pub u32);
impl RegCore {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreReg0targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreReg0targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreReg0targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreReg0adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreReg0adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreReg0adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreReg1targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreReg1targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreReg1targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreReg1adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreReg1adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreReg1adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreReg2targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreReg2targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreReg2targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreReg2adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreReg2adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreReg2adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCore {
    #[inline(always)]
    fn default() -> RegCore {
        RegCore(0)
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RegCoreClr(pub u32);
impl RegCoreClr {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreClrReg0targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreClrReg0targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreClrReg0targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreClrReg0adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreClrReg0adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreClrReg0adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreClrReg1targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreClrReg1targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreClrReg1targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreClrReg1adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreClrReg1adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreClrReg1adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreClrReg2targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreClrReg2targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreClrReg2targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreClrReg2adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreClrReg2adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreClrReg2adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreClrRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreClrRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreClrRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCoreClr {
    #[inline(always)]
    fn default() -> RegCoreClr {
        RegCoreClr(0)
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RegCoreSet(pub u32);
impl RegCoreSet {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreSetReg0targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreSetReg0targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreSetReg0targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreSetReg0adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreSetReg0adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreSetReg0adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreSetReg1targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreSetReg1targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreSetReg1targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreSetReg1adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreSetReg1adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreSetReg1adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreSetReg2targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreSetReg2targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreSetReg2targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreSetReg2adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreSetReg2adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreSetReg2adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreSetRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreSetRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreSetRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCoreSet {
    #[inline(always)]
    fn default() -> RegCoreSet {
        RegCoreSet(0)
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RegCoreTog(pub u32);
impl RegCoreTog {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreTogReg0targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreTogReg0targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreTogReg0targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreTogReg0adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreTogReg0adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreTogReg0adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreTogReg1targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreTogReg1targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreTogReg1targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreTogReg1adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreTogReg1adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreTogReg1adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreTogReg2targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreTogReg2targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreTogReg2targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreTogReg2adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreTogReg2adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreTogReg2adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreTogRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreTogRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreTogRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCoreTog {
    #[inline(always)]
    fn default() -> RegCoreTog {
        RegCoreTog(0)
    }
}
