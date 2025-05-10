#[doc = "ADC_ETC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcEtc {
    ptr: *mut u8,
}
unsafe impl Send for AdcEtc {}
unsafe impl Sync for AdcEtc {}
impl AdcEtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC_ETC Global Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ETC DONE0 and DONE1 IRQ State Register"]
    #[inline(always)]
    pub const fn done0_1_irq(self) -> crate::common::Reg<regs::Done01irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
    #[inline(always)]
    pub const fn done2_3_err_irq(
        self,
    ) -> crate::common::Reg<regs::Done23errIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ETC DMA control Register"]
    #[inline(always)]
    pub const fn dma_ctrl(self) -> crate::common::Reg<regs::DmaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig0_ctrl(self) -> crate::common::Reg<regs::Trig0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig0_counter(self) -> crate::common::Reg<regs::Trig0counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig0_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig0chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig0_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig0chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig0_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig0chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig0_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig0chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig0_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig0result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig0_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig0result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig0_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig0result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig0_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig0result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig1_ctrl(self) -> crate::common::Reg<regs::Trig1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig1_counter(self) -> crate::common::Reg<regs::Trig1counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig1_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig1chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig1_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig1chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig1_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig1chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig1_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig1chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig1_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig1result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig1_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig1result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig1_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig1result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig1_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig1result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig2_ctrl(self) -> crate::common::Reg<regs::Trig2ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig2_counter(self) -> crate::common::Reg<regs::Trig2counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig2_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig2chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig2_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig2chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig2_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig2chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig2_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig2chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig2_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig2result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig2_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig2result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig2_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig2result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig2_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig2result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig3_ctrl(self) -> crate::common::Reg<regs::Trig3ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig3_counter(self) -> crate::common::Reg<regs::Trig3counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig3_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig3chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig3_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig3chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig3_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig3chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig3_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig3chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig3_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig3result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig3_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig3result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig3_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig3result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig3_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig3result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig4_ctrl(self) -> crate::common::Reg<regs::Trig4ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig4_counter(self) -> crate::common::Reg<regs::Trig4counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig4_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig4chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig4_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig4chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig4_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig4chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig4_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig4chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig4_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig4result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig4_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig4result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig4_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig4result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig4_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig4result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig5_ctrl(self) -> crate::common::Reg<regs::Trig5ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig5_counter(self) -> crate::common::Reg<regs::Trig5counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig5_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig5chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig5_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig5chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig5_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig5chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig5_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig5chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig5_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig5result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig5_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig5result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig5_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig5result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig5_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig5result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig6_ctrl(self) -> crate::common::Reg<regs::Trig6ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig6_counter(self) -> crate::common::Reg<regs::Trig6counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig6_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig6chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig6_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig6chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig6_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig6chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig6_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig6chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig6_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig6result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig6_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig6result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig6_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig6result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig6_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig6result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig7_ctrl(self) -> crate::common::Reg<regs::Trig7ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig7_counter(self) -> crate::common::Reg<regs::Trig7counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig7_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig7chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig7_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig7chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig7_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig7chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig7_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig7chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig7_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig7result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig7_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig7result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig7_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig7result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig7_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig7result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
