#[doc = "DTCM Magic Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DtcmMagicAddr(pub u32);
impl DtcmMagicAddr {
    #[doc = "DTCM Write Read Select"]
    #[inline(always)]
    pub const fn dtcm_wr_rd_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Write Read Select"]
    #[inline(always)]
    pub const fn set_dtcm_wr_rd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTCM Magic Address"]
    #[inline(always)]
    pub const fn dtcm_magic_addr(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0xffff;
        val as u16
    }
    #[doc = "DTCM Magic Address"]
    #[inline(always)]
    pub const fn set_dtcm_magic_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 1usize)) | (((val as u32) & 0xffff) << 1usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for DtcmMagicAddr {
    #[inline(always)]
    fn default() -> DtcmMagicAddr {
        DtcmMagicAddr(0)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IntSigEn(pub u32);
impl IntSigEn {
    #[doc = "ITCM Magic Address Match Interrupt Enable"]
    #[inline(always)]
    pub const fn itcm_mam_sig_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Magic Address Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_itcm_mam_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTCM Magic Address Match Interrupt Enable"]
    #[inline(always)]
    pub const fn dtcm_mam_sig_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Magic Address Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtcm_mam_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OCRAM Magic Address Match Interrupt Enable"]
    #[inline(always)]
    pub const fn ocram_mam_sig_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Magic Address Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ocram_mam_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn itcm_err_sig_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_itcm_err_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DTCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn dtcm_err_sig_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtcm_err_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn ocram_err_sig_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ocram_err_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IntSigEn {
    #[inline(always)]
    fn default() -> IntSigEn {
        IntSigEn(0)
    }
}
#[doc = "Interrupt Status Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IntStatEn(pub u32);
impl IntStatEn {
    #[doc = "ITCM Magic Address Match Status Enable"]
    #[inline(always)]
    pub const fn itcm_mam_stat_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Magic Address Match Status Enable"]
    #[inline(always)]
    pub const fn set_itcm_mam_stat_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTCM Magic Address Match Status Enable"]
    #[inline(always)]
    pub const fn dtcm_mam_stat_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Magic Address Match Status Enable"]
    #[inline(always)]
    pub const fn set_dtcm_mam_stat_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OCRAM Magic Address Match Status Enable"]
    #[inline(always)]
    pub const fn ocram_mam_stat_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Magic Address Match Status Enable"]
    #[inline(always)]
    pub const fn set_ocram_mam_stat_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITCM Access Error Status Enable"]
    #[inline(always)]
    pub const fn itcm_err_stat_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Access Error Status Enable"]
    #[inline(always)]
    pub const fn set_itcm_err_stat_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DTCM Access Error Status Enable"]
    #[inline(always)]
    pub const fn dtcm_err_stat_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Access Error Status Enable"]
    #[inline(always)]
    pub const fn set_dtcm_err_stat_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM Access Error Status Enable"]
    #[inline(always)]
    pub const fn ocram_err_stat_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Access Error Status Enable"]
    #[inline(always)]
    pub const fn set_ocram_err_stat_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IntStatEn {
    #[inline(always)]
    fn default() -> IntStatEn {
        IntStatEn(0)
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "ITCM Magic Address Match Status"]
    #[inline(always)]
    pub const fn itcm_mam_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Magic Address Match Status"]
    #[inline(always)]
    pub const fn set_itcm_mam_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTCM Magic Address Match Status"]
    #[inline(always)]
    pub const fn dtcm_mam_status(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Magic Address Match Status"]
    #[inline(always)]
    pub const fn set_dtcm_mam_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OCRAM Magic Address Match Status"]
    #[inline(always)]
    pub const fn ocram_mam_status(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Magic Address Match Status"]
    #[inline(always)]
    pub const fn set_ocram_mam_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITCM Access Error Status"]
    #[inline(always)]
    pub const fn itcm_err_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Access Error Status"]
    #[inline(always)]
    pub const fn set_itcm_err_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DTCM Access Error Status"]
    #[inline(always)]
    pub const fn dtcm_err_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DTCM Access Error Status"]
    #[inline(always)]
    pub const fn set_dtcm_err_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM Access Error Status"]
    #[inline(always)]
    pub const fn ocram_err_status(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Access Error Status"]
    #[inline(always)]
    pub const fn set_ocram_err_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
#[doc = "ITCM Magic Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ItcmMagicAddr(pub u32);
impl ItcmMagicAddr {
    #[doc = "ITCM Write Read Select"]
    #[inline(always)]
    pub const fn itcm_wr_rd_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ITCM Write Read Select"]
    #[inline(always)]
    pub const fn set_itcm_wr_rd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ITCM Magic Address"]
    #[inline(always)]
    pub const fn itcm_magic_addr(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0xffff;
        val as u16
    }
    #[doc = "ITCM Magic Address"]
    #[inline(always)]
    pub const fn set_itcm_magic_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 1usize)) | (((val as u32) & 0xffff) << 1usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for ItcmMagicAddr {
    #[inline(always)]
    fn default() -> ItcmMagicAddr {
        ItcmMagicAddr(0)
    }
}
#[doc = "OCRAM Magic Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OcramMagicAddr(pub u32);
impl OcramMagicAddr {
    #[doc = "OCRAM Write Read Select"]
    #[inline(always)]
    pub const fn ocram_wr_rd_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM Write Read Select"]
    #[inline(always)]
    pub const fn set_ocram_wr_rd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OCRAM Magic Address"]
    #[inline(always)]
    pub const fn ocram_magic_addr(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0xffff;
        val as u16
    }
    #[doc = "OCRAM Magic Address"]
    #[inline(always)]
    pub const fn set_ocram_magic_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 1usize)) | (((val as u32) & 0xffff) << 1usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for OcramMagicAddr {
    #[inline(always)]
    fn default() -> OcramMagicAddr {
        OcramMagicAddr(0)
    }
}
#[doc = "TCM CRTL Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TcmCtrl(pub u32);
impl TcmCtrl {
    #[doc = "TCM Write Wait Mode Enable"]
    #[inline(always)]
    pub const fn tcm_wwait_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TCM Write Wait Mode Enable"]
    #[inline(always)]
    pub const fn set_tcm_wwait_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCM Read Wait Mode Enable"]
    #[inline(always)]
    pub const fn tcm_rwait_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCM Read Wait Mode Enable"]
    #[inline(always)]
    pub const fn set_tcm_rwait_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force RAM Clock Always On"]
    #[inline(always)]
    pub const fn force_clk_on(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force RAM Clock Always On"]
    #[inline(always)]
    pub const fn set_force_clk_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for TcmCtrl {
    #[inline(always)]
    fn default() -> TcmCtrl {
        TcmCtrl(0)
    }
}
