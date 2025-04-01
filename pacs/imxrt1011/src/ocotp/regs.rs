#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpCtrl(pub u32);
impl HwOcotpCtrl {
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> super::vals::WrUnlock {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::WrUnlock::from_bits(val as u16)
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: super::vals::WrUnlock) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrl {
    #[inline(always)]
    fn default() -> HwOcotpCtrl {
        HwOcotpCtrl(0)
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpCtrlClr(pub u32);
impl HwOcotpCtrlClr {
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrlClr {
    #[inline(always)]
    fn default() -> HwOcotpCtrlClr {
        HwOcotpCtrlClr(0)
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpCtrlSet(pub u32);
impl HwOcotpCtrlSet {
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrlSet {
    #[inline(always)]
    fn default() -> HwOcotpCtrlSet {
        HwOcotpCtrlSet(0)
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpCtrlTog(pub u32);
impl HwOcotpCtrlTog {
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "OTP write and read access address register"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locked Region Access Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Shadow Registers"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Unlock"]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HwOcotpCtrlTog {
    #[inline(always)]
    fn default() -> HwOcotpCtrlTog {
        HwOcotpCtrlTog(0)
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpLock(pub u32);
impl HwOcotpLock {
    #[doc = "BOOT_CFG Write Lock Status"]
    #[inline(always)]
    pub const fn boot_cfg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "BOOT_CFG Write Lock Status"]
    #[inline(always)]
    pub const fn set_boot_cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "SJC_RESP Lock Status"]
    #[inline(always)]
    pub const fn sjc_resp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SJC_RESP Lock Status"]
    #[inline(always)]
    pub const fn set_sjc_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OTFAD Write Lock Status"]
    #[inline(always)]
    pub const fn otfad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "OTFAD Write Lock Status"]
    #[inline(always)]
    pub const fn set_otfad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "GP1 Write Lock Status"]
    #[inline(always)]
    pub const fn gp1(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "GP1 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "GP2 Write Lock Status"]
    #[inline(always)]
    pub const fn gp2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "GP2 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "SW_GP1 Write Lock Status"]
    #[inline(always)]
    pub const fn sw_gp1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SW_GP1 Write Lock Status"]
    #[inline(always)]
    pub const fn set_sw_gp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ANALOG Write Lock Status"]
    #[inline(always)]
    pub const fn analog(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "ANALOG Write Lock Status"]
    #[inline(always)]
    pub const fn set_analog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "SW_GP2 Write Lock Status"]
    #[inline(always)]
    pub const fn sw_gp2_lock(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SW_GP2 Write Lock Status"]
    #[inline(always)]
    pub const fn set_sw_gp2_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "MISC_CONF Write Lock Status"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "MISC_CONF Write Lock Status"]
    #[inline(always)]
    pub const fn set_misc_conf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SW_GP2 Read Lock Status"]
    #[inline(always)]
    pub const fn sw_gp2_rlock(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SW_GP2 Read Lock Status"]
    #[inline(always)]
    pub const fn set_sw_gp2_rlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GP3 Write Lock Status"]
    #[inline(always)]
    pub const fn gp3(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "GP3 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "FIELD RETURN Status"]
    #[inline(always)]
    pub const fn field_return(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIELD RETURN Status"]
    #[inline(always)]
    pub const fn set_field_return(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpLock {
    #[inline(always)]
    fn default() -> HwOcotpLock {
        HwOcotpLock(0)
    }
}
#[doc = "OTP Controller Write Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpReadCtrl(pub u32);
impl HwOcotpReadCtrl {
    #[doc = "Read Fuse"]
    #[inline(always)]
    pub const fn read_fuse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Fuse"]
    #[inline(always)]
    pub const fn set_read_fuse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for HwOcotpReadCtrl {
    #[inline(always)]
    fn default() -> HwOcotpReadCtrl {
        HwOcotpReadCtrl(0)
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpScs(pub u32);
impl HwOcotpScs {
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScs {
    #[inline(always)]
    fn default() -> HwOcotpScs {
        HwOcotpScs(0)
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpScsClr(pub u32);
impl HwOcotpScsClr {
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScsClr {
    #[inline(always)]
    fn default() -> HwOcotpScsClr {
        HwOcotpScsClr(0)
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpScsSet(pub u32);
impl HwOcotpScsSet {
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScsSet {
    #[inline(always)]
    fn default() -> HwOcotpScsSet {
        HwOcotpScsSet(0)
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpScsTog(pub u32);
impl HwOcotpScsTog {
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn hab_jde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HAB JTAG Debug Enable"]
    #[inline(always)]
    pub const fn set_hab_jde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Spare"]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HwOcotpScsTog {
    #[inline(always)]
    fn default() -> HwOcotpScsTog {
        HwOcotpScsTog(0)
    }
}
#[doc = "Sticky bit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpSwSticky(pub u32);
impl HwOcotpSwSticky {
    #[doc = "SRK Revoke Lock"]
    #[inline(always)]
    pub const fn srk_revoke_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRK Revoke Lock"]
    #[inline(always)]
    pub const fn set_srk_revoke_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Field Return Lock"]
    #[inline(always)]
    pub const fn field_return_lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Field Return Lock"]
    #[inline(always)]
    pub const fn set_field_return_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for HwOcotpSwSticky {
    #[inline(always)]
    fn default() -> HwOcotpSwSticky {
        HwOcotpSwSticky(0)
    }
}
#[doc = "OTP Controller Timing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpTiming(pub u32);
impl HwOcotpTiming {
    #[doc = "Write Strobe Period"]
    #[inline(always)]
    pub const fn strobe_prog(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Write Strobe Period"]
    #[inline(always)]
    pub const fn set_strobe_prog(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Relax Count Value"]
    #[inline(always)]
    pub const fn relax(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Relax Count Value"]
    #[inline(always)]
    pub const fn set_relax(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Read Strobe Period"]
    #[inline(always)]
    pub const fn strobe_read(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Read Strobe Period"]
    #[inline(always)]
    pub const fn set_strobe_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Wait Interval"]
    #[inline(always)]
    pub const fn wait(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x3f;
        val as u8
    }
    #[doc = "Wait Interval"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 22usize)) | (((val as u32) & 0x3f) << 22usize);
    }
}
impl Default for HwOcotpTiming {
    #[inline(always)]
    fn default() -> HwOcotpTiming {
        HwOcotpTiming(0)
    }
}
#[doc = "OTP Controller Timing Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpTiming2(pub u32);
impl HwOcotpTiming2 {
    #[doc = "Relax Prog. count value"]
    #[inline(always)]
    pub const fn relax_prog(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Relax Prog. count value"]
    #[inline(always)]
    pub const fn set_relax_prog(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Relax Read count value"]
    #[inline(always)]
    pub const fn relax_read(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Relax Read count value"]
    #[inline(always)]
    pub const fn set_relax_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Auto read and write time interval"]
    #[inline(always)]
    pub const fn relax1(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0xff;
        val as u8
    }
    #[doc = "Auto read and write time interval"]
    #[inline(always)]
    pub const fn set_relax1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 22usize)) | (((val as u32) & 0xff) << 22usize);
    }
}
impl Default for HwOcotpTiming2 {
    #[inline(always)]
    fn default() -> HwOcotpTiming2 {
        HwOcotpTiming2(0)
    }
}
#[doc = "OTP Controller Version Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HwOcotpVersion(pub u32);
impl HwOcotpVersion {
    #[doc = "RTL Version Steping"]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTL Version Steping"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor RTL Version"]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor RTL Version"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major RTL Version"]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major RTL Version"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for HwOcotpVersion {
    #[inline(always)]
    fn default() -> HwOcotpVersion {
        HwOcotpVersion(0)
    }
}
