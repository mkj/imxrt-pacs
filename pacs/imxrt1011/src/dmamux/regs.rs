#[doc = "Channel index Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Chcfg(pub u32);
impl Chcfg {
    #[doc = "DMA Channel Source (Slot Number)"]
    #[inline(always)]
    pub const fn source(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "DMA Channel Source (Slot Number)"]
    #[inline(always)]
    pub const fn set_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "DMA Channel Always Enable"]
    #[inline(always)]
    pub const fn a_on(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel Always Enable"]
    #[inline(always)]
    pub const fn set_a_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA Channel Trigger Enable"]
    #[inline(always)]
    pub const fn trig(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel Trigger Enable"]
    #[inline(always)]
    pub const fn set_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA Mux Channel Enable"]
    #[inline(always)]
    pub const fn enbl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Mux Channel Enable"]
    #[inline(always)]
    pub const fn set_enbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Chcfg {
    #[inline(always)]
    fn default() -> Chcfg {
        Chcfg(0)
    }
}
