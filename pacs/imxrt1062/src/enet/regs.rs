#[doc = "Timer Correction Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atcor(pub u32);
impl Atcor {
    #[doc = "Correction Counter Wrap-Around Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cor(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub const fn set_cor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for Atcor {
    #[inline(always)]
    fn default() -> Atcor {
        Atcor(0)
    }
}
impl core::fmt::Debug for Atcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atcor").field("cor", &self.cor()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atcor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Atcor {{ cor: {=u32:?} }}", self.cor())
    }
}
#[doc = "Adjustable Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atcr(pub u32);
impl Atcr {
    #[doc = "Enable Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timer"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable One-Shot Offset Event"]
    #[must_use]
    #[inline(always)]
    pub const fn offen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable One-Shot Offset Event"]
    #[inline(always)]
    pub const fn set_offen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset Timer On Offset Event"]
    #[must_use]
    #[inline(always)]
    pub const fn offrst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timer On Offset Event"]
    #[inline(always)]
    pub const fn set_offrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Periodical Event"]
    #[must_use]
    #[inline(always)]
    pub const fn peren(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Periodical Event"]
    #[inline(always)]
    pub const fn set_peren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables event signal output external pin frc_evt_period assertion on period event"]
    #[must_use]
    #[inline(always)]
    pub const fn pinper(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables event signal output external pin frc_evt_period assertion on period event"]
    #[inline(always)]
    pub const fn set_pinper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reset Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn restart(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timer"]
    #[inline(always)]
    pub const fn set_restart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capture(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Timer Value"]
    #[inline(always)]
    pub const fn set_capture(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Timer Slave Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slave(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timer Slave Mode"]
    #[inline(always)]
    pub const fn set_slave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Atcr {
    #[inline(always)]
    fn default() -> Atcr {
        Atcr(0)
    }
}
impl core::fmt::Debug for Atcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atcr")
            .field("en", &self.en())
            .field("offen", &self.offen())
            .field("offrst", &self.offrst())
            .field("peren", &self.peren())
            .field("pinper", &self.pinper())
            .field("restart", &self.restart())
            .field("capture", &self.capture())
            .field("slave", &self.slave())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Atcr {{ en: {=bool:?}, offen: {=bool:?}, offrst: {=bool:?}, peren: {=bool:?}, pinper: {=bool:?}, restart: {=bool:?}, capture: {=bool:?}, slave: {=bool:?} }}" , self . en () , self . offen () , self . offrst () , self . peren () , self . pinper () , self . restart () , self . capture () , self . slave ())
    }
}
#[doc = "Time-Stamping Clock Period Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atinc(pub u32);
impl Atinc {
    #[doc = "Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn inc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline(always)]
    pub const fn set_inc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Correction Increment Value"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_corr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Correction Increment Value"]
    #[inline(always)]
    pub const fn set_inc_corr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for Atinc {
    #[inline(always)]
    fn default() -> Atinc {
        Atinc(0)
    }
}
impl core::fmt::Debug for Atinc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atinc")
            .field("inc", &self.inc())
            .field("inc_corr", &self.inc_corr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atinc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Atinc {{ inc: {=u8:?}, inc_corr: {=u8:?} }}",
            self.inc(),
            self.inc_corr()
        )
    }
}
#[doc = "Ethernet Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Ethernet MAC Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet MAC Reset"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ethernet Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn etheren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet Enable"]
    #[inline(always)]
    pub const fn set_etheren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Magic Packet Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn magicen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet Detection Enable"]
    #[inline(always)]
    pub const fn set_magicen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Sleep Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Mode Enable"]
    #[inline(always)]
    pub const fn set_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EN1588 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en1588(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EN1588 Enable"]
    #[inline(always)]
    pub const fn set_en1588(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Descriptor Byte Swapping Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbswp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub const fn set_dbswp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
impl core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr")
            .field("reset", &self.reset())
            .field("etheren", &self.etheren())
            .field("magicen", &self.magicen())
            .field("sleep", &self.sleep())
            .field("en1588", &self.en1588())
            .field("dbgen", &self.dbgen())
            .field("dbswp", &self.dbswp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ecr {{ reset: {=bool:?}, etheren: {=bool:?}, magicen: {=bool:?}, sleep: {=bool:?}, en1588: {=bool:?}, dbgen: {=bool:?}, dbswp: {=bool:?} }}" , self . reset () , self . etheren () , self . magicen () , self . sleep () , self . en1588 () , self . dbgen () , self . dbswp ())
    }
}
#[doc = "Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eimr(pub u32);
impl Eimr {
    #[doc = "TS_TIMER Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_timer(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ts_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TS_AVAIL Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_avail(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ts_avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "WAKEUP Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PLR Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn plr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "PLR Interrupt Mask"]
    #[inline(always)]
    pub const fn set_plr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "UN Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn un(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "UN Interrupt Mask"]
    #[inline(always)]
    pub const fn set_un(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RL Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rl(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "RL Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LC Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LC Interrupt Mask"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "EBERR Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn eberr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "EBERR Interrupt Mask"]
    #[inline(always)]
    pub const fn set_eberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MII Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mii(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MII Interrupt Mask"]
    #[inline(always)]
    pub const fn set_mii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "RXB Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "RXB Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rxb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "RXF Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "RXF Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TXB Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txb(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TXB Interrupt Mask"]
    #[inline(always)]
    pub const fn set_txb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "TXF Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "TXF Interrupt Mask"]
    #[inline(always)]
    pub const fn set_txf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GRA Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn gra(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GRA Interrupt Mask"]
    #[inline(always)]
    pub const fn set_gra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "BABT Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn babt(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "BABT Interrupt Mask"]
    #[inline(always)]
    pub const fn set_babt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "BABR Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn babr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "BABR Interrupt Mask"]
    #[inline(always)]
    pub const fn set_babr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Eimr {
    #[inline(always)]
    fn default() -> Eimr {
        Eimr(0)
    }
}
impl core::fmt::Debug for Eimr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eimr")
            .field("ts_timer", &self.ts_timer())
            .field("ts_avail", &self.ts_avail())
            .field("wakeup", &self.wakeup())
            .field("plr", &self.plr())
            .field("un", &self.un())
            .field("rl", &self.rl())
            .field("lc", &self.lc())
            .field("eberr", &self.eberr())
            .field("mii", &self.mii())
            .field("rxb", &self.rxb())
            .field("rxf", &self.rxf())
            .field("txb", &self.txb())
            .field("txf", &self.txf())
            .field("gra", &self.gra())
            .field("babt", &self.babt())
            .field("babr", &self.babr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eimr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Eimr {{ ts_timer: {=bool:?}, ts_avail: {=bool:?}, wakeup: {=bool:?}, plr: {=bool:?}, un: {=bool:?}, rl: {=bool:?}, lc: {=bool:?}, eberr: {=bool:?}, mii: {=bool:?}, rxb: {=bool:?}, rxf: {=bool:?}, txb: {=bool:?}, txf: {=bool:?}, gra: {=bool:?}, babt: {=bool:?}, babr: {=bool:?} }}" , self . ts_timer () , self . ts_avail () , self . wakeup () , self . plr () , self . un () , self . rl () , self . lc () , self . eberr () , self . mii () , self . rxb () , self . rxf () , self . txb () , self . txf () , self . gra () , self . babt () , self . babr ())
    }
}
#[doc = "Interrupt Event Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eir(pub u32);
impl Eir {
    #[doc = "Timestamp Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_timer(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Timer"]
    #[inline(always)]
    pub const fn set_ts_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transmit Timestamp Available"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_avail(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Timestamp Available"]
    #[inline(always)]
    pub const fn set_ts_avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Node Wakeup Request Indication"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Node Wakeup Request Indication"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Payload Receive Error"]
    #[must_use]
    #[inline(always)]
    pub const fn plr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Payload Receive Error"]
    #[inline(always)]
    pub const fn set_plr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmit FIFO Underrun"]
    #[must_use]
    #[inline(always)]
    pub const fn un(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Underrun"]
    #[inline(always)]
    pub const fn set_un(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Collision Retry Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Collision Retry Limit"]
    #[inline(always)]
    pub const fn set_rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Late Collision"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Late Collision"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Ethernet Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn eberr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet Bus Error"]
    #[inline(always)]
    pub const fn set_eberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MII Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn mii(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MII Interrupt."]
    #[inline(always)]
    pub const fn set_mii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Receive Buffer Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Interrupt"]
    #[inline(always)]
    pub const fn set_rxb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Receive Frame Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Frame Interrupt"]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Transmit Buffer Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txb(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Interrupt"]
    #[inline(always)]
    pub const fn set_txb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Transmit Frame Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Frame Interrupt"]
    #[inline(always)]
    pub const fn set_txf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Graceful Stop Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn gra(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Graceful Stop Complete"]
    #[inline(always)]
    pub const fn set_gra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Babbling Transmit Error"]
    #[must_use]
    #[inline(always)]
    pub const fn babt(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Babbling Transmit Error"]
    #[inline(always)]
    pub const fn set_babt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Babbling Receive Error"]
    #[must_use]
    #[inline(always)]
    pub const fn babr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Babbling Receive Error"]
    #[inline(always)]
    pub const fn set_babr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Eir {
    #[inline(always)]
    fn default() -> Eir {
        Eir(0)
    }
}
impl core::fmt::Debug for Eir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eir")
            .field("ts_timer", &self.ts_timer())
            .field("ts_avail", &self.ts_avail())
            .field("wakeup", &self.wakeup())
            .field("plr", &self.plr())
            .field("un", &self.un())
            .field("rl", &self.rl())
            .field("lc", &self.lc())
            .field("eberr", &self.eberr())
            .field("mii", &self.mii())
            .field("rxb", &self.rxb())
            .field("rxf", &self.rxf())
            .field("txb", &self.txb())
            .field("txf", &self.txf())
            .field("gra", &self.gra())
            .field("babt", &self.babt())
            .field("babr", &self.babr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eir {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Eir {{ ts_timer: {=bool:?}, ts_avail: {=bool:?}, wakeup: {=bool:?}, plr: {=bool:?}, un: {=bool:?}, rl: {=bool:?}, lc: {=bool:?}, eberr: {=bool:?}, mii: {=bool:?}, rxb: {=bool:?}, rxf: {=bool:?}, txb: {=bool:?}, txf: {=bool:?}, gra: {=bool:?}, babt: {=bool:?}, babr: {=bool:?} }}" , self . ts_timer () , self . ts_avail () , self . wakeup () , self . plr () , self . un () , self . rl () , self . lc () , self . eberr () , self . mii () , self . rxb () , self . rxf () , self . txb () , self . txf () , self . gra () , self . babt () , self . babr ())
    }
}
#[doc = "Frame Truncation Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftrl(pub u32);
impl Ftrl {
    #[doc = "Frame Truncation Length"]
    #[must_use]
    #[inline(always)]
    pub const fn trunc_fl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Frame Truncation Length"]
    #[inline(always)]
    pub const fn set_trunc_fl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Ftrl {
    #[inline(always)]
    fn default() -> Ftrl {
        Ftrl(0)
    }
}
impl core::fmt::Debug for Ftrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftrl")
            .field("trunc_fl", &self.trunc_fl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ftrl {{ trunc_fl: {=u16:?} }}", self.trunc_fl())
    }
}
#[doc = "Frames Received with Alignment Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRalign(pub u32);
impl IeeeRalign {
    #[doc = "Number of frames received with alignment error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames received with alignment error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRalign {
    #[inline(always)]
    fn default() -> IeeeRalign {
        IeeeRalign(0)
    }
}
impl core::fmt::Debug for IeeeRalign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRalign")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRalign {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRalign {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Received with CRC Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRcrc(pub u32);
impl IeeeRcrc {
    #[doc = "Number of frames received with CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames received with CRC error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRcrc {
    #[inline(always)]
    fn default() -> IeeeRcrc {
        IeeeRcrc(0)
    }
}
impl core::fmt::Debug for IeeeRcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRcrc")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRcrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRcrc {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames not Counted Correctly Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRdrop(pub u32);
impl IeeeRdrop {
    #[doc = "Frame count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRdrop {
    #[inline(always)]
    fn default() -> IeeeRdrop {
        IeeeRdrop(0)
    }
}
impl core::fmt::Debug for IeeeRdrop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRdrop")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRdrop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRdrop {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Flow Control Pause Frames Received Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRfdxfc(pub u32);
impl IeeeRfdxfc {
    #[doc = "Number of flow-control pause frames received"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of flow-control pause frames received"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRfdxfc {
    #[inline(always)]
    fn default() -> IeeeRfdxfc {
        IeeeRfdxfc(0)
    }
}
impl core::fmt::Debug for IeeeRfdxfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRfdxfc")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRfdxfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRfdxfc {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Received OK Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRframeOk(pub u32);
impl IeeeRframeOk {
    #[doc = "Number of frames received OK"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames received OK"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRframeOk {
    #[inline(always)]
    fn default() -> IeeeRframeOk {
        IeeeRframeOk(0)
    }
}
impl core::fmt::Debug for IeeeRframeOk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRframeOk")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRframeOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRframeOk {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Receive FIFO Overflow Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeRmacerr(pub u32);
impl IeeeRmacerr {
    #[doc = "Receive FIFO overflow count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive FIFO overflow count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeRmacerr {
    #[inline(always)]
    fn default() -> IeeeRmacerr {
        IeeeRmacerr(0)
    }
}
impl core::fmt::Debug for IeeeRmacerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeRmacerr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeRmacerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeRmacerr {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeT1col(pub u32);
impl IeeeT1col {
    #[doc = "Number of frames transmitted with one collision"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with one collision"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeT1col {
    #[inline(always)]
    fn default() -> IeeeT1col {
        IeeeT1col(0)
    }
}
impl core::fmt::Debug for IeeeT1col {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeT1col")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeT1col {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeT1col {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTcserr(pub u32);
impl IeeeTcserr {
    #[doc = "Number of frames transmitted with carrier sense error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with carrier sense error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTcserr {
    #[inline(always)]
    fn default() -> IeeeTcserr {
        IeeeTcserr(0)
    }
}
impl core::fmt::Debug for IeeeTcserr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTcserr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTcserr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTcserr {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTdef(pub u32);
impl IeeeTdef {
    #[doc = "Number of frames transmitted with deferral delay"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with deferral delay"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTdef {
    #[inline(always)]
    fn default() -> IeeeTdef {
        IeeeTdef(0)
    }
}
impl core::fmt::Debug for IeeeTdef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTdef")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTdef {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTdef {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTexcol(pub u32);
impl IeeeTexcol {
    #[doc = "Number of frames transmitted with excessive collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with excessive collisions"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTexcol {
    #[inline(always)]
    fn default() -> IeeeTexcol {
        IeeeTexcol(0)
    }
}
impl core::fmt::Debug for IeeeTexcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTexcol")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTexcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTexcol {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTfdxfc(pub u32);
impl IeeeTfdxfc {
    #[doc = "Number of flow-control pause frames transmitted"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of flow-control pause frames transmitted"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTfdxfc {
    #[inline(always)]
    fn default() -> IeeeTfdxfc {
        IeeeTfdxfc(0)
    }
}
impl core::fmt::Debug for IeeeTfdxfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTfdxfc")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTfdxfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTfdxfc {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted OK Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTframeOk(pub u32);
impl IeeeTframeOk {
    #[doc = "Number of frames transmitted OK"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted OK"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTframeOk {
    #[inline(always)]
    fn default() -> IeeeTframeOk {
        IeeeTframeOk(0)
    }
}
impl core::fmt::Debug for IeeeTframeOk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTframeOk")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTframeOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTframeOk {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTlcol(pub u32);
impl IeeeTlcol {
    #[doc = "Number of frames transmitted with late collision"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with late collision"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTlcol {
    #[inline(always)]
    fn default() -> IeeeTlcol {
        IeeeTlcol(0)
    }
}
impl core::fmt::Debug for IeeeTlcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTlcol")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTlcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTlcol {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTmacerr(pub u32);
impl IeeeTmacerr {
    #[doc = "Number of frames transmitted with transmit FIFO underrun"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with transmit FIFO underrun"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTmacerr {
    #[inline(always)]
    fn default() -> IeeeTmacerr {
        IeeeTmacerr(0)
    }
}
impl core::fmt::Debug for IeeeTmacerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTmacerr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTmacerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTmacerr {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTmcol(pub u32);
impl IeeeTmcol {
    #[doc = "Number of frames transmitted with multiple collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of frames transmitted with multiple collisions"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTmcol {
    #[inline(always)]
    fn default() -> IeeeTmcol {
        IeeeTmcol(0)
    }
}
impl core::fmt::Debug for IeeeTmcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTmcol")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTmcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTmcol {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Reserved Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeeTsqe(pub u32);
impl IeeeTsqe {
    #[doc = "This read-only field is reserved and always has the value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This read-only field is reserved and always has the value 0"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IeeeTsqe {
    #[inline(always)]
    fn default() -> IeeeTsqe {
        IeeeTsqe(0)
    }
}
impl core::fmt::Debug for IeeeTsqe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IeeeTsqe")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IeeeTsqe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IeeeTsqe {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "MIB Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mibc(pub u32);
impl Mibc {
    #[doc = "MIB Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn mib_clear(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "MIB Clear"]
    #[inline(always)]
    pub const fn set_mib_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "MIB Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn mib_idle(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "MIB Idle"]
    #[inline(always)]
    pub const fn set_mib_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Disable MIB Logic"]
    #[must_use]
    #[inline(always)]
    pub const fn mib_dis(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Disable MIB Logic"]
    #[inline(always)]
    pub const fn set_mib_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mibc {
    #[inline(always)]
    fn default() -> Mibc {
        Mibc(0)
    }
}
impl core::fmt::Debug for Mibc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mibc")
            .field("mib_clear", &self.mib_clear())
            .field("mib_idle", &self.mib_idle())
            .field("mib_dis", &self.mib_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mibc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mibc {{ mib_clear: {=bool:?}, mib_idle: {=bool:?}, mib_dis: {=bool:?} }}",
            self.mib_clear(),
            self.mib_idle(),
            self.mib_dis()
        )
    }
}
#[doc = "MII Management Frame Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmfr(pub u32);
impl Mmfr {
    #[doc = "Management Frame Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Management Frame Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Turn Around"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Turn Around"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Register Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "Register Address"]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "PHY Address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY Address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
    #[doc = "Operation Code"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Operation Code"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Start Of Frame Delimiter"]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Start Of Frame Delimiter"]
    #[inline(always)]
    pub const fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Mmfr {
    #[inline(always)]
    fn default() -> Mmfr {
        Mmfr(0)
    }
}
impl core::fmt::Debug for Mmfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mmfr")
            .field("data", &self.data())
            .field("ta", &self.ta())
            .field("ra", &self.ra())
            .field("pa", &self.pa())
            .field("op", &self.op())
            .field("st", &self.st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mmfr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mmfr {{ data: {=u16:?}, ta: {=u8:?}, ra: {=u8:?}, pa: {=u8:?}, op: {=u8:?}, st: {=u8:?} }}" , self . data () , self . ta () , self . ra () , self . pa () , self . op () , self . st ())
    }
}
#[doc = "Maximum Receive Buffer Size Register - Ring 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrbr(pub u32);
impl Mrbr {
    #[doc = "Receive buffer size in bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn r_buf_size(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive buffer size in bytes"]
    #[inline(always)]
    pub const fn set_r_buf_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 4usize)) | (((val as u32) & 0x03ff) << 4usize);
    }
}
impl Default for Mrbr {
    #[inline(always)]
    fn default() -> Mrbr {
        Mrbr(0)
    }
}
impl core::fmt::Debug for Mrbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrbr")
            .field("r_buf_size", &self.r_buf_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mrbr {{ r_buf_size: {=u16:?} }}", self.r_buf_size())
    }
}
#[doc = "MII Speed Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mscr(pub u32);
impl Mscr {
    #[doc = "MII Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_speed(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "MII Speed"]
    #[inline(always)]
    pub const fn set_mii_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Disable Preamble"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_pre(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Preamble"]
    #[inline(always)]
    pub const fn set_dis_pre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hold time On MDIO Output"]
    #[must_use]
    #[inline(always)]
    pub const fn holdtime(&self) -> super::vals::Holdtime {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Holdtime::from_bits(val as u8)
    }
    #[doc = "Hold time On MDIO Output"]
    #[inline(always)]
    pub const fn set_holdtime(&mut self, val: super::vals::Holdtime) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Mscr {
    #[inline(always)]
    fn default() -> Mscr {
        Mscr(0)
    }
}
impl core::fmt::Debug for Mscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mscr")
            .field("mii_speed", &self.mii_speed())
            .field("dis_pre", &self.dis_pre())
            .field("holdtime", &self.holdtime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mscr {{ mii_speed: {=u8:?}, dis_pre: {=bool:?}, holdtime: {:?} }}",
            self.mii_speed(),
            self.dis_pre(),
            self.holdtime()
        )
    }
}
#[doc = "Opcode/Pause Duration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opd(pub u32);
impl Opd {
    #[doc = "Pause Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_dur(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Pause Duration"]
    #[inline(always)]
    pub const fn set_pause_dur(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Opcode Field In PAUSE Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn opcode(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Opcode Field In PAUSE Frames"]
    #[inline(always)]
    pub const fn set_opcode(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Opd {
    #[inline(always)]
    fn default() -> Opd {
        Opd(0)
    }
}
impl core::fmt::Debug for Opd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opd")
            .field("pause_dur", &self.pause_dur())
            .field("opcode", &self.opcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Opd {{ pause_dur: {=u16:?}, opcode: {=u16:?} }}",
            self.pause_dur(),
            self.opcode()
        )
    }
}
#[doc = "Physical Address Upper Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Paur(pub u32);
impl Paur {
    #[doc = "Type Field In PAUSE Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Type Field In PAUSE Frames"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[must_use]
    #[inline(always)]
    pub const fn paddr2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline(always)]
    pub const fn set_paddr2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Paur {
    #[inline(always)]
    fn default() -> Paur {
        Paur(0)
    }
}
impl core::fmt::Debug for Paur {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Paur")
            .field("type_", &self.type_())
            .field("paddr2", &self.paddr2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Paur {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Paur {{ type_: {=u16:?}, paddr2: {=u16:?} }}",
            self.type_(),
            self.paddr2()
        )
    }
}
#[doc = "Receive Accelerator Function Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Racc(pub u32);
impl Racc {
    #[doc = "Enable Padding Removal For Short IP Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn padrem(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Padding Removal For Short IP Frames"]
    #[inline(always)]
    pub const fn set_padrem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[must_use]
    #[inline(always)]
    pub const fn ipdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline(always)]
    pub const fn set_ipdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[must_use]
    #[inline(always)]
    pub const fn prodis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline(always)]
    pub const fn set_prodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Discard Of Frames With MAC Layer Errors"]
    #[must_use]
    #[inline(always)]
    pub const fn linedis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Discard Of Frames With MAC Layer Errors"]
    #[inline(always)]
    pub const fn set_linedis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX FIFO Shift-16"]
    #[must_use]
    #[inline(always)]
    pub const fn shift16(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO Shift-16"]
    #[inline(always)]
    pub const fn set_shift16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Racc {
    #[inline(always)]
    fn default() -> Racc {
        Racc(0)
    }
}
impl core::fmt::Debug for Racc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Racc")
            .field("padrem", &self.padrem())
            .field("ipdis", &self.ipdis())
            .field("prodis", &self.prodis())
            .field("linedis", &self.linedis())
            .field("shift16", &self.shift16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Racc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Racc {{ padrem: {=bool:?}, ipdis: {=bool:?}, prodis: {=bool:?}, linedis: {=bool:?}, shift16: {=bool:?} }}" , self . padrem () , self . ipdis () , self . prodis () , self . linedis () , self . shift16 ())
    }
}
#[doc = "Receive FIFO Almost Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Raem(pub u32);
impl Raem {
    #[doc = "Value Of The Receive FIFO Almost Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_almost_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Receive FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub const fn set_rx_almost_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Raem {
    #[inline(always)]
    fn default() -> Raem {
        Raem(0)
    }
}
impl core::fmt::Debug for Raem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Raem")
            .field("rx_almost_empty", &self.rx_almost_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Raem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Raem {{ rx_almost_empty: {=u8:?} }}",
            self.rx_almost_empty()
        )
    }
}
#[doc = "Receive FIFO Almost Full Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rafl(pub u32);
impl Rafl {
    #[doc = "Value Of The Receive FIFO Almost Full Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_almost_full(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub const fn set_rx_almost_full(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rafl {
    #[inline(always)]
    fn default() -> Rafl {
        Rafl(0)
    }
}
impl core::fmt::Debug for Rafl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rafl")
            .field("rx_almost_full", &self.rx_almost_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rafl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rafl {{ rx_almost_full: {=u8:?} }}",
            self.rx_almost_full()
        )
    }
}
#[doc = "Receive Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Internal Loopback"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Loopback"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable Receive On Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn drt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Receive On Transmit"]
    #[inline(always)]
    pub const fn set_drt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Media Independent Interface Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Media Independent Interface Mode"]
    #[inline(always)]
    pub const fn set_mii_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Promiscuous Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn prom(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Promiscuous Mode"]
    #[inline(always)]
    pub const fn set_prom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Broadcast Frame Reject"]
    #[must_use]
    #[inline(always)]
    pub const fn bc_rej(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Broadcast Frame Reject"]
    #[inline(always)]
    pub const fn set_bc_rej(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Flow Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fce(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Flow Control Enable"]
    #[inline(always)]
    pub const fn set_fce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RMII Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RMII Mode Enable"]
    #[inline(always)]
    pub const fn set_rmii_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables 10-Mbit/s mode of the RMII ."]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_10t(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub const fn set_rmii_10t(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Frame Padding Remove On Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn paden(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub const fn set_paden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Terminate/Forward Pause Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn paufwd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub const fn set_paufwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Terminate/Forward Received CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn crcfwd(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Terminate/Forward Received CRC"]
    #[inline(always)]
    pub const fn set_crcfwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "MAC Control Frame Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cfen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Control Frame Enable"]
    #[inline(always)]
    pub const fn set_cfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Maximum Frame Length"]
    #[must_use]
    #[inline(always)]
    pub const fn max_fl(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Maximum Frame Length"]
    #[inline(always)]
    pub const fn set_max_fl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
    #[doc = "Payload Length Check Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn nlc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Payload Length Check Disable"]
    #[inline(always)]
    pub const fn set_nlc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Graceful Receive Stopped"]
    #[must_use]
    #[inline(always)]
    pub const fn grs(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Graceful Receive Stopped"]
    #[inline(always)]
    pub const fn set_grs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr")
            .field("loop_", &self.loop_())
            .field("drt", &self.drt())
            .field("mii_mode", &self.mii_mode())
            .field("prom", &self.prom())
            .field("bc_rej", &self.bc_rej())
            .field("fce", &self.fce())
            .field("rmii_mode", &self.rmii_mode())
            .field("rmii_10t", &self.rmii_10t())
            .field("paden", &self.paden())
            .field("paufwd", &self.paufwd())
            .field("crcfwd", &self.crcfwd())
            .field("cfen", &self.cfen())
            .field("max_fl", &self.max_fl())
            .field("nlc", &self.nlc())
            .field("grs", &self.grs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rcr {{ loop_: {=bool:?}, drt: {=bool:?}, mii_mode: {=bool:?}, prom: {=bool:?}, bc_rej: {=bool:?}, fce: {=bool:?}, rmii_mode: {=bool:?}, rmii_10t: {=bool:?}, paden: {=bool:?}, paufwd: {=bool:?}, crcfwd: {=bool:?}, cfen: {=bool:?}, max_fl: {=u16:?}, nlc: {=bool:?}, grs: {=bool:?} }}" , self . loop_ () , self . drt () , self . mii_mode () , self . prom () , self . bc_rej () , self . fce () , self . rmii_mode () , self . rmii_10t () , self . paden () , self . paufwd () , self . crcfwd () , self . cfen () , self . max_fl () , self . nlc () , self . grs ())
    }
}
#[doc = "Receive Descriptor Active Register - Ring 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdar(pub u32);
impl Rdar {
    #[doc = "Receive Descriptor Active"]
    #[must_use]
    #[inline(always)]
    pub const fn rdar(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Descriptor Active"]
    #[inline(always)]
    pub const fn set_rdar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Rdar {
    #[inline(always)]
    fn default() -> Rdar {
        Rdar(0)
    }
}
impl core::fmt::Debug for Rdar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdar").field("rdar", &self.rdar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdar {{ rdar: {=bool:?} }}", self.rdar())
    }
}
#[doc = "Receive Descriptor Ring 0 Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdsr(pub u32);
impl Rdsr {
    #[doc = "Pointer to the beginning of the receive buffer descriptor queue."]
    #[must_use]
    #[inline(always)]
    pub const fn r_des_start(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline(always)]
    pub const fn set_r_des_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Rdsr {
    #[inline(always)]
    fn default() -> Rdsr {
        Rdsr(0)
    }
}
impl core::fmt::Debug for Rdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdsr")
            .field("r_des_start", &self.r_des_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdsr {{ r_des_start: {=u32:?} }}", self.r_des_start())
    }
}
#[doc = "Rx Broadcast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRbcPkt(pub u32);
impl RmonRbcPkt {
    #[doc = "Number of receive broadcast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive broadcast packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRbcPkt {
    #[inline(always)]
    fn default() -> RmonRbcPkt {
        RmonRbcPkt(0)
    }
}
impl core::fmt::Debug for RmonRbcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRbcPkt")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRbcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRbcPkt {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRcrcAlign(pub u32);
impl RmonRcrcAlign {
    #[doc = "Number of receive packets with CRC or align error"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets with CRC or align error"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRcrcAlign {
    #[inline(always)]
    fn default() -> RmonRcrcAlign {
        RmonRcrcAlign(0)
    }
}
impl core::fmt::Debug for RmonRcrcAlign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRcrcAlign")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRcrcAlign {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRcrcAlign {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRfrag(pub u32);
impl RmonRfrag {
    #[doc = "Number of receive packets with less than 64 bytes and bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets with less than 64 bytes and bad CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRfrag {
    #[inline(always)]
    fn default() -> RmonRfrag {
        RmonRfrag(0)
    }
}
impl core::fmt::Debug for RmonRfrag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRfrag")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRfrag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRfrag {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRjab(pub u32);
impl RmonRjab {
    #[doc = "Number of receive packets greater than MAX_FL and bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets greater than MAX_FL and bad CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRjab {
    #[inline(always)]
    fn default() -> RmonRjab {
        RmonRjab(0)
    }
}
impl core::fmt::Debug for RmonRjab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRjab")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRjab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRjab {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Multicast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRmcPkt(pub u32);
impl RmonRmcPkt {
    #[doc = "Number of receive multicast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive multicast packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRmcPkt {
    #[inline(always)]
    fn default() -> RmonRmcPkt {
        RmonRmcPkt(0)
    }
}
impl core::fmt::Debug for RmonRmcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRmcPkt")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRmcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRmcPkt {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRoversize(pub u32);
impl RmonRoversize {
    #[doc = "Number of receive packets greater than MAX_FL and good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets greater than MAX_FL and good CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRoversize {
    #[inline(always)]
    fn default() -> RmonRoversize {
        RmonRoversize(0)
    }
}
impl core::fmt::Debug for RmonRoversize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRoversize")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRoversize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRoversize {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRp1024to2047(pub u32);
impl RmonRp1024to2047 {
    #[doc = "Number of 1024- to 2047-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 1024- to 2047-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRp1024to2047 {
    #[inline(always)]
    fn default() -> RmonRp1024to2047 {
        RmonRp1024to2047(0)
    }
}
impl core::fmt::Debug for RmonRp1024to2047 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRp1024to2047")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRp1024to2047 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRp1024to2047 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRp128to255(pub u32);
impl RmonRp128to255 {
    #[doc = "Number of 128- to 255-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 128- to 255-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRp128to255 {
    #[inline(always)]
    fn default() -> RmonRp128to255 {
        RmonRp128to255(0)
    }
}
impl core::fmt::Debug for RmonRp128to255 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRp128to255")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRp128to255 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRp128to255 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRp256to511(pub u32);
impl RmonRp256to511 {
    #[doc = "Number of 256- to 511-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 256- to 511-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRp256to511 {
    #[inline(always)]
    fn default() -> RmonRp256to511 {
        RmonRp256to511(0)
    }
}
impl core::fmt::Debug for RmonRp256to511 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRp256to511")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRp256to511 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRp256to511 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRp512to1023(pub u32);
impl RmonRp512to1023 {
    #[doc = "Number of 512- to 1023-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 512- to 1023-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRp512to1023 {
    #[inline(always)]
    fn default() -> RmonRp512to1023 {
        RmonRp512to1023(0)
    }
}
impl core::fmt::Debug for RmonRp512to1023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRp512to1023")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRp512to1023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRp512to1023 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 64-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRp64(pub u32);
impl RmonRp64 {
    #[doc = "Number of 64-byte receive packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 64-byte receive packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRp64 {
    #[inline(always)]
    fn default() -> RmonRp64 {
        RmonRp64(0)
    }
}
impl core::fmt::Debug for RmonRp64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRp64")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRp64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRp64 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRp65to127(pub u32);
impl RmonRp65to127 {
    #[doc = "Number of 65- to 127-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 65- to 127-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRp65to127 {
    #[inline(always)]
    fn default() -> RmonRp65to127 {
        RmonRp65to127(0)
    }
}
impl core::fmt::Debug for RmonRp65to127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRp65to127")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRp65to127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRp65to127 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packet Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRpackets(pub u32);
impl RmonRpackets {
    #[doc = "Number of packets received"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of packets received"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRpackets {
    #[inline(always)]
    fn default() -> RmonRpackets {
        RmonRpackets(0)
    }
}
impl core::fmt::Debug for RmonRpackets {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRpackets")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRpackets {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRpackets {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRpgte2048(pub u32);
impl RmonRpgte2048 {
    #[doc = "Number of greater-than-2048-byte recieve packets"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of greater-than-2048-byte recieve packets"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRpgte2048 {
    #[inline(always)]
    fn default() -> RmonRpgte2048 {
        RmonRpgte2048(0)
    }
}
impl core::fmt::Debug for RmonRpgte2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRpgte2048")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRpgte2048 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRpgte2048 {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonRundersize(pub u32);
impl RmonRundersize {
    #[doc = "Number of receive packets with less than 64 bytes and good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of receive packets with less than 64 bytes and good CRC"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonRundersize {
    #[inline(always)]
    fn default() -> RmonRundersize {
        RmonRundersize(0)
    }
}
impl core::fmt::Debug for RmonRundersize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonRundersize")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonRundersize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonRundersize {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "Tx Broadcast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTbcPkt(pub u32);
impl RmonTbcPkt {
    #[doc = "Number of broadcast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of broadcast packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTbcPkt {
    #[inline(always)]
    fn default() -> RmonTbcPkt {
        RmonTbcPkt(0)
    }
}
impl core::fmt::Debug for RmonTbcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTbcPkt")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTbcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTbcPkt {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Collision Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTcol(pub u32);
impl RmonTcol {
    #[doc = "Number of transmit collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit collisions"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTcol {
    #[inline(always)]
    fn default() -> RmonTcol {
        RmonTcol(0)
    }
}
impl core::fmt::Debug for RmonTcol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTcol")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTcol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTcol {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTcrcAlign(pub u32);
impl RmonTcrcAlign {
    #[doc = "Number of packets with CRC/align error"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of packets with CRC/align error"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTcrcAlign {
    #[inline(always)]
    fn default() -> RmonTcrcAlign {
        RmonTcrcAlign(0)
    }
}
impl core::fmt::Debug for RmonTcrcAlign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTcrcAlign")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTcrcAlign {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTcrcAlign {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTfrag(pub u32);
impl RmonTfrag {
    #[doc = "Number of packets less than 64 bytes with bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of packets less than 64 bytes with bad CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTfrag {
    #[inline(always)]
    fn default() -> RmonTfrag {
        RmonTfrag(0)
    }
}
impl core::fmt::Debug for RmonTfrag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTfrag")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTfrag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTfrag {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTjab(pub u32);
impl RmonTjab {
    #[doc = "Number of transmit packets greater than MAX_FL bytes and bad CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets greater than MAX_FL bytes and bad CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTjab {
    #[inline(always)]
    fn default() -> RmonTjab {
        RmonTjab(0)
    }
}
impl core::fmt::Debug for RmonTjab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTjab")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTjab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTjab {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Multicast Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTmcPkt(pub u32);
impl RmonTmcPkt {
    #[doc = "Number of multicast packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of multicast packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTmcPkt {
    #[inline(always)]
    fn default() -> RmonTmcPkt {
        RmonTmcPkt(0)
    }
}
impl core::fmt::Debug for RmonTmcPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTmcPkt")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTmcPkt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTmcPkt {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonToversize(pub u32);
impl RmonToversize {
    #[doc = "Number of transmit packets greater than MAX_FL bytes with good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets greater than MAX_FL bytes with good CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonToversize {
    #[inline(always)]
    fn default() -> RmonToversize {
        RmonToversize(0)
    }
}
impl core::fmt::Debug for RmonToversize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonToversize")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonToversize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonToversize {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTp1024to2047(pub u32);
impl RmonTp1024to2047 {
    #[doc = "Number of 1024- to 2047-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 1024- to 2047-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTp1024to2047 {
    #[inline(always)]
    fn default() -> RmonTp1024to2047 {
        RmonTp1024to2047(0)
    }
}
impl core::fmt::Debug for RmonTp1024to2047 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTp1024to2047")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTp1024to2047 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTp1024to2047 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTp128to255(pub u32);
impl RmonTp128to255 {
    #[doc = "Number of 128- to 255-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 128- to 255-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTp128to255 {
    #[inline(always)]
    fn default() -> RmonTp128to255 {
        RmonTp128to255(0)
    }
}
impl core::fmt::Debug for RmonTp128to255 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTp128to255")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTp128to255 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTp128to255 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTp256to511(pub u32);
impl RmonTp256to511 {
    #[doc = "Number of 256- to 511-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 256- to 511-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTp256to511 {
    #[inline(always)]
    fn default() -> RmonTp256to511 {
        RmonTp256to511(0)
    }
}
impl core::fmt::Debug for RmonTp256to511 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTp256to511")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTp256to511 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTp256to511 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTp512to1023(pub u32);
impl RmonTp512to1023 {
    #[doc = "Number of 512- to 1023-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 512- to 1023-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTp512to1023 {
    #[inline(always)]
    fn default() -> RmonTp512to1023 {
        RmonTp512to1023(0)
    }
}
impl core::fmt::Debug for RmonTp512to1023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTp512to1023")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTp512to1023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTp512to1023 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 64-Byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTp64(pub u32);
impl RmonTp64 {
    #[doc = "Number of 64-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 64-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTp64 {
    #[inline(always)]
    fn default() -> RmonTp64 {
        RmonTp64(0)
    }
}
impl core::fmt::Debug for RmonTp64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTp64")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTp64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTp64 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTp65to127(pub u32);
impl RmonTp65to127 {
    #[doc = "Number of 65- to 127-byte transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of 65- to 127-byte transmit packets"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTp65to127 {
    #[inline(always)]
    fn default() -> RmonTp65to127 {
        RmonTp65to127(0)
    }
}
impl core::fmt::Debug for RmonTp65to127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTp65to127")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTp65to127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTp65to127 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packet Count Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTpackets(pub u32);
impl RmonTpackets {
    #[doc = "Packet count"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Packet count"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTpackets {
    #[inline(always)]
    fn default() -> RmonTpackets {
        RmonTpackets(0)
    }
}
impl core::fmt::Debug for RmonTpackets {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTpackets")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTpackets {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTpackets {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTpgte2048(pub u32);
impl RmonTpgte2048 {
    #[doc = "Number of transmit packets greater than 2048 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets greater than 2048 bytes"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTpgte2048 {
    #[inline(always)]
    fn default() -> RmonTpgte2048 {
        RmonTpgte2048(0)
    }
}
impl core::fmt::Debug for RmonTpgte2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTpgte2048")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTpgte2048 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTpgte2048 {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmonTundersize(pub u32);
impl RmonTundersize {
    #[doc = "Number of transmit packets less than 64 bytes with good CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn txpkts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of transmit packets less than 64 bytes with good CRC"]
    #[inline(always)]
    pub const fn set_txpkts(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RmonTundersize {
    #[inline(always)]
    fn default() -> RmonTundersize {
        RmonTundersize(0)
    }
}
impl core::fmt::Debug for RmonTundersize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmonTundersize")
            .field("txpkts", &self.txpkts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmonTundersize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmonTundersize {{ txpkts: {=u16:?} }}", self.txpkts())
    }
}
#[doc = "Receive FIFO Section Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsem(pub u32);
impl Rsem {
    #[doc = "Value Of The Receive FIFO Section Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_section_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn set_rx_section_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Status FIFO Section Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn stat_section_empty(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "RX Status FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn set_stat_section_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Rsem {
    #[inline(always)]
    fn default() -> Rsem {
        Rsem(0)
    }
}
impl core::fmt::Debug for Rsem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsem")
            .field("rx_section_empty", &self.rx_section_empty())
            .field("stat_section_empty", &self.stat_section_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsem {{ rx_section_empty: {=u8:?}, stat_section_empty: {=u8:?} }}",
            self.rx_section_empty(),
            self.stat_section_empty()
        )
    }
}
#[doc = "Receive FIFO Section Full Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsfl(pub u32);
impl Rsfl {
    #[doc = "Value Of Receive FIFO Section Full Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_section_full(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub const fn set_rx_section_full(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rsfl {
    #[inline(always)]
    fn default() -> Rsfl {
        Rsfl(0)
    }
}
impl core::fmt::Debug for Rsfl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsfl")
            .field("rx_section_full", &self.rx_section_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsfl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsfl {{ rx_section_full: {=u8:?} }}",
            self.rx_section_full()
        )
    }
}
#[doc = "Receive Interrupt Coalescing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxic0(pub u32);
impl Rxic0 {
    #[doc = "Interrupt coalescing timer threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ictt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub const fn set_ictt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn icft(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub const fn set_icft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iccs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub const fn set_iccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn icen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[inline(always)]
    pub const fn set_icen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxic0 {
    #[inline(always)]
    fn default() -> Rxic0 {
        Rxic0(0)
    }
}
impl core::fmt::Debug for Rxic0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxic0")
            .field("ictt", &self.ictt())
            .field("icft", &self.icft())
            .field("iccs", &self.iccs())
            .field("icen", &self.icen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxic0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxic0 {{ ictt: {=u16:?}, icft: {=u8:?}, iccs: {=bool:?}, icen: {=bool:?} }}",
            self.ictt(),
            self.icft(),
            self.iccs(),
            self.icen()
        )
    }
}
#[doc = "Transmit Accelerator Function Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tacc(pub u32);
impl Tacc {
    #[doc = "TX FIFO Shift-16"]
    #[must_use]
    #[inline(always)]
    pub const fn shift16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO Shift-16"]
    #[inline(always)]
    pub const fn set_shift16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables insertion of IP header checksum."]
    #[must_use]
    #[inline(always)]
    pub const fn ipchk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables insertion of IP header checksum."]
    #[inline(always)]
    pub const fn set_ipchk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables insertion of protocol checksum."]
    #[must_use]
    #[inline(always)]
    pub const fn prochk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables insertion of protocol checksum."]
    #[inline(always)]
    pub const fn set_prochk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Tacc {
    #[inline(always)]
    fn default() -> Tacc {
        Tacc(0)
    }
}
impl core::fmt::Debug for Tacc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tacc")
            .field("shift16", &self.shift16())
            .field("ipchk", &self.ipchk())
            .field("prochk", &self.prochk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tacc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tacc {{ shift16: {=bool:?}, ipchk: {=bool:?}, prochk: {=bool:?} }}",
            self.shift16(),
            self.ipchk(),
            self.prochk()
        )
    }
}
#[doc = "Transmit FIFO Almost Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Taem(pub u32);
impl Taem {
    #[doc = "Value of Transmit FIFO Almost Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_almost_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value of Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub const fn set_tx_almost_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Taem {
    #[inline(always)]
    fn default() -> Taem {
        Taem(0)
    }
}
impl core::fmt::Debug for Taem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Taem")
            .field("tx_almost_empty", &self.tx_almost_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Taem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Taem {{ tx_almost_empty: {=u8:?} }}",
            self.tx_almost_empty()
        )
    }
}
#[doc = "Transmit FIFO Almost Full Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tafl(pub u32);
impl Tafl {
    #[doc = "Value Of The Transmit FIFO Almost Full Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_almost_full(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub const fn set_tx_almost_full(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tafl {
    #[inline(always)]
    fn default() -> Tafl {
        Tafl(0)
    }
}
impl core::fmt::Debug for Tafl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tafl")
            .field("tx_almost_full", &self.tx_almost_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tafl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tafl {{ tx_almost_full: {=u8:?} }}",
            self.tx_almost_full()
        )
    }
}
#[doc = "Transmit Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Graceful Transmit Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn gts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Graceful Transmit Stop"]
    #[inline(always)]
    pub const fn set_gts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Full-Duplex Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fden(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Full-Duplex Enable"]
    #[inline(always)]
    pub const fn set_fden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit Frame Control Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn tfc_pause(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Frame Control Pause"]
    #[inline(always)]
    pub const fn set_tfc_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive Frame Control Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn rfc_pause(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Frame Control Pause"]
    #[inline(always)]
    pub const fn set_rfc_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Source MAC Address Select On Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn addsel(&self) -> super::vals::Addsel {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Addsel::from_bits(val as u8)
    }
    #[doc = "Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub const fn set_addsel(&mut self, val: super::vals::Addsel) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Set MAC Address On Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn addins(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set MAC Address On Transmit"]
    #[inline(always)]
    pub const fn set_addins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Forward Frame From Application With CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn crcfwd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Frame From Application With CRC"]
    #[inline(always)]
    pub const fn set_crcfwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("gts", &self.gts())
            .field("fden", &self.fden())
            .field("tfc_pause", &self.tfc_pause())
            .field("rfc_pause", &self.rfc_pause())
            .field("addsel", &self.addsel())
            .field("addins", &self.addins())
            .field("crcfwd", &self.crcfwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr {{ gts: {=bool:?}, fden: {=bool:?}, tfc_pause: {=bool:?}, rfc_pause: {=bool:?}, addsel: {:?}, addins: {=bool:?}, crcfwd: {=bool:?} }}" , self . gts () , self . fden () , self . tfc_pause () , self . rfc_pause () , self . addsel () , self . addins () , self . crcfwd ())
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr0(pub u32);
impl Tcsr0 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr0tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr0tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr0tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr0tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr0tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr0tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr0 {
    #[inline(always)]
    fn default() -> Tcsr0 {
        Tcsr0(0)
    }
}
impl core::fmt::Debug for Tcsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr0")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr0 {{ tdre: {=bool:?}, tmode: {:?}, tie: {=bool:?}, tf: {=bool:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr1(pub u32);
impl Tcsr1 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr1tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr1tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr1tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr1tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr1tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr1tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr1 {
    #[inline(always)]
    fn default() -> Tcsr1 {
        Tcsr1(0)
    }
}
impl core::fmt::Debug for Tcsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr1")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr1 {{ tdre: {=bool:?}, tmode: {:?}, tie: {=bool:?}, tf: {=bool:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr2(pub u32);
impl Tcsr2 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr2tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr2tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr2tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr2tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr2tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr2tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr2 {
    #[inline(always)]
    fn default() -> Tcsr2 {
        Tcsr2(0)
    }
}
impl core::fmt::Debug for Tcsr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr2")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr2 {{ tdre: {=bool:?}, tmode: {:?}, tie: {=bool:?}, tf: {=bool:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Timer Control Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr3(pub u32);
impl Tcsr3 {
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tcsr3tmode {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Tcsr3tmode::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tcsr3tmode) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Flag"]
    #[inline(always)]
    pub const fn set_tf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer PulseWidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpwc(&self) -> super::vals::Tcsr3tpwc {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Tcsr3tpwc::from_bits(val as u8)
    }
    #[doc = "Timer PulseWidth Control"]
    #[inline(always)]
    pub const fn set_tpwc(&mut self, val: super::vals::Tcsr3tpwc) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u32) & 0x1f) << 11usize);
    }
}
impl Default for Tcsr3 {
    #[inline(always)]
    fn default() -> Tcsr3 {
        Tcsr3(0)
    }
}
impl core::fmt::Debug for Tcsr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr3")
            .field("tdre", &self.tdre())
            .field("tmode", &self.tmode())
            .field("tie", &self.tie())
            .field("tf", &self.tf())
            .field("tpwc", &self.tpwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr3 {{ tdre: {=bool:?}, tmode: {:?}, tie: {=bool:?}, tf: {=bool:?}, tpwc: {:?} }}",
            self.tdre(),
            self.tmode(),
            self.tie(),
            self.tf(),
            self.tpwc()
        )
    }
}
#[doc = "Transmit Descriptor Active Register - Ring 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdar(pub u32);
impl Tdar {
    #[doc = "Transmit Descriptor Active"]
    #[must_use]
    #[inline(always)]
    pub const fn tdar(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Descriptor Active"]
    #[inline(always)]
    pub const fn set_tdar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Tdar {
    #[inline(always)]
    fn default() -> Tdar {
        Tdar(0)
    }
}
impl core::fmt::Debug for Tdar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdar").field("tdar", &self.tdar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdar {{ tdar: {=bool:?} }}", self.tdar())
    }
}
#[doc = "Transmit Buffer Descriptor Ring 0 Start Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdsr(pub u32);
impl Tdsr {
    #[doc = "Pointer to the beginning of the transmit buffer descriptor queue."]
    #[must_use]
    #[inline(always)]
    pub const fn x_des_start(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline(always)]
    pub const fn set_x_des_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Tdsr {
    #[inline(always)]
    fn default() -> Tdsr {
        Tdsr(0)
    }
}
impl core::fmt::Debug for Tdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdsr")
            .field("x_des_start", &self.x_des_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdsr {{ x_des_start: {=u32:?} }}", self.x_des_start())
    }
}
#[doc = "Transmit FIFO Watermark Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfwr(pub u32);
impl Tfwr {
    #[doc = "Transmit FIFO Write"]
    #[must_use]
    #[inline(always)]
    pub const fn tfwr(&self) -> super::vals::Tfwr {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Tfwr::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Write"]
    #[inline(always)]
    pub const fn set_tfwr(&mut self, val: super::vals::Tfwr) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Store And Forward Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn strfwd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Store And Forward Enable"]
    #[inline(always)]
    pub const fn set_strfwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Tfwr {
    #[inline(always)]
    fn default() -> Tfwr {
        Tfwr(0)
    }
}
impl core::fmt::Debug for Tfwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfwr")
            .field("tfwr", &self.tfwr())
            .field("strfwd", &self.strfwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfwr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tfwr {{ tfwr: {:?}, strfwd: {=bool:?} }}",
            self.tfwr(),
            self.strfwd()
        )
    }
}
#[doc = "Timer Global Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tgsr(pub u32);
impl Tgsr {
    #[doc = "Copy Of Timer Flag For Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn tf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Copy Of Timer Flag For Channel 0"]
    #[inline(always)]
    pub const fn set_tf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Copy Of Timer Flag For Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Copy Of Timer Flag For Channel 1"]
    #[inline(always)]
    pub const fn set_tf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Copy Of Timer Flag For Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Copy Of Timer Flag For Channel 2"]
    #[inline(always)]
    pub const fn set_tf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Copy Of Timer Flag For Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn tf3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Copy Of Timer Flag For Channel 3"]
    #[inline(always)]
    pub const fn set_tf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Tgsr {
    #[inline(always)]
    fn default() -> Tgsr {
        Tgsr(0)
    }
}
impl core::fmt::Debug for Tgsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tgsr")
            .field("tf0", &self.tf0())
            .field("tf1", &self.tf1())
            .field("tf2", &self.tf2())
            .field("tf3", &self.tf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tgsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tgsr {{ tf0: {=bool:?}, tf1: {=bool:?}, tf2: {=bool:?}, tf3: {=bool:?} }}",
            self.tf0(),
            self.tf1(),
            self.tf2(),
            self.tf3()
        )
    }
}
#[doc = "Transmit Inter-Packet Gap"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tipg(pub u32);
impl Tipg {
    #[doc = "Transmit Inter-Packet Gap"]
    #[must_use]
    #[inline(always)]
    pub const fn ipg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub const fn set_ipg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Tipg {
    #[inline(always)]
    fn default() -> Tipg {
        Tipg(0)
    }
}
impl core::fmt::Debug for Tipg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tipg").field("ipg", &self.ipg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tipg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tipg {{ ipg: {=u8:?} }}", self.ipg())
    }
}
#[doc = "Transmit FIFO Section Empty Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsem(pub u32);
impl Tsem {
    #[doc = "Value Of The Transmit FIFO Section Empty Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_section_empty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn set_tx_section_empty(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tsem {
    #[inline(always)]
    fn default() -> Tsem {
        Tsem(0)
    }
}
impl core::fmt::Debug for Tsem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsem")
            .field("tx_section_empty", &self.tx_section_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsem {{ tx_section_empty: {=u8:?} }}",
            self.tx_section_empty()
        )
    }
}
#[doc = "Transmit Interrupt Coalescing Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txic0(pub u32);
impl Txic0 {
    #[doc = "Interrupt coalescing timer threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ictt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub const fn set_ictt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn icft(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub const fn set_icft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iccs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub const fn set_iccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn icen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Coalescing Enable"]
    #[inline(always)]
    pub const fn set_icen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Txic0 {
    #[inline(always)]
    fn default() -> Txic0 {
        Txic0(0)
    }
}
impl core::fmt::Debug for Txic0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txic0")
            .field("ictt", &self.ictt())
            .field("icft", &self.icft())
            .field("iccs", &self.iccs())
            .field("icen", &self.icen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txic0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txic0 {{ ictt: {=u16:?}, icft: {=u8:?}, iccs: {=bool:?}, icen: {=bool:?} }}",
            self.ictt(),
            self.icft(),
            self.iccs(),
            self.icen()
        )
    }
}
