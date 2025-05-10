#[doc = "MPU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enables the MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the MPU."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or with the MPU disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn hfnmiena(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controls whether handlers executing with priority less than 0 access memory with the MPU enabled or with the MPU disabled."]
    #[inline(always)]
    pub const fn set_hfnmiena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn privdefena(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_privdefena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enable", &self.enable())
            .field("hfnmiena", &self.hfnmiena())
            .field("privdefena", &self.privdefena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enable: {=bool:?}, hfnmiena: {=bool:?}, privdefena: {=bool:?} }}",
            self.enable(),
            self.hfnmiena(),
            self.privdefena()
        )
    }
}
#[doc = "MPU Region Attribute and Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rasr(pub u32);
impl Rasr {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn srd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[inline(always)]
    pub const fn set_srd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn c(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[must_use]
    #[inline(always)]
    pub const fn tex(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[inline(always)]
    pub const fn set_tex(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RasrAp {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::RasrAp::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RasrAp) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Rasr {
    #[inline(always)]
    fn default() -> Rasr {
        Rasr(0)
    }
}
impl core::fmt::Debug for Rasr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rasr")
            .field("enable", &self.enable())
            .field("size", &self.size())
            .field("srd", &self.srd())
            .field("b", &self.b())
            .field("c", &self.c())
            .field("s", &self.s())
            .field("tex", &self.tex())
            .field("ap", &self.ap())
            .field("xn", &self.xn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rasr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rasr {{ enable: {=bool:?}, size: {=u8:?}, srd: {=u8:?}, b: {=bool:?}, c: {=bool:?}, s: {=bool:?}, tex: {=u8:?}, ap: {:?}, xn: {=bool:?} }}" , self . enable () , self . size () , self . srd () , self . b () , self . c () , self . s () , self . tex () , self . ap () , self . xn ())
    }
}
#[doc = "MPU Region Attribute and Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RasrA1(pub u32);
impl RasrA1 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn srd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[inline(always)]
    pub const fn set_srd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn c(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[must_use]
    #[inline(always)]
    pub const fn tex(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[inline(always)]
    pub const fn set_tex(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RasrA1ap {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::RasrA1ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RasrA1ap) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RasrA1 {
    #[inline(always)]
    fn default() -> RasrA1 {
        RasrA1(0)
    }
}
impl core::fmt::Debug for RasrA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RasrA1")
            .field("enable", &self.enable())
            .field("size", &self.size())
            .field("srd", &self.srd())
            .field("b", &self.b())
            .field("c", &self.c())
            .field("s", &self.s())
            .field("tex", &self.tex())
            .field("ap", &self.ap())
            .field("xn", &self.xn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RasrA1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RasrA1 {{ enable: {=bool:?}, size: {=u8:?}, srd: {=u8:?}, b: {=bool:?}, c: {=bool:?}, s: {=bool:?}, tex: {=u8:?}, ap: {:?}, xn: {=bool:?} }}" , self . enable () , self . size () , self . srd () , self . b () , self . c () , self . s () , self . tex () , self . ap () , self . xn ())
    }
}
#[doc = "MPU Region Attribute and Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RasrA2(pub u32);
impl RasrA2 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn srd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[inline(always)]
    pub const fn set_srd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn c(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[must_use]
    #[inline(always)]
    pub const fn tex(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[inline(always)]
    pub const fn set_tex(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RasrA2ap {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::RasrA2ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RasrA2ap) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RasrA2 {
    #[inline(always)]
    fn default() -> RasrA2 {
        RasrA2(0)
    }
}
impl core::fmt::Debug for RasrA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RasrA2")
            .field("enable", &self.enable())
            .field("size", &self.size())
            .field("srd", &self.srd())
            .field("b", &self.b())
            .field("c", &self.c())
            .field("s", &self.s())
            .field("tex", &self.tex())
            .field("ap", &self.ap())
            .field("xn", &self.xn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RasrA2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RasrA2 {{ enable: {=bool:?}, size: {=u8:?}, srd: {=u8:?}, b: {=bool:?}, c: {=bool:?}, s: {=bool:?}, tex: {=u8:?}, ap: {:?}, xn: {=bool:?} }}" , self . enable () , self . size () , self . srd () , self . b () , self . c () , self . s () , self . tex () , self . ap () , self . xn ())
    }
}
#[doc = "MPU Region Attribute and Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RasrA3(pub u32);
impl RasrA3 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the region size. The region size, in bytes, is 2**(SIZE+1). SIZE field values less than 4 are reserved, because the smallest supported region size is 32 bytes."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn srd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled. SRD\\[0-7\\]=0 - subregion enabled. SRD\\[0-7\\]=1 - subregion disabled."]
    #[inline(always)]
    pub const fn set_srd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute B. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn c(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Memory type attribute C. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region."]
    #[inline(always)]
    pub const fn set_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[must_use]
    #[inline(always)]
    pub const fn tex(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Memory type attribute TEX. The TEX\\[2:0\\], C, and B bits together indicate the memory type of the region"]
    #[inline(always)]
    pub const fn set_tex(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RasrA3ap {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::RasrA3ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RasrA3ap) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RasrA3 {
    #[inline(always)]
    fn default() -> RasrA3 {
        RasrA3(0)
    }
}
impl core::fmt::Debug for RasrA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RasrA3")
            .field("enable", &self.enable())
            .field("size", &self.size())
            .field("srd", &self.srd())
            .field("b", &self.b())
            .field("c", &self.c())
            .field("s", &self.s())
            .field("tex", &self.tex())
            .field("ap", &self.ap())
            .field("xn", &self.xn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RasrA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RasrA3 {{ enable: {=bool:?}, size: {=u8:?}, srd: {=u8:?}, b: {=bool:?}, c: {=bool:?}, s: {=bool:?}, tex: {=u8:?}, ap: {:?}, xn: {=bool:?} }}" , self . enable () , self . size () , self . srd () , self . b () , self . c () , self . s () , self . tex () , self . ap () , self . xn ())
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbar(pub u32);
impl Rbar {
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Base address of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address of the region."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rbar {
    #[inline(always)]
    fn default() -> Rbar {
        Rbar(0)
    }
}
impl core::fmt::Debug for Rbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbar")
            .field("region", &self.region())
            .field("valid", &self.valid())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rbar {{ region: {=u8:?}, valid: {=bool:?}, addr: {=u32:?} }}",
            self.region(),
            self.valid(),
            self.addr()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA1(pub u32);
impl RbarA1 {
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Base address of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address of the region."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA1 {
    #[inline(always)]
    fn default() -> RbarA1 {
        RbarA1(0)
    }
}
impl core::fmt::Debug for RbarA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbarA1")
            .field("region", &self.region())
            .field("valid", &self.valid())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbarA1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RbarA1 {{ region: {=u8:?}, valid: {=bool:?}, addr: {=u32:?} }}",
            self.region(),
            self.valid(),
            self.addr()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA2(pub u32);
impl RbarA2 {
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Base address of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address of the region."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA2 {
    #[inline(always)]
    fn default() -> RbarA2 {
        RbarA2(0)
    }
}
impl core::fmt::Debug for RbarA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbarA2")
            .field("region", &self.region())
            .field("valid", &self.valid())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbarA2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RbarA2 {{ region: {=u8:?}, valid: {=bool:?}, addr: {=u32:?} }}",
            self.region(),
            self.valid(),
            self.addr()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA3(pub u32);
impl RbarA3 {
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "On writes, can specify the number of the region to update. On reads, returns bits\\[3:0\\] of MPU_RNR."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Base address of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address of the region."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA3 {
    #[inline(always)]
    fn default() -> RbarA3 {
        RbarA3(0)
    }
}
impl core::fmt::Debug for RbarA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbarA3")
            .field("region", &self.region())
            .field("valid", &self.valid())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbarA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RbarA3 {{ region: {=u8:?}, valid: {=bool:?}, addr: {=u32:?} }}",
            self.region(),
            self.valid(),
            self.addr()
        )
    }
}
#[doc = "MPU Region Number Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rnr(pub u32);
impl Rnr {
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rnr {
    #[inline(always)]
    fn default() -> Rnr {
        Rnr(0)
    }
}
impl core::fmt::Debug for Rnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rnr")
            .field("region", &self.region())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rnr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rnr {{ region: {=u8:?} }}", self.region())
    }
}
#[doc = "The MPU Type Register indicates how many regions the MPU support. Software can use it to determine if the processor implements an MPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type(pub u32);
impl Type {
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv7-M only supports a unified MPU"]
    #[must_use]
    #[inline(always)]
    pub const fn separate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv7-M only supports a unified MPU"]
    #[inline(always)]
    pub const fn set_separate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn dregion(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[inline(always)]
    pub const fn set_dregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Instruction region. RAZ. Armv7-M only supports a unified MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn iregion(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Instruction region. RAZ. Armv7-M only supports a unified MPU."]
    #[inline(always)]
    pub const fn set_iregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Type {
    #[inline(always)]
    fn default() -> Type {
        Type(0)
    }
}
impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Type")
            .field("separate", &self.separate())
            .field("dregion", &self.dregion())
            .field("iregion", &self.iregion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Type {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Type {{ separate: {=bool:?}, dregion: {=u8:?}, iregion: {=u8:?} }}",
            self.separate(),
            self.dregion(),
            self.iregion()
        )
    }
}
