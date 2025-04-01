#[doc = "GPIO interrupt configuration register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt configuration field for GPIO interrupt 16"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> super::vals::Icr {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::Icr::from_bits(val as u8)
    }
    #[doc = "Interrupt configuration field for GPIO interrupt 16"]
    #[inline(always)]
    pub const fn set_pin(&mut self, n: usize, val: super::vals::Icr) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
