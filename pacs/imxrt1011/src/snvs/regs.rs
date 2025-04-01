#[doc = "SNVS_HP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpcomr(pub u32);
impl Hpcomr {
    #[doc = "SSM State Transition Transition state of the system security monitor"]
    #[inline(always)]
    pub const fn ssm_st(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSM State Transition Transition state of the system security monitor"]
    #[inline(always)]
    pub const fn set_ssm_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    pub const fn ssm_st_dis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    pub const fn set_ssm_st_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    pub const fn ssm_sfns_dis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    pub const fn set_ssm_sfns_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    #[inline(always)]
    pub const fn lp_swr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    #[inline(always)]
    pub const fn set_lp_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    pub const fn lp_swr_dis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    pub const fn set_lp_swr_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    pub const fn sw_sv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    pub const fn set_sw_sv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    pub const fn sw_fsv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    pub const fn set_sw_fsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    pub const fn sw_lpsv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    pub const fn set_sw_lpsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[inline(always)]
    pub const fn prog_zmk(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[inline(always)]
    pub const fn set_prog_zmk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    pub const fn mks_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    pub const fn set_mks_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    pub const fn hac_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    pub const fn set_hac_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[inline(always)]
    pub const fn hac_load(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[inline(always)]
    pub const fn set_hac_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[inline(always)]
    pub const fn hac_clear(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[inline(always)]
    pub const fn set_hac_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    pub const fn hac_stop(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    pub const fn set_hac_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    pub const fn npswa_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    pub const fn set_npswa_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpcomr {
    #[inline(always)]
    fn default() -> Hpcomr {
        Hpcomr(0)
    }
}
#[doc = "SNVS_HP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpcr(pub u32);
impl Hpcr {
    #[doc = "HP Real Time Counter Enable"]
    #[inline(always)]
    pub const fn rtc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HP Real Time Counter Enable"]
    #[inline(always)]
    pub const fn set_rtc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    pub const fn hpta_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    pub const fn set_hpta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    pub const fn dis_pi(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    pub const fn set_dis_pi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    pub const fn pi_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    pub const fn set_pi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    pub const fn pi_freq(&self) -> super::vals::PiFreq {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::PiFreq::from_bits(val as u8)
    }
    #[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    pub const fn set_pi_freq(&mut self, val: super::vals::PiFreq) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    pub const fn hpcalb_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    pub const fn set_hpcalb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    pub const fn hpcalb_val(&self) -> super::vals::HpcalbVal {
        let val = (self.0 >> 10usize) & 0x1f;
        super::vals::HpcalbVal::from_bits(val as u8)
    }
    #[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    pub const fn set_hpcalb_val(&mut self, val: super::vals::HpcalbVal) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "HP Time Synchronize"]
    #[inline(always)]
    pub const fn hp_ts(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "HP Time Synchronize"]
    #[inline(always)]
    pub const fn set_hp_ts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Button Configuration"]
    #[inline(always)]
    pub const fn btn_config(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Button Configuration"]
    #[inline(always)]
    pub const fn set_btn_config(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Button interrupt mask"]
    #[inline(always)]
    pub const fn btn_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Button interrupt mask"]
    #[inline(always)]
    pub const fn set_btn_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Hpcr {
    #[inline(always)]
    fn default() -> Hpcr {
        Hpcr(0)
    }
}
#[doc = "SNVS_HP Lock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hplr(pub u32);
impl Hplr {
    #[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub const fn zmk_wsl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_wsl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub const fn zmk_rsl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_rsl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub const fn srtc_sl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub const fn set_srtc_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub const fn lpcalb_sl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub const fn set_lpcalb_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub const fn mc_sl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub const fn set_mc_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub const fn gpr_sl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub const fn set_gpr_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub const fn lpsvcr_sl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub const fn set_lpsvcr_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Soft Lock When set, prevents any writes to the LPTGFCR"]
    #[inline(always)]
    pub const fn lptgfcr_sl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Soft Lock When set, prevents any writes to the LPTGFCR"]
    #[inline(always)]
    pub const fn set_lptgfcr_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub const fn lpsecr_sl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub const fn set_lpsecr_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    pub const fn mks_sl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    pub const fn set_mks_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    pub const fn hpsvcr_l(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    pub const fn set_hpsvcr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    pub const fn hpsicr_l(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    pub const fn set_hpsicr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    pub const fn hac_l(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    pub const fn set_hac_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Hplr {
    #[inline(always)]
    fn default() -> Hplr {
        Hplr(0)
    }
}
#[doc = "SNVS_HP Real Time Counter MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hprtcmr(pub u32);
impl Hprtcmr {
    #[doc = "HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    pub const fn rtc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Hprtcmr {
    #[inline(always)]
    fn default() -> Hprtcmr {
        Hprtcmr(0)
    }
}
#[doc = "SNVS_HP Security Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpsicr(pub u32);
impl Hpsicr {
    #[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    pub const fn sv0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    pub const fn set_sv0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    pub const fn sv1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    pub const fn set_sv1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    pub const fn sv2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    pub const fn set_sv2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    pub const fn sv3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    pub const fn set_sv3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    pub const fn sv4_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    pub const fn set_sv4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    pub const fn sv5_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    pub const fn set_sv5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    pub const fn lpsvi_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    pub const fn set_lpsvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpsicr {
    #[inline(always)]
    fn default() -> Hpsicr {
        Hpsicr(0)
    }
}
#[doc = "SNVS_HP Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpsr(pub u32);
impl Hpsr {
    #[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    pub const fn hpta(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    pub const fn set_hpta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    pub const fn pi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    pub const fn set_pi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[inline(always)]
    pub const fn lpdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[inline(always)]
    pub const fn set_lpdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Button Value of the BTN input"]
    #[inline(always)]
    pub const fn btn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Button Value of the BTN input"]
    #[inline(always)]
    pub const fn set_btn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    pub const fn bi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    pub const fn set_bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[inline(always)]
    pub const fn ssm_state(&self) -> super::vals::SsmState {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::SsmState::from_bits(val as u8)
    }
    #[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[inline(always)]
    pub const fn set_ssm_state(&mut self, val: super::vals::SsmState) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "System Security Configuration This field reflects the three security configuration inputs to SNVS"]
    #[inline(always)]
    pub const fn sys_security_cfg(&self) -> super::vals::SysSecurityCfg {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::SysSecurityCfg::from_bits(val as u8)
    }
    #[doc = "System Security Configuration This field reflects the three security configuration inputs to SNVS"]
    #[inline(always)]
    pub const fn set_sys_security_cfg(&mut self, val: super::vals::SysSecurityCfg) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
    #[inline(always)]
    pub const fn sys_secure_boot(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
    #[inline(always)]
    pub const fn set_sys_secure_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[inline(always)]
    pub const fn otpmk_syndrome(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[inline(always)]
    pub const fn set_otpmk_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "One Time Programmable Master Key is Equal to Zero"]
    #[inline(always)]
    pub const fn otpmk_zero(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "One Time Programmable Master Key is Equal to Zero"]
    #[inline(always)]
    pub const fn set_otpmk_zero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Zeroizable Master Key is Equal to Zero"]
    #[inline(always)]
    pub const fn zmk_zero(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key is Equal to Zero"]
    #[inline(always)]
    pub const fn set_zmk_zero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpsr {
    #[inline(always)]
    fn default() -> Hpsr {
        Hpsr(0)
    }
}
#[doc = "SNVS_HP Security Violation Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpsvcr(pub u32);
impl Hpsvcr {
    #[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    pub const fn sv0_cfg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv0_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    pub const fn sv1_cfg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv1_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    pub const fn sv2_cfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv2_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    pub const fn sv3_cfg(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv3_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    pub const fn sv4_cfg(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv4_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    pub const fn sv5_cfg(&self) -> super::vals::Sv5cfg {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Sv5cfg::from_bits(val as u8)
    }
    #[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    pub const fn set_sv5_cfg(&mut self, val: super::vals::Sv5cfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    pub const fn lpsv_cfg(&self) -> super::vals::LpsvCfg {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::LpsvCfg::from_bits(val as u8)
    }
    #[doc = "LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    pub const fn set_lpsv_cfg(&mut self, val: super::vals::LpsvCfg) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Hpsvcr {
    #[inline(always)]
    fn default() -> Hpsvcr {
        Hpsvcr(0)
    }
}
#[doc = "SNVS_HP Security Violation Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpsvsr(pub u32);
impl Hpsvsr {
    #[doc = "Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub const fn sv0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub const fn sv1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub const fn sv2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub const fn sv3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub const fn sv4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub const fn sv5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub const fn set_sv5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn sw_sv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn set_sw_sv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn sw_fsv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn set_sw_fsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn sw_lpsv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[inline(always)]
    pub const fn set_sw_lpsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[inline(always)]
    pub const fn zmk_syndrome(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[inline(always)]
    pub const fn set_zmk_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    pub const fn zmk_ecc_fail(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    pub const fn set_zmk_ecc_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LP Security Violation A security volation was detected in the SNVS low power section"]
    #[inline(always)]
    pub const fn lp_sec_vio(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Violation A security volation was detected in the SNVS low power section"]
    #[inline(always)]
    pub const fn set_lp_sec_vio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hpsvsr {
    #[inline(always)]
    fn default() -> Hpsvsr {
        Hpsvsr(0)
    }
}
#[doc = "SNVS_HP Time Alarm MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hptamr(pub u32);
impl Hptamr {
    #[doc = "HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    pub const fn hpta_ms(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    pub const fn set_hpta_ms(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Hptamr {
    #[inline(always)]
    fn default() -> Hptamr {
        Hptamr(0)
    }
}
#[doc = "SNVS_HP Version ID Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpvidr1(pub u32);
impl Hpvidr1 {
    #[doc = "SNVS block minor version number"]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS block minor version number"]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SNVS block major version number"]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS block major version number"]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "SNVS block ID"]
    #[inline(always)]
    pub const fn ip_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "SNVS block ID"]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hpvidr1 {
    #[inline(always)]
    fn default() -> Hpvidr1 {
        Hpvidr1(0)
    }
}
#[doc = "SNVS_HP Version ID Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpvidr2(pub u32);
impl Hpvidr2 {
    #[doc = "SNVS Configuration Options"]
    #[inline(always)]
    pub const fn config_opt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS Configuration Options"]
    #[inline(always)]
    pub const fn set_config_opt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SNVS ECO Revision"]
    #[inline(always)]
    pub const fn eco_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS ECO Revision"]
    #[inline(always)]
    pub const fn set_eco_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "SNVS Integration Options"]
    #[inline(always)]
    pub const fn intg_opt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "SNVS Integration Options"]
    #[inline(always)]
    pub const fn set_intg_opt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
    #[inline(always)]
    pub const fn ip_era(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
    #[inline(always)]
    pub const fn set_ip_era(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hpvidr2 {
    #[inline(always)]
    fn default() -> Hpvidr2 {
        Hpvidr2(0)
    }
}
#[doc = "SNVS_LP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpcr(pub u32);
impl Lpcr {
    #[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    pub const fn srtc_env(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    pub const fn set_srtc_env(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    pub const fn lpta_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    pub const fn set_lpta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    pub const fn mc_env(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    pub const fn set_mc_env(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    pub const fn lpwui_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    pub const fn set_lpwui_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    pub const fn srtc_inv_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    pub const fn set_srtc_inv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    pub const fn dp_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    pub const fn set_dp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    pub const fn top(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    pub const fn set_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    pub const fn lvd_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    pub const fn set_lvd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    pub const fn lpcalb_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    pub const fn set_lpcalb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    pub const fn lpcalb_val(&self) -> super::vals::LpcalbVal {
        let val = (self.0 >> 10usize) & 0x1f;
        super::vals::LpcalbVal::from_bits(val as u8)
    }
    #[doc = "LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    pub const fn set_lpcalb_val(&mut self, val: super::vals::LpcalbVal) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val.to_bits() as u32) & 0x1f) << 10usize);
    }
    #[doc = "This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    pub const fn btn_press_time(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    pub const fn set_btn_press_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    pub const fn debounce(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    pub const fn set_debounce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    pub const fn on_time(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    pub const fn set_on_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    pub const fn pk_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    pub const fn set_pk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    pub const fn pk_override(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    pub const fn set_pk_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    pub const fn gpr_z_dis(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    pub const fn set_gpr_z_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Lpcr {
    #[inline(always)]
    fn default() -> Lpcr {
        Lpcr(0)
    }
}
#[doc = "SNVS_LP Lock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lplr(pub u32);
impl Lplr {
    #[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub const fn zmk_whl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_whl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub const fn zmk_rhl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub const fn set_zmk_rhl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub const fn srtc_hl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub const fn set_srtc_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub const fn lpcalb_hl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub const fn set_lpcalb_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub const fn mc_hl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub const fn set_mc_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub const fn gpr_hl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub const fn set_gpr_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub const fn lpsvcr_hl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub const fn set_lpsvcr_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Hard Lock When set, prevents any writes to the LPTGFCR"]
    #[inline(always)]
    pub const fn lptgfcr_hl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LP Tamper Glitch Filter Configuration Register Hard Lock When set, prevents any writes to the LPTGFCR"]
    #[inline(always)]
    pub const fn set_lptgfcr_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub const fn lpsecr_hl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub const fn set_lpsecr_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    pub const fn mks_hl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    pub const fn set_mks_hl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Lplr {
    #[inline(always)]
    fn default() -> Lplr {
        Lplr(0)
    }
}
#[doc = "SNVS_LP Master Key Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpmkcr(pub u32);
impl Lpmkcr {
    #[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    pub const fn master_key_sel(&self) -> super::vals::MasterKeySel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MasterKeySel::from_bits(val as u8)
    }
    #[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    pub const fn set_master_key_sel(&mut self, val: super::vals::MasterKeySel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    pub const fn zmk_hwp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    pub const fn set_zmk_hwp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    pub const fn zmk_val(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    pub const fn set_zmk_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    pub const fn zmk_ecc_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    pub const fn set_zmk_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[inline(always)]
    pub const fn zmk_ecc_value(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[inline(always)]
    pub const fn set_zmk_ecc_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
}
impl Default for Lpmkcr {
    #[inline(always)]
    fn default() -> Lpmkcr {
        Lpmkcr(0)
    }
}
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpsmcmr(pub u32);
impl Lpsmcmr {
    #[doc = "Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    pub const fn mon_counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    pub const fn set_mon_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    #[inline(always)]
    pub const fn mc_era_bits(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    #[inline(always)]
    pub const fn set_mc_era_bits(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Lpsmcmr {
    #[inline(always)]
    fn default() -> Lpsmcmr {
        Lpsmcmr(0)
    }
}
#[doc = "SNVS_LP Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpsr(pub u32);
impl Lpsr {
    #[doc = "LP Time Alarm"]
    #[inline(always)]
    pub const fn lpta(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LP Time Alarm"]
    #[inline(always)]
    pub const fn set_lpta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure Real Time Counter Rollover"]
    #[inline(always)]
    pub const fn srtcr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Real Time Counter Rollover"]
    #[inline(always)]
    pub const fn set_srtcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Monotonic Counter Rollover"]
    #[inline(always)]
    pub const fn mcr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Monotonic Counter Rollover"]
    #[inline(always)]
    pub const fn set_mcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Digital Low Voltage Event Detected"]
    #[inline(always)]
    pub const fn lvd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Low Voltage Event Detected"]
    #[inline(always)]
    pub const fn set_lvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Tampering Detected"]
    #[inline(always)]
    pub const fn ctd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Tampering Detected"]
    #[inline(always)]
    pub const fn set_ctd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Temperature Tamper Detected"]
    #[inline(always)]
    pub const fn ttd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Tamper Detected"]
    #[inline(always)]
    pub const fn set_ttd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Voltage Tampering Detected"]
    #[inline(always)]
    pub const fn vtd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Tampering Detected"]
    #[inline(always)]
    pub const fn set_vtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Wire-Mesh Tampering 1 Detected"]
    #[inline(always)]
    pub const fn wmt1d(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tampering 1 Detected"]
    #[inline(always)]
    pub const fn set_wmt1d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wire-Mesh Tampering 2 Detected"]
    #[inline(always)]
    pub const fn wmt2d(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tampering 2 Detected"]
    #[inline(always)]
    pub const fn set_wmt2d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "External Tampering 1 Detected"]
    #[inline(always)]
    pub const fn et1d(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 1 Detected"]
    #[inline(always)]
    pub const fn set_et1d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "External Tampering 2 Detected"]
    #[inline(always)]
    pub const fn et2d(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 2 Detected"]
    #[inline(always)]
    pub const fn set_et2d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    pub const fn esvd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    pub const fn set_esvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    pub const fn eo(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    pub const fn set_eo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub const fn spof(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub const fn set_spof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub const fn spon(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub const fn set_spon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[inline(always)]
    pub const fn lpns(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[inline(always)]
    pub const fn set_lpns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[inline(always)]
    pub const fn lps(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[inline(always)]
    pub const fn set_lps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Lpsr {
    #[inline(always)]
    fn default() -> Lpsr {
        Lpsr(0)
    }
}
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpsrtcmr(pub u32);
impl Lpsrtcmr {
    #[doc = "LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    pub const fn srtc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    pub const fn set_srtc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Lpsrtcmr {
    #[inline(always)]
    fn default() -> Lpsrtcmr {
        Lpsrtcmr(0)
    }
}
#[doc = "SNVS_LP Security Violation Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lpsvcr(pub u32);
impl Lpsvcr {
    #[doc = "Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline(always)]
    pub const fn sv0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline(always)]
    pub const fn set_sv0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline(always)]
    pub const fn sv1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline(always)]
    pub const fn set_sv1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline(always)]
    pub const fn sv2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline(always)]
    pub const fn set_sv2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline(always)]
    pub const fn sv3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline(always)]
    pub const fn set_sv3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline(always)]
    pub const fn sv4_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline(always)]
    pub const fn set_sv4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline(always)]
    pub const fn sv5_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline(always)]
    pub const fn set_sv5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Lpsvcr {
    #[inline(always)]
    fn default() -> Lpsvcr {
        Lpsvcr(0)
    }
}
#[doc = "SNVS_LP Tamper Detect Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lptdcr(pub u32);
impl Lptdcr {
    #[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    pub const fn srtcr_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    pub const fn set_srtcr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    pub const fn mcr_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    pub const fn set_mcr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Tamper Enable When set, a clock monitor tamper generates an LP security violation."]
    #[inline(always)]
    pub const fn ct_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Tamper Enable When set, a clock monitor tamper generates an LP security violation."]
    #[inline(always)]
    pub const fn set_ct_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Temperature Tamper Enable When set, a temperature monitor tamper generates an LP security violation"]
    #[inline(always)]
    pub const fn tt_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Tamper Enable When set, a temperature monitor tamper generates an LP security violation"]
    #[inline(always)]
    pub const fn set_tt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Voltage Tamper Enable Voltage Tamper Enable should be enabled 500 us after setting SCSC_SOSC_CTR \\[VOLT_TEMP_TAMPER_EN\\]"]
    #[inline(always)]
    pub const fn vt_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Tamper Enable Voltage Tamper Enable should be enabled 500 us after setting SCSC_SOSC_CTR \\[VOLT_TEMP_TAMPER_EN\\]"]
    #[inline(always)]
    pub const fn set_vt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Wire-Mesh Tampering 1 Enable When set, wire-mesh tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn wmt1_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tampering 1 Enable When set, wire-mesh tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_wmt1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wire-Mesh Tampering 2 Enable When set, wire-mesh tampering 2 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn wmt2_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tampering 2 Enable When set, wire-mesh tampering 2 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_wmt2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn et1_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_et1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "External Tampering 2 Enable When set, external tampering 2 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn et2_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 2 Enable When set, external tampering 2 detection generates an LP security violation"]
    #[inline(always)]
    pub const fn set_et2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline(always)]
    pub const fn et1p(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline(always)]
    pub const fn set_et1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "External Tampering 2 Polarity This bit is used to determine the polarity of external tamper 2."]
    #[inline(always)]
    pub const fn et2p(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "External Tampering 2 Polarity This bit is used to determine the polarity of external tamper 2."]
    #[inline(always)]
    pub const fn set_et2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    pub const fn pfd_observ(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    pub const fn set_pfd_observ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    pub const fn por_observ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    pub const fn set_por_observ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub const fn ltdc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub const fn set_ltdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    #[inline(always)]
    pub const fn htdc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    #[inline(always)]
    pub const fn set_htdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub const fn vrc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub const fn set_vrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    pub const fn oscb(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    pub const fn set_oscb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Lptdcr {
    #[inline(always)]
    fn default() -> Lptdcr {
        Lptdcr(0)
    }
}
#[doc = "SNVS_LP Tamper Glitch Filters Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Lptgfcr(pub u32);
impl Lptgfcr {
    #[doc = "Wire-Mesh Tamper Glitch Filter Configures the length of the digital glitch filter for the wire-mesh tamper 1 and 2 pins between 1 and 63 SRTC clock cycles"]
    #[inline(always)]
    pub const fn wmtgf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Configures the length of the digital glitch filter for the wire-mesh tamper 1 and 2 pins between 1 and 63 SRTC clock cycles"]
    #[inline(always)]
    pub const fn set_wmtgf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Enable When set, enables the wire-mesh tamper glitch filter"]
    #[inline(always)]
    pub const fn wmtgf_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Enable When set, enables the wire-mesh tamper glitch filter"]
    #[inline(always)]
    pub const fn set_wmtgf_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "External Tamper Glitch Filter 1 Configures the length of the digital glitch filter for the external tamper 1 pin between 128 and 32640 SRTC clock cycles"]
    #[inline(always)]
    pub const fn etgf1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "External Tamper Glitch Filter 1 Configures the length of the digital glitch filter for the external tamper 1 pin between 128 and 32640 SRTC clock cycles"]
    #[inline(always)]
    pub const fn set_etgf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "External Tamper Glitch Filter 1 Enable When set, enables the external tamper glitch filter 1."]
    #[inline(always)]
    pub const fn etgf1_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "External Tamper Glitch Filter 1 Enable When set, enables the external tamper glitch filter 1."]
    #[inline(always)]
    pub const fn set_etgf1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "External Tamper Glitch Filter 2 Configures the length of the digital glitch filter for the external tamper 2 pin between 128 and 32640 SRTC clock cycles"]
    #[inline(always)]
    pub const fn etgf2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "External Tamper Glitch Filter 2 Configures the length of the digital glitch filter for the external tamper 2 pin between 128 and 32640 SRTC clock cycles"]
    #[inline(always)]
    pub const fn set_etgf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "External Tamper Glitch Filter 2 Enable When set, enables the external tamper glitch filter 2."]
    #[inline(always)]
    pub const fn etgf2_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "External Tamper Glitch Filter 2 Enable When set, enables the external tamper glitch filter 2."]
    #[inline(always)]
    pub const fn set_etgf2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Lptgfcr {
    #[inline(always)]
    fn default() -> Lptgfcr {
        Lptgfcr(0)
    }
}
