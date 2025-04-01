#[doc = "ADC_ETC"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "ETC DONE0 and DONE1 IRQ State Register"]
    #[inline(always)]
    pub const fn done0_1_irq(self) -> crate::common::Reg<regs::Done01irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register"]
    #[inline(always)]
    pub const fn done2_3_err_irq(
        self,
    ) -> crate::common::Reg<regs::Done23errIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "ETC DMA control Register"]
    #[inline(always)]
    pub const fn dma_ctrl(self) -> crate::common::Reg<regs::DmaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig0_ctrl(self) -> crate::common::Reg<regs::Trig0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig0_counter(self) -> crate::common::Reg<regs::Trig0counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig0_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig0chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig0_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig0chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig0_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig0chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig0_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig0chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig0_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig0result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig0_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig0result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig0_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig0result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig0_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig0result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig1_ctrl(self) -> crate::common::Reg<regs::Trig1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig1_counter(self) -> crate::common::Reg<regs::Trig1counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig1_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig1chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig1_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig1chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig1_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig1chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig1_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig1chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig1_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig1result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig1_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig1result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig1_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig1result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig1_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig1result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig2_ctrl(self) -> crate::common::Reg<regs::Trig2ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig2_counter(self) -> crate::common::Reg<regs::Trig2counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig2_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig2chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig2_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig2chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig2_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig2chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig2_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig2chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig2_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig2result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig2_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig2result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig2_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig2result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig2_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig2result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "ETC_TRIG Control Register"]
    #[inline(always)]
    pub const fn trig3_ctrl(self) -> crate::common::Reg<regs::Trig3ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "ETC_TRIG Counter Register"]
    #[inline(always)]
    pub const fn trig3_counter(self) -> crate::common::Reg<regs::Trig3counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 0/1 Register"]
    #[inline(always)]
    pub const fn trig3_chain_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig3chain10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 2/3 Register"]
    #[inline(always)]
    pub const fn trig3_chain_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig3chain32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 4/5 Register"]
    #[inline(always)]
    pub const fn trig3_chain_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig3chain54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "ETC_TRIG Chain 6/7 Register"]
    #[inline(always)]
    pub const fn trig3_chain_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig3chain76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 1/0 Register"]
    #[inline(always)]
    pub const fn trig3_result_1_0(
        self,
    ) -> crate::common::Reg<regs::Trig3result10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 3/2 Register"]
    #[inline(always)]
    pub const fn trig3_result_3_2(
        self,
    ) -> crate::common::Reg<regs::Trig3result32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 5/4 Register"]
    #[inline(always)]
    pub const fn trig3_result_5_4(
        self,
    ) -> crate::common::Reg<regs::Trig3result54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "ETC_TRIG Result Data 7/6 Register"]
    #[inline(always)]
    pub const fn trig3_result_7_6(
        self,
    ) -> crate::common::Reg<regs::Trig3result76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
}
pub mod regs;
pub mod vals;
