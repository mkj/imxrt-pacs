#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Bfcrt01(pub u32);
impl Bfcrt01 {
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Pt1dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pt1dc::from_bits(val as u8)
    }
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Pt1dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Pt1cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pt1cc::from_bits(val as u8)
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Pt1cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Pt1bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pt1bc::from_bits(val as u8)
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Pt1bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Pt1ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pt1ac::from_bits(val as u8)
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Pt1ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Pt0dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pt0dc::from_bits(val as u8)
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Pt0dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Pt0cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pt0cc::from_bits(val as u8)
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Pt0cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Pt0bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pt0bc::from_bits(val as u8)
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Pt0bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Pt0ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pt0ac::from_bits(val as u8)
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Pt0ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt01 {
    #[inline(always)]
    fn default() -> Bfcrt01 {
        Bfcrt01(0)
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Bfcrt23(pub u32);
impl Bfcrt23 {
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Pt3dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pt3dc::from_bits(val as u8)
    }
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Pt3dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Pt3cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pt3cc::from_bits(val as u8)
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Pt3cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Pt3bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pt3bc::from_bits(val as u8)
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Pt3bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Pt3ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pt3ac::from_bits(val as u8)
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Pt3ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Pt2dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pt2dc::from_bits(val as u8)
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Pt2dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Pt2cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pt2cc::from_bits(val as u8)
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Pt2cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Pt2bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pt2bc::from_bits(val as u8)
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Pt2bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Pt2ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pt2ac::from_bits(val as u8)
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Pt2ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt23 {
    #[inline(always)]
    fn default() -> Bfcrt23 {
        Bfcrt23(0)
    }
}
