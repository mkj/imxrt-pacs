#[doc = "ADC_ETC Global Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "TRIG enable register."]
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
    #[doc = "Pre-divider for trig delay and interval"]
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
    #[doc = "Software synchronous reset, active high."]
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
#[doc = "ETC DMA control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DmaCtrl(pub u32);
impl DmaCtrl {
    #[doc = "Enable DMA request when TRIG0 done."]
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
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Done01irq(pub u32);
impl Done01irq {
    #[doc = "TRIG0 done0 interrupt detection."]
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
#[doc = "ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Done23errIrq(pub u32);
impl Done23errIrq {
    #[doc = "TRIG0 done2 interrupt detection."]
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
    #[doc = "TRIG0 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig0_done3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TRIG1 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig1_done3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TRIG2 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig2_done3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TRIG3 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig3_done3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TRIG4 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig4_done3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TRIG5 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig5_done3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TRIG6 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig6_done3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TRIG7 done3 interrupt detection."]
    #[inline(always)]
    pub const fn trig7_done3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done3 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TRIG0 error interrupt detection."]
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
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0chain10(pub u32);
impl Trig0chain10 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig0chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig0chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn ie0_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn set_ie0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig0chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig0chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn ie1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn set_ie1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig0chain10 {
    #[inline(always)]
    fn default() -> Trig0chain10 {
        Trig0chain10(0)
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0chain32(pub u32);
impl Trig0chain32 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig0chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig0chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn ie2_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn set_ie2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig0chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig0chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn ie3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn set_ie3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig0chain32 {
    #[inline(always)]
    fn default() -> Trig0chain32 {
        Trig0chain32(0)
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0chain54(pub u32);
impl Trig0chain54 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig0chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig0chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn ie4_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn set_ie4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig0chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig0chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn ie5_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn set_ie5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig0chain54 {
    #[inline(always)]
    fn default() -> Trig0chain54 {
        Trig0chain54(0)
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0chain76(pub u32);
impl Trig0chain76 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig0chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig0chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn ie6_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn set_ie6_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig0chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig0chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn ie7_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn set_ie7_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig0chain76 {
    #[inline(always)]
    fn default() -> Trig0chain76 {
        Trig0chain76(0)
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0counter(pub u32);
impl Trig0counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
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
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0ctrl(pub u32);
impl Trig0ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
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
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0result10(pub u32);
impl Trig0result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0result32(pub u32);
impl Trig0result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0result54(pub u32);
impl Trig0result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig0result76(pub u32);
impl Trig0result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1chain10(pub u32);
impl Trig1chain10 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig1chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig1chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn ie0_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn set_ie0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig1chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig1chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn ie1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn set_ie1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig1chain10 {
    #[inline(always)]
    fn default() -> Trig1chain10 {
        Trig1chain10(0)
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1chain32(pub u32);
impl Trig1chain32 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig1chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig1chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn ie2_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn set_ie2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig1chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig1chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn ie3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn set_ie3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig1chain32 {
    #[inline(always)]
    fn default() -> Trig1chain32 {
        Trig1chain32(0)
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1chain54(pub u32);
impl Trig1chain54 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig1chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig1chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn ie4_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn set_ie4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig1chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig1chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn ie5_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn set_ie5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig1chain54 {
    #[inline(always)]
    fn default() -> Trig1chain54 {
        Trig1chain54(0)
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1chain76(pub u32);
impl Trig1chain76 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig1chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig1chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn ie6_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn set_ie6_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig1chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig1chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn ie7_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn set_ie7_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig1chain76 {
    #[inline(always)]
    fn default() -> Trig1chain76 {
        Trig1chain76(0)
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1counter(pub u32);
impl Trig1counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
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
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1ctrl(pub u32);
impl Trig1ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
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
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1result10(pub u32);
impl Trig1result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1result32(pub u32);
impl Trig1result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1result54(pub u32);
impl Trig1result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig1result76(pub u32);
impl Trig1result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2chain10(pub u32);
impl Trig2chain10 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig2chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig2chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn ie0_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn set_ie0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig2chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig2chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn ie1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn set_ie1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig2chain10 {
    #[inline(always)]
    fn default() -> Trig2chain10 {
        Trig2chain10(0)
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2chain32(pub u32);
impl Trig2chain32 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig2chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig2chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn ie2_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn set_ie2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig2chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig2chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn ie3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn set_ie3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig2chain32 {
    #[inline(always)]
    fn default() -> Trig2chain32 {
        Trig2chain32(0)
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2chain54(pub u32);
impl Trig2chain54 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig2chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig2chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn ie4_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn set_ie4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig2chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig2chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn ie5_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn set_ie5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig2chain54 {
    #[inline(always)]
    fn default() -> Trig2chain54 {
        Trig2chain54(0)
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2chain76(pub u32);
impl Trig2chain76 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig2chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig2chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn ie6_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn set_ie6_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig2chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig2chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn ie7_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn set_ie7_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig2chain76 {
    #[inline(always)]
    fn default() -> Trig2chain76 {
        Trig2chain76(0)
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2counter(pub u32);
impl Trig2counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
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
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2ctrl(pub u32);
impl Trig2ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
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
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2result10(pub u32);
impl Trig2result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2result32(pub u32);
impl Trig2result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2result54(pub u32);
impl Trig2result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig2result76(pub u32);
impl Trig2result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3chain10(pub u32);
impl Trig3chain10 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig3chain10ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain10ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig3chain10ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn ie0_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 0."]
    #[inline(always)]
    pub const fn set_ie0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig3chain10ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain10ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig3chain10ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn ie1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 1."]
    #[inline(always)]
    pub const fn set_ie1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig3chain10 {
    #[inline(always)]
    fn default() -> Trig3chain10 {
        Trig3chain10(0)
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3chain32(pub u32);
impl Trig3chain32 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig3chain32ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain32ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig3chain32ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn ie2_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 2."]
    #[inline(always)]
    pub const fn set_ie2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig3chain32ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain32ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig3chain32ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn ie3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 3."]
    #[inline(always)]
    pub const fn set_ie3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig3chain32 {
    #[inline(always)]
    fn default() -> Trig3chain32 {
        Trig3chain32(0)
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3chain54(pub u32);
impl Trig3chain54 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig3chain54ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain54ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig3chain54ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn ie4_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 4."]
    #[inline(always)]
    pub const fn set_ie4_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig3chain54ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain54ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig3chain54ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn ie5_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 5."]
    #[inline(always)]
    pub const fn set_ie5_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig3chain54 {
    #[inline(always)]
    fn default() -> Trig3chain54 {
        Trig3chain54(0)
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3chain76(pub u32);
impl Trig3chain76 {
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig3chain76ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3chain76ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig3chain76ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn ie6_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 6."]
    #[inline(always)]
    pub const fn set_ie6_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC channel selection"]
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
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig3chain76ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3chain76ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig3chain76ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn ie7_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ enable of segment 7."]
    #[inline(always)]
    pub const fn set_ie7_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig3chain76 {
    #[inline(always)]
    fn default() -> Trig3chain76 {
        Trig3chain76(0)
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3counter(pub u32);
impl Trig3counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
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
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3ctrl(pub u32);
impl Trig3ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
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
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3result10(pub u32);
impl Trig3result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3result32(pub u32);
impl Trig3result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3result54(pub u32);
impl Trig3result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Trig3result76(pub u32);
impl Trig3result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
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
