#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Debug(pub u32);
impl Debug {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Debug {
    #[inline(always)]
    fn default() -> Debug {
        Debug(0)
    }
}
#[doc = "UTMI Debug Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Debug0status(pub u32);
impl Debug0status {
    #[doc = "Running count of the failed pseudo-random generator loopback"]
    #[inline(always)]
    pub const fn loop_back_fail_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Running count of the failed pseudo-random generator loopback"]
    #[inline(always)]
    pub const fn set_loop_back_fail_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Running count of the UTMI_RXERROR."]
    #[inline(always)]
    pub const fn utmi_rxerror_fail_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Running count of the UTMI_RXERROR."]
    #[inline(always)]
    pub const fn set_utmi_rxerror_fail_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Running count of the squelch reset instead of normal end for HS RX."]
    #[inline(always)]
    pub const fn squelch_count(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "Running count of the squelch reset instead of normal end for HS RX."]
    #[inline(always)]
    pub const fn set_squelch_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Debug0status {
    #[inline(always)]
    fn default() -> Debug0status {
        Debug0status(0)
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Debug1(pub u32);
impl Debug1 {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1 {
    #[inline(always)]
    fn default() -> Debug1 {
        Debug1(0)
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Debug1clr(pub u32);
impl Debug1clr {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1clr {
    #[inline(always)]
    fn default() -> Debug1clr {
        Debug1clr(0)
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Debug1set(pub u32);
impl Debug1set {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1set {
    #[inline(always)]
    fn default() -> Debug1set {
        Debug1set(0)
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Debug1tog(pub u32);
impl Debug1tog {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1tog {
    #[inline(always)]
    fn default() -> Debug1tog {
        Debug1tog(0)
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugClr(pub u32);
impl DebugClr {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DebugClr {
    #[inline(always)]
    fn default() -> DebugClr {
        DebugClr(0)
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugSet(pub u32);
impl DebugSet {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DebugSet {
    #[inline(always)]
    fn default() -> DebugSet {
        DebugSet(0)
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugTog(pub u32);
impl DebugTog {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DebugTog {
    #[inline(always)]
    fn default() -> DebugTog {
        DebugTog(0)
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pwd(pub u32);
impl Pwd {
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
    }
}
impl Default for Pwd {
    #[inline(always)]
    fn default() -> Pwd {
        Pwd(0)
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PwdClr(pub u32);
impl PwdClr {
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
    }
}
impl Default for PwdClr {
    #[inline(always)]
    fn default() -> PwdClr {
        PwdClr(0)
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PwdSet(pub u32);
impl PwdSet {
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
    }
}
impl Default for PwdSet {
    #[inline(always)]
    fn default() -> PwdSet {
        PwdSet(0)
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PwdTog(pub u32);
impl PwdTog {
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
    }
}
impl Default for PwdTog {
    #[inline(always)]
    fn default() -> PwdTog {
        PwdTog(0)
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rx(pub u32);
impl Rx {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for Rx {
    #[inline(always)]
    fn default() -> Rx {
        Rx(0)
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RxClr(pub u32);
impl RxClr {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for RxClr {
    #[inline(always)]
    fn default() -> RxClr {
        RxClr(0)
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RxSet(pub u32);
impl RxSet {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for RxSet {
    #[inline(always)]
    fn default() -> RxSet {
        RxSet(0)
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RxTog(pub u32);
impl RxTog {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for RxTog {
    #[inline(always)]
    fn default() -> RxTog {
        RxTog(0)
    }
}
#[doc = "USB PHY Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Indicates that the device has disconnected while in high-speed host mode."]
    #[inline(always)]
    pub const fn hostdiscondetect_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected while in high-speed host mode."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    #[inline(always)]
    pub const fn devplugin_status(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    #[inline(always)]
    pub const fn set_devplugin_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    pub const fn otgid_status(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    pub const fn set_otgid_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    #[inline(always)]
    pub const fn resume_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    #[inline(always)]
    pub const fn set_resume_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd4(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tx(pub u32);
impl Tx {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Tx {
    #[inline(always)]
    fn default() -> Tx {
        Tx(0)
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TxClr(pub u32);
impl TxClr {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for TxClr {
    #[inline(always)]
    fn default() -> TxClr {
        TxClr(0)
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TxSet(pub u32);
impl TxSet {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for TxSet {
    #[inline(always)]
    fn default() -> TxSet {
        TxSet(0)
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TxTog(pub u32);
impl TxTog {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for TxTog {
    #[inline(always)]
    fn default() -> TxTog {
        TxTog(0)
    }
}
#[doc = "UTMI RTL Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Fixed read-only value reflecting the stepping of the RTL version."]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fixed read-only value reflecting the stepping of the RTL version."]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version."]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL version."]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL version."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
