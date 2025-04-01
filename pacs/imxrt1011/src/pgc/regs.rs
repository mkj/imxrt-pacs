#[doc = "PGC CPU Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CpuCtrl(pub u32);
impl CpuCtrl {
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CpuCtrl {
    #[inline(always)]
    fn default() -> CpuCtrl {
        CpuCtrl(0)
    }
}
#[doc = "PGC CPU Pull Down Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CpuPdnscr(pub u32);
impl CpuPdnscr {
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub const fn iso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub const fn iso2sw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub const fn set_iso2sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for CpuPdnscr {
    #[inline(always)]
    fn default() -> CpuPdnscr {
        CpuPdnscr(0)
    }
}
#[doc = "PGC CPU Power Up Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CpuPupscr(pub u32);
impl CpuPupscr {
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub const fn sw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub const fn sw2iso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub const fn set_sw2iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for CpuPupscr {
    #[inline(always)]
    fn default() -> CpuPupscr {
        CpuPupscr(0)
    }
}
#[doc = "PGC CPU Power Gating Controller Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CpuSr(pub u32);
impl CpuSr {
    #[doc = "Power status"]
    #[inline(always)]
    pub const fn psr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power status"]
    #[inline(always)]
    pub const fn set_psr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CpuSr {
    #[inline(always)]
    fn default() -> CpuSr {
        CpuSr(0)
    }
}
#[doc = "PGC Mega Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MegaCtrl(pub u32);
impl MegaCtrl {
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub const fn pcr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MegaCtrl {
    #[inline(always)]
    fn default() -> MegaCtrl {
        MegaCtrl(0)
    }
}
#[doc = "PGC Mega Pull Down Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MegaPdnscr(pub u32);
impl MegaPdnscr {
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub const fn iso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub const fn iso2sw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub const fn set_iso2sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for MegaPdnscr {
    #[inline(always)]
    fn default() -> MegaPdnscr {
        MegaPdnscr(0)
    }
}
#[doc = "PGC Mega Power Up Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MegaPupscr(pub u32);
impl MegaPupscr {
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub const fn sw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub const fn sw2iso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub const fn set_sw2iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for MegaPupscr {
    #[inline(always)]
    fn default() -> MegaPupscr {
        MegaPupscr(0)
    }
}
#[doc = "PGC Mega Power Gating Controller Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MegaSr(pub u32);
impl MegaSr {
    #[doc = "Power status"]
    #[inline(always)]
    pub const fn psr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power status"]
    #[inline(always)]
    pub const fn set_psr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MegaSr {
    #[inline(always)]
    fn default() -> MegaSr {
        MegaSr(0)
    }
}
