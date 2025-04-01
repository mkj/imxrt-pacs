#[doc = "GPR1 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr1(pub u32);
impl Gpr1 {
    #[doc = "SAI1 MCLK1 source select"]
    #[inline(always)]
    pub const fn sai1_mclk1_sel(&self) -> super::vals::Sai1mclk1sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sai1mclk1sel::from_bits(val as u8)
    }
    #[doc = "SAI1 MCLK1 source select"]
    #[inline(always)]
    pub const fn set_sai1_mclk1_sel(&mut self, val: super::vals::Sai1mclk1sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SAI1 MCLK2 source select"]
    #[inline(always)]
    pub const fn sai1_mclk2_sel(&self) -> super::vals::Sai1mclk2sel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sai1mclk2sel::from_bits(val as u8)
    }
    #[doc = "SAI1 MCLK2 source select"]
    #[inline(always)]
    pub const fn set_sai1_mclk2_sel(&mut self, val: super::vals::Sai1mclk2sel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "SAI1 MCLK3 source select"]
    #[inline(always)]
    pub const fn sai1_mclk3_sel(&self) -> super::vals::Sai1mclk3sel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sai1mclk3sel::from_bits(val as u8)
    }
    #[doc = "SAI1 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai1_mclk3_sel(&mut self, val: super::vals::Sai1mclk3sel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "SAI3 MCLK3 source select"]
    #[inline(always)]
    pub const fn sai3_mclk3_sel(&self) -> super::vals::Sai3mclk3sel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sai3mclk3sel::from_bits(val as u8)
    }
    #[doc = "SAI3 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai3_mclk3_sel(&mut self, val: super::vals::Sai3mclk3sel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Global Interrupt"]
    #[inline(always)]
    pub const fn gint(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt"]
    #[inline(always)]
    pub const fn set_gint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "sai1.MCLK signal direction control"]
    #[inline(always)]
    pub const fn sai1_mclk_dir(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "sai1.MCLK signal direction control"]
    #[inline(always)]
    pub const fn set_sai1_mclk_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "sai3.MCLK signal direction control"]
    #[inline(always)]
    pub const fn sai3_mclk_dir(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "sai3.MCLK signal direction control"]
    #[inline(always)]
    pub const fn set_sai3_mclk_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub const fn exc_mon(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub const fn set_exc_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Arm CM7 platform AHB clock enable"]
    #[inline(always)]
    pub const fn cm7_force_hclk_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Arm CM7 platform AHB clock enable"]
    #[inline(always)]
    pub const fn set_cm7_force_hclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr1 {
    #[inline(always)]
    fn default() -> Gpr1 {
        Gpr1(0)
    }
}
#[doc = "GPR10 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr10(pub u32);
impl Gpr10 {
    #[doc = "Arm non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub const fn niden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Arm invasive debug enable"]
    #[inline(always)]
    pub const fn dbg_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Arm invasive debug enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline(always)]
    pub const fn sec_err_resp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline(always)]
    pub const fn set_sec_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DCP Key selection bit."]
    #[inline(always)]
    pub const fn dcpkey_ocotp_or_keymux(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DCP Key selection bit."]
    #[inline(always)]
    pub const fn set_dcpkey_ocotp_or_keymux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub const fn ocram_tz_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub const fn set_ocram_tz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub const fn ocram_tz_addr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub const fn set_ocram_tz_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "Lock NIDEN field for changes"]
    #[inline(always)]
    pub const fn lock_niden(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Lock NIDEN field for changes"]
    #[inline(always)]
    pub const fn set_lock_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock DBG_EN field for changes"]
    #[inline(always)]
    pub const fn lock_dbg_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock DBG_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_dbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub const fn lock_sec_err_resp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub const fn set_lock_sec_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub const fn lock_dcpkey_ocotp_or_keymux(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub const fn set_lock_dcpkey_ocotp_or_keymux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub const fn lock_ocram_tz_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram_tz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub const fn lock_ocram_tz_addr(&self) -> super::vals::LockOcramTzAddr {
        let val = (self.0 >> 25usize) & 0x1f;
        super::vals::LockOcramTzAddr::from_bits(val as u8)
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram_tz_addr(&mut self, val: super::vals::LockOcramTzAddr) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val.to_bits() as u32) & 0x1f) << 25usize);
    }
}
impl Default for Gpr10 {
    #[inline(always)]
    fn default() -> Gpr10 {
        Gpr10(0)
    }
}
#[doc = "GPR11 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr11(pub u32);
impl Gpr11 {
    #[doc = "Access control of memory region-0"]
    #[inline(always)]
    pub const fn m7_apc_ac_r0_ctrl(&self) -> super::vals::M7apcAcR0ctrl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::M7apcAcR0ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-0"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r0_ctrl(&mut self, val: super::vals::M7apcAcR0ctrl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Access control of memory region-1"]
    #[inline(always)]
    pub const fn m7_apc_ac_r1_ctrl(&self) -> super::vals::M7apcAcR1ctrl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::M7apcAcR1ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-1"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r1_ctrl(&mut self, val: super::vals::M7apcAcR1ctrl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Access control of memory region-2"]
    #[inline(always)]
    pub const fn m7_apc_ac_r2_ctrl(&self) -> super::vals::M7apcAcR2ctrl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::M7apcAcR2ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-2"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r2_ctrl(&mut self, val: super::vals::M7apcAcR2ctrl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Access control of memory region-3"]
    #[inline(always)]
    pub const fn m7_apc_ac_r3_ctrl(&self) -> super::vals::M7apcAcR3ctrl {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::M7apcAcR3ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-3"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r3_ctrl(&mut self, val: super::vals::M7apcAcR3ctrl) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r0_ctrl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r1_ctrl(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r2_ctrl(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r3_ctrl(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
}
impl Default for Gpr11 {
    #[inline(always)]
    fn default() -> Gpr11 {
        Gpr11(0)
    }
}
#[doc = "GPR12 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr12(pub u32);
impl Gpr12 {
    #[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn flexio1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_flexio1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub const fn flexio1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_flexio1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Gpr12 {
    #[inline(always)]
    fn default() -> Gpr12 {
        Gpr12(0)
    }
}
#[doc = "GPR13 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr13(pub u32);
impl Gpr13 {
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub const fn cache_usb(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub const fn set_cache_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Gpr13 {
    #[inline(always)]
    fn default() -> Gpr13 {
        Gpr13(0)
    }
}
#[doc = "GPR16 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr16(pub u32);
impl Gpr16 {
    #[doc = "FlexRAM bank config source select"]
    #[inline(always)]
    pub const fn flexram_bank_cfg_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FlexRAM bank config source select"]
    #[inline(always)]
    pub const fn set_flexram_bank_cfg_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock CM7_INIT_VTOR field for changes"]
    #[inline(always)]
    pub const fn lock_vtor(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Lock CM7_INIT_VTOR field for changes"]
    #[inline(always)]
    pub const fn set_lock_vtor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Vector table offset register out of reset"]
    #[inline(always)]
    pub const fn cm7_init_vtor(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Vector table offset register out of reset"]
    #[inline(always)]
    pub const fn set_cm7_init_vtor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Gpr16 {
    #[inline(always)]
    fn default() -> Gpr16 {
        Gpr16(0)
    }
}
#[doc = "GPR17 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr17(pub u32);
impl Gpr17 {
    #[doc = "FlexRAM bank config value"]
    #[inline(always)]
    pub const fn flexram_bank_cfg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "FlexRAM bank config value"]
    #[inline(always)]
    pub const fn set_flexram_bank_cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Gpr17 {
    #[inline(always)]
    fn default() -> Gpr17 {
        Gpr17(0)
    }
}
#[doc = "GPR18 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr18(pub u32);
impl Gpr18 {
    #[doc = "lock M7_APC_AC_R0_BOT field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r0_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R0_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-0"]
    #[inline(always)]
    pub const fn m7_apc_ac_r0_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-0"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r0_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr18 {
    #[inline(always)]
    fn default() -> Gpr18 {
        Gpr18(0)
    }
}
#[doc = "GPR19 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr19(pub u32);
impl Gpr19 {
    #[doc = "lock M7_APC_AC_R0_TOP field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r0_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R0_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-0"]
    #[inline(always)]
    pub const fn m7_apc_ac_r0_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-0"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r0_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr19 {
    #[inline(always)]
    fn default() -> Gpr19 {
        Gpr19(0)
    }
}
#[doc = "GPR2 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr2(pub u32);
impl Gpr2 {
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn axbs_p_m0_high_priority(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn set_axbs_p_m0_high_priority(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn axbs_p_m1_high_priority(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn set_axbs_p_m1_high_priority(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[inline(always)]
    pub const fn axbs_p_force_round_robin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[inline(always)]
    pub const fn set_axbs_p_force_round_robin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable power saving features on L2 memory"]
    #[inline(always)]
    pub const fn l2_mem_en_powersaving(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable power saving features on L2 memory"]
    #[inline(always)]
    pub const fn set_l2_mem_en_powersaving(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    #[inline(always)]
    pub const fn ram_auto_clk_gating_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    #[inline(always)]
    pub const fn set_ram_auto_clk_gating_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit controls how memory (OCRAM) enters Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low"]
    #[inline(always)]
    pub const fn l2_mem_deepsleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls how memory (OCRAM) enters Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low"]
    #[inline(always)]
    pub const fn set_l2_mem_deepsleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Divider ratio control for mclk from hmclk"]
    #[inline(always)]
    pub const fn mqs_clk_div(&self) -> super::vals::MqsClkDiv {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::MqsClkDiv::from_bits(val as u8)
    }
    #[doc = "Divider ratio control for mclk from hmclk"]
    #[inline(always)]
    pub const fn set_mqs_clk_div(&mut self, val: super::vals::MqsClkDiv) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "MQS software reset"]
    #[inline(always)]
    pub const fn mqs_sw_rst(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "MQS software reset"]
    #[inline(always)]
    pub const fn set_mqs_sw_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "MQS enable."]
    #[inline(always)]
    pub const fn mqs_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "MQS enable."]
    #[inline(always)]
    pub const fn set_mqs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Medium Quality Sound (MQS) Oversample"]
    #[inline(always)]
    pub const fn mqs_oversample(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Medium Quality Sound (MQS) Oversample"]
    #[inline(always)]
    pub const fn set_mqs_oversample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Gpr2 {
    #[inline(always)]
    fn default() -> Gpr2 {
        Gpr2(0)
    }
}
#[doc = "GPR20 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr20(pub u32);
impl Gpr20 {
    #[doc = "lock M7_APC_AC_R1_BOT field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r1_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R1_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-1"]
    #[inline(always)]
    pub const fn m7_apc_ac_r1_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-1"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r1_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr20 {
    #[inline(always)]
    fn default() -> Gpr20 {
        Gpr20(0)
    }
}
#[doc = "GPR21 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr21(pub u32);
impl Gpr21 {
    #[doc = "lock M7_APC_AC_R1_TOP field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r1_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R1_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-1"]
    #[inline(always)]
    pub const fn m7_apc_ac_r1_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-1"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r1_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr21 {
    #[inline(always)]
    fn default() -> Gpr21 {
        Gpr21(0)
    }
}
#[doc = "GPR22 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr22(pub u32);
impl Gpr22 {
    #[doc = "lock M7_APC_AC_R2_BOT field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r2_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R2_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-2"]
    #[inline(always)]
    pub const fn m7_apc_ac_r2_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-2"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r2_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr22 {
    #[inline(always)]
    fn default() -> Gpr22 {
        Gpr22(0)
    }
}
#[doc = "GPR23 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr23(pub u32);
impl Gpr23 {
    #[doc = "lock M7_APC_AC_R2_TOP field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r2_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R2_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-2"]
    #[inline(always)]
    pub const fn m7_apc_ac_r2_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-2"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r2_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr23 {
    #[inline(always)]
    fn default() -> Gpr23 {
        Gpr23(0)
    }
}
#[doc = "GPR24 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr24(pub u32);
impl Gpr24 {
    #[doc = "lock M7_APC_AC_R3_BOT field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r3_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R3_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-3"]
    #[inline(always)]
    pub const fn m7_apc_ac_r3_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-3"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r3_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr24 {
    #[inline(always)]
    fn default() -> Gpr24 {
        Gpr24(0)
    }
}
#[doc = "GPR25 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr25(pub u32);
impl Gpr25 {
    #[doc = "lock M7_APC_AC_R3_TOP field for changes"]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r3_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R3_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-3"]
    #[inline(always)]
    pub const fn m7_apc_ac_r3_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-3"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r3_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr25 {
    #[inline(always)]
    fn default() -> Gpr25 {
        Gpr25(0)
    }
}
#[doc = "GPR27 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr27(pub u32);
impl Gpr27 {
    #[doc = "Start address of flexspi1"]
    #[inline(always)]
    pub const fn flexspi_remap_addr_start(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start address of flexspi1"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Gpr27 {
    #[inline(always)]
    fn default() -> Gpr27 {
        Gpr27(0)
    }
}
#[doc = "GPR28 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr28(pub u32);
impl Gpr28 {
    #[doc = "End address of flexspi1"]
    #[inline(always)]
    pub const fn flexspi_remap_addr_end(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End address of flexspi1"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_end(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Gpr28 {
    #[inline(always)]
    fn default() -> Gpr28 {
        Gpr28(0)
    }
}
#[doc = "GPR29 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr29(pub u32);
impl Gpr29 {
    #[doc = "Offset address of flexspi1"]
    #[inline(always)]
    pub const fn flexspi_remap_addr_offset(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Offset address of flexspi1"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Gpr29 {
    #[inline(always)]
    fn default() -> Gpr29 {
        Gpr29(0)
    }
}
#[doc = "GPR3 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr3(pub u32);
impl Gpr3 {
    #[doc = "Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
    #[inline(always)]
    pub const fn dcp_key_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
    #[inline(always)]
    pub const fn set_dcp_key_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Gpr3 {
    #[inline(always)]
    fn default() -> Gpr3 {
        Gpr3(0)
    }
}
#[doc = "GPR4 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr4(pub u32);
impl Gpr4 {
    #[doc = "EDMA stop request."]
    #[inline(always)]
    pub const fn edma_stop_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EDMA stop request."]
    #[inline(always)]
    pub const fn set_edma_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TRNG stop request."]
    #[inline(always)]
    pub const fn trng_stop_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG stop request."]
    #[inline(always)]
    pub const fn set_trng_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SAI1 stop request."]
    #[inline(always)]
    pub const fn sai1_stop_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 stop request."]
    #[inline(always)]
    pub const fn set_sai1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SAI3 stop request."]
    #[inline(always)]
    pub const fn sai3_stop_req(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 stop request."]
    #[inline(always)]
    pub const fn set_sai3_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PIT stop request."]
    #[inline(always)]
    pub const fn pit_stop_req(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PIT stop request."]
    #[inline(always)]
    pub const fn set_pit_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FlexSPI stop request."]
    #[inline(always)]
    pub const fn flexspi_stop_req(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FlexSPI stop request."]
    #[inline(always)]
    pub const fn set_flexspi_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FlexIO1 stop request."]
    #[inline(always)]
    pub const fn flexio1_stop_req(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FlexIO1 stop request."]
    #[inline(always)]
    pub const fn set_flexio1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    #[inline(always)]
    pub const fn edma_stop_ack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    #[inline(always)]
    pub const fn set_edma_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRNG stop acknowledge"]
    #[inline(always)]
    pub const fn trng_stop_ack(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG stop acknowledge"]
    #[inline(always)]
    pub const fn set_trng_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SAI1 stop acknowledge"]
    #[inline(always)]
    pub const fn sai1_stop_ack(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SAI3 stop acknowledge"]
    #[inline(always)]
    pub const fn sai3_stop_ack(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai3_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PIT stop acknowledge"]
    #[inline(always)]
    pub const fn pit_stop_ack(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "PIT stop acknowledge"]
    #[inline(always)]
    pub const fn set_pit_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXSPI stop acknowledge"]
    #[inline(always)]
    pub const fn flexspi_stop_ack(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexspi_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    #[inline(always)]
    pub const fn flexio1_stop_ack(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexio1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Gpr4 {
    #[inline(always)]
    fn default() -> Gpr4 {
        Gpr4(0)
    }
}
#[doc = "GPR5 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr5(pub u32);
impl Gpr5 {
    #[doc = "WDOG1 Timeout Mask"]
    #[inline(always)]
    pub const fn wdog1_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG1 Timeout Mask"]
    #[inline(always)]
    pub const fn set_wdog1_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "WDOG2 Timeout Mask"]
    #[inline(always)]
    pub const fn wdog2_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG2 Timeout Mask"]
    #[inline(always)]
    pub const fn set_wdog2_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub const fn vref_1m_clk_gpt1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub const fn set_vref_1m_clk_gpt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub const fn vref_1m_clk_gpt2(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub const fn set_vref_1m_clk_gpt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Gpr5 {
    #[inline(always)]
    fn default() -> Gpr5 {
        Gpr5(0)
    }
}
#[doc = "GPR6 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr6(pub u32);
impl Gpr6 {
    #[doc = "IOMUXC XBAR_INOUT2 function direction select"]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IOMUXC XBAR_INOUT2 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "IOMUXC XBAR_INOUT3 function direction select"]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_3(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "IOMUXC XBAR_INOUT3 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Gpr6 {
    #[inline(always)]
    fn default() -> Gpr6 {
        Gpr6(0)
    }
}
#[doc = "GPR7 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr7(pub u32);
impl Gpr7 {
    #[doc = "LPI2C1 stop request"]
    #[inline(always)]
    pub const fn lpi2c1_stop_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPI2C2 stop request"]
    #[inline(always)]
    pub const fn lpi2c2_stop_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c2_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPSPI1 stop request"]
    #[inline(always)]
    pub const fn lpspi1_stop_req(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 stop request"]
    #[inline(always)]
    pub const fn set_lpspi1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI2 stop request"]
    #[inline(always)]
    pub const fn lpspi2_stop_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 stop request"]
    #[inline(always)]
    pub const fn set_lpspi2_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn lpuart1_stop_req(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn set_lpuart1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn lpuart2_stop_req(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn set_lpuart2_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPUART3 stop request"]
    #[inline(always)]
    pub const fn lpuart3_stop_req(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 stop request"]
    #[inline(always)]
    pub const fn set_lpuart3_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "LPUART4 stop request"]
    #[inline(always)]
    pub const fn lpuart4_stop_req(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 stop request"]
    #[inline(always)]
    pub const fn set_lpuart4_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "LPI2C1 stop acknowledge"]
    #[inline(always)]
    pub const fn lpi2c1_stop_ack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "LPI2C2 stop acknowledge"]
    #[inline(always)]
    pub const fn lpi2c2_stop_ack(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c2_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPSPI1 stop acknowledge"]
    #[inline(always)]
    pub const fn lpspi1_stop_ack(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI2 stop acknowledge"]
    #[inline(always)]
    pub const fn lpspi2_stop_ack(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi2_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn lpuart1_stop_ack(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn lpuart2_stop_ack(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart2_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3 stop acknowledge"]
    #[inline(always)]
    pub const fn lpuart3_stop_ack(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart3_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4 stop acknowledge"]
    #[inline(always)]
    pub const fn lpuart4_stop_ack(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart4_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Gpr7 {
    #[inline(always)]
    fn default() -> Gpr7 {
        Gpr7(0)
    }
}
#[doc = "GPR8 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr8(pub u32);
impl Gpr8 {
    #[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpi2c1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpi2c1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpi2c2_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c2_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpi2c2_ipg_doze(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c2_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpspi1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpspi1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpspi2_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi2_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpspi2_ipg_doze(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi2_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpuart1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpuart1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpuart2_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart2_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpuart2_ipg_doze(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart2_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpuart3_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart3_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpuart3_ipg_doze(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart3_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn lpuart4_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart4_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub const fn lpuart4_ipg_doze(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart4_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Gpr8 {
    #[inline(always)]
    fn default() -> Gpr8 {
        Gpr8(0)
    }
}
