#[doc = "GPIO interrupt configuration register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt configuration field for GPIO interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> super::vals::Icr {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::Icr::from_bits(val as u8)
    }
    #[doc = "Interrupt configuration field for GPIO interrupt 0"]
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
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("pin[0]", &self.pin(0usize))
            .field("pin[1]", &self.pin(1usize))
            .field("pin[2]", &self.pin(2usize))
            .field("pin[3]", &self.pin(3usize))
            .field("pin[4]", &self.pin(4usize))
            .field("pin[5]", &self.pin(5usize))
            .field("pin[6]", &self.pin(6usize))
            .field("pin[7]", &self.pin(7usize))
            .field("pin[8]", &self.pin(8usize))
            .field("pin[9]", &self.pin(9usize))
            .field("pin[10]", &self.pin(10usize))
            .field("pin[11]", &self.pin(11usize))
            .field("pin[12]", &self.pin(12usize))
            .field("pin[13]", &self.pin(13usize))
            .field("pin[14]", &self.pin(14usize))
            .field("pin[15]", &self.pin(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Icr {{ pin[0]: {:?}, pin[1]: {:?}, pin[2]: {:?}, pin[3]: {:?}, pin[4]: {:?}, pin[5]: {:?}, pin[6]: {:?}, pin[7]: {:?}, pin[8]: {:?}, pin[9]: {:?}, pin[10]: {:?}, pin[11]: {:?}, pin[12]: {:?}, pin[13]: {:?}, pin[14]: {:?}, pin[15]: {:?} }}" , self . pin (0usize) , self . pin (1usize) , self . pin (2usize) , self . pin (3usize) , self . pin (4usize) , self . pin (5usize) , self . pin (6usize) , self . pin (7usize) , self . pin (8usize) , self . pin (9usize) , self . pin (10usize) , self . pin (11usize) , self . pin (12usize) , self . pin (13usize) , self . pin (14usize) , self . pin (15usize))
    }
}
