#[doc = "Config security level register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Csl(pub u32);
impl Csl {
    #[doc = "Secure user read access control for the second slave"]
    #[inline(always)]
    pub const fn sur_s2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Secure user read access control for the second slave"]
    #[inline(always)]
    pub const fn set_sur_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub const fn ssr_s2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub const fn set_ssr_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Non-secure user read access control for the second slave"]
    #[inline(always)]
    pub const fn nur_s2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure user read access control for the second slave"]
    #[inline(always)]
    pub const fn set_nur_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub const fn nsr_s2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub const fn set_nsr_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure user write access control for the second slave"]
    #[inline(always)]
    pub const fn suw_s2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Secure user write access control for the second slave"]
    #[inline(always)]
    pub const fn set_suw_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub const fn ssw_s2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub const fn set_ssw_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure user write access control for the second slave"]
    #[inline(always)]
    pub const fn nuw_s2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure user write access control for the second slave"]
    #[inline(always)]
    pub const fn set_nuw_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub const fn nsw_s2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub const fn set_nsw_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    pub const fn lock_s2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    pub const fn set_lock_s2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure user read access control for the first slave"]
    #[inline(always)]
    pub const fn sur_s1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Secure user read access control for the first slave"]
    #[inline(always)]
    pub const fn set_sur_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub const fn ssr_s1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub const fn set_ssr_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Non-secure user read access control for the first slave"]
    #[inline(always)]
    pub const fn nur_s1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure user read access control for the first slave"]
    #[inline(always)]
    pub const fn set_nur_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub const fn nsr_s1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub const fn set_nsr_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Secure user write access control for the first slave"]
    #[inline(always)]
    pub const fn suw_s1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Secure user write access control for the first slave"]
    #[inline(always)]
    pub const fn set_suw_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub const fn ssw_s1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub const fn set_ssw_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Non-secure user write access control for the first slave"]
    #[inline(always)]
    pub const fn nuw_s1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure user write access control for the first slave"]
    #[inline(always)]
    pub const fn set_nuw_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub const fn nsw_s1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub const fn set_nsw_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    pub const fn lock_s1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    pub const fn set_lock_s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Csl {
    #[inline(always)]
    fn default() -> Csl {
        Csl(0)
    }
}
#[doc = "HP0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hp0(pub u32);
impl Hp0 {
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the eDMA"]
    #[inline(always)]
    pub const fn hp_dma(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the eDMA"]
    #[inline(always)]
    pub const fn set_hp_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn l_dma(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn set_l_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the LCDIF"]
    #[inline(always)]
    pub const fn hp_lcdif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the LCDIF"]
    #[inline(always)]
    pub const fn set_hp_lcdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn l_lcdif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn set_l_lcdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the CSI"]
    #[inline(always)]
    pub const fn hp_csi(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the CSI"]
    #[inline(always)]
    pub const fn set_hp_csi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn l_csi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn set_l_csi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the PXP"]
    #[inline(always)]
    pub const fn hp_pxp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the PXP"]
    #[inline(always)]
    pub const fn set_hp_pxp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn l_pxp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn set_l_pxp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the DCP"]
    #[inline(always)]
    pub const fn hp_dcp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the DCP"]
    #[inline(always)]
    pub const fn set_hp_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn l_dcp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn set_l_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET"]
    #[inline(always)]
    pub const fn hp_enet(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the ENET"]
    #[inline(always)]
    pub const fn set_hp_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn l_enet(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn set_l_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC1"]
    #[inline(always)]
    pub const fn hp_usdhc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC1"]
    #[inline(always)]
    pub const fn set_hp_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn l_usdhc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn set_l_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC2"]
    #[inline(always)]
    pub const fn hp_usdhc2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USDHC2"]
    #[inline(always)]
    pub const fn set_hp_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub const fn l_usdhc2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub const fn set_l_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the TPSMP"]
    #[inline(always)]
    pub const fn hp_tpsmp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the TPSMP"]
    #[inline(always)]
    pub const fn set_hp_tpsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub const fn l_tpsmp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub const fn set_l_tpsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USB"]
    #[inline(always)]
    pub const fn hp_usb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\] of the USB"]
    #[inline(always)]
    pub const fn set_hp_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub const fn l_usb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub const fn set_l_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Hp0 {
    #[inline(always)]
    fn default() -> Hp0 {
        Hp0(0)
    }
}
#[doc = "HPCONTROL0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hpcontrol0(pub u32);
impl Hpcontrol0 {
    #[doc = "Indicates the privilege/user mode for the eDMA"]
    #[inline(always)]
    pub const fn hpc_dma(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the eDMA"]
    #[inline(always)]
    pub const fn set_hpc_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn l_dma(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn set_l_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the privilege/user mode for the LCDIF"]
    #[inline(always)]
    pub const fn hpc_lcdif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the LCDIF"]
    #[inline(always)]
    pub const fn set_hpc_lcdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn l_lcdif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn set_l_lcdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates the privilege/user mode for the CSI"]
    #[inline(always)]
    pub const fn hpc_csi(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the CSI"]
    #[inline(always)]
    pub const fn set_hpc_csi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn l_csi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn set_l_csi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates the privilege/user mode for the PXP"]
    #[inline(always)]
    pub const fn hpc_pxp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the PXP"]
    #[inline(always)]
    pub const fn set_hpc_pxp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn l_pxp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn set_l_pxp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates the privilege/user mode for the DCP"]
    #[inline(always)]
    pub const fn hpc_dcp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the DCP"]
    #[inline(always)]
    pub const fn set_hpc_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn l_dcp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn set_l_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates the privilege/user mode for the ENET"]
    #[inline(always)]
    pub const fn hpc_enet(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the ENET"]
    #[inline(always)]
    pub const fn set_hpc_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn l_enet(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn set_l_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Indicates the privilege/user mode for the USDHC1"]
    #[inline(always)]
    pub const fn hpc_usdhc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the USDHC1"]
    #[inline(always)]
    pub const fn set_hpc_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn l_usdhc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn set_l_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Indicates the privilege/user mode for the USDHC2"]
    #[inline(always)]
    pub const fn hpc_usdhc2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the USDHC2"]
    #[inline(always)]
    pub const fn set_hpc_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2."]
    #[inline(always)]
    pub const fn l_usdhc2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2."]
    #[inline(always)]
    pub const fn set_l_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Indicates the privilege/user mode for the TPSMP"]
    #[inline(always)]
    pub const fn hpc_tpsmp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the TPSMP"]
    #[inline(always)]
    pub const fn set_hpc_tpsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP."]
    #[inline(always)]
    pub const fn l_tpsmp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP."]
    #[inline(always)]
    pub const fn set_l_tpsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Indicates the privilege/user mode for the USB"]
    #[inline(always)]
    pub const fn hpc_usb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the privilege/user mode for the USB"]
    #[inline(always)]
    pub const fn set_hpc_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit set by the TZ software for the USB."]
    #[inline(always)]
    pub const fn l_usb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USB."]
    #[inline(always)]
    pub const fn set_l_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Hpcontrol0 {
    #[inline(always)]
    fn default() -> Hpcontrol0 {
        Hpcontrol0(0)
    }
}
#[doc = "Secure access register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sa(pub u32);
impl Sa {
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_dma(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn l_dma(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub const fn set_l_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_lcdif(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_lcdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn l_lcdif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub const fn set_l_lcdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_csi(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_csi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn l_csi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub const fn set_l_csi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Non-Secure Access Policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_pxp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Non-Secure Access Policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_pxp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn l_pxp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub const fn set_l_pxp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_dcp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn l_dcp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub const fn set_l_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_enet(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn l_enet(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub const fn set_l_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_usdhc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn l_usdhc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub const fn set_l_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_usdhc2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub const fn l_usdhc2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub const fn set_l_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_tpsmp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_tpsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub const fn l_tpsmp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub const fn set_l_tpsmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn nsa_usb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure access policy indicator bit"]
    #[inline(always)]
    pub const fn set_nsa_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub const fn l_usb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub const fn set_l_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Sa {
    #[inline(always)]
    fn default() -> Sa {
        Sa(0)
    }
}
