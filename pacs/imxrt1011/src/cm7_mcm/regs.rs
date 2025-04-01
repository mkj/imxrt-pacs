#[doc = "Interrupt Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Iscr(pub u32);
impl Iscr {
    #[doc = "Write Abort on Slave"]
    #[inline(always)]
    pub const fn wabs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write Abort on Slave"]
    #[inline(always)]
    pub const fn set_wabs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write Abort on Slave Overrun"]
    #[inline(always)]
    pub const fn wabso(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write Abort on Slave Overrun"]
    #[inline(always)]
    pub const fn set_wabso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FPU Invalid Operation interrupt Status"]
    #[inline(always)]
    pub const fn fioc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Invalid Operation interrupt Status"]
    #[inline(always)]
    pub const fn set_fioc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FPU Divide-by-Zero Interrupt Status"]
    #[inline(always)]
    pub const fn fdzc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Divide-by-Zero Interrupt Status"]
    #[inline(always)]
    pub const fn set_fdzc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FPU Overflow interrupt status"]
    #[inline(always)]
    pub const fn fofc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Overflow interrupt status"]
    #[inline(always)]
    pub const fn set_fofc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FPU Underflow Interrupt Status"]
    #[inline(always)]
    pub const fn fufc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Underflow Interrupt Status"]
    #[inline(always)]
    pub const fn set_fufc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FPU Inexact Interrupt Status"]
    #[inline(always)]
    pub const fn fixc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Inexact Interrupt Status"]
    #[inline(always)]
    pub const fn set_fixc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FPU Input Denormal Interrupt Status"]
    #[inline(always)]
    pub const fn fidc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Input Denormal Interrupt Status"]
    #[inline(always)]
    pub const fn set_fidc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TCM Write Abort Interrupt enable"]
    #[inline(always)]
    pub const fn wabe(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TCM Write Abort Interrupt enable"]
    #[inline(always)]
    pub const fn set_wabe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "FPU Invalid Operation Interrupt Enable"]
    #[inline(always)]
    pub const fn fioce(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Invalid Operation Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fioce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "FPU Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub const fn fdzce(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fdzce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "FPU Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn fofce(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fofce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "FPU Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn fufce(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fufce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "FPU Inexact Interrupt Enable"]
    #[inline(always)]
    pub const fn fixce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Inexact Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fixce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "FPU Input Denormal Interrupt Enable"]
    #[inline(always)]
    pub const fn fidce(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FPU Input Denormal Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fidce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Iscr {
    #[inline(always)]
    fn default() -> Iscr {
        Iscr(0)
    }
}
