#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Force Error"]
    #[inline(always)]
    pub const fn ferr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Force Error"]
    #[inline(always)]
    pub const fn set_ferr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force Security Violation Mode"]
    #[inline(always)]
    pub const fn fsvm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force Security Violation Mode"]
    #[inline(always)]
    pub const fn set_fsvm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Force Logically Disabled Mode"]
    #[inline(always)]
    pub const fn fldm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Force Logically Disabled Mode"]
    #[inline(always)]
    pub const fn set_fldm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Key Blob Scramble Enable"]
    #[inline(always)]
    pub const fn kbse(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Key Blob Scramble Enable"]
    #[inline(always)]
    pub const fn set_kbse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Key Blob Processing Enable"]
    #[inline(always)]
    pub const fn kbpe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key Blob Processing Enable"]
    #[inline(always)]
    pub const fn set_kbpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Restricted Register Access Enable"]
    #[inline(always)]
    pub const fn rrae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Restricted Register Access Enable"]
    #[inline(always)]
    pub const fn set_rrae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Start key blob processing"]
    #[inline(always)]
    pub const fn skbp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Start key blob processing"]
    #[inline(always)]
    pub const fn set_skbp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Global OTFAD Enable"]
    #[inline(always)]
    pub const fn ge(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Global OTFAD Enable"]
    #[inline(always)]
    pub const fn set_ge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "AES Region Descriptor Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtxRgdW0(pub u32);
impl CtxRgdW0 {
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn srtaddr(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_srtaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for CtxRgdW0 {
    #[inline(always)]
    fn default() -> CtxRgdW0 {
        CtxRgdW0(0)
    }
}
#[doc = "AES Region Descriptor Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtxRgdW1(pub u32);
impl CtxRgdW1 {
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AES Decryption Enable."]
    #[inline(always)]
    pub const fn ade(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AES Decryption Enable."]
    #[inline(always)]
    pub const fn set_ade(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read-Only"]
    #[inline(always)]
    pub const fn ro(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read-Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "End Address"]
    #[inline(always)]
    pub const fn endaddr(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "End Address"]
    #[inline(always)]
    pub const fn set_endaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for CtxRgdW1 {
    #[inline(always)]
    fn default() -> CtxRgdW1 {
        CtxRgdW1(0)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Key Blob Error"]
    #[inline(always)]
    pub const fn kberr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Key Blob Error"]
    #[inline(always)]
    pub const fn set_kberr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MDPC Present"]
    #[inline(always)]
    pub const fn mdpcp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MDPC Present"]
    #[inline(always)]
    pub const fn set_mdpcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Operating Mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Operating Mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Number of Contexts"]
    #[inline(always)]
    pub const fn nctx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Contexts"]
    #[inline(always)]
    pub const fn set_nctx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn ctxer0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn set_ctxer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn ctxer1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn set_ctxer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn ctxer2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn set_ctxer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn ctxer3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Context Error"]
    #[inline(always)]
    pub const fn set_ctxer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn ctxie0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn set_ctxie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn ctxie1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn set_ctxie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn ctxie2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn set_ctxie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn ctxie3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Context Integrity Error"]
    #[inline(always)]
    pub const fn set_ctxie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Hardware Revision Level"]
    #[inline(always)]
    pub const fn hrl(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware Revision Level"]
    #[inline(always)]
    pub const fn set_hrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Restricted Register Access Mode"]
    #[inline(always)]
    pub const fn rram(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Restricted Register Access Mode"]
    #[inline(always)]
    pub const fn set_rram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Enable Mode"]
    #[inline(always)]
    pub const fn gem(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Global Enable Mode"]
    #[inline(always)]
    pub const fn set_gem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Key Blob Processing Enable"]
    #[inline(always)]
    pub const fn kbpe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Key Blob Processing Enable"]
    #[inline(always)]
    pub const fn set_kbpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Key Blob Processing Done"]
    #[inline(always)]
    pub const fn kbd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Key Blob Processing Done"]
    #[inline(always)]
    pub const fn set_kbd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
