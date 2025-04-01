#[doc = "GPR3 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gpr3(pub u32);
impl Gpr3 {
    #[doc = "Set to enable LPSR mode."]
    #[inline(always)]
    pub const fn lpsr_mode_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set to enable LPSR mode."]
    #[inline(always)]
    pub const fn set_lpsr_mode_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DCDC captured status clear"]
    #[inline(always)]
    pub const fn dcdc_status_capt_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC captured status clear"]
    #[inline(always)]
    pub const fn set_dcdc_status_capt_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "POR_B pad control"]
    #[inline(always)]
    pub const fn por_pull_type(&self) -> super::vals::PorPullType {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PorPullType::from_bits(val as u8)
    }
    #[doc = "POR_B pad control"]
    #[inline(always)]
    pub const fn set_por_pull_type(&mut self, val: super::vals::PorPullType) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DCDC_IN low voltage detect."]
    #[inline(always)]
    pub const fn dcdc_in_low_vol(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC_IN low voltage detect."]
    #[inline(always)]
    pub const fn set_dcdc_in_low_vol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DCDC output over current alert"]
    #[inline(always)]
    pub const fn dcdc_over_cur(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC output over current alert"]
    #[inline(always)]
    pub const fn set_dcdc_over_cur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DCDC output over voltage alert"]
    #[inline(always)]
    pub const fn dcdc_over_vol(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC output over voltage alert"]
    #[inline(always)]
    pub const fn set_dcdc_over_vol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DCDC status OK"]
    #[inline(always)]
    pub const fn dcdc_sts_dc_ok(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC status OK"]
    #[inline(always)]
    pub const fn set_dcdc_sts_dc_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Gpr3 {
    #[inline(always)]
    fn default() -> Gpr3 {
        Gpr3(0)
    }
}
