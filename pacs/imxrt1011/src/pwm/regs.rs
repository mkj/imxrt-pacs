#[doc = "PWM Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dtsrcsel(pub u32);
impl Dtsrcsel {
    #[doc = "Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub const fn sm0sel45(&self) -> super::vals::Sm0sel45 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm0sel45(&mut self, val: super::vals::Sm0sel45) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub const fn sm0sel23(&self) -> super::vals::Sm0sel23 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm0sel23(&mut self, val: super::vals::Sm0sel23) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub const fn sm1sel45(&self) -> super::vals::Sm1sel45 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm1sel45(&mut self, val: super::vals::Sm1sel45) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub const fn sm1sel23(&self) -> super::vals::Sm1sel23 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm1sel23(&mut self, val: super::vals::Sm1sel23) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub const fn sm2sel45(&self) -> super::vals::Sm2sel45 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm2sel45(&mut self, val: super::vals::Sm2sel45) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub const fn sm2sel23(&self) -> super::vals::Sm2sel23 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sm2sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm2sel23(&mut self, val: super::vals::Sm2sel23) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub const fn sm3sel45(&self) -> super::vals::Sm3sel45 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Sm3sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm3sel45(&mut self, val: super::vals::Sm3sel45) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub const fn sm3sel23(&self) -> super::vals::Sm3sel23 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sm3sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm3sel23(&mut self, val: super::vals::Sm3sel23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Dtsrcsel {
    #[inline(always)]
    fn default() -> Dtsrcsel {
        Dtsrcsel(0)
    }
}
#[doc = "Fault Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fctrl0(pub u32);
impl Fctrl0 {
    #[doc = "Fault Interrupt Enables"]
    #[inline(always)]
    pub const fn fie(&self) -> super::vals::Fie {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fie::from_bits(val as u8)
    }
    #[doc = "Fault Interrupt Enables"]
    #[inline(always)]
    pub const fn set_fie(&mut self, val: super::vals::Fie) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Fault Safety Mode"]
    #[inline(always)]
    pub const fn fsafe(&self) -> super::vals::Fsafe {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Fsafe::from_bits(val as u8)
    }
    #[doc = "Fault Safety Mode"]
    #[inline(always)]
    pub const fn set_fsafe(&mut self, val: super::vals::Fsafe) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Automatic Fault Clearing"]
    #[inline(always)]
    pub const fn fauto(&self) -> super::vals::Fauto {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Fauto::from_bits(val as u8)
    }
    #[doc = "Automatic Fault Clearing"]
    #[inline(always)]
    pub const fn set_fauto(&mut self, val: super::vals::Fauto) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Fault Level"]
    #[inline(always)]
    pub const fn flvl(&self) -> super::vals::Flvl {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Flvl::from_bits(val as u8)
    }
    #[doc = "Fault Level"]
    #[inline(always)]
    pub const fn set_flvl(&mut self, val: super::vals::Flvl) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Fctrl0 {
    #[inline(always)]
    fn default() -> Fctrl0 {
        Fctrl0(0)
    }
}
#[doc = "Fault Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fctrl20(pub u32);
impl Fctrl20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub const fn nocomb(&self) -> super::vals::Nocomb {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Nocomb::from_bits(val as u8)
    }
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub const fn set_nocomb(&mut self, val: super::vals::Nocomb) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Fctrl20 {
    #[inline(always)]
    fn default() -> Fctrl20 {
        Fctrl20(0)
    }
}
#[doc = "Fault Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ffilt0(pub u32);
impl Ffilt0 {
    #[doc = "Fault Filter Period"]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fault Filter Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Fault Filter Count"]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Fault Filter Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub const fn gstr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub const fn set_gstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ffilt0 {
    #[inline(always)]
    fn default() -> Ffilt0 {
        Ffilt0(0)
    }
}
#[doc = "Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fsts0(pub u32);
impl Fsts0 {
    #[doc = "Fault Flags"]
    #[inline(always)]
    pub const fn fflag(&self) -> super::vals::Fflag {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fflag::from_bits(val as u8)
    }
    #[doc = "Fault Flags"]
    #[inline(always)]
    pub const fn set_fflag(&mut self, val: super::vals::Fflag) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Full Cycle"]
    #[inline(always)]
    pub const fn ffull(&self) -> super::vals::Ffull {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ffull::from_bits(val as u8)
    }
    #[doc = "Full Cycle"]
    #[inline(always)]
    pub const fn set_ffull(&mut self, val: super::vals::Ffull) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Filtered Fault Pins"]
    #[inline(always)]
    pub const fn ffpin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filtered Fault Pins"]
    #[inline(always)]
    pub const fn set_ffpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Half Cycle Fault Recovery"]
    #[inline(always)]
    pub const fn fhalf(&self) -> super::vals::Fhalf {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Fhalf::from_bits(val as u8)
    }
    #[doc = "Half Cycle Fault Recovery"]
    #[inline(always)]
    pub const fn set_fhalf(&mut self, val: super::vals::Fhalf) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Fsts0 {
    #[inline(always)]
    fn default() -> Fsts0 {
        Fsts0(0)
    }
}
#[doc = "Fault Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ftst0(pub u32);
impl Ftst0 {
    #[doc = "Fault Test"]
    #[inline(always)]
    pub const fn ftest(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Test"]
    #[inline(always)]
    pub const fn set_ftest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ftst0 {
    #[inline(always)]
    fn default() -> Ftst0 {
        Ftst0(0)
    }
}
#[doc = "Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mask(pub u32);
impl Mask {
    #[doc = "PWM_X Masks"]
    #[inline(always)]
    pub const fn maskx(&self) -> super::vals::Maskx {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Maskx::from_bits(val as u8)
    }
    #[doc = "PWM_X Masks"]
    #[inline(always)]
    pub const fn set_maskx(&mut self, val: super::vals::Maskx) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Masks"]
    #[inline(always)]
    pub const fn maskb(&self) -> super::vals::Maskb {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Maskb::from_bits(val as u8)
    }
    #[doc = "PWM_B Masks"]
    #[inline(always)]
    pub const fn set_maskb(&mut self, val: super::vals::Maskb) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Masks"]
    #[inline(always)]
    pub const fn maska(&self) -> super::vals::Maska {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Maska::from_bits(val as u8)
    }
    #[doc = "PWM_A Masks"]
    #[inline(always)]
    pub const fn set_maska(&mut self, val: super::vals::Maska) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Update Mask Bits Immediately"]
    #[inline(always)]
    pub const fn update_mask(&self) -> super::vals::UpdateMask {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::UpdateMask::from_bits(val as u8)
    }
    #[doc = "Update Mask Bits Immediately"]
    #[inline(always)]
    pub const fn set_update_mask(&mut self, val: super::vals::UpdateMask) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
#[doc = "Master Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc = "Load Okay"]
    #[inline(always)]
    pub const fn ldok(&self) -> super::vals::Ldok {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay"]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: super::vals::Ldok) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Clear Load Okay"]
    #[inline(always)]
    pub const fn cldok(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Load Okay"]
    #[inline(always)]
    pub const fn set_cldok(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Run"]
    #[inline(always)]
    pub const fn run(&self) -> super::vals::Run {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Run::from_bits(val as u8)
    }
    #[doc = "Run"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: super::vals::Run) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Current Polarity"]
    #[inline(always)]
    pub const fn ipol(&self) -> super::vals::Ipol {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Ipol::from_bits(val as u8)
    }
    #[doc = "Current Polarity"]
    #[inline(always)]
    pub const fn set_ipol(&mut self, val: super::vals::Ipol) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
#[doc = "Master Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mctrl2(pub u32);
impl Mctrl2 {
    #[doc = "Monitor PLL State"]
    #[inline(always)]
    pub const fn monpll(&self) -> super::vals::Monpll {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Monpll::from_bits(val as u8)
    }
    #[doc = "Monitor PLL State"]
    #[inline(always)]
    pub const fn set_monpll(&mut self, val: super::vals::Monpll) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Mctrl2 {
    #[inline(always)]
    fn default() -> Mctrl2 {
        Mctrl2(0)
    }
}
#[doc = "Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Outen(pub u32);
impl Outen {
    #[doc = "PWM_X Output Enables"]
    #[inline(always)]
    pub const fn pwmx_en(&self) -> super::vals::PwmxEn {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PwmxEn::from_bits(val as u8)
    }
    #[doc = "PWM_X Output Enables"]
    #[inline(always)]
    pub const fn set_pwmx_en(&mut self, val: super::vals::PwmxEn) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Output Enables"]
    #[inline(always)]
    pub const fn pwmb_en(&self) -> super::vals::PwmbEn {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::PwmbEn::from_bits(val as u8)
    }
    #[doc = "PWM_B Output Enables"]
    #[inline(always)]
    pub const fn set_pwmb_en(&mut self, val: super::vals::PwmbEn) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Output Enables"]
    #[inline(always)]
    pub const fn pwma_en(&self) -> super::vals::PwmaEn {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::PwmaEn::from_bits(val as u8)
    }
    #[doc = "PWM_A Output Enables"]
    #[inline(always)]
    pub const fn set_pwma_en(&mut self, val: super::vals::PwmaEn) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for Outen {
    #[inline(always)]
    fn default() -> Outen {
        Outen(0)
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcaptcompa(pub u32);
impl Smcaptcompa {
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Smcaptcompa {
    #[inline(always)]
    fn default() -> Smcaptcompa {
        Smcaptcompa(0)
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcaptcompb(pub u32);
impl Smcaptcompb {
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Smcaptcompb {
    #[inline(always)]
    fn default() -> Smcaptcompb {
        Smcaptcompb(0)
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcaptcompx(pub u32);
impl Smcaptcompx {
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Smcaptcompx {
    #[inline(always)]
    fn default() -> Smcaptcompx {
        Smcaptcompx(0)
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcaptctrla(pub u32);
impl Smcaptctrla {
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn oneshota(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Edga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Edga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Edga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Edga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Edga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn inp_sela(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
}
impl Default for Smcaptctrla {
    #[inline(always)]
    fn default() -> Smcaptctrla {
        Smcaptctrla(0)
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcaptctrlb(pub u32);
impl Smcaptctrlb {
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn oneshotb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Edgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Edgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Edgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Edgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Edgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn inp_selb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
}
impl Default for Smcaptctrlb {
    #[inline(always)]
    fn default() -> Smcaptctrlb {
        Smcaptctrlb(0)
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcaptctrlx(pub u32);
impl Smcaptctrlx {
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn oneshotx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Edgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Edgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Edgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Edgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Edgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn inp_selx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
}
impl Default for Smcaptctrlx {
    #[inline(always)]
    fn default() -> Smcaptctrlx {
        Smcaptctrlx(0)
    }
}
#[doc = "Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcnt(pub u32);
impl Smcnt {
    #[doc = "Counter Register Bits"]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcnt {
    #[inline(always)]
    fn default() -> Smcnt {
        Smcnt(0)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smctrl(pub u32);
impl Smctrl {
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PWMX Double Switching Enable"]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWMX Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn ldmod(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWMA and PWMB"]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWMA and PWMB"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Prsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Prsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn compmode(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Ldfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Ldfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Ldfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Smctrl {
    #[inline(always)]
    fn default() -> Smctrl {
        Smctrl(0)
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smctrl2(pub u32);
impl Smctrl2 {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn reload_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::ForceSel::from_bits(val as u8)
    }
    #[doc = "This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FRCEN"]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FRCEN"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn indep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "WAIT Enable"]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "WAIT Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Smctrl2 {
    #[inline(always)]
    fn default() -> Smctrl2 {
        Smctrl2(0)
    }
}
#[doc = "Capture Value 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval0(pub u32);
impl Smcval0 {
    #[doc = "CAPTVAL0"]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAPTVAL0"]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcval0 {
    #[inline(always)]
    fn default() -> Smcval0 {
        Smcval0(0)
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval0cyc(pub u32);
impl Smcval0cyc {
    #[doc = "CVAL0CYC"]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CVAL0CYC"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Smcval0cyc {
    #[inline(always)]
    fn default() -> Smcval0cyc {
        Smcval0cyc(0)
    }
}
#[doc = "Capture Value 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval1(pub u32);
impl Smcval1 {
    #[doc = "CAPTVAL1"]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAPTVAL1"]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcval1 {
    #[inline(always)]
    fn default() -> Smcval1 {
        Smcval1(0)
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval1cyc(pub u32);
impl Smcval1cyc {
    #[doc = "CVAL1CYC"]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CVAL1CYC"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Smcval1cyc {
    #[inline(always)]
    fn default() -> Smcval1cyc {
        Smcval1cyc(0)
    }
}
#[doc = "Capture Value 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval2(pub u32);
impl Smcval2 {
    #[doc = "CAPTVAL2"]
    #[inline(always)]
    pub const fn captval2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAPTVAL2"]
    #[inline(always)]
    pub const fn set_captval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcval2 {
    #[inline(always)]
    fn default() -> Smcval2 {
        Smcval2(0)
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval2cyc(pub u32);
impl Smcval2cyc {
    #[doc = "CVAL2CYC"]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CVAL2CYC"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Smcval2cyc {
    #[inline(always)]
    fn default() -> Smcval2cyc {
        Smcval2cyc(0)
    }
}
#[doc = "Capture Value 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval3(pub u32);
impl Smcval3 {
    #[doc = "CAPTVAL3"]
    #[inline(always)]
    pub const fn captval3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAPTVAL3"]
    #[inline(always)]
    pub const fn set_captval3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcval3 {
    #[inline(always)]
    fn default() -> Smcval3 {
        Smcval3(0)
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval3cyc(pub u32);
impl Smcval3cyc {
    #[doc = "CVAL3CYC"]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CVAL3CYC"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Smcval3cyc {
    #[inline(always)]
    fn default() -> Smcval3cyc {
        Smcval3cyc(0)
    }
}
#[doc = "Capture Value 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval4(pub u32);
impl Smcval4 {
    #[doc = "CAPTVAL4"]
    #[inline(always)]
    pub const fn captval4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAPTVAL4"]
    #[inline(always)]
    pub const fn set_captval4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcval4 {
    #[inline(always)]
    fn default() -> Smcval4 {
        Smcval4(0)
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval4cyc(pub u32);
impl Smcval4cyc {
    #[doc = "CVAL4CYC"]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CVAL4CYC"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Smcval4cyc {
    #[inline(always)]
    fn default() -> Smcval4cyc {
        Smcval4cyc(0)
    }
}
#[doc = "Capture Value 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval5(pub u32);
impl Smcval5 {
    #[doc = "CAPTVAL5"]
    #[inline(always)]
    pub const fn captval5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CAPTVAL5"]
    #[inline(always)]
    pub const fn set_captval5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smcval5 {
    #[inline(always)]
    fn default() -> Smcval5 {
        Smcval5(0)
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smcval5cyc(pub u32);
impl Smcval5cyc {
    #[doc = "CVAL5CYC"]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CVAL5CYC"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Smcval5cyc {
    #[inline(always)]
    fn default() -> Smcval5cyc {
        Smcval5cyc(0)
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smdismap0(pub u32);
impl Smdismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Smdismap0 {
    #[inline(always)]
    fn default() -> Smdismap0 {
        Smdismap0(0)
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smdmaen(pub u32);
impl Smdmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Captde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Captde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Captde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn fand(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Smdmaen {
    #[inline(always)]
    fn default() -> Smdmaen {
        Smdmaen(0)
    }
}
#[doc = "Deadtime Count Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smdtcnt0(pub u32);
impl Smdtcnt0 {
    #[doc = "DTCNT0"]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DTCNT0"]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smdtcnt0 {
    #[inline(always)]
    fn default() -> Smdtcnt0 {
        Smdtcnt0(0)
    }
}
#[doc = "Deadtime Count Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smdtcnt1(pub u32);
impl Smdtcnt1 {
    #[doc = "DTCNT1"]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DTCNT1"]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smdtcnt1 {
    #[inline(always)]
    fn default() -> Smdtcnt1 {
        Smdtcnt1(0)
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smfracval1(pub u32);
impl Smfracval1 {
    #[doc = "Fractional Value 1 Register"]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1 Register"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval1 {
    #[inline(always)]
    fn default() -> Smfracval1 {
        Smfracval1(0)
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smfracval2(pub u32);
impl Smfracval2 {
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval2 {
    #[inline(always)]
    fn default() -> Smfracval2 {
        Smfracval2(0)
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smfracval3(pub u32);
impl Smfracval3 {
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval3 {
    #[inline(always)]
    fn default() -> Smfracval3 {
        Smfracval3(0)
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smfracval4(pub u32);
impl Smfracval4 {
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval4 {
    #[inline(always)]
    fn default() -> Smfracval4 {
        Smfracval4(0)
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smfracval5(pub u32);
impl Smfracval5 {
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval5 {
    #[inline(always)]
    fn default() -> Smfracval5 {
        Smfracval5(0)
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smfrctrl(pub u32);
impl Smfrctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Fractional Delay Circuit Power Up"]
    #[inline(always)]
    pub const fn frac_pu(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Delay Circuit Power Up"]
    #[inline(always)]
    pub const fn set_frac_pu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Smfrctrl {
    #[inline(always)]
    fn default() -> Smfrctrl {
        Smfrctrl(0)
    }
}
#[doc = "Initial Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sminit(pub u32);
impl Sminit {
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits"]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Sminit {
    #[inline(always)]
    fn default() -> Sminit {
        Sminit(0)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sminten(pub u32);
impl Sminten {
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Cmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Cmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Sminten {
    #[inline(always)]
    fn default() -> Sminten {
        Sminten(0)
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smoctrl(pub u32);
impl Smoctrl {
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Pwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Pwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Pwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Pwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Pwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Pwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Smoctrl {
    #[inline(always)]
    fn default() -> Smoctrl {
        Smoctrl(0)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smsts(pub u32);
impl Smsts {
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Cmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Cmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Smsts {
    #[inline(always)]
    fn default() -> Smsts {
        Smsts(0)
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smtctrl(pub u32);
impl Smtctrl {
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::OutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::OutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::OutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Trigger frequency"]
    #[inline(always)]
    pub const fn trgfrq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn pwbot1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn pwaot0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Smtctrl {
    #[inline(always)]
    fn default() -> Smtctrl {
        Smtctrl(0)
    }
}
#[doc = "Value Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smval0(pub u32);
impl Smval0 {
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smval0 {
    #[inline(always)]
    fn default() -> Smval0 {
        Smval0(0)
    }
}
#[doc = "Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smval1(pub u32);
impl Smval1 {
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smval1 {
    #[inline(always)]
    fn default() -> Smval1 {
        Smval1(0)
    }
}
#[doc = "Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smval2(pub u32);
impl Smval2 {
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smval2 {
    #[inline(always)]
    fn default() -> Smval2 {
        Smval2(0)
    }
}
#[doc = "Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smval3(pub u32);
impl Smval3 {
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smval3 {
    #[inline(always)]
    fn default() -> Smval3 {
        Smval3(0)
    }
}
#[doc = "Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smval4(pub u32);
impl Smval4 {
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smval4 {
    #[inline(always)]
    fn default() -> Smval4 {
        Smval4(0)
    }
}
#[doc = "Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Smval5(pub u32);
impl Smval5 {
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Smval5 {
    #[inline(always)]
    fn default() -> Smval5 {
        Smval5(0)
    }
}
#[doc = "Software Controlled Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Swcout(pub u32);
impl Swcout {
    #[doc = "Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn sm0out45(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn sm0out23(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn sm1out45(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn sm1out23(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn sm2out45(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn sm2out23(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn sm3out45(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm3out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn sm3out23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm3out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Swcout {
    #[inline(always)]
    fn default() -> Swcout {
        Swcout(0)
    }
}
