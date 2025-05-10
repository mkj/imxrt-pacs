#[doc = "USB OTG1 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtg1ctrl(pub u32);
impl UsbOtg1ctrl {
    #[doc = "Disable OTG1 Overcurrent Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_dis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Disable OTG1 Overcurrent Detection"]
    #[inline(always)]
    pub const fn set_over_cur_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_pol(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline(always)]
    pub const fn set_over_cur_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_pol(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline(always)]
    pub const fn set_pwr_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OTG1 Software Wake-up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Software Wake-up Enable"]
    #[inline(always)]
    pub const fn set_wkup_sw_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "OTG1 Software Wake-up"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Software Wake-up"]
    #[inline(always)]
    pub const fn set_wkup_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "OTG1 Wake-up on ID change enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_id_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Wake-up on ID change enable"]
    #[inline(always)]
    pub const fn set_wkup_id_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OTG1 wake-up on VBUS change enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_vbus_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 wake-up on VBUS change enable"]
    #[inline(always)]
    pub const fn set_wkup_vbus_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up on DPDM change enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_dpdm_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up on DPDM change enable"]
    #[inline(always)]
    pub const fn set_wkup_dpdm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
    #[must_use]
    #[inline(always)]
    pub const fn wir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
    #[inline(always)]
    pub const fn set_wir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UsbOtg1ctrl {
    #[inline(always)]
    fn default() -> UsbOtg1ctrl {
        UsbOtg1ctrl(0)
    }
}
impl core::fmt::Debug for UsbOtg1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtg1ctrl")
            .field("over_cur_dis", &self.over_cur_dis())
            .field("over_cur_pol", &self.over_cur_pol())
            .field("pwr_pol", &self.pwr_pol())
            .field("wie", &self.wie())
            .field("wkup_sw_en", &self.wkup_sw_en())
            .field("wkup_sw", &self.wkup_sw())
            .field("wkup_id_en", &self.wkup_id_en())
            .field("wkup_vbus_en", &self.wkup_vbus_en())
            .field("wkup_dpdm_en", &self.wkup_dpdm_en())
            .field("wir", &self.wir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtg1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "UsbOtg1ctrl {{ over_cur_dis: {=bool:?}, over_cur_pol: {=bool:?}, pwr_pol: {=bool:?}, wie: {=bool:?}, wkup_sw_en: {=bool:?}, wkup_sw: {=bool:?}, wkup_id_en: {=bool:?}, wkup_vbus_en: {=bool:?}, wkup_dpdm_en: {=bool:?}, wir: {=bool:?} }}" , self . over_cur_dis () , self . over_cur_pol () , self . pwr_pol () , self . wie () , self . wkup_sw_en () , self . wkup_sw () , self . wkup_id_en () , self . wkup_vbus_en () , self . wkup_dpdm_en () , self . wir ())
    }
}
#[doc = "OTG1 UTMI PHY Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtg1phyCtrl0(pub u32);
impl UsbOtg1phyCtrl0 {
    #[doc = "Indicating whether OTG1 UTMI PHY clock is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_clk_vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicating whether OTG1 UTMI PHY clock is valid"]
    #[inline(always)]
    pub const fn set_utmi_clk_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UsbOtg1phyCtrl0 {
    #[inline(always)]
    fn default() -> UsbOtg1phyCtrl0 {
        UsbOtg1phyCtrl0(0)
    }
}
impl core::fmt::Debug for UsbOtg1phyCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtg1phyCtrl0")
            .field("utmi_clk_vld", &self.utmi_clk_vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtg1phyCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbOtg1phyCtrl0 {{ utmi_clk_vld: {=bool:?} }}",
            self.utmi_clk_vld()
        )
    }
}
