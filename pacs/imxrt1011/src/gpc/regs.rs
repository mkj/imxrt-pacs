#[doc = "GPC Interface control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cntr(pub u32);
impl Cntr {
    #[doc = "MEGA domain (FlexRAM PDRAM1) power down request"]
    #[inline(always)]
    pub const fn mega_pdn_req(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MEGA domain (FlexRAM PDRAM1) power down request"]
    #[inline(always)]
    pub const fn set_mega_pdn_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "MEGA domain (FlexRAM PDRAM1) power up request"]
    #[inline(always)]
    pub const fn mega_pup_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MEGA domain (FlexRAM PDRAM1) power up request"]
    #[inline(always)]
    pub const fn set_mega_pup_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    pub const fn pdram0_pge(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    pub const fn set_pdram0_pge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Cntr {
    #[inline(always)]
    fn default() -> Cntr {
        Cntr(0)
    }
}
