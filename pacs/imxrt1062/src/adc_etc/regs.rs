#[doc = "ADC_ETC Global Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "TRIG enable register."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_enable(&self) -> super::vals::TrigEnable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::TrigEnable::from_bits(val as u8)
    }
    #[doc = "TRIG enable register."]
    #[inline(always)]
    pub const fn set_trig_enable(&mut self, val: super::vals::TrigEnable) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "TSC0 TRIG enable register."]
    #[must_use]
    #[inline(always)]
    pub const fn ext0_trig_enable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TSC0 TRIG enable register."]
    #[inline(always)]
    pub const fn set_ext0_trig_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "External TSC0 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn ext0_trig_priority(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "External TSC0 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[inline(always)]
    pub const fn set_ext0_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "TSC1 TRIG enable register."]
    #[must_use]
    #[inline(always)]
    pub const fn ext1_trig_enable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TSC1 TRIG enable register."]
    #[inline(always)]
    pub const fn set_ext1_trig_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External TSC1 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn ext1_trig_priority(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "External TSC1 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[inline(always)]
    pub const fn set_ext1_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Pre-divider for trig delay and interval"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_divider(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pre-divider for trig delay and interval"]
    #[inline(always)]
    pub const fn set_pre_divider(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Select the trigger type of the DMA_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_mode_sel(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Select the trigger type of the DMA_REQ."]
    #[inline(always)]
    pub const fn set_dma_mode_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TSC Bypass To use ADC2, this bit should be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn tsc_bypass(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "TSC Bypass To use ADC2, this bit should be cleared."]
    #[inline(always)]
    pub const fn set_tsc_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software synchronous reset, active high."]
    #[must_use]
    #[inline(always)]
    pub const fn softrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software synchronous reset, active high."]
    #[inline(always)]
    pub const fn set_softrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("trig_enable", &self.trig_enable())
            .field("ext0_trig_enable", &self.ext0_trig_enable())
            .field("ext0_trig_priority", &self.ext0_trig_priority())
            .field("ext1_trig_enable", &self.ext1_trig_enable())
            .field("ext1_trig_priority", &self.ext1_trig_priority())
            .field("pre_divider", &self.pre_divider())
            .field("dma_mode_sel", &self.dma_mode_sel())
            .field("tsc_bypass", &self.tsc_bypass())
            .field("softrst", &self.softrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ trig_enable: {:?}, ext0_trig_enable: {=bool:?}, ext0_trig_priority: {=u8:?}, ext1_trig_enable: {=bool:?}, ext1_trig_priority: {=u8:?}, pre_divider: {=u8:?}, dma_mode_sel: {=bool:?}, tsc_bypass: {=bool:?}, softrst: {=bool:?} }}" , self . trig_enable () , self . ext0_trig_enable () , self . ext0_trig_priority () , self . ext1_trig_enable () , self . ext1_trig_priority () , self . pre_divider () , self . dma_mode_sel () , self . tsc_bypass () , self . softrst ())
    }
}
#[doc = "ETC DMA control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCtrl(pub u32);
impl DmaCtrl {
    #[doc = "Enable DMA request when TRIG0 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG0 done."]
    #[inline(always)]
    pub const fn set_trig0_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable DMA request when TRIG1 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG1 done."]
    #[inline(always)]
    pub const fn set_trig1_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable DMA request when TRIG2 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG2 done."]
    #[inline(always)]
    pub const fn set_trig2_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable DMA request when TRIG3 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_enable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG3 done."]
    #[inline(always)]
    pub const fn set_trig3_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable DMA request when TRIG4 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG4 done."]
    #[inline(always)]
    pub const fn set_trig4_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable DMA request when TRIG5 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_enable(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG5 done."]
    #[inline(always)]
    pub const fn set_trig5_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable DMA request when TRIG6 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG6 done."]
    #[inline(always)]
    pub const fn set_trig6_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable DMA request when TRIG7 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_enable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG7 done."]
    #[inline(always)]
    pub const fn set_trig7_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_req(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig0_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_req(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig1_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_req(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig2_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_req(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig3_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_req(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig4_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_req(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig5_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_req(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig6_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_req(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig7_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for DmaCtrl {
    #[inline(always)]
    fn default() -> DmaCtrl {
        DmaCtrl(0)
    }
}
impl core::fmt::Debug for DmaCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCtrl")
            .field("trig0_enable", &self.trig0_enable())
            .field("trig1_enable", &self.trig1_enable())
            .field("trig2_enable", &self.trig2_enable())
            .field("trig3_enable", &self.trig3_enable())
            .field("trig4_enable", &self.trig4_enable())
            .field("trig5_enable", &self.trig5_enable())
            .field("trig6_enable", &self.trig6_enable())
            .field("trig7_enable", &self.trig7_enable())
            .field("trig0_req", &self.trig0_req())
            .field("trig1_req", &self.trig1_req())
            .field("trig2_req", &self.trig2_req())
            .field("trig3_req", &self.trig3_req())
            .field("trig4_req", &self.trig4_req())
            .field("trig5_req", &self.trig5_req())
            .field("trig6_req", &self.trig6_req())
            .field("trig7_req", &self.trig7_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DmaCtrl {{ trig0_enable: {=bool:?}, trig1_enable: {=bool:?}, trig2_enable: {=bool:?}, trig3_enable: {=bool:?}, trig4_enable: {=bool:?}, trig5_enable: {=bool:?}, trig6_enable: {=bool:?}, trig7_enable: {=bool:?}, trig0_req: {=bool:?}, trig1_req: {=bool:?}, trig2_req: {=bool:?}, trig3_req: {=bool:?}, trig4_req: {=bool:?}, trig5_req: {=bool:?}, trig6_req: {=bool:?}, trig7_req: {=bool:?} }}" , self . trig0_enable () , self . trig1_enable () , self . trig2_enable () , self . trig3_enable () , self . trig4_enable () , self . trig5_enable () , self . trig6_enable () , self . trig7_enable () , self . trig0_req () , self . trig1_req () , self . trig2_req () , self . trig3_req () , self . trig4_req () , self . trig5_req () , self . trig6_req () , self . trig7_req ())
    }
}
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Done01irq(pub u32);
impl Done01irq {
    #[doc = "TRIG0 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_done0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TRIG1 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_done0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIG2 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_done0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TRIG3 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_done0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TRIG4 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_done0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TRIG5 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_done0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TRIG6 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_done0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TRIG7 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_done0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TRIG0 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_done1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRIG1 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_done1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TRIG2 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_done1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TRIG3 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_done1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TRIG4 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_done1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRIG5 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_done1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TRIG6 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_done1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TRIG7 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_done1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Done01irq {
    #[inline(always)]
    fn default() -> Done01irq {
        Done01irq(0)
    }
}
impl core::fmt::Debug for Done01irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Done01irq")
            .field("trig0_done0", &self.trig0_done0())
            .field("trig1_done0", &self.trig1_done0())
            .field("trig2_done0", &self.trig2_done0())
            .field("trig3_done0", &self.trig3_done0())
            .field("trig4_done0", &self.trig4_done0())
            .field("trig5_done0", &self.trig5_done0())
            .field("trig6_done0", &self.trig6_done0())
            .field("trig7_done0", &self.trig7_done0())
            .field("trig0_done1", &self.trig0_done1())
            .field("trig1_done1", &self.trig1_done1())
            .field("trig2_done1", &self.trig2_done1())
            .field("trig3_done1", &self.trig3_done1())
            .field("trig4_done1", &self.trig4_done1())
            .field("trig5_done1", &self.trig5_done1())
            .field("trig6_done1", &self.trig6_done1())
            .field("trig7_done1", &self.trig7_done1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Done01irq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Done01irq {{ trig0_done0: {=bool:?}, trig1_done0: {=bool:?}, trig2_done0: {=bool:?}, trig3_done0: {=bool:?}, trig4_done0: {=bool:?}, trig5_done0: {=bool:?}, trig6_done0: {=bool:?}, trig7_done0: {=bool:?}, trig0_done1: {=bool:?}, trig1_done1: {=bool:?}, trig2_done1: {=bool:?}, trig3_done1: {=bool:?}, trig4_done1: {=bool:?}, trig5_done1: {=bool:?}, trig6_done1: {=bool:?}, trig7_done1: {=bool:?} }}" , self . trig0_done0 () , self . trig1_done0 () , self . trig2_done0 () , self . trig3_done0 () , self . trig4_done0 () , self . trig5_done0 () , self . trig6_done0 () , self . trig7_done0 () , self . trig0_done1 () , self . trig1_done1 () , self . trig2_done1 () , self . trig3_done1 () , self . trig4_done1 () , self . trig5_done1 () , self . trig6_done1 () , self . trig7_done1 ())
    }
}
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Done23errIrq(pub u32);
impl Done23errIrq {
    #[doc = "TRIG0 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_done2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TRIG1 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_done2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIG2 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_done2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TRIG3 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_done2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TRIG4 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_done2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TRIG5 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_done2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TRIG6 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_done2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TRIG7 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_done2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TRIG0 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_err(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRIG1 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_err(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TRIG2 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_err(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TRIG3 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_err(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TRIG4 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_err(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRIG5 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_err(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TRIG6 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_err(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TRIG7 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_err(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Done23errIrq {
    #[inline(always)]
    fn default() -> Done23errIrq {
        Done23errIrq(0)
    }
}
impl core::fmt::Debug for Done23errIrq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Done23errIrq")
            .field("trig0_done2", &self.trig0_done2())
            .field("trig1_done2", &self.trig1_done2())
            .field("trig2_done2", &self.trig2_done2())
            .field("trig3_done2", &self.trig3_done2())
            .field("trig4_done2", &self.trig4_done2())
            .field("trig5_done2", &self.trig5_done2())
            .field("trig6_done2", &self.trig6_done2())
            .field("trig7_done2", &self.trig7_done2())
            .field("trig0_err", &self.trig0_err())
            .field("trig1_err", &self.trig1_err())
            .field("trig2_err", &self.trig2_err())
            .field("trig3_err", &self.trig3_err())
            .field("trig4_err", &self.trig4_err())
            .field("trig5_err", &self.trig5_err())
            .field("trig6_err", &self.trig6_err())
            .field("trig7_err", &self.trig7_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Done23errIrq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Done23errIrq {{ trig0_done2: {=bool:?}, trig1_done2: {=bool:?}, trig2_done2: {=bool:?}, trig3_done2: {=bool:?}, trig4_done2: {=bool:?}, trig5_done2: {=bool:?}, trig6_done2: {=bool:?}, trig7_done2: {=bool:?}, trig0_err: {=bool:?}, trig1_err: {=bool:?}, trig2_err: {=bool:?}, trig3_err: {=bool:?}, trig4_err: {=bool:?}, trig5_err: {=bool:?}, trig6_err: {=bool:?}, trig7_err: {=bool:?} }}" , self . trig0_done2 () , self . trig1_done2 () , self . trig2_done2 () , self . trig3_done2 () , self . trig4_done2 () , self . trig5_done2 () , self . trig6_done2 () , self . trig7_done2 () , self . trig0_err () , self . trig1_err () , self . trig2_err () , self . trig3_err () , self . trig4_err () , self . trig5_err () , self . trig6_err () , self . trig7_err ())
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0chain10(pub u32);
impl Trig0chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig0chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig0chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig0chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig0chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig0chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig0chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig0chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig0chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig0chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig0chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig0chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig0chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0chain10 {
    #[inline(always)]
    fn default() -> Trig0chain10 {
        Trig0chain10(0)
    }
}
impl core::fmt::Debug for Trig0chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig0chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0chain32(pub u32);
impl Trig0chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig0chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig0chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig0chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig0chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig0chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig0chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig0chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig0chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig0chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig0chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig0chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig0chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0chain32 {
    #[inline(always)]
    fn default() -> Trig0chain32 {
        Trig0chain32(0)
    }
}
impl core::fmt::Debug for Trig0chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig0chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0chain54(pub u32);
impl Trig0chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig0chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig0chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig0chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig0chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig0chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig0chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig0chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig0chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig0chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig0chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig0chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig0chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0chain54 {
    #[inline(always)]
    fn default() -> Trig0chain54 {
        Trig0chain54(0)
    }
}
impl core::fmt::Debug for Trig0chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig0chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0chain76(pub u32);
impl Trig0chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig0chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig0chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig0chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig0chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig0chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig0chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig0chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig0chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig0chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig0chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig0chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig0chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0chain76 {
    #[inline(always)]
    fn default() -> Trig0chain76 {
        Trig0chain76(0)
    }
}
impl core::fmt::Debug for Trig0chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig0chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0counter(pub u32);
impl Trig0counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig0counter {
    #[inline(always)]
    fn default() -> Trig0counter {
        Trig0counter(0)
    }
}
impl core::fmt::Debug for Trig0counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0ctrl(pub u32);
impl Trig0ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig0ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig0ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig0ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig0ctrl {
    #[inline(always)]
    fn default() -> Trig0ctrl {
        Trig0ctrl(0)
    }
}
impl core::fmt::Debug for Trig0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig0ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0result10(pub u32);
impl Trig0result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0result10 {
    #[inline(always)]
    fn default() -> Trig0result10 {
        Trig0result10(0)
    }
}
impl core::fmt::Debug for Trig0result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0result32(pub u32);
impl Trig0result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0result32 {
    #[inline(always)]
    fn default() -> Trig0result32 {
        Trig0result32(0)
    }
}
impl core::fmt::Debug for Trig0result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0result54(pub u32);
impl Trig0result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0result54 {
    #[inline(always)]
    fn default() -> Trig0result54 {
        Trig0result54(0)
    }
}
impl core::fmt::Debug for Trig0result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0result76(pub u32);
impl Trig0result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0result76 {
    #[inline(always)]
    fn default() -> Trig0result76 {
        Trig0result76(0)
    }
}
impl core::fmt::Debug for Trig0result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1chain10(pub u32);
impl Trig1chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig1chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig1chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig1chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig1chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig1chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig1chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig1chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig1chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig1chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig1chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig1chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig1chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1chain10 {
    #[inline(always)]
    fn default() -> Trig1chain10 {
        Trig1chain10(0)
    }
}
impl core::fmt::Debug for Trig1chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig1chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1chain32(pub u32);
impl Trig1chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig1chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig1chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig1chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig1chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig1chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig1chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig1chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig1chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig1chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig1chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig1chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig1chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1chain32 {
    #[inline(always)]
    fn default() -> Trig1chain32 {
        Trig1chain32(0)
    }
}
impl core::fmt::Debug for Trig1chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig1chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1chain54(pub u32);
impl Trig1chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig1chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig1chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig1chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig1chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig1chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig1chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig1chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig1chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig1chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig1chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig1chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig1chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1chain54 {
    #[inline(always)]
    fn default() -> Trig1chain54 {
        Trig1chain54(0)
    }
}
impl core::fmt::Debug for Trig1chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig1chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1chain76(pub u32);
impl Trig1chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig1chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig1chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig1chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig1chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig1chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig1chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig1chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig1chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig1chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig1chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig1chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig1chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1chain76 {
    #[inline(always)]
    fn default() -> Trig1chain76 {
        Trig1chain76(0)
    }
}
impl core::fmt::Debug for Trig1chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig1chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1counter(pub u32);
impl Trig1counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig1counter {
    #[inline(always)]
    fn default() -> Trig1counter {
        Trig1counter(0)
    }
}
impl core::fmt::Debug for Trig1counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1ctrl(pub u32);
impl Trig1ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig1ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig1ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig1ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig1ctrl {
    #[inline(always)]
    fn default() -> Trig1ctrl {
        Trig1ctrl(0)
    }
}
impl core::fmt::Debug for Trig1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig1ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1result10(pub u32);
impl Trig1result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1result10 {
    #[inline(always)]
    fn default() -> Trig1result10 {
        Trig1result10(0)
    }
}
impl core::fmt::Debug for Trig1result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1result32(pub u32);
impl Trig1result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1result32 {
    #[inline(always)]
    fn default() -> Trig1result32 {
        Trig1result32(0)
    }
}
impl core::fmt::Debug for Trig1result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1result54(pub u32);
impl Trig1result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1result54 {
    #[inline(always)]
    fn default() -> Trig1result54 {
        Trig1result54(0)
    }
}
impl core::fmt::Debug for Trig1result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1result76(pub u32);
impl Trig1result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1result76 {
    #[inline(always)]
    fn default() -> Trig1result76 {
        Trig1result76(0)
    }
}
impl core::fmt::Debug for Trig1result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2chain10(pub u32);
impl Trig2chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig2chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig2chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig2chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig2chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig2chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig2chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig2chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig2chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig2chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig2chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig2chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig2chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2chain10 {
    #[inline(always)]
    fn default() -> Trig2chain10 {
        Trig2chain10(0)
    }
}
impl core::fmt::Debug for Trig2chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig2chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2chain32(pub u32);
impl Trig2chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig2chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig2chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig2chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig2chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig2chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig2chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig2chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig2chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig2chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig2chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig2chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig2chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2chain32 {
    #[inline(always)]
    fn default() -> Trig2chain32 {
        Trig2chain32(0)
    }
}
impl core::fmt::Debug for Trig2chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig2chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2chain54(pub u32);
impl Trig2chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig2chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig2chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig2chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig2chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig2chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig2chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig2chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig2chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig2chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig2chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig2chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig2chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2chain54 {
    #[inline(always)]
    fn default() -> Trig2chain54 {
        Trig2chain54(0)
    }
}
impl core::fmt::Debug for Trig2chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig2chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2chain76(pub u32);
impl Trig2chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig2chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig2chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig2chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig2chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig2chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig2chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig2chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig2chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig2chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig2chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig2chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig2chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2chain76 {
    #[inline(always)]
    fn default() -> Trig2chain76 {
        Trig2chain76(0)
    }
}
impl core::fmt::Debug for Trig2chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig2chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2counter(pub u32);
impl Trig2counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig2counter {
    #[inline(always)]
    fn default() -> Trig2counter {
        Trig2counter(0)
    }
}
impl core::fmt::Debug for Trig2counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2ctrl(pub u32);
impl Trig2ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig2ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig2ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig2ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig2ctrl {
    #[inline(always)]
    fn default() -> Trig2ctrl {
        Trig2ctrl(0)
    }
}
impl core::fmt::Debug for Trig2ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig2ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2result10(pub u32);
impl Trig2result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2result10 {
    #[inline(always)]
    fn default() -> Trig2result10 {
        Trig2result10(0)
    }
}
impl core::fmt::Debug for Trig2result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2result32(pub u32);
impl Trig2result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2result32 {
    #[inline(always)]
    fn default() -> Trig2result32 {
        Trig2result32(0)
    }
}
impl core::fmt::Debug for Trig2result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2result54(pub u32);
impl Trig2result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2result54 {
    #[inline(always)]
    fn default() -> Trig2result54 {
        Trig2result54(0)
    }
}
impl core::fmt::Debug for Trig2result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2result76(pub u32);
impl Trig2result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2result76 {
    #[inline(always)]
    fn default() -> Trig2result76 {
        Trig2result76(0)
    }
}
impl core::fmt::Debug for Trig2result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3chain10(pub u32);
impl Trig3chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig3chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig3chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig3chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig3chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig3chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig3chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig3chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig3chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig3chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig3chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig3chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig3chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3chain10 {
    #[inline(always)]
    fn default() -> Trig3chain10 {
        Trig3chain10(0)
    }
}
impl core::fmt::Debug for Trig3chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig3chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3chain32(pub u32);
impl Trig3chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig3chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig3chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig3chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig3chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig3chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig3chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig3chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig3chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig3chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig3chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig3chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig3chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3chain32 {
    #[inline(always)]
    fn default() -> Trig3chain32 {
        Trig3chain32(0)
    }
}
impl core::fmt::Debug for Trig3chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig3chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3chain54(pub u32);
impl Trig3chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig3chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig3chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig3chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig3chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig3chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig3chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig3chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig3chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig3chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig3chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig3chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig3chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3chain54 {
    #[inline(always)]
    fn default() -> Trig3chain54 {
        Trig3chain54(0)
    }
}
impl core::fmt::Debug for Trig3chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig3chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3chain76(pub u32);
impl Trig3chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig3chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig3chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig3chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig3chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig3chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig3chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig3chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig3chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig3chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig3chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig3chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig3chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3chain76 {
    #[inline(always)]
    fn default() -> Trig3chain76 {
        Trig3chain76(0)
    }
}
impl core::fmt::Debug for Trig3chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig3chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3counter(pub u32);
impl Trig3counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig3counter {
    #[inline(always)]
    fn default() -> Trig3counter {
        Trig3counter(0)
    }
}
impl core::fmt::Debug for Trig3counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3ctrl(pub u32);
impl Trig3ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig3ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig3ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig3ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig3ctrl {
    #[inline(always)]
    fn default() -> Trig3ctrl {
        Trig3ctrl(0)
    }
}
impl core::fmt::Debug for Trig3ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig3ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3result10(pub u32);
impl Trig3result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3result10 {
    #[inline(always)]
    fn default() -> Trig3result10 {
        Trig3result10(0)
    }
}
impl core::fmt::Debug for Trig3result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3result32(pub u32);
impl Trig3result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3result32 {
    #[inline(always)]
    fn default() -> Trig3result32 {
        Trig3result32(0)
    }
}
impl core::fmt::Debug for Trig3result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3result54(pub u32);
impl Trig3result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3result54 {
    #[inline(always)]
    fn default() -> Trig3result54 {
        Trig3result54(0)
    }
}
impl core::fmt::Debug for Trig3result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3result76(pub u32);
impl Trig3result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3result76 {
    #[inline(always)]
    fn default() -> Trig3result76 {
        Trig3result76(0)
    }
}
impl core::fmt::Debug for Trig3result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4chain10(pub u32);
impl Trig4chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig4chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig4chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig4chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig4chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig4chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig4chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig4chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig4chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig4chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig4chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig4chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig4chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4chain10 {
    #[inline(always)]
    fn default() -> Trig4chain10 {
        Trig4chain10(0)
    }
}
impl core::fmt::Debug for Trig4chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig4chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4chain32(pub u32);
impl Trig4chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig4chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig4chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig4chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig4chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig4chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig4chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig4chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig4chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig4chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig4chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig4chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig4chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4chain32 {
    #[inline(always)]
    fn default() -> Trig4chain32 {
        Trig4chain32(0)
    }
}
impl core::fmt::Debug for Trig4chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig4chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4chain54(pub u32);
impl Trig4chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig4chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig4chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig4chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig4chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig4chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig4chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig4chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig4chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig4chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig4chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig4chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig4chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4chain54 {
    #[inline(always)]
    fn default() -> Trig4chain54 {
        Trig4chain54(0)
    }
}
impl core::fmt::Debug for Trig4chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig4chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4chain76(pub u32);
impl Trig4chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig4chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig4chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig4chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig4chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig4chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig4chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig4chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig4chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig4chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig4chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig4chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig4chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4chain76 {
    #[inline(always)]
    fn default() -> Trig4chain76 {
        Trig4chain76(0)
    }
}
impl core::fmt::Debug for Trig4chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig4chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4counter(pub u32);
impl Trig4counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig4counter {
    #[inline(always)]
    fn default() -> Trig4counter {
        Trig4counter(0)
    }
}
impl core::fmt::Debug for Trig4counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4ctrl(pub u32);
impl Trig4ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig4ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig4ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig4ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig4ctrl {
    #[inline(always)]
    fn default() -> Trig4ctrl {
        Trig4ctrl(0)
    }
}
impl core::fmt::Debug for Trig4ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig4ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4result10(pub u32);
impl Trig4result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4result10 {
    #[inline(always)]
    fn default() -> Trig4result10 {
        Trig4result10(0)
    }
}
impl core::fmt::Debug for Trig4result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4result32(pub u32);
impl Trig4result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4result32 {
    #[inline(always)]
    fn default() -> Trig4result32 {
        Trig4result32(0)
    }
}
impl core::fmt::Debug for Trig4result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4result54(pub u32);
impl Trig4result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4result54 {
    #[inline(always)]
    fn default() -> Trig4result54 {
        Trig4result54(0)
    }
}
impl core::fmt::Debug for Trig4result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4result76(pub u32);
impl Trig4result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4result76 {
    #[inline(always)]
    fn default() -> Trig4result76 {
        Trig4result76(0)
    }
}
impl core::fmt::Debug for Trig4result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5chain10(pub u32);
impl Trig5chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig5chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig5chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig5chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig5chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig5chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig5chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig5chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig5chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig5chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig5chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig5chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig5chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5chain10 {
    #[inline(always)]
    fn default() -> Trig5chain10 {
        Trig5chain10(0)
    }
}
impl core::fmt::Debug for Trig5chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig5chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5chain32(pub u32);
impl Trig5chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig5chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig5chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig5chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig5chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig5chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig5chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig5chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig5chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig5chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig5chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig5chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig5chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5chain32 {
    #[inline(always)]
    fn default() -> Trig5chain32 {
        Trig5chain32(0)
    }
}
impl core::fmt::Debug for Trig5chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig5chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5chain54(pub u32);
impl Trig5chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig5chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig5chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig5chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig5chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig5chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig5chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig5chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig5chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig5chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig5chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig5chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig5chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5chain54 {
    #[inline(always)]
    fn default() -> Trig5chain54 {
        Trig5chain54(0)
    }
}
impl core::fmt::Debug for Trig5chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig5chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5chain76(pub u32);
impl Trig5chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig5chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig5chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig5chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig5chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig5chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig5chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig5chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig5chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig5chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig5chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig5chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig5chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5chain76 {
    #[inline(always)]
    fn default() -> Trig5chain76 {
        Trig5chain76(0)
    }
}
impl core::fmt::Debug for Trig5chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig5chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5counter(pub u32);
impl Trig5counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig5counter {
    #[inline(always)]
    fn default() -> Trig5counter {
        Trig5counter(0)
    }
}
impl core::fmt::Debug for Trig5counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5ctrl(pub u32);
impl Trig5ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig5ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig5ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig5ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig5ctrl {
    #[inline(always)]
    fn default() -> Trig5ctrl {
        Trig5ctrl(0)
    }
}
impl core::fmt::Debug for Trig5ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig5ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5result10(pub u32);
impl Trig5result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5result10 {
    #[inline(always)]
    fn default() -> Trig5result10 {
        Trig5result10(0)
    }
}
impl core::fmt::Debug for Trig5result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5result32(pub u32);
impl Trig5result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5result32 {
    #[inline(always)]
    fn default() -> Trig5result32 {
        Trig5result32(0)
    }
}
impl core::fmt::Debug for Trig5result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5result54(pub u32);
impl Trig5result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5result54 {
    #[inline(always)]
    fn default() -> Trig5result54 {
        Trig5result54(0)
    }
}
impl core::fmt::Debug for Trig5result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5result76(pub u32);
impl Trig5result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5result76 {
    #[inline(always)]
    fn default() -> Trig5result76 {
        Trig5result76(0)
    }
}
impl core::fmt::Debug for Trig5result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6chain10(pub u32);
impl Trig6chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig6chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig6chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig6chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig6chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig6chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig6chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig6chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig6chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig6chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig6chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig6chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig6chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6chain10 {
    #[inline(always)]
    fn default() -> Trig6chain10 {
        Trig6chain10(0)
    }
}
impl core::fmt::Debug for Trig6chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig6chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6chain32(pub u32);
impl Trig6chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig6chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig6chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig6chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig6chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig6chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig6chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig6chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig6chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig6chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig6chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig6chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig6chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6chain32 {
    #[inline(always)]
    fn default() -> Trig6chain32 {
        Trig6chain32(0)
    }
}
impl core::fmt::Debug for Trig6chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig6chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6chain54(pub u32);
impl Trig6chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig6chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig6chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig6chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig6chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig6chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig6chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig6chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig6chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig6chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig6chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig6chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig6chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6chain54 {
    #[inline(always)]
    fn default() -> Trig6chain54 {
        Trig6chain54(0)
    }
}
impl core::fmt::Debug for Trig6chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig6chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6chain76(pub u32);
impl Trig6chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig6chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig6chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig6chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig6chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig6chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig6chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig6chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig6chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig6chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig6chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig6chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig6chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6chain76 {
    #[inline(always)]
    fn default() -> Trig6chain76 {
        Trig6chain76(0)
    }
}
impl core::fmt::Debug for Trig6chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig6chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6counter(pub u32);
impl Trig6counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig6counter {
    #[inline(always)]
    fn default() -> Trig6counter {
        Trig6counter(0)
    }
}
impl core::fmt::Debug for Trig6counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6ctrl(pub u32);
impl Trig6ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig6ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig6ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig6ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig6ctrl {
    #[inline(always)]
    fn default() -> Trig6ctrl {
        Trig6ctrl(0)
    }
}
impl core::fmt::Debug for Trig6ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig6ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6result10(pub u32);
impl Trig6result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6result10 {
    #[inline(always)]
    fn default() -> Trig6result10 {
        Trig6result10(0)
    }
}
impl core::fmt::Debug for Trig6result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6result32(pub u32);
impl Trig6result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6result32 {
    #[inline(always)]
    fn default() -> Trig6result32 {
        Trig6result32(0)
    }
}
impl core::fmt::Debug for Trig6result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6result54(pub u32);
impl Trig6result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6result54 {
    #[inline(always)]
    fn default() -> Trig6result54 {
        Trig6result54(0)
    }
}
impl core::fmt::Debug for Trig6result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6result76(pub u32);
impl Trig6result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6result76 {
    #[inline(always)]
    fn default() -> Trig6result76 {
        Trig6result76(0)
    }
}
impl core::fmt::Debug for Trig6result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7chain10(pub u32);
impl Trig7chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig7chain10csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7chain10csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig7chain10csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig7chain10hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7chain10hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig7chain10hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig7chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig7chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig7chain10csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7chain10csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig7chain10csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig7chain10hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7chain10hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig7chain10hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig7chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig7chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7chain10 {
    #[inline(always)]
    fn default() -> Trig7chain10 {
        Trig7chain10(0)
    }
}
impl core::fmt::Debug for Trig7chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig7chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}" , self . csel0 () , self . hwts0 () , self . b2b0 () , self . ie0 () , self . csel1 () , self . hwts1 () , self . b2b1 () , self . ie1 ())
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7chain32(pub u32);
impl Trig7chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig7chain32csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7chain32csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig7chain32csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig7chain32hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7chain32hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig7chain32hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig7chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig7chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig7chain32csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7chain32csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig7chain32csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig7chain32hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7chain32hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig7chain32hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig7chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig7chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7chain32 {
    #[inline(always)]
    fn default() -> Trig7chain32 {
        Trig7chain32(0)
    }
}
impl core::fmt::Debug for Trig7chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig7chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}" , self . csel2 () , self . hwts2 () , self . b2b2 () , self . ie2 () , self . csel3 () , self . hwts3 () , self . b2b3 () , self . ie3 ())
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7chain54(pub u32);
impl Trig7chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig7chain54csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7chain54csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig7chain54csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig7chain54hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7chain54hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig7chain54hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig7chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig7chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig7chain54csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7chain54csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig7chain54csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig7chain54hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7chain54hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig7chain54hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig7chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig7chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7chain54 {
    #[inline(always)]
    fn default() -> Trig7chain54 {
        Trig7chain54(0)
    }
}
impl core::fmt::Debug for Trig7chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig7chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}" , self . csel4 () , self . hwts4 () , self . b2b4 () , self . ie4 () , self . csel5 () , self . hwts5 () , self . b2b5 () , self . ie5 ())
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7chain76(pub u32);
impl Trig7chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig7chain76csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7chain76csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig7chain76csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig7chain76hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7chain76hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig7chain76hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig7chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig7chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig7chain76csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7chain76csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig7chain76csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig7chain76hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7chain76hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig7chain76hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig7chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig7chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7chain76 {
    #[inline(always)]
    fn default() -> Trig7chain76 {
        Trig7chain76(0)
    }
}
impl core::fmt::Debug for Trig7chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig7chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}" , self . csel6 () , self . hwts6 () , self . b2b6 () , self . ie6 () , self . csel7 () , self . hwts7 () , self . b2b7 () , self . ie7 ())
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7counter(pub u32);
impl Trig7counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig7counter {
    #[inline(always)]
    fn default() -> Trig7counter {
        Trig7counter(0)
    }
}
impl core::fmt::Debug for Trig7counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7ctrl(pub u32);
impl Trig7ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig7ctrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig7ctrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig7ctrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig7ctrl {
    #[inline(always)]
    fn default() -> Trig7ctrl {
        Trig7ctrl(0)
    }
}
impl core::fmt::Debug for Trig7ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Trig7ctrl {{ sw_trig: {=bool:?}, trig_mode: {=bool:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {=bool:?} }}" , self . sw_trig () , self . trig_mode () , self . trig_chain () , self . trig_priority () , self . sync_mode ())
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7result10(pub u32);
impl Trig7result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7result10 {
    #[inline(always)]
    fn default() -> Trig7result10 {
        Trig7result10(0)
    }
}
impl core::fmt::Debug for Trig7result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7result32(pub u32);
impl Trig7result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7result32 {
    #[inline(always)]
    fn default() -> Trig7result32 {
        Trig7result32(0)
    }
}
impl core::fmt::Debug for Trig7result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7result54(pub u32);
impl Trig7result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7result54 {
    #[inline(always)]
    fn default() -> Trig7result54 {
        Trig7result54(0)
    }
}
impl core::fmt::Debug for Trig7result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7result76(pub u32);
impl Trig7result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7result76 {
    #[inline(always)]
    fn default() -> Trig7result76 {
        Trig7result76(0)
    }
}
impl core::fmt::Debug for Trig7result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
