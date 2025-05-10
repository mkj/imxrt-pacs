#[doc = "Nested Vectored Interrupt Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvic {
    ptr: *mut u8,
}
unsafe impl Send for Nvic {}
unsafe impl Sync for Nvic {}
impl Nvic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "Interrupt Priority Register 0"]
    #[inline(always)]
    pub const fn nvicip0(self) -> crate::common::Reg<regs::Nvicip0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Interrupt Priority Register 1"]
    #[inline(always)]
    pub const fn nvicip1(self) -> crate::common::Reg<regs::Nvicip1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0301usize) as _) }
    }
    #[doc = "Interrupt Priority Register 2"]
    #[inline(always)]
    pub const fn nvicip2(self) -> crate::common::Reg<regs::Nvicip2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0302usize) as _) }
    }
    #[doc = "Interrupt Priority Register 3"]
    #[inline(always)]
    pub const fn nvicip3(self) -> crate::common::Reg<regs::Nvicip3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0303usize) as _) }
    }
    #[doc = "Interrupt Priority Register 4"]
    #[inline(always)]
    pub const fn nvicip4(self) -> crate::common::Reg<regs::Nvicip4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Priority Register 5"]
    #[inline(always)]
    pub const fn nvicip5(self) -> crate::common::Reg<regs::Nvicip5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0305usize) as _) }
    }
    #[doc = "Interrupt Priority Register 6"]
    #[inline(always)]
    pub const fn nvicip6(self) -> crate::common::Reg<regs::Nvicip6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0306usize) as _) }
    }
    #[doc = "Interrupt Priority Register 7"]
    #[inline(always)]
    pub const fn nvicip7(self) -> crate::common::Reg<regs::Nvicip7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0307usize) as _) }
    }
    #[doc = "Interrupt Priority Register 8"]
    #[inline(always)]
    pub const fn nvicip8(self) -> crate::common::Reg<regs::Nvicip8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Interrupt Priority Register 9"]
    #[inline(always)]
    pub const fn nvicip9(self) -> crate::common::Reg<regs::Nvicip9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0309usize) as _) }
    }
    #[doc = "Interrupt Priority Register 10"]
    #[inline(always)]
    pub const fn nvicip10(self) -> crate::common::Reg<regs::Nvicip10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 11"]
    #[inline(always)]
    pub const fn nvicip11(self) -> crate::common::Reg<regs::Nvicip11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030busize) as _) }
    }
    #[doc = "Interrupt Priority Register 12"]
    #[inline(always)]
    pub const fn nvicip12(self) -> crate::common::Reg<regs::Nvicip12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 13"]
    #[inline(always)]
    pub const fn nvicip13(self) -> crate::common::Reg<regs::Nvicip13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 14"]
    #[inline(always)]
    pub const fn nvicip14(self) -> crate::common::Reg<regs::Nvicip14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 15"]
    #[inline(always)]
    pub const fn nvicip15(self) -> crate::common::Reg<regs::Nvicip15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 16"]
    #[inline(always)]
    pub const fn nvicip16(self) -> crate::common::Reg<regs::Nvicip16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Interrupt Priority Register 17"]
    #[inline(always)]
    pub const fn nvicip17(self) -> crate::common::Reg<regs::Nvicip17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0311usize) as _) }
    }
    #[doc = "Interrupt Priority Register 18"]
    #[inline(always)]
    pub const fn nvicip18(self) -> crate::common::Reg<regs::Nvicip18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0312usize) as _) }
    }
    #[doc = "Interrupt Priority Register 19"]
    #[inline(always)]
    pub const fn nvicip19(self) -> crate::common::Reg<regs::Nvicip19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0313usize) as _) }
    }
    #[doc = "Interrupt Priority Register 20"]
    #[inline(always)]
    pub const fn nvicip20(self) -> crate::common::Reg<regs::Nvicip20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "Interrupt Priority Register 21"]
    #[inline(always)]
    pub const fn nvicip21(self) -> crate::common::Reg<regs::Nvicip21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0315usize) as _) }
    }
    #[doc = "Interrupt Priority Register 22"]
    #[inline(always)]
    pub const fn nvicip22(self) -> crate::common::Reg<regs::Nvicip22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0316usize) as _) }
    }
    #[doc = "Interrupt Priority Register 23"]
    #[inline(always)]
    pub const fn nvicip23(self) -> crate::common::Reg<regs::Nvicip23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0317usize) as _) }
    }
    #[doc = "Interrupt Priority Register 24"]
    #[inline(always)]
    pub const fn nvicip24(self) -> crate::common::Reg<regs::Nvicip24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[doc = "Interrupt Priority Register 25"]
    #[inline(always)]
    pub const fn nvicip25(self) -> crate::common::Reg<regs::Nvicip25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0319usize) as _) }
    }
    #[doc = "Interrupt Priority Register 26"]
    #[inline(always)]
    pub const fn nvicip26(self) -> crate::common::Reg<regs::Nvicip26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 27"]
    #[inline(always)]
    pub const fn nvicip27(self) -> crate::common::Reg<regs::Nvicip27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031busize) as _) }
    }
    #[doc = "Interrupt Priority Register 28"]
    #[inline(always)]
    pub const fn nvicip28(self) -> crate::common::Reg<regs::Nvicip28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 29"]
    #[inline(always)]
    pub const fn nvicip29(self) -> crate::common::Reg<regs::Nvicip29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 30"]
    #[inline(always)]
    pub const fn nvicip30(self) -> crate::common::Reg<regs::Nvicip30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 31"]
    #[inline(always)]
    pub const fn nvicip31(self) -> crate::common::Reg<regs::Nvicip31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 32"]
    #[inline(always)]
    pub const fn nvicip32(self) -> crate::common::Reg<regs::Nvicip32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "Interrupt Priority Register 33"]
    #[inline(always)]
    pub const fn nvicip33(self) -> crate::common::Reg<regs::Nvicip33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0321usize) as _) }
    }
    #[doc = "Interrupt Priority Register 34"]
    #[inline(always)]
    pub const fn nvicip34(self) -> crate::common::Reg<regs::Nvicip34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0322usize) as _) }
    }
    #[doc = "Interrupt Priority Register 35"]
    #[inline(always)]
    pub const fn nvicip35(self) -> crate::common::Reg<regs::Nvicip35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0323usize) as _) }
    }
    #[doc = "Interrupt Priority Register 36"]
    #[inline(always)]
    pub const fn nvicip36(self) -> crate::common::Reg<regs::Nvicip36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "Interrupt Priority Register 37"]
    #[inline(always)]
    pub const fn nvicip37(self) -> crate::common::Reg<regs::Nvicip37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0325usize) as _) }
    }
    #[doc = "Interrupt Priority Register 38"]
    #[inline(always)]
    pub const fn nvicip38(self) -> crate::common::Reg<regs::Nvicip38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0326usize) as _) }
    }
    #[doc = "Interrupt Priority Register 39"]
    #[inline(always)]
    pub const fn nvicip39(self) -> crate::common::Reg<regs::Nvicip39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0327usize) as _) }
    }
    #[doc = "Interrupt Priority Register 40"]
    #[inline(always)]
    pub const fn nvicip40(self) -> crate::common::Reg<regs::Nvicip40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "Interrupt Priority Register 41"]
    #[inline(always)]
    pub const fn nvicip41(self) -> crate::common::Reg<regs::Nvicip41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0329usize) as _) }
    }
    #[doc = "Interrupt Priority Register 42"]
    #[inline(always)]
    pub const fn nvicip42(self) -> crate::common::Reg<regs::Nvicip42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 43"]
    #[inline(always)]
    pub const fn nvicip43(self) -> crate::common::Reg<regs::Nvicip43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032busize) as _) }
    }
    #[doc = "Interrupt Priority Register 44"]
    #[inline(always)]
    pub const fn nvicip44(self) -> crate::common::Reg<regs::Nvicip44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 45"]
    #[inline(always)]
    pub const fn nvicip45(self) -> crate::common::Reg<regs::Nvicip45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 46"]
    #[inline(always)]
    pub const fn nvicip46(self) -> crate::common::Reg<regs::Nvicip46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 47"]
    #[inline(always)]
    pub const fn nvicip47(self) -> crate::common::Reg<regs::Nvicip47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 48"]
    #[inline(always)]
    pub const fn nvicip48(self) -> crate::common::Reg<regs::Nvicip48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[doc = "Interrupt Priority Register 49"]
    #[inline(always)]
    pub const fn nvicip49(self) -> crate::common::Reg<regs::Nvicip49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0331usize) as _) }
    }
    #[doc = "Interrupt Priority Register 50"]
    #[inline(always)]
    pub const fn nvicip50(self) -> crate::common::Reg<regs::Nvicip50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0332usize) as _) }
    }
    #[doc = "Interrupt Priority Register 51"]
    #[inline(always)]
    pub const fn nvicip51(self) -> crate::common::Reg<regs::Nvicip51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0333usize) as _) }
    }
    #[doc = "Interrupt Priority Register 52"]
    #[inline(always)]
    pub const fn nvicip52(self) -> crate::common::Reg<regs::Nvicip52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[doc = "Interrupt Priority Register 53"]
    #[inline(always)]
    pub const fn nvicip53(self) -> crate::common::Reg<regs::Nvicip53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0335usize) as _) }
    }
    #[doc = "Interrupt Priority Register 54"]
    #[inline(always)]
    pub const fn nvicip54(self) -> crate::common::Reg<regs::Nvicip54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0336usize) as _) }
    }
    #[doc = "Interrupt Priority Register 55"]
    #[inline(always)]
    pub const fn nvicip55(self) -> crate::common::Reg<regs::Nvicip55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0337usize) as _) }
    }
    #[doc = "Interrupt Priority Register 56"]
    #[inline(always)]
    pub const fn nvicip56(self) -> crate::common::Reg<regs::Nvicip56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[doc = "Interrupt Priority Register 57"]
    #[inline(always)]
    pub const fn nvicip57(self) -> crate::common::Reg<regs::Nvicip57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0339usize) as _) }
    }
    #[doc = "Interrupt Priority Register 58"]
    #[inline(always)]
    pub const fn nvicip58(self) -> crate::common::Reg<regs::Nvicip58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 59"]
    #[inline(always)]
    pub const fn nvicip59(self) -> crate::common::Reg<regs::Nvicip59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033busize) as _) }
    }
    #[doc = "Interrupt Priority Register 60"]
    #[inline(always)]
    pub const fn nvicip60(self) -> crate::common::Reg<regs::Nvicip60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 61"]
    #[inline(always)]
    pub const fn nvicip61(self) -> crate::common::Reg<regs::Nvicip61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 62"]
    #[inline(always)]
    pub const fn nvicip62(self) -> crate::common::Reg<regs::Nvicip62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 63"]
    #[inline(always)]
    pub const fn nvicip63(self) -> crate::common::Reg<regs::Nvicip63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 64"]
    #[inline(always)]
    pub const fn nvicip64(self) -> crate::common::Reg<regs::Nvicip64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "Interrupt Priority Register 65"]
    #[inline(always)]
    pub const fn nvicip65(self) -> crate::common::Reg<regs::Nvicip65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0341usize) as _) }
    }
    #[doc = "Interrupt Priority Register 66"]
    #[inline(always)]
    pub const fn nvicip66(self) -> crate::common::Reg<regs::Nvicip66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0342usize) as _) }
    }
    #[doc = "Interrupt Priority Register 67"]
    #[inline(always)]
    pub const fn nvicip67(self) -> crate::common::Reg<regs::Nvicip67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0343usize) as _) }
    }
    #[doc = "Interrupt Priority Register 68"]
    #[inline(always)]
    pub const fn nvicip68(self) -> crate::common::Reg<regs::Nvicip68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "Interrupt Priority Register 69"]
    #[inline(always)]
    pub const fn nvicip69(self) -> crate::common::Reg<regs::Nvicip69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0345usize) as _) }
    }
    #[doc = "Interrupt Priority Register 70"]
    #[inline(always)]
    pub const fn nvicip70(self) -> crate::common::Reg<regs::Nvicip70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0346usize) as _) }
    }
    #[doc = "Interrupt Priority Register 71"]
    #[inline(always)]
    pub const fn nvicip71(self) -> crate::common::Reg<regs::Nvicip71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0347usize) as _) }
    }
    #[doc = "Interrupt Priority Register 72"]
    #[inline(always)]
    pub const fn nvicip72(self) -> crate::common::Reg<regs::Nvicip72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
    }
    #[doc = "Interrupt Priority Register 73"]
    #[inline(always)]
    pub const fn nvicip73(self) -> crate::common::Reg<regs::Nvicip73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0349usize) as _) }
    }
    #[doc = "Interrupt Priority Register 74"]
    #[inline(always)]
    pub const fn nvicip74(self) -> crate::common::Reg<regs::Nvicip74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 75"]
    #[inline(always)]
    pub const fn nvicip75(self) -> crate::common::Reg<regs::Nvicip75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034busize) as _) }
    }
    #[doc = "Interrupt Priority Register 76"]
    #[inline(always)]
    pub const fn nvicip76(self) -> crate::common::Reg<regs::Nvicip76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 77"]
    #[inline(always)]
    pub const fn nvicip77(self) -> crate::common::Reg<regs::Nvicip77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 78"]
    #[inline(always)]
    pub const fn nvicip78(self) -> crate::common::Reg<regs::Nvicip78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 79"]
    #[inline(always)]
    pub const fn nvicip79(self) -> crate::common::Reg<regs::Nvicip79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 80"]
    #[inline(always)]
    pub const fn nvicip80(self) -> crate::common::Reg<regs::Nvicip80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0350usize) as _) }
    }
    #[doc = "Interrupt Priority Register 81"]
    #[inline(always)]
    pub const fn nvicip81(self) -> crate::common::Reg<regs::Nvicip81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0351usize) as _) }
    }
    #[doc = "Interrupt Priority Register 82"]
    #[inline(always)]
    pub const fn nvicip82(self) -> crate::common::Reg<regs::Nvicip82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0352usize) as _) }
    }
    #[doc = "Interrupt Priority Register 83"]
    #[inline(always)]
    pub const fn nvicip83(self) -> crate::common::Reg<regs::Nvicip83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0353usize) as _) }
    }
    #[doc = "Interrupt Priority Register 84"]
    #[inline(always)]
    pub const fn nvicip84(self) -> crate::common::Reg<regs::Nvicip84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0354usize) as _) }
    }
    #[doc = "Interrupt Priority Register 85"]
    #[inline(always)]
    pub const fn nvicip85(self) -> crate::common::Reg<regs::Nvicip85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0355usize) as _) }
    }
    #[doc = "Interrupt Priority Register 86"]
    #[inline(always)]
    pub const fn nvicip86(self) -> crate::common::Reg<regs::Nvicip86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0356usize) as _) }
    }
    #[doc = "Interrupt Priority Register 87"]
    #[inline(always)]
    pub const fn nvicip87(self) -> crate::common::Reg<regs::Nvicip87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0357usize) as _) }
    }
    #[doc = "Interrupt Priority Register 88"]
    #[inline(always)]
    pub const fn nvicip88(self) -> crate::common::Reg<regs::Nvicip88, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0358usize) as _) }
    }
    #[doc = "Interrupt Priority Register 89"]
    #[inline(always)]
    pub const fn nvicip89(self) -> crate::common::Reg<regs::Nvicip89, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0359usize) as _) }
    }
    #[doc = "Interrupt Priority Register 90"]
    #[inline(always)]
    pub const fn nvicip90(self) -> crate::common::Reg<regs::Nvicip90, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 91"]
    #[inline(always)]
    pub const fn nvicip91(self) -> crate::common::Reg<regs::Nvicip91, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035busize) as _) }
    }
    #[doc = "Interrupt Priority Register 92"]
    #[inline(always)]
    pub const fn nvicip92(self) -> crate::common::Reg<regs::Nvicip92, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 93"]
    #[inline(always)]
    pub const fn nvicip93(self) -> crate::common::Reg<regs::Nvicip93, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 94"]
    #[inline(always)]
    pub const fn nvicip94(self) -> crate::common::Reg<regs::Nvicip94, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 95"]
    #[inline(always)]
    pub const fn nvicip95(self) -> crate::common::Reg<regs::Nvicip95, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 96"]
    #[inline(always)]
    pub const fn nvicip96(self) -> crate::common::Reg<regs::Nvicip96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0360usize) as _) }
    }
    #[doc = "Interrupt Priority Register 97"]
    #[inline(always)]
    pub const fn nvicip97(self) -> crate::common::Reg<regs::Nvicip97, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0361usize) as _) }
    }
    #[doc = "Interrupt Priority Register 98"]
    #[inline(always)]
    pub const fn nvicip98(self) -> crate::common::Reg<regs::Nvicip98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0362usize) as _) }
    }
    #[doc = "Interrupt Priority Register 99"]
    #[inline(always)]
    pub const fn nvicip99(self) -> crate::common::Reg<regs::Nvicip99, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0363usize) as _) }
    }
    #[doc = "Interrupt Priority Register 100"]
    #[inline(always)]
    pub const fn nvicip100(self) -> crate::common::Reg<regs::Nvicip100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0364usize) as _) }
    }
    #[doc = "Interrupt Priority Register 101"]
    #[inline(always)]
    pub const fn nvicip101(self) -> crate::common::Reg<regs::Nvicip101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0365usize) as _) }
    }
    #[doc = "Interrupt Priority Register 102"]
    #[inline(always)]
    pub const fn nvicip102(self) -> crate::common::Reg<regs::Nvicip102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0366usize) as _) }
    }
    #[doc = "Interrupt Priority Register 103"]
    #[inline(always)]
    pub const fn nvicip103(self) -> crate::common::Reg<regs::Nvicip103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0367usize) as _) }
    }
    #[doc = "Interrupt Priority Register 104"]
    #[inline(always)]
    pub const fn nvicip104(self) -> crate::common::Reg<regs::Nvicip104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0368usize) as _) }
    }
    #[doc = "Interrupt Priority Register 105"]
    #[inline(always)]
    pub const fn nvicip105(self) -> crate::common::Reg<regs::Nvicip105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0369usize) as _) }
    }
    #[doc = "Interrupt Priority Register 106"]
    #[inline(always)]
    pub const fn nvicip106(self) -> crate::common::Reg<regs::Nvicip106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 107"]
    #[inline(always)]
    pub const fn nvicip107(self) -> crate::common::Reg<regs::Nvicip107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036busize) as _) }
    }
    #[doc = "Interrupt Priority Register 108"]
    #[inline(always)]
    pub const fn nvicip108(self) -> crate::common::Reg<regs::Nvicip108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 109"]
    #[inline(always)]
    pub const fn nvicip109(self) -> crate::common::Reg<regs::Nvicip109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 110"]
    #[inline(always)]
    pub const fn nvicip110(self) -> crate::common::Reg<regs::Nvicip110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 111"]
    #[inline(always)]
    pub const fn nvicip111(self) -> crate::common::Reg<regs::Nvicip111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 112"]
    #[inline(always)]
    pub const fn nvicip112(self) -> crate::common::Reg<regs::Nvicip112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[doc = "Interrupt Priority Register 113"]
    #[inline(always)]
    pub const fn nvicip113(self) -> crate::common::Reg<regs::Nvicip113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0371usize) as _) }
    }
    #[doc = "Interrupt Priority Register 114"]
    #[inline(always)]
    pub const fn nvicip114(self) -> crate::common::Reg<regs::Nvicip114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0372usize) as _) }
    }
    #[doc = "Interrupt Priority Register 115"]
    #[inline(always)]
    pub const fn nvicip115(self) -> crate::common::Reg<regs::Nvicip115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0373usize) as _) }
    }
    #[doc = "Interrupt Priority Register 116"]
    #[inline(always)]
    pub const fn nvicip116(self) -> crate::common::Reg<regs::Nvicip116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0374usize) as _) }
    }
    #[doc = "Interrupt Priority Register 117"]
    #[inline(always)]
    pub const fn nvicip117(self) -> crate::common::Reg<regs::Nvicip117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0375usize) as _) }
    }
    #[doc = "Interrupt Priority Register 118"]
    #[inline(always)]
    pub const fn nvicip118(self) -> crate::common::Reg<regs::Nvicip118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0376usize) as _) }
    }
    #[doc = "Interrupt Priority Register 119"]
    #[inline(always)]
    pub const fn nvicip119(self) -> crate::common::Reg<regs::Nvicip119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0377usize) as _) }
    }
    #[doc = "Interrupt Priority Register 120"]
    #[inline(always)]
    pub const fn nvicip120(self) -> crate::common::Reg<regs::Nvicip120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[doc = "Interrupt Priority Register 121"]
    #[inline(always)]
    pub const fn nvicip121(self) -> crate::common::Reg<regs::Nvicip121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0379usize) as _) }
    }
    #[doc = "Interrupt Priority Register 122"]
    #[inline(always)]
    pub const fn nvicip122(self) -> crate::common::Reg<regs::Nvicip122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 123"]
    #[inline(always)]
    pub const fn nvicip123(self) -> crate::common::Reg<regs::Nvicip123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037busize) as _) }
    }
    #[doc = "Interrupt Priority Register 124"]
    #[inline(always)]
    pub const fn nvicip124(self) -> crate::common::Reg<regs::Nvicip124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 125"]
    #[inline(always)]
    pub const fn nvicip125(self) -> crate::common::Reg<regs::Nvicip125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 126"]
    #[inline(always)]
    pub const fn nvicip126(self) -> crate::common::Reg<regs::Nvicip126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 127"]
    #[inline(always)]
    pub const fn nvicip127(self) -> crate::common::Reg<regs::Nvicip127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 128"]
    #[inline(always)]
    pub const fn nvicip128(self) -> crate::common::Reg<regs::Nvicip128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[doc = "Interrupt Priority Register 129"]
    #[inline(always)]
    pub const fn nvicip129(self) -> crate::common::Reg<regs::Nvicip129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0381usize) as _) }
    }
    #[doc = "Interrupt Priority Register 130"]
    #[inline(always)]
    pub const fn nvicip130(self) -> crate::common::Reg<regs::Nvicip130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0382usize) as _) }
    }
    #[doc = "Interrupt Priority Register 131"]
    #[inline(always)]
    pub const fn nvicip131(self) -> crate::common::Reg<regs::Nvicip131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0383usize) as _) }
    }
    #[doc = "Interrupt Priority Register 132"]
    #[inline(always)]
    pub const fn nvicip132(self) -> crate::common::Reg<regs::Nvicip132, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[doc = "Interrupt Priority Register 133"]
    #[inline(always)]
    pub const fn nvicip133(self) -> crate::common::Reg<regs::Nvicip133, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0385usize) as _) }
    }
    #[doc = "Interrupt Priority Register 134"]
    #[inline(always)]
    pub const fn nvicip134(self) -> crate::common::Reg<regs::Nvicip134, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0386usize) as _) }
    }
    #[doc = "Interrupt Priority Register 135"]
    #[inline(always)]
    pub const fn nvicip135(self) -> crate::common::Reg<regs::Nvicip135, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0387usize) as _) }
    }
    #[doc = "Interrupt Priority Register 136"]
    #[inline(always)]
    pub const fn nvicip136(self) -> crate::common::Reg<regs::Nvicip136, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[doc = "Interrupt Priority Register 137"]
    #[inline(always)]
    pub const fn nvicip137(self) -> crate::common::Reg<regs::Nvicip137, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0389usize) as _) }
    }
    #[doc = "Interrupt Priority Register 138"]
    #[inline(always)]
    pub const fn nvicip138(self) -> crate::common::Reg<regs::Nvicip138, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 139"]
    #[inline(always)]
    pub const fn nvicip139(self) -> crate::common::Reg<regs::Nvicip139, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038busize) as _) }
    }
    #[doc = "Interrupt Priority Register 140"]
    #[inline(always)]
    pub const fn nvicip140(self) -> crate::common::Reg<regs::Nvicip140, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 141"]
    #[inline(always)]
    pub const fn nvicip141(self) -> crate::common::Reg<regs::Nvicip141, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038dusize) as _) }
    }
    #[doc = "Interrupt Priority Register 142"]
    #[inline(always)]
    pub const fn nvicip142(self) -> crate::common::Reg<regs::Nvicip142, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038eusize) as _) }
    }
    #[doc = "Interrupt Priority Register 143"]
    #[inline(always)]
    pub const fn nvicip143(self) -> crate::common::Reg<regs::Nvicip143, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038fusize) as _) }
    }
    #[doc = "Interrupt Priority Register 144"]
    #[inline(always)]
    pub const fn nvicip144(self) -> crate::common::Reg<regs::Nvicip144, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize) as _) }
    }
    #[doc = "Interrupt Priority Register 145"]
    #[inline(always)]
    pub const fn nvicip145(self) -> crate::common::Reg<regs::Nvicip145, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0391usize) as _) }
    }
    #[doc = "Interrupt Priority Register 146"]
    #[inline(always)]
    pub const fn nvicip146(self) -> crate::common::Reg<regs::Nvicip146, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0392usize) as _) }
    }
    #[doc = "Interrupt Priority Register 147"]
    #[inline(always)]
    pub const fn nvicip147(self) -> crate::common::Reg<regs::Nvicip147, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0393usize) as _) }
    }
    #[doc = "Interrupt Priority Register 148"]
    #[inline(always)]
    pub const fn nvicip148(self) -> crate::common::Reg<regs::Nvicip148, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[doc = "Interrupt Priority Register 149"]
    #[inline(always)]
    pub const fn nvicip149(self) -> crate::common::Reg<regs::Nvicip149, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0395usize) as _) }
    }
    #[doc = "Interrupt Priority Register 150"]
    #[inline(always)]
    pub const fn nvicip150(self) -> crate::common::Reg<regs::Nvicip150, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0396usize) as _) }
    }
    #[doc = "Interrupt Priority Register 151"]
    #[inline(always)]
    pub const fn nvicip151(self) -> crate::common::Reg<regs::Nvicip151, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0397usize) as _) }
    }
    #[doc = "Interrupt Priority Register 152"]
    #[inline(always)]
    pub const fn nvicip152(self) -> crate::common::Reg<regs::Nvicip152, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0398usize) as _) }
    }
    #[doc = "Interrupt Priority Register 153"]
    #[inline(always)]
    pub const fn nvicip153(self) -> crate::common::Reg<regs::Nvicip153, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0399usize) as _) }
    }
    #[doc = "Interrupt Priority Register 154"]
    #[inline(always)]
    pub const fn nvicip154(self) -> crate::common::Reg<regs::Nvicip154, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039ausize) as _) }
    }
    #[doc = "Interrupt Priority Register 155"]
    #[inline(always)]
    pub const fn nvicip155(self) -> crate::common::Reg<regs::Nvicip155, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039busize) as _) }
    }
    #[doc = "Interrupt Priority Register 156"]
    #[inline(always)]
    pub const fn nvicip156(self) -> crate::common::Reg<regs::Nvicip156, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039cusize) as _) }
    }
    #[doc = "Interrupt Priority Register 157"]
    #[inline(always)]
    pub const fn nvicip157(self) -> crate::common::Reg<regs::Nvicip157, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039dusize) as _) }
    }
    #[doc = "Software Trigger Interrupt Register"]
    #[inline(always)]
    pub const fn nvicstir(self) -> crate::common::Reg<regs::Nvicstir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize) as _) }
    }
}
pub mod regs;
