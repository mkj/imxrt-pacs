#[doc = "Temperature Monitor"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempmon {
    ptr: *mut u8,
}
unsafe impl Send for Tempmon {}
unsafe impl Sync for Tempmon {}
impl Tempmon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0(self) -> crate::common::Reg<regs::Tempsense0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0_set(
        self,
    ) -> crate::common::Reg<regs::Tempsense0set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0_clr(
        self,
    ) -> crate::common::Reg<regs::Tempsense0clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "Tempsensor Control Register 0"]
    #[inline(always)]
    pub const fn tempsense0_tog(
        self,
    ) -> crate::common::Reg<regs::Tempsense0tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(396usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1(self) -> crate::common::Reg<regs::Tempsense1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(400usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1_set(
        self,
    ) -> crate::common::Reg<regs::Tempsense1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(404usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1_clr(
        self,
    ) -> crate::common::Reg<regs::Tempsense1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(408usize) as _) }
    }
    #[doc = "Tempsensor Control Register 1"]
    #[inline(always)]
    pub const fn tempsense1_tog(
        self,
    ) -> crate::common::Reg<regs::Tempsense1tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(412usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2(self) -> crate::common::Reg<regs::Tempsense2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2_set(
        self,
    ) -> crate::common::Reg<regs::Tempsense2set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(660usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2_clr(
        self,
    ) -> crate::common::Reg<regs::Tempsense2clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(664usize) as _) }
    }
    #[doc = "Tempsensor Control Register 2"]
    #[inline(always)]
    pub const fn tempsense2_tog(
        self,
    ) -> crate::common::Reg<regs::Tempsense2tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(668usize) as _) }
    }
}
pub mod regs;
