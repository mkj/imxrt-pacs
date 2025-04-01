#[doc = "Crossbar Switch"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Xbara {
    ptr: *mut u8,
}
unsafe impl Send for Xbara {}
unsafe impl Sync for Xbara {}
impl Xbara {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Crossbar A Select Register 0"]
    #[inline(always)]
    pub const fn sel0(self) -> crate::common::Reg<regs::Sel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Crossbar A Select Register 1"]
    #[inline(always)]
    pub const fn sel1(self) -> crate::common::Reg<regs::Sel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2usize) as _) }
    }
    #[doc = "Crossbar A Select Register 2"]
    #[inline(always)]
    pub const fn sel2(self) -> crate::common::Reg<regs::Sel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Crossbar A Select Register 3"]
    #[inline(always)]
    pub const fn sel3(self) -> crate::common::Reg<regs::Sel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(6usize) as _) }
    }
    #[doc = "Crossbar A Select Register 4"]
    #[inline(always)]
    pub const fn sel4(self) -> crate::common::Reg<regs::Sel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Crossbar A Select Register 5"]
    #[inline(always)]
    pub const fn sel5(self) -> crate::common::Reg<regs::Sel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(10usize) as _) }
    }
    #[doc = "Crossbar A Select Register 6"]
    #[inline(always)]
    pub const fn sel6(self) -> crate::common::Reg<regs::Sel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Crossbar A Select Register 7"]
    #[inline(always)]
    pub const fn sel7(self) -> crate::common::Reg<regs::Sel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(14usize) as _) }
    }
    #[doc = "Crossbar A Select Register 8"]
    #[inline(always)]
    pub const fn sel8(self) -> crate::common::Reg<regs::Sel8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Crossbar A Select Register 9"]
    #[inline(always)]
    pub const fn sel9(self) -> crate::common::Reg<regs::Sel9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(18usize) as _) }
    }
    #[doc = "Crossbar A Select Register 10"]
    #[inline(always)]
    pub const fn sel10(self) -> crate::common::Reg<regs::Sel10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Crossbar A Select Register 11"]
    #[inline(always)]
    pub const fn sel11(self) -> crate::common::Reg<regs::Sel11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(22usize) as _) }
    }
    #[doc = "Crossbar A Select Register 12"]
    #[inline(always)]
    pub const fn sel12(self) -> crate::common::Reg<regs::Sel12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Crossbar A Select Register 13"]
    #[inline(always)]
    pub const fn sel13(self) -> crate::common::Reg<regs::Sel13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(26usize) as _) }
    }
    #[doc = "Crossbar A Select Register 14"]
    #[inline(always)]
    pub const fn sel14(self) -> crate::common::Reg<regs::Sel14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Crossbar A Select Register 15"]
    #[inline(always)]
    pub const fn sel15(self) -> crate::common::Reg<regs::Sel15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(30usize) as _) }
    }
    #[doc = "Crossbar A Select Register 16"]
    #[inline(always)]
    pub const fn sel16(self) -> crate::common::Reg<regs::Sel16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Crossbar A Select Register 17"]
    #[inline(always)]
    pub const fn sel17(self) -> crate::common::Reg<regs::Sel17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(34usize) as _) }
    }
    #[doc = "Crossbar A Select Register 18"]
    #[inline(always)]
    pub const fn sel18(self) -> crate::common::Reg<regs::Sel18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Crossbar A Select Register 19"]
    #[inline(always)]
    pub const fn sel19(self) -> crate::common::Reg<regs::Sel19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(38usize) as _) }
    }
    #[doc = "Crossbar A Select Register 20"]
    #[inline(always)]
    pub const fn sel20(self) -> crate::common::Reg<regs::Sel20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Crossbar A Select Register 21"]
    #[inline(always)]
    pub const fn sel21(self) -> crate::common::Reg<regs::Sel21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(42usize) as _) }
    }
    #[doc = "Crossbar A Select Register 22"]
    #[inline(always)]
    pub const fn sel22(self) -> crate::common::Reg<regs::Sel22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Crossbar A Select Register 23"]
    #[inline(always)]
    pub const fn sel23(self) -> crate::common::Reg<regs::Sel23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(46usize) as _) }
    }
    #[doc = "Crossbar A Select Register 24"]
    #[inline(always)]
    pub const fn sel24(self) -> crate::common::Reg<regs::Sel24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Crossbar A Select Register 25"]
    #[inline(always)]
    pub const fn sel25(self) -> crate::common::Reg<regs::Sel25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(50usize) as _) }
    }
    #[doc = "Crossbar A Select Register 26"]
    #[inline(always)]
    pub const fn sel26(self) -> crate::common::Reg<regs::Sel26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Crossbar A Select Register 27"]
    #[inline(always)]
    pub const fn sel27(self) -> crate::common::Reg<regs::Sel27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(54usize) as _) }
    }
    #[doc = "Crossbar A Select Register 28"]
    #[inline(always)]
    pub const fn sel28(self) -> crate::common::Reg<regs::Sel28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Crossbar A Select Register 29"]
    #[inline(always)]
    pub const fn sel29(self) -> crate::common::Reg<regs::Sel29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(58usize) as _) }
    }
    #[doc = "Crossbar A Select Register 30"]
    #[inline(always)]
    pub const fn sel30(self) -> crate::common::Reg<regs::Sel30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Crossbar A Select Register 31"]
    #[inline(always)]
    pub const fn sel31(self) -> crate::common::Reg<regs::Sel31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(62usize) as _) }
    }
    #[doc = "Crossbar A Select Register 32"]
    #[inline(always)]
    pub const fn sel32(self) -> crate::common::Reg<regs::Sel32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Crossbar A Select Register 33"]
    #[inline(always)]
    pub const fn sel33(self) -> crate::common::Reg<regs::Sel33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(66usize) as _) }
    }
    #[doc = "Crossbar A Select Register 34"]
    #[inline(always)]
    pub const fn sel34(self) -> crate::common::Reg<regs::Sel34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Crossbar A Select Register 35"]
    #[inline(always)]
    pub const fn sel35(self) -> crate::common::Reg<regs::Sel35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(70usize) as _) }
    }
    #[doc = "Crossbar A Select Register 36"]
    #[inline(always)]
    pub const fn sel36(self) -> crate::common::Reg<regs::Sel36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Crossbar A Select Register 37"]
    #[inline(always)]
    pub const fn sel37(self) -> crate::common::Reg<regs::Sel37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(74usize) as _) }
    }
    #[doc = "Crossbar A Select Register 38"]
    #[inline(always)]
    pub const fn sel38(self) -> crate::common::Reg<regs::Sel38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Crossbar A Select Register 39"]
    #[inline(always)]
    pub const fn sel39(self) -> crate::common::Reg<regs::Sel39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(78usize) as _) }
    }
    #[doc = "Crossbar A Select Register 40"]
    #[inline(always)]
    pub const fn sel40(self) -> crate::common::Reg<regs::Sel40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Crossbar A Select Register 41"]
    #[inline(always)]
    pub const fn sel41(self) -> crate::common::Reg<regs::Sel41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(82usize) as _) }
    }
    #[doc = "Crossbar A Select Register 42"]
    #[inline(always)]
    pub const fn sel42(self) -> crate::common::Reg<regs::Sel42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Crossbar A Select Register 43"]
    #[inline(always)]
    pub const fn sel43(self) -> crate::common::Reg<regs::Sel43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(86usize) as _) }
    }
    #[doc = "Crossbar A Select Register 44"]
    #[inline(always)]
    pub const fn sel44(self) -> crate::common::Reg<regs::Sel44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Crossbar A Select Register 45"]
    #[inline(always)]
    pub const fn sel45(self) -> crate::common::Reg<regs::Sel45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(90usize) as _) }
    }
    #[doc = "Crossbar A Select Register 46"]
    #[inline(always)]
    pub const fn sel46(self) -> crate::common::Reg<regs::Sel46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Crossbar A Select Register 47"]
    #[inline(always)]
    pub const fn sel47(self) -> crate::common::Reg<regs::Sel47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(94usize) as _) }
    }
    #[doc = "Crossbar A Select Register 48"]
    #[inline(always)]
    pub const fn sel48(self) -> crate::common::Reg<regs::Sel48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Crossbar A Select Register 49"]
    #[inline(always)]
    pub const fn sel49(self) -> crate::common::Reg<regs::Sel49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(98usize) as _) }
    }
    #[doc = "Crossbar A Select Register 50"]
    #[inline(always)]
    pub const fn sel50(self) -> crate::common::Reg<regs::Sel50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Crossbar A Select Register 51"]
    #[inline(always)]
    pub const fn sel51(self) -> crate::common::Reg<regs::Sel51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(102usize) as _) }
    }
    #[doc = "Crossbar A Select Register 52"]
    #[inline(always)]
    pub const fn sel52(self) -> crate::common::Reg<regs::Sel52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Crossbar A Select Register 53"]
    #[inline(always)]
    pub const fn sel53(self) -> crate::common::Reg<regs::Sel53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(106usize) as _) }
    }
    #[doc = "Crossbar A Select Register 54"]
    #[inline(always)]
    pub const fn sel54(self) -> crate::common::Reg<regs::Sel54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Crossbar A Select Register 55"]
    #[inline(always)]
    pub const fn sel55(self) -> crate::common::Reg<regs::Sel55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(110usize) as _) }
    }
    #[doc = "Crossbar A Select Register 56"]
    #[inline(always)]
    pub const fn sel56(self) -> crate::common::Reg<regs::Sel56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Crossbar A Select Register 57"]
    #[inline(always)]
    pub const fn sel57(self) -> crate::common::Reg<regs::Sel57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(114usize) as _) }
    }
    #[doc = "Crossbar A Select Register 58"]
    #[inline(always)]
    pub const fn sel58(self) -> crate::common::Reg<regs::Sel58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "Crossbar A Select Register 59"]
    #[inline(always)]
    pub const fn sel59(self) -> crate::common::Reg<regs::Sel59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(118usize) as _) }
    }
    #[doc = "Crossbar A Select Register 60"]
    #[inline(always)]
    pub const fn sel60(self) -> crate::common::Reg<regs::Sel60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "Crossbar A Select Register 61"]
    #[inline(always)]
    pub const fn sel61(self) -> crate::common::Reg<regs::Sel61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(122usize) as _) }
    }
    #[doc = "Crossbar A Select Register 62"]
    #[inline(always)]
    pub const fn sel62(self) -> crate::common::Reg<regs::Sel62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "Crossbar A Select Register 63"]
    #[inline(always)]
    pub const fn sel63(self) -> crate::common::Reg<regs::Sel63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(126usize) as _) }
    }
    #[doc = "Crossbar A Select Register 64"]
    #[inline(always)]
    pub const fn sel64(self) -> crate::common::Reg<regs::Sel64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Crossbar A Select Register 65"]
    #[inline(always)]
    pub const fn sel65(self) -> crate::common::Reg<regs::Sel65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(130usize) as _) }
    }
    #[doc = "Crossbar A Control Register 0"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Crossbar A Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(134usize) as _) }
    }
}
pub mod regs;
pub mod vals;
