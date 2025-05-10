#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt010(pub u16);
impl Bfcrt010 {
    #[doc = "Product term 1, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt010pt1dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt010pt1dc::from_bits(val as u8)
    }
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt010pt1dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 1, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt010pt1cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt010pt1cc::from_bits(val as u8)
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt010pt1cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 1, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt010pt1bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt010pt1bc::from_bits(val as u8)
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt010pt1bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 1, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt010pt1ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt010pt1ac::from_bits(val as u8)
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt010pt1ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 0, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt010pt0dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt010pt0dc::from_bits(val as u8)
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt010pt0dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 0, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt010pt0cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt010pt0cc::from_bits(val as u8)
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt010pt0cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 0, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt010pt0bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt010pt0bc::from_bits(val as u8)
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt010pt0bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 0, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt010pt0ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt010pt0ac::from_bits(val as u8)
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt010pt0ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt010 {
    #[inline(always)]
    fn default() -> Bfcrt010 {
        Bfcrt010(0)
    }
}
impl core::fmt::Debug for Bfcrt010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt010")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt010 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt010 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}" , self . pt1_dc () , self . pt1_cc () , self . pt1_bc () , self . pt1_ac () , self . pt0_dc () , self . pt0_cc () , self . pt0_bc () , self . pt0_ac ())
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt011(pub u16);
impl Bfcrt011 {
    #[doc = "Product term 1, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt011pt1dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt011pt1dc::from_bits(val as u8)
    }
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt011pt1dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 1, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt011pt1cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt011pt1cc::from_bits(val as u8)
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt011pt1cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 1, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt011pt1bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt011pt1bc::from_bits(val as u8)
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt011pt1bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 1, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt011pt1ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt011pt1ac::from_bits(val as u8)
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt011pt1ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 0, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt011pt0dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt011pt0dc::from_bits(val as u8)
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt011pt0dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 0, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt011pt0cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt011pt0cc::from_bits(val as u8)
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt011pt0cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 0, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt011pt0bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt011pt0bc::from_bits(val as u8)
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt011pt0bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 0, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt011pt0ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt011pt0ac::from_bits(val as u8)
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt011pt0ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt011 {
    #[inline(always)]
    fn default() -> Bfcrt011 {
        Bfcrt011(0)
    }
}
impl core::fmt::Debug for Bfcrt011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt011")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt011 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt011 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}" , self . pt1_dc () , self . pt1_cc () , self . pt1_bc () , self . pt1_ac () , self . pt0_dc () , self . pt0_cc () , self . pt0_bc () , self . pt0_ac ())
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt012(pub u16);
impl Bfcrt012 {
    #[doc = "Product term 1, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt012pt1dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt012pt1dc::from_bits(val as u8)
    }
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt012pt1dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 1, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt012pt1cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt012pt1cc::from_bits(val as u8)
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt012pt1cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 1, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt012pt1bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt012pt1bc::from_bits(val as u8)
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt012pt1bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 1, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt012pt1ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt012pt1ac::from_bits(val as u8)
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt012pt1ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 0, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt012pt0dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt012pt0dc::from_bits(val as u8)
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt012pt0dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 0, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt012pt0cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt012pt0cc::from_bits(val as u8)
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt012pt0cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 0, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt012pt0bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt012pt0bc::from_bits(val as u8)
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt012pt0bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 0, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt012pt0ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt012pt0ac::from_bits(val as u8)
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt012pt0ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt012 {
    #[inline(always)]
    fn default() -> Bfcrt012 {
        Bfcrt012(0)
    }
}
impl core::fmt::Debug for Bfcrt012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt012")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt012 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt012 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}" , self . pt1_dc () , self . pt1_cc () , self . pt1_bc () , self . pt1_ac () , self . pt0_dc () , self . pt0_cc () , self . pt0_bc () , self . pt0_ac ())
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt013(pub u16);
impl Bfcrt013 {
    #[doc = "Product term 1, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt013pt1dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt013pt1dc::from_bits(val as u8)
    }
    #[doc = "Product term 1, D input configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt013pt1dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 1, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt013pt1cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt013pt1cc::from_bits(val as u8)
    }
    #[doc = "Product term 1, C input configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt013pt1cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 1, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt013pt1bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt013pt1bc::from_bits(val as u8)
    }
    #[doc = "Product term 1, B input configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt013pt1bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 1, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt013pt1ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt013pt1ac::from_bits(val as u8)
    }
    #[doc = "Product term 1, A input configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt013pt1ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 0, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt013pt0dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt013pt0dc::from_bits(val as u8)
    }
    #[doc = "Product term 0, D input configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt013pt0dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 0, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt013pt0cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt013pt0cc::from_bits(val as u8)
    }
    #[doc = "Product term 0, C input configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt013pt0cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 0, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt013pt0bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt013pt0bc::from_bits(val as u8)
    }
    #[doc = "Product term 0, B input configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt013pt0bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 0, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt013pt0ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt013pt0ac::from_bits(val as u8)
    }
    #[doc = "Product term 0, A input configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt013pt0ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt013 {
    #[inline(always)]
    fn default() -> Bfcrt013 {
        Bfcrt013(0)
    }
}
impl core::fmt::Debug for Bfcrt013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt013")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt013 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt013 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}" , self . pt1_dc () , self . pt1_cc () , self . pt1_bc () , self . pt1_ac () , self . pt0_dc () , self . pt0_cc () , self . pt0_bc () , self . pt0_ac ())
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt230(pub u16);
impl Bfcrt230 {
    #[doc = "Product term 3, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt230pt3dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt230pt3dc::from_bits(val as u8)
    }
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt230pt3dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 3, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt230pt3cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt230pt3cc::from_bits(val as u8)
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt230pt3cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 3, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt230pt3bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt230pt3bc::from_bits(val as u8)
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt230pt3bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 3, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt230pt3ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt230pt3ac::from_bits(val as u8)
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt230pt3ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 2, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt230pt2dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt230pt2dc::from_bits(val as u8)
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt230pt2dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 2, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt230pt2cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt230pt2cc::from_bits(val as u8)
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt230pt2cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 2, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt230pt2bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt230pt2bc::from_bits(val as u8)
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt230pt2bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 2, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt230pt2ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt230pt2ac::from_bits(val as u8)
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt230pt2ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt230 {
    #[inline(always)]
    fn default() -> Bfcrt230 {
        Bfcrt230(0)
    }
}
impl core::fmt::Debug for Bfcrt230 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt230")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt230 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt230 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}" , self . pt3_dc () , self . pt3_cc () , self . pt3_bc () , self . pt3_ac () , self . pt2_dc () , self . pt2_cc () , self . pt2_bc () , self . pt2_ac ())
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt231(pub u16);
impl Bfcrt231 {
    #[doc = "Product term 3, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt231pt3dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt231pt3dc::from_bits(val as u8)
    }
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt231pt3dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 3, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt231pt3cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt231pt3cc::from_bits(val as u8)
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt231pt3cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 3, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt231pt3bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt231pt3bc::from_bits(val as u8)
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt231pt3bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 3, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt231pt3ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt231pt3ac::from_bits(val as u8)
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt231pt3ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 2, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt231pt2dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt231pt2dc::from_bits(val as u8)
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt231pt2dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 2, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt231pt2cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt231pt2cc::from_bits(val as u8)
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt231pt2cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 2, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt231pt2bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt231pt2bc::from_bits(val as u8)
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt231pt2bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 2, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt231pt2ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt231pt2ac::from_bits(val as u8)
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt231pt2ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt231 {
    #[inline(always)]
    fn default() -> Bfcrt231 {
        Bfcrt231(0)
    }
}
impl core::fmt::Debug for Bfcrt231 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt231")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt231 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt231 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}" , self . pt3_dc () , self . pt3_cc () , self . pt3_bc () , self . pt3_ac () , self . pt2_dc () , self . pt2_cc () , self . pt2_bc () , self . pt2_ac ())
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt232(pub u16);
impl Bfcrt232 {
    #[doc = "Product term 3, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt232pt3dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt232pt3dc::from_bits(val as u8)
    }
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt232pt3dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 3, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt232pt3cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt232pt3cc::from_bits(val as u8)
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt232pt3cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 3, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt232pt3bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt232pt3bc::from_bits(val as u8)
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt232pt3bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 3, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt232pt3ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt232pt3ac::from_bits(val as u8)
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt232pt3ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 2, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt232pt2dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt232pt2dc::from_bits(val as u8)
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt232pt2dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 2, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt232pt2cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt232pt2cc::from_bits(val as u8)
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt232pt2cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 2, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt232pt2bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt232pt2bc::from_bits(val as u8)
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt232pt2bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 2, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt232pt2ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt232pt2ac::from_bits(val as u8)
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt232pt2ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt232 {
    #[inline(always)]
    fn default() -> Bfcrt232 {
        Bfcrt232(0)
    }
}
impl core::fmt::Debug for Bfcrt232 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt232")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt232 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt232 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}" , self . pt3_dc () , self . pt3_cc () , self . pt3_bc () , self . pt3_ac () , self . pt2_dc () , self . pt2_cc () , self . pt2_bc () , self . pt2_ac ())
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt233(pub u16);
impl Bfcrt233 {
    #[doc = "Product term 3, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt233pt3dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt233pt3dc::from_bits(val as u8)
    }
    #[doc = "Product term 3, D input configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt233pt3dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product term 3, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt233pt3cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt233pt3cc::from_bits(val as u8)
    }
    #[doc = "Product term 3, C input configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt233pt3cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product term 3, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt233pt3bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt233pt3bc::from_bits(val as u8)
    }
    #[doc = "Product term 3, B input configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt233pt3bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product term 3, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt233pt3ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt233pt3ac::from_bits(val as u8)
    }
    #[doc = "Product term 3, A input configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt233pt3ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product term 2, D input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt233pt2dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt233pt2dc::from_bits(val as u8)
    }
    #[doc = "Product term 2, D input configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt233pt2dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product term 2, C input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt233pt2cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt233pt2cc::from_bits(val as u8)
    }
    #[doc = "Product term 2, C input configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt233pt2cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product term 2, B input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt233pt2bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt233pt2bc::from_bits(val as u8)
    }
    #[doc = "Product term 2, B input configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt233pt2bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product term 2, A input configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt233pt2ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt233pt2ac::from_bits(val as u8)
    }
    #[doc = "Product term 2, A input configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt233pt2ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt233 {
    #[inline(always)]
    fn default() -> Bfcrt233 {
        Bfcrt233(0)
    }
}
impl core::fmt::Debug for Bfcrt233 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt233")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt233 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bfcrt233 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}" , self . pt3_dc () , self . pt3_cc () , self . pt3_bc () , self . pt3_ac () , self . pt2_dc () , self . pt2_cc () , self . pt2_bc () , self . pt2_ac ())
    }
}
