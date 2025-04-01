#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LowpwrCtrl(pub u32);
impl LowpwrCtrl {
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn osc_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(&mut self, val: super::vals::LowpwrCtrlXtaloscPwrupDelay) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrl {
    #[inline(always)]
    fn default() -> LowpwrCtrl {
        LowpwrCtrl(0)
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LowpwrCtrlClr(pub u32);
impl LowpwrCtrlClr {
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn osc_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlClrXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlClrXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(
        &mut self,
        val: super::vals::LowpwrCtrlClrXtaloscPwrupDelay,
    ) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrlClr {
    #[inline(always)]
    fn default() -> LowpwrCtrlClr {
        LowpwrCtrlClr(0)
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LowpwrCtrlSet(pub u32);
impl LowpwrCtrlSet {
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn osc_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlSetXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlSetXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(
        &mut self,
        val: super::vals::LowpwrCtrlSetXtaloscPwrupDelay,
    ) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrlSet {
    #[inline(always)]
    fn default() -> LowpwrCtrlSet {
        LowpwrCtrlSet(0)
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LowpwrCtrlTog(pub u32);
impl LowpwrCtrlTog {
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn osc_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlTogXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlTogXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(
        &mut self,
        val: super::vals::LowpwrCtrlTogXtaloscPwrupDelay,
    ) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrlTog {
    #[inline(always)]
    fn default() -> LowpwrCtrlTog {
        LowpwrCtrlTog(0)
    }
}
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
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0reftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0reftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0reftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0stopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0stopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0clrReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0clrReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0clrReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0clrStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0clrStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0setReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0setReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0setReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0setStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0setStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0togReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0togReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0togReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0togStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0togStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc0tog {
    #[inline(always)]
    fn default() -> Misc0tog {
        Misc0tog(0)
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig0(pub u32);
impl OscConfig0 {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0 {
    #[inline(always)]
    fn default() -> OscConfig0 {
        OscConfig0(0)
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig0clr(pub u32);
impl OscConfig0clr {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0clr {
    #[inline(always)]
    fn default() -> OscConfig0clr {
        OscConfig0clr(0)
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig0set(pub u32);
impl OscConfig0set {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0set {
    #[inline(always)]
    fn default() -> OscConfig0set {
        OscConfig0set(0)
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig0tog(pub u32);
impl OscConfig0tog {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0tog {
    #[inline(always)]
    fn default() -> OscConfig0tog {
        OscConfig0tog(0)
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig1(pub u32);
impl OscConfig1 {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1 {
    #[inline(always)]
    fn default() -> OscConfig1 {
        OscConfig1(0)
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig1clr(pub u32);
impl OscConfig1clr {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1clr {
    #[inline(always)]
    fn default() -> OscConfig1clr {
        OscConfig1clr(0)
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig1set(pub u32);
impl OscConfig1set {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1set {
    #[inline(always)]
    fn default() -> OscConfig1set {
        OscConfig1set(0)
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig1tog(pub u32);
impl OscConfig1tog {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1tog {
    #[inline(always)]
    fn default() -> OscConfig1tog {
        OscConfig1tog(0)
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig2(pub u32);
impl OscConfig2 {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2 {
    #[inline(always)]
    fn default() -> OscConfig2 {
        OscConfig2(0)
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig2clr(pub u32);
impl OscConfig2clr {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2clr {
    #[inline(always)]
    fn default() -> OscConfig2clr {
        OscConfig2clr(0)
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig2set(pub u32);
impl OscConfig2set {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2set {
    #[inline(always)]
    fn default() -> OscConfig2set {
        OscConfig2set(0)
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OscConfig2tog(pub u32);
impl OscConfig2tog {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2tog {
    #[inline(always)]
    fn default() -> OscConfig2tog {
        OscConfig2tog(0)
    }
}
