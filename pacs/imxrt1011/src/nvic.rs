#[doc = "Nested Vectored Interrupt Controller"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Interrupt Set Enable Register n"]
    #[inline(always)]
    pub const fn nviciser3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register n"]
    #[inline(always)]
    pub const fn nvicicer3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register n"]
    #[inline(always)]
    pub const fn nvicispr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register n"]
    #[inline(always)]
    pub const fn nvicicpr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(396usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Interrupt Active bit Register n"]
    #[inline(always)]
    pub const fn nviciabr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "Interrupt Priority Register 0"]
    #[inline(always)]
    pub const fn nvicip0(self) -> crate::common::Reg<regs::Nvicip0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "Interrupt Priority Register 1"]
    #[inline(always)]
    pub const fn nvicip1(self) -> crate::common::Reg<regs::Nvicip1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(769usize) as _) }
    }
    #[doc = "Interrupt Priority Register 2"]
    #[inline(always)]
    pub const fn nvicip2(self) -> crate::common::Reg<regs::Nvicip2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(770usize) as _) }
    }
    #[doc = "Interrupt Priority Register 3"]
    #[inline(always)]
    pub const fn nvicip3(self) -> crate::common::Reg<regs::Nvicip3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(771usize) as _) }
    }
    #[doc = "Interrupt Priority Register 4"]
    #[inline(always)]
    pub const fn nvicip4(self) -> crate::common::Reg<regs::Nvicip4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize) as _) }
    }
    #[doc = "Interrupt Priority Register 5"]
    #[inline(always)]
    pub const fn nvicip5(self) -> crate::common::Reg<regs::Nvicip5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(773usize) as _) }
    }
    #[doc = "Interrupt Priority Register 6"]
    #[inline(always)]
    pub const fn nvicip6(self) -> crate::common::Reg<regs::Nvicip6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(774usize) as _) }
    }
    #[doc = "Interrupt Priority Register 7"]
    #[inline(always)]
    pub const fn nvicip7(self) -> crate::common::Reg<regs::Nvicip7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(775usize) as _) }
    }
    #[doc = "Interrupt Priority Register 8"]
    #[inline(always)]
    pub const fn nvicip8(self) -> crate::common::Reg<regs::Nvicip8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(776usize) as _) }
    }
    #[doc = "Interrupt Priority Register 9"]
    #[inline(always)]
    pub const fn nvicip9(self) -> crate::common::Reg<regs::Nvicip9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(777usize) as _) }
    }
    #[doc = "Interrupt Priority Register 10"]
    #[inline(always)]
    pub const fn nvicip10(self) -> crate::common::Reg<regs::Nvicip10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(778usize) as _) }
    }
    #[doc = "Interrupt Priority Register 11"]
    #[inline(always)]
    pub const fn nvicip11(self) -> crate::common::Reg<regs::Nvicip11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(779usize) as _) }
    }
    #[doc = "Interrupt Priority Register 12"]
    #[inline(always)]
    pub const fn nvicip12(self) -> crate::common::Reg<regs::Nvicip12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(780usize) as _) }
    }
    #[doc = "Interrupt Priority Register 13"]
    #[inline(always)]
    pub const fn nvicip13(self) -> crate::common::Reg<regs::Nvicip13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(781usize) as _) }
    }
    #[doc = "Interrupt Priority Register 14"]
    #[inline(always)]
    pub const fn nvicip14(self) -> crate::common::Reg<regs::Nvicip14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(782usize) as _) }
    }
    #[doc = "Interrupt Priority Register 15"]
    #[inline(always)]
    pub const fn nvicip15(self) -> crate::common::Reg<regs::Nvicip15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(783usize) as _) }
    }
    #[doc = "Interrupt Priority Register 16"]
    #[inline(always)]
    pub const fn nvicip16(self) -> crate::common::Reg<regs::Nvicip16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(784usize) as _) }
    }
    #[doc = "Interrupt Priority Register 17"]
    #[inline(always)]
    pub const fn nvicip17(self) -> crate::common::Reg<regs::Nvicip17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(785usize) as _) }
    }
    #[doc = "Interrupt Priority Register 18"]
    #[inline(always)]
    pub const fn nvicip18(self) -> crate::common::Reg<regs::Nvicip18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(786usize) as _) }
    }
    #[doc = "Interrupt Priority Register 19"]
    #[inline(always)]
    pub const fn nvicip19(self) -> crate::common::Reg<regs::Nvicip19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(787usize) as _) }
    }
    #[doc = "Interrupt Priority Register 20"]
    #[inline(always)]
    pub const fn nvicip20(self) -> crate::common::Reg<regs::Nvicip20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(788usize) as _) }
    }
    #[doc = "Interrupt Priority Register 21"]
    #[inline(always)]
    pub const fn nvicip21(self) -> crate::common::Reg<regs::Nvicip21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(789usize) as _) }
    }
    #[doc = "Interrupt Priority Register 22"]
    #[inline(always)]
    pub const fn nvicip22(self) -> crate::common::Reg<regs::Nvicip22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(790usize) as _) }
    }
    #[doc = "Interrupt Priority Register 23"]
    #[inline(always)]
    pub const fn nvicip23(self) -> crate::common::Reg<regs::Nvicip23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(791usize) as _) }
    }
    #[doc = "Interrupt Priority Register 24"]
    #[inline(always)]
    pub const fn nvicip24(self) -> crate::common::Reg<regs::Nvicip24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(792usize) as _) }
    }
    #[doc = "Interrupt Priority Register 25"]
    #[inline(always)]
    pub const fn nvicip25(self) -> crate::common::Reg<regs::Nvicip25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(793usize) as _) }
    }
    #[doc = "Interrupt Priority Register 26"]
    #[inline(always)]
    pub const fn nvicip26(self) -> crate::common::Reg<regs::Nvicip26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(794usize) as _) }
    }
    #[doc = "Interrupt Priority Register 27"]
    #[inline(always)]
    pub const fn nvicip27(self) -> crate::common::Reg<regs::Nvicip27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(795usize) as _) }
    }
    #[doc = "Interrupt Priority Register 28"]
    #[inline(always)]
    pub const fn nvicip28(self) -> crate::common::Reg<regs::Nvicip28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(796usize) as _) }
    }
    #[doc = "Interrupt Priority Register 29"]
    #[inline(always)]
    pub const fn nvicip29(self) -> crate::common::Reg<regs::Nvicip29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(797usize) as _) }
    }
    #[doc = "Interrupt Priority Register 30"]
    #[inline(always)]
    pub const fn nvicip30(self) -> crate::common::Reg<regs::Nvicip30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(798usize) as _) }
    }
    #[doc = "Interrupt Priority Register 31"]
    #[inline(always)]
    pub const fn nvicip31(self) -> crate::common::Reg<regs::Nvicip31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(799usize) as _) }
    }
    #[doc = "Interrupt Priority Register 32"]
    #[inline(always)]
    pub const fn nvicip32(self) -> crate::common::Reg<regs::Nvicip32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(800usize) as _) }
    }
    #[doc = "Interrupt Priority Register 33"]
    #[inline(always)]
    pub const fn nvicip33(self) -> crate::common::Reg<regs::Nvicip33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(801usize) as _) }
    }
    #[doc = "Interrupt Priority Register 34"]
    #[inline(always)]
    pub const fn nvicip34(self) -> crate::common::Reg<regs::Nvicip34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(802usize) as _) }
    }
    #[doc = "Interrupt Priority Register 35"]
    #[inline(always)]
    pub const fn nvicip35(self) -> crate::common::Reg<regs::Nvicip35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(803usize) as _) }
    }
    #[doc = "Interrupt Priority Register 36"]
    #[inline(always)]
    pub const fn nvicip36(self) -> crate::common::Reg<regs::Nvicip36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(804usize) as _) }
    }
    #[doc = "Interrupt Priority Register 37"]
    #[inline(always)]
    pub const fn nvicip37(self) -> crate::common::Reg<regs::Nvicip37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(805usize) as _) }
    }
    #[doc = "Interrupt Priority Register 38"]
    #[inline(always)]
    pub const fn nvicip38(self) -> crate::common::Reg<regs::Nvicip38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(806usize) as _) }
    }
    #[doc = "Interrupt Priority Register 39"]
    #[inline(always)]
    pub const fn nvicip39(self) -> crate::common::Reg<regs::Nvicip39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(807usize) as _) }
    }
    #[doc = "Interrupt Priority Register 40"]
    #[inline(always)]
    pub const fn nvicip40(self) -> crate::common::Reg<regs::Nvicip40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(808usize) as _) }
    }
    #[doc = "Interrupt Priority Register 41"]
    #[inline(always)]
    pub const fn nvicip41(self) -> crate::common::Reg<regs::Nvicip41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(809usize) as _) }
    }
    #[doc = "Interrupt Priority Register 42"]
    #[inline(always)]
    pub const fn nvicip42(self) -> crate::common::Reg<regs::Nvicip42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(810usize) as _) }
    }
    #[doc = "Interrupt Priority Register 43"]
    #[inline(always)]
    pub const fn nvicip43(self) -> crate::common::Reg<regs::Nvicip43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(811usize) as _) }
    }
    #[doc = "Interrupt Priority Register 44"]
    #[inline(always)]
    pub const fn nvicip44(self) -> crate::common::Reg<regs::Nvicip44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(812usize) as _) }
    }
    #[doc = "Interrupt Priority Register 45"]
    #[inline(always)]
    pub const fn nvicip45(self) -> crate::common::Reg<regs::Nvicip45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(813usize) as _) }
    }
    #[doc = "Interrupt Priority Register 46"]
    #[inline(always)]
    pub const fn nvicip46(self) -> crate::common::Reg<regs::Nvicip46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(814usize) as _) }
    }
    #[doc = "Interrupt Priority Register 47"]
    #[inline(always)]
    pub const fn nvicip47(self) -> crate::common::Reg<regs::Nvicip47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(815usize) as _) }
    }
    #[doc = "Interrupt Priority Register 48"]
    #[inline(always)]
    pub const fn nvicip48(self) -> crate::common::Reg<regs::Nvicip48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(816usize) as _) }
    }
    #[doc = "Interrupt Priority Register 49"]
    #[inline(always)]
    pub const fn nvicip49(self) -> crate::common::Reg<regs::Nvicip49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(817usize) as _) }
    }
    #[doc = "Interrupt Priority Register 50"]
    #[inline(always)]
    pub const fn nvicip50(self) -> crate::common::Reg<regs::Nvicip50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(818usize) as _) }
    }
    #[doc = "Interrupt Priority Register 51"]
    #[inline(always)]
    pub const fn nvicip51(self) -> crate::common::Reg<regs::Nvicip51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(819usize) as _) }
    }
    #[doc = "Interrupt Priority Register 52"]
    #[inline(always)]
    pub const fn nvicip52(self) -> crate::common::Reg<regs::Nvicip52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(820usize) as _) }
    }
    #[doc = "Interrupt Priority Register 53"]
    #[inline(always)]
    pub const fn nvicip53(self) -> crate::common::Reg<regs::Nvicip53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(821usize) as _) }
    }
    #[doc = "Interrupt Priority Register 54"]
    #[inline(always)]
    pub const fn nvicip54(self) -> crate::common::Reg<regs::Nvicip54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(822usize) as _) }
    }
    #[doc = "Interrupt Priority Register 55"]
    #[inline(always)]
    pub const fn nvicip55(self) -> crate::common::Reg<regs::Nvicip55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(823usize) as _) }
    }
    #[doc = "Interrupt Priority Register 56"]
    #[inline(always)]
    pub const fn nvicip56(self) -> crate::common::Reg<regs::Nvicip56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(824usize) as _) }
    }
    #[doc = "Interrupt Priority Register 57"]
    #[inline(always)]
    pub const fn nvicip57(self) -> crate::common::Reg<regs::Nvicip57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(825usize) as _) }
    }
    #[doc = "Interrupt Priority Register 58"]
    #[inline(always)]
    pub const fn nvicip58(self) -> crate::common::Reg<regs::Nvicip58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(826usize) as _) }
    }
    #[doc = "Interrupt Priority Register 59"]
    #[inline(always)]
    pub const fn nvicip59(self) -> crate::common::Reg<regs::Nvicip59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(827usize) as _) }
    }
    #[doc = "Interrupt Priority Register 60"]
    #[inline(always)]
    pub const fn nvicip60(self) -> crate::common::Reg<regs::Nvicip60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(828usize) as _) }
    }
    #[doc = "Interrupt Priority Register 61"]
    #[inline(always)]
    pub const fn nvicip61(self) -> crate::common::Reg<regs::Nvicip61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(829usize) as _) }
    }
    #[doc = "Interrupt Priority Register 62"]
    #[inline(always)]
    pub const fn nvicip62(self) -> crate::common::Reg<regs::Nvicip62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(830usize) as _) }
    }
    #[doc = "Interrupt Priority Register 63"]
    #[inline(always)]
    pub const fn nvicip63(self) -> crate::common::Reg<regs::Nvicip63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(831usize) as _) }
    }
    #[doc = "Interrupt Priority Register 64"]
    #[inline(always)]
    pub const fn nvicip64(self) -> crate::common::Reg<regs::Nvicip64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "Interrupt Priority Register 65"]
    #[inline(always)]
    pub const fn nvicip65(self) -> crate::common::Reg<regs::Nvicip65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(833usize) as _) }
    }
    #[doc = "Interrupt Priority Register 66"]
    #[inline(always)]
    pub const fn nvicip66(self) -> crate::common::Reg<regs::Nvicip66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(834usize) as _) }
    }
    #[doc = "Interrupt Priority Register 67"]
    #[inline(always)]
    pub const fn nvicip67(self) -> crate::common::Reg<regs::Nvicip67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(835usize) as _) }
    }
    #[doc = "Interrupt Priority Register 68"]
    #[inline(always)]
    pub const fn nvicip68(self) -> crate::common::Reg<regs::Nvicip68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(836usize) as _) }
    }
    #[doc = "Interrupt Priority Register 69"]
    #[inline(always)]
    pub const fn nvicip69(self) -> crate::common::Reg<regs::Nvicip69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(837usize) as _) }
    }
    #[doc = "Interrupt Priority Register 70"]
    #[inline(always)]
    pub const fn nvicip70(self) -> crate::common::Reg<regs::Nvicip70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(838usize) as _) }
    }
    #[doc = "Interrupt Priority Register 71"]
    #[inline(always)]
    pub const fn nvicip71(self) -> crate::common::Reg<regs::Nvicip71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(839usize) as _) }
    }
    #[doc = "Interrupt Priority Register 72"]
    #[inline(always)]
    pub const fn nvicip72(self) -> crate::common::Reg<regs::Nvicip72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(840usize) as _) }
    }
    #[doc = "Interrupt Priority Register 73"]
    #[inline(always)]
    pub const fn nvicip73(self) -> crate::common::Reg<regs::Nvicip73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(841usize) as _) }
    }
    #[doc = "Interrupt Priority Register 74"]
    #[inline(always)]
    pub const fn nvicip74(self) -> crate::common::Reg<regs::Nvicip74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(842usize) as _) }
    }
    #[doc = "Interrupt Priority Register 75"]
    #[inline(always)]
    pub const fn nvicip75(self) -> crate::common::Reg<regs::Nvicip75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(843usize) as _) }
    }
    #[doc = "Interrupt Priority Register 76"]
    #[inline(always)]
    pub const fn nvicip76(self) -> crate::common::Reg<regs::Nvicip76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(844usize) as _) }
    }
    #[doc = "Interrupt Priority Register 77"]
    #[inline(always)]
    pub const fn nvicip77(self) -> crate::common::Reg<regs::Nvicip77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(845usize) as _) }
    }
    #[doc = "Interrupt Priority Register 78"]
    #[inline(always)]
    pub const fn nvicip78(self) -> crate::common::Reg<regs::Nvicip78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(846usize) as _) }
    }
    #[doc = "Interrupt Priority Register 79"]
    #[inline(always)]
    pub const fn nvicip79(self) -> crate::common::Reg<regs::Nvicip79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(847usize) as _) }
    }
    #[doc = "Software Trigger Interrupt Register"]
    #[inline(always)]
    pub const fn nvicstir(self) -> crate::common::Reg<regs::Nvicstir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3584usize) as _) }
    }
}
pub mod regs;
