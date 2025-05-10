#[doc = "Offset region 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AddrOffset0(pub u32);
impl AddrOffset0 {
    #[doc = "Signed offset for BEE region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_offset0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Signed offset for BEE region 0"]
    #[inline(always)]
    pub const fn set_addr_offset0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Lock bits for addr_offset0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_offset0_lock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Lock bits for addr_offset0"]
    #[inline(always)]
    pub const fn set_addr_offset0_lock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AddrOffset0 {
    #[inline(always)]
    fn default() -> AddrOffset0 {
        AddrOffset0(0)
    }
}
impl core::fmt::Debug for AddrOffset0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AddrOffset0")
            .field("addr_offset0", &self.addr_offset0())
            .field("addr_offset0_lock", &self.addr_offset0_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AddrOffset0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AddrOffset0 {{ addr_offset0: {=u16:?}, addr_offset0_lock: {=u16:?} }}",
            self.addr_offset0(),
            self.addr_offset0_lock()
        )
    }
}
#[doc = "Offset region 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AddrOffset1(pub u32);
impl AddrOffset1 {
    #[doc = "Signed offset for BEE region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_offset1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Signed offset for BEE region 1"]
    #[inline(always)]
    pub const fn set_addr_offset1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Lock bits for addr_offset1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_offset1_lock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Lock bits for addr_offset1"]
    #[inline(always)]
    pub const fn set_addr_offset1_lock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AddrOffset1 {
    #[inline(always)]
    fn default() -> AddrOffset1 {
        AddrOffset1(0)
    }
}
impl core::fmt::Debug for AddrOffset1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AddrOffset1")
            .field("addr_offset1", &self.addr_offset1())
            .field("addr_offset1_lock", &self.addr_offset1_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AddrOffset1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AddrOffset1 {{ addr_offset1: {=u16:?}, addr_offset1_lock: {=u16:?} }}",
            self.addr_offset1(),
            self.addr_offset1_lock()
        )
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "BEE enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn bee_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BEE enable bit"]
    #[inline(always)]
    pub const fn set_bee_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock enable input, low inactive"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_clk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock enable input, low inactive"]
    #[inline(always)]
    pub const fn set_ctrl_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Soft reset input, low active"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_sftrst_n(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Soft reset input, low active"]
    #[inline(always)]
    pub const fn set_ctrl_sftrst_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AES-128 key is ready Load AES key by changing this bit from 0 to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn key_valid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AES-128 key is ready Load AES key by changing this bit from 0 to 1."]
    #[inline(always)]
    pub const fn set_key_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "AES key region select"]
    #[must_use]
    #[inline(always)]
    pub const fn key_region_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AES key region select"]
    #[inline(always)]
    pub const fn set_key_region_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    #[must_use]
    #[inline(always)]
    pub const fn ac_prot_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    #[inline(always)]
    pub const fn set_ac_prot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Endian swap control for the 16 bytes input and output data of AES core."]
    #[must_use]
    #[inline(always)]
    pub const fn little_endian(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline(always)]
    pub const fn set_little_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Security level of the allowed access for memory region0"]
    #[must_use]
    #[inline(always)]
    pub const fn security_level_r0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Security level of the allowed access for memory region0"]
    #[inline(always)]
    pub const fn set_security_level_r0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "AES mode of region0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_aes_mode_r0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AES mode of region0"]
    #[inline(always)]
    pub const fn set_ctrl_aes_mode_r0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Security level of the allowed access for memory region1"]
    #[must_use]
    #[inline(always)]
    pub const fn security_level_r1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Security level of the allowed access for memory region1"]
    #[inline(always)]
    pub const fn set_security_level_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "AES mode of region1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_aes_mode_r1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "AES mode of region1"]
    #[inline(always)]
    pub const fn set_ctrl_aes_mode_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lock bit for bee_enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bee_enable_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for bee_enable"]
    #[inline(always)]
    pub const fn set_bee_enable_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock bit for ctrl_clk_en"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_clk_en_lock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for ctrl_clk_en"]
    #[inline(always)]
    pub const fn set_ctrl_clk_en_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for ctrl_sftrst"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_sftrst_n_lock(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for ctrl_sftrst"]
    #[inline(always)]
    pub const fn set_ctrl_sftrst_n_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock bit for region1 address boundary"]
    #[must_use]
    #[inline(always)]
    pub const fn region1_addr_lock(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for region1 address boundary"]
    #[inline(always)]
    pub const fn set_region1_addr_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Lock bit for key_valid"]
    #[must_use]
    #[inline(always)]
    pub const fn key_valid_lock(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for key_valid"]
    #[inline(always)]
    pub const fn set_key_valid_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock bit for key_region_sel"]
    #[must_use]
    #[inline(always)]
    pub const fn key_region_sel_lock(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for key_region_sel"]
    #[inline(always)]
    pub const fn set_key_region_sel_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Lock bit for ac_prot"]
    #[must_use]
    #[inline(always)]
    pub const fn ac_prot_en_lock(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for ac_prot"]
    #[inline(always)]
    pub const fn set_ac_prot_en_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock bit for little_endian"]
    #[must_use]
    #[inline(always)]
    pub const fn little_endian_lock(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for little_endian"]
    #[inline(always)]
    pub const fn set_little_endian_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Lock bits for security_level_r0"]
    #[must_use]
    #[inline(always)]
    pub const fn security_level_r0_lock(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Lock bits for security_level_r0"]
    #[inline(always)]
    pub const fn set_security_level_r0_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Lock bit for region0 ctrl_aes_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_aes_mode_r0_lock(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for region0 ctrl_aes_mode"]
    #[inline(always)]
    pub const fn set_ctrl_aes_mode_r0_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Lock bit for region0 AES key"]
    #[must_use]
    #[inline(always)]
    pub const fn region0_key_lock(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for region0 AES key"]
    #[inline(always)]
    pub const fn set_region0_key_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Lock bits for security_level_r1"]
    #[must_use]
    #[inline(always)]
    pub const fn security_level_r1_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Lock bits for security_level_r1"]
    #[inline(always)]
    pub const fn set_security_level_r1_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Lock bit for region1 ctrl_aes_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_aes_mode_r1_lock(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for region1 ctrl_aes_mode"]
    #[inline(always)]
    pub const fn set_ctrl_aes_mode_r1_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Lock bit for region1 AES key"]
    #[must_use]
    #[inline(always)]
    pub const fn region1_key_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for region1 AES key"]
    #[inline(always)]
    pub const fn set_region1_key_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("bee_enable", &self.bee_enable())
            .field("ctrl_clk_en", &self.ctrl_clk_en())
            .field("ctrl_sftrst_n", &self.ctrl_sftrst_n())
            .field("key_valid", &self.key_valid())
            .field("key_region_sel", &self.key_region_sel())
            .field("ac_prot_en", &self.ac_prot_en())
            .field("little_endian", &self.little_endian())
            .field("security_level_r0", &self.security_level_r0())
            .field("ctrl_aes_mode_r0", &self.ctrl_aes_mode_r0())
            .field("security_level_r1", &self.security_level_r1())
            .field("ctrl_aes_mode_r1", &self.ctrl_aes_mode_r1())
            .field("bee_enable_lock", &self.bee_enable_lock())
            .field("ctrl_clk_en_lock", &self.ctrl_clk_en_lock())
            .field("ctrl_sftrst_n_lock", &self.ctrl_sftrst_n_lock())
            .field("region1_addr_lock", &self.region1_addr_lock())
            .field("key_valid_lock", &self.key_valid_lock())
            .field("key_region_sel_lock", &self.key_region_sel_lock())
            .field("ac_prot_en_lock", &self.ac_prot_en_lock())
            .field("little_endian_lock", &self.little_endian_lock())
            .field("security_level_r0_lock", &self.security_level_r0_lock())
            .field("ctrl_aes_mode_r0_lock", &self.ctrl_aes_mode_r0_lock())
            .field("region0_key_lock", &self.region0_key_lock())
            .field("security_level_r1_lock", &self.security_level_r1_lock())
            .field("ctrl_aes_mode_r1_lock", &self.ctrl_aes_mode_r1_lock())
            .field("region1_key_lock", &self.region1_key_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ bee_enable: {=bool:?}, ctrl_clk_en: {=bool:?}, ctrl_sftrst_n: {=bool:?}, key_valid: {=bool:?}, key_region_sel: {=bool:?}, ac_prot_en: {=bool:?}, little_endian: {=bool:?}, security_level_r0: {=u8:?}, ctrl_aes_mode_r0: {=bool:?}, security_level_r1: {=u8:?}, ctrl_aes_mode_r1: {=bool:?}, bee_enable_lock: {=bool:?}, ctrl_clk_en_lock: {=bool:?}, ctrl_sftrst_n_lock: {=bool:?}, region1_addr_lock: {=bool:?}, key_valid_lock: {=bool:?}, key_region_sel_lock: {=bool:?}, ac_prot_en_lock: {=bool:?}, little_endian_lock: {=bool:?}, security_level_r0_lock: {=u8:?}, ctrl_aes_mode_r0_lock: {=bool:?}, region0_key_lock: {=bool:?}, security_level_r1_lock: {=u8:?}, ctrl_aes_mode_r1_lock: {=bool:?}, region1_key_lock: {=bool:?} }}" , self . bee_enable () , self . ctrl_clk_en () , self . ctrl_sftrst_n () , self . key_valid () , self . key_region_sel () , self . ac_prot_en () , self . little_endian () , self . security_level_r0 () , self . ctrl_aes_mode_r0 () , self . security_level_r1 () , self . ctrl_aes_mode_r1 () , self . bee_enable_lock () , self . ctrl_clk_en_lock () , self . ctrl_sftrst_n_lock () , self . region1_addr_lock () , self . key_valid_lock () , self . key_region_sel_lock () , self . ac_prot_en_lock () , self . little_endian_lock () , self . security_level_r0_lock () , self . ctrl_aes_mode_r0_lock () , self . region0_key_lock () , self . security_level_r1_lock () , self . ctrl_aes_mode_r1_lock () , self . region1_key_lock ())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_vec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    #[inline(always)]
    pub const fn set_irq_vec(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "1'b1: BEE is idle; 1'b0: BEE is active"]
    #[must_use]
    #[inline(always)]
    pub const fn bee_idle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1: BEE is idle; 1'b0: BEE is active"]
    #[inline(always)]
    pub const fn set_bee_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("irq_vec", &self.irq_vec())
            .field("bee_idle", &self.bee_idle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ irq_vec: {=u8:?}, bee_idle: {=bool:?} }}",
            self.irq_vec(),
            self.bee_idle()
        )
    }
}
