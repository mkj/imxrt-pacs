#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "OTP write and read access address register"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {:?} }}" , self . addr () , self . busy () , self . error () , self . reload_shadows () , self . wr_unlock ())
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "OTP write and read access address register"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
impl core::fmt::Debug for CtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlClr")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlClr {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}" , self . addr () , self . busy () , self . error () , self . reload_shadows () , self . wr_unlock ())
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "OTP write and read access address register"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
impl core::fmt::Debug for CtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlSet")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlSet {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}" , self . addr () , self . busy () , self . error () , self . reload_shadows () , self . wr_unlock ())
    }
}
#[doc = "OTP Controller Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "OTP write and read access address register"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
impl core::fmt::Debug for CtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTog")
            .field("addr", &self.addr())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("reload_shadows", &self.reload_shadows())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlTog {{ addr: {=u8:?}, busy: {=bool:?}, error: {=bool:?}, reload_shadows: {=bool:?}, wr_unlock: {=u16:?} }}" , self . addr () , self . busy () , self . error () , self . reload_shadows () , self . wr_unlock ())
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "BOOT_CFG Write Lock Status"]
    #[must_use]
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
    #[must_use]
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
    #[doc = "GP4 Read Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp4_rlock(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GP4 Read Lock Status"]
    #[inline(always)]
    pub const fn set_gp4_rlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "MAC_ADDR Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "MAC_ADDR Write Lock Status"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "GP1 Write Lock Status"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[doc = "GP4 Write Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gp4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "GP4 Write Lock Status"]
    #[inline(always)]
    pub const fn set_gp4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "GP3 Write Lock Status"]
    #[must_use]
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
    #[must_use]
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
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("boot_cfg", &self.boot_cfg())
            .field("sjc_resp", &self.sjc_resp())
            .field("gp4_rlock", &self.gp4_rlock())
            .field("mac_addr", &self.mac_addr())
            .field("gp1", &self.gp1())
            .field("gp2", &self.gp2())
            .field("sw_gp1", &self.sw_gp1())
            .field("analog", &self.analog())
            .field("sw_gp2_lock", &self.sw_gp2_lock())
            .field("misc_conf", &self.misc_conf())
            .field("sw_gp2_rlock", &self.sw_gp2_rlock())
            .field("gp4", &self.gp4())
            .field("gp3", &self.gp3())
            .field("field_return", &self.field_return())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lock {{ boot_cfg: {=u8:?}, sjc_resp: {=bool:?}, gp4_rlock: {=bool:?}, mac_addr: {=u8:?}, gp1: {=u8:?}, gp2: {=u8:?}, sw_gp1: {=bool:?}, analog: {=u8:?}, sw_gp2_lock: {=bool:?}, misc_conf: {=bool:?}, sw_gp2_rlock: {=bool:?}, gp4: {=u8:?}, gp3: {=u8:?}, field_return: {=bool:?} }}" , self . boot_cfg () , self . sjc_resp () , self . gp4_rlock () , self . mac_addr () , self . gp1 () , self . gp2 () , self . sw_gp1 () , self . analog () , self . sw_gp2_lock () , self . misc_conf () , self . sw_gp2_rlock () , self . gp4 () , self . gp3 () , self . field_return ())
    }
}
#[doc = "OTP Controller Write Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadCtrl(pub u32);
impl ReadCtrl {
    #[doc = "READ_FUSE"]
    #[must_use]
    #[inline(always)]
    pub const fn read_fuse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READ_FUSE"]
    #[inline(always)]
    pub const fn set_read_fuse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ReadCtrl {
    #[inline(always)]
    fn default() -> ReadCtrl {
        ReadCtrl(0)
    }
}
impl core::fmt::Debug for ReadCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadCtrl")
            .field("read_fuse", &self.read_fuse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ReadCtrl {{ read_fuse: {=bool:?} }}", self.read_fuse())
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scs(pub u32);
impl Scs {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
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
impl Default for Scs {
    #[inline(always)]
    fn default() -> Scs {
        Scs(0)
    }
}
impl core::fmt::Debug for Scs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scs")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scs {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScsClr(pub u32);
impl ScsClr {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
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
impl Default for ScsClr {
    #[inline(always)]
    fn default() -> ScsClr {
        ScsClr(0)
    }
}
impl core::fmt::Debug for ScsClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScsClr")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScsClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScsClr {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScsSet(pub u32);
impl ScsSet {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
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
impl Default for ScsSet {
    #[inline(always)]
    fn default() -> ScsSet {
        ScsSet(0)
    }
}
impl core::fmt::Debug for ScsSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScsSet")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScsSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScsSet {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Software Controllable Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScsTog(pub u32);
impl ScsTog {
    #[doc = "HAB JTAG Debug Enable"]
    #[must_use]
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
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[must_use]
    #[inline(always)]
    pub const fn spare(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Unallocated read/write bits for implementation specific software use."]
    #[inline(always)]
    pub const fn set_spare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 1usize)) | (((val as u32) & 0x3fff_ffff) << 1usize);
    }
    #[doc = "Lock"]
    #[must_use]
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
impl Default for ScsTog {
    #[inline(always)]
    fn default() -> ScsTog {
        ScsTog(0)
    }
}
impl core::fmt::Debug for ScsTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ScsTog")
            .field("hab_jde", &self.hab_jde())
            .field("spare", &self.spare())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ScsTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ScsTog {{ hab_jde: {=bool:?}, spare: {=u32:?}, lock: {=bool:?} }}",
            self.hab_jde(),
            self.spare(),
            self.lock()
        )
    }
}
#[doc = "Sticky bit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwSticky(pub u32);
impl SwSticky {
    #[doc = "SRK Revoke Lock"]
    #[must_use]
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
    #[must_use]
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
impl Default for SwSticky {
    #[inline(always)]
    fn default() -> SwSticky {
        SwSticky(0)
    }
}
impl core::fmt::Debug for SwSticky {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwSticky")
            .field("srk_revoke_lock", &self.srk_revoke_lock())
            .field("field_return_lock", &self.field_return_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwSticky {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwSticky {{ srk_revoke_lock: {=bool:?}, field_return_lock: {=bool:?} }}",
            self.srk_revoke_lock(),
            self.field_return_lock()
        )
    }
}
#[doc = "OTP Controller Timing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing(pub u32);
impl Timing {
    #[doc = "Write Strobe Period"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl Default for Timing {
    #[inline(always)]
    fn default() -> Timing {
        Timing(0)
    }
}
impl core::fmt::Debug for Timing {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing")
            .field("strobe_prog", &self.strobe_prog())
            .field("relax", &self.relax())
            .field("strobe_read", &self.strobe_read())
            .field("wait", &self.wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Timing {{ strobe_prog: {=u16:?}, relax: {=u8:?}, strobe_read: {=u8:?}, wait: {=u8:?} }}" , self . strobe_prog () , self . relax () , self . strobe_read () , self . wait ())
    }
}
#[doc = "OTP Controller Timing Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing2(pub u32);
impl Timing2 {
    #[doc = "Relax Prog. count value"]
    #[must_use]
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
    #[must_use]
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
}
impl Default for Timing2 {
    #[inline(always)]
    fn default() -> Timing2 {
        Timing2(0)
    }
}
impl core::fmt::Debug for Timing2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timing2")
            .field("relax_prog", &self.relax_prog())
            .field("relax_read", &self.relax_read())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timing2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timing2 {{ relax_prog: {=u16:?}, relax_read: {=u8:?} }}",
            self.relax_prog(),
            self.relax_read()
        )
    }
}
#[doc = "OTP Controller Version Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "RTL Version Steping"]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("step", &self.step())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ step: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.step(),
            self.minor(),
            self.major()
        )
    }
}
