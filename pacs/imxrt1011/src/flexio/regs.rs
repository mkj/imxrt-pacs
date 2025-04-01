#[doc = "FlexIO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "FlexIO Enable"]
    #[inline(always)]
    pub const fn flexen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FlexIO Enable"]
    #[inline(always)]
    pub const fn set_flexen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn swrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fast Access"]
    #[inline(always)]
    pub const fn fastacc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fast Access"]
    #[inline(always)]
    pub const fn set_fastacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn dozen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Shifter Number"]
    #[inline(always)]
    pub const fn shifter(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Number"]
    #[inline(always)]
    pub const fn set_shifter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Number"]
    #[inline(always)]
    pub const fn timer(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Number"]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pin Number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pin Number"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn trigger(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn set_trigger(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
#[doc = "Shifter Configuration N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shiftcfg(pub u32);
impl Shiftcfg {
    #[doc = "Shifter Start bit"]
    #[inline(always)]
    pub const fn sstart(&self) -> super::vals::Sstart {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sstart::from_bits(val as u8)
    }
    #[doc = "Shifter Start bit"]
    #[inline(always)]
    pub const fn set_sstart(&mut self, val: super::vals::Sstart) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Shifter Stop bit"]
    #[inline(always)]
    pub const fn sstop(&self) -> super::vals::Sstop {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sstop::from_bits(val as u8)
    }
    #[doc = "Shifter Stop bit"]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: super::vals::Sstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Source"]
    #[inline(always)]
    pub const fn insrc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Input Source"]
    #[inline(always)]
    pub const fn set_insrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Parallel Width"]
    #[inline(always)]
    pub const fn pwidth(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Parallel Width"]
    #[inline(always)]
    pub const fn set_pwidth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Shiftcfg {
    #[inline(always)]
    fn default() -> Shiftcfg {
        Shiftcfg(0)
    }
}
#[doc = "Shifter Control N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shiftctl(pub u32);
impl Shiftctl {
    #[doc = "Shifter Mode"]
    #[inline(always)]
    pub const fn smod(&self) -> super::vals::Smod {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Smod::from_bits(val as u8)
    }
    #[doc = "Shifter Mode"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: super::vals::Smod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Shifter Pin Polarity"]
    #[inline(always)]
    pub const fn pinpol(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Shifter Pin Polarity"]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Shifter Pin Select"]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shifter Pin Select"]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Shifter Pin Configuration"]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::ShiftctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::ShiftctlPincfg::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::ShiftctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Timer Polarity"]
    #[inline(always)]
    pub const fn timpol(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Polarity"]
    #[inline(always)]
    pub const fn set_timpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Timer Select"]
    #[inline(always)]
    pub const fn timsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Timer Select"]
    #[inline(always)]
    pub const fn set_timsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Shiftctl {
    #[inline(always)]
    fn default() -> Shiftctl {
        Shiftctl(0)
    }
}
#[doc = "Shifter Error Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shifteien(pub u32);
impl Shifteien {
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn seie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shifteien {
    #[inline(always)]
    fn default() -> Shifteien {
        Shifteien(0)
    }
}
#[doc = "Shifter Error Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shifterr(pub u32);
impl Shifterr {
    #[doc = "Shifter Error Flags"]
    #[inline(always)]
    pub const fn sef(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Flags"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shifterr {
    #[inline(always)]
    fn default() -> Shifterr {
        Shifterr(0)
    }
}
#[doc = "Shifter Status DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shiftsden(pub u32);
impl Shiftsden {
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn ssde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn set_ssde(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftsden {
    #[inline(always)]
    fn default() -> Shiftsden {
        Shiftsden(0)
    }
}
#[doc = "Shifter Status Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shiftsien(pub u32);
impl Shiftsien {
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn ssie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ssie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftsien {
    #[inline(always)]
    fn default() -> Shiftsien {
        Shiftsien(0)
    }
}
#[doc = "Shifter Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shiftstat(pub u32);
impl Shiftstat {
    #[doc = "Shifter Status Flag"]
    #[inline(always)]
    pub const fn ssf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Flag"]
    #[inline(always)]
    pub const fn set_ssf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftstat {
    #[inline(always)]
    fn default() -> Shiftstat {
        Shiftstat(0)
    }
}
#[doc = "Shifter State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shiftstate(pub u32);
impl Shiftstate {
    #[doc = "Current State Pointer"]
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current State Pointer"]
    #[inline(always)]
    pub const fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Shiftstate {
    #[inline(always)]
    fn default() -> Shiftstate {
        Shiftstate(0)
    }
}
#[doc = "Timer Configuration N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timcfg(pub u32);
impl Timcfg {
    #[doc = "Timer Start Bit"]
    #[inline(always)]
    pub const fn tstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Start Bit"]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Stop Bit"]
    #[inline(always)]
    pub const fn tstop(&self) -> super::vals::Tstop {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tstop::from_bits(val as u8)
    }
    #[doc = "Timer Stop Bit"]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: super::vals::Tstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn timena(&self) -> super::vals::Timena {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Timena::from_bits(val as u8)
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn set_timena(&mut self, val: super::vals::Timena) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Timer Disable"]
    #[inline(always)]
    pub const fn timdis(&self) -> super::vals::Timdis {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Timdis::from_bits(val as u8)
    }
    #[doc = "Timer Disable"]
    #[inline(always)]
    pub const fn set_timdis(&mut self, val: super::vals::Timdis) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Timer Reset"]
    #[inline(always)]
    pub const fn timrst(&self) -> super::vals::Timrst {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Timrst::from_bits(val as u8)
    }
    #[doc = "Timer Reset"]
    #[inline(always)]
    pub const fn set_timrst(&mut self, val: super::vals::Timrst) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Timer Decrement"]
    #[inline(always)]
    pub const fn timdec(&self) -> super::vals::Timdec {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Timdec::from_bits(val as u8)
    }
    #[doc = "Timer Decrement"]
    #[inline(always)]
    pub const fn set_timdec(&mut self, val: super::vals::Timdec) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Timer Output"]
    #[inline(always)]
    pub const fn timout(&self) -> super::vals::Timout {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Timout::from_bits(val as u8)
    }
    #[doc = "Timer Output"]
    #[inline(always)]
    pub const fn set_timout(&mut self, val: super::vals::Timout) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Timcfg {
    #[inline(always)]
    fn default() -> Timcfg {
        Timcfg(0)
    }
}
#[doc = "Timer Compare N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timcmp(pub u32);
impl Timcmp {
    #[doc = "Timer Compare Value"]
    #[inline(always)]
    pub const fn cmp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Compare Value"]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timcmp {
    #[inline(always)]
    fn default() -> Timcmp {
        Timcmp(0)
    }
}
#[doc = "Timer Control N Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timctl(pub u32);
impl Timctl {
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn timod(&self) -> super::vals::Timod {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Timod::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_timod(&mut self, val: super::vals::Timod) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timer Pin Polarity"]
    #[inline(always)]
    pub const fn pinpol(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Pin Polarity"]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Timer Pin Configuration"]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::TimctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::TimctlPincfg::from_bits(val as u8)
    }
    #[doc = "Timer Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::TimctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn trgsrc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_trgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Trigger Polarity"]
    #[inline(always)]
    pub const fn trgpol(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Polarity"]
    #[inline(always)]
    pub const fn set_trgpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn trgsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Timctl {
    #[inline(always)]
    fn default() -> Timctl {
        Timctl(0)
    }
}
#[doc = "Timer Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timien(pub u32);
impl Timien {
    #[doc = "Timer Status Interrupt Enable"]
    #[inline(always)]
    pub const fn teie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timien {
    #[inline(always)]
    fn default() -> Timien {
        Timien(0)
    }
}
#[doc = "Timer Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timstat(pub u32);
impl Timstat {
    #[doc = "Timer Status Flags"]
    #[inline(always)]
    pub const fn tsf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Flags"]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timstat {
    #[inline(always)]
    fn default() -> Timstat {
        Timstat(0)
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
