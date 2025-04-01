#[doc = "Interrupt Priority Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip0(pub u32);
impl Nvicip0 {
    #[doc = "Priority of the INT_DMA0 interrupt 0"]
    #[inline(always)]
    pub const fn pri0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA0 interrupt 0"]
    #[inline(always)]
    pub const fn set_pri0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip0 {
    #[inline(always)]
    fn default() -> Nvicip0 {
        Nvicip0(0)
    }
}
#[doc = "Interrupt Priority Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip1(pub u32);
impl Nvicip1 {
    #[doc = "Priority of the INT_DMA1 interrupt 1"]
    #[inline(always)]
    pub const fn pri1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA1 interrupt 1"]
    #[inline(always)]
    pub const fn set_pri1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip1 {
    #[inline(always)]
    fn default() -> Nvicip1 {
        Nvicip1(0)
    }
}
#[doc = "Interrupt Priority Register 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip10(pub u32);
impl Nvicip10 {
    #[doc = "Priority of the INT_DMA10 interrupt 10"]
    #[inline(always)]
    pub const fn pri10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA10 interrupt 10"]
    #[inline(always)]
    pub const fn set_pri10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip10 {
    #[inline(always)]
    fn default() -> Nvicip10 {
        Nvicip10(0)
    }
}
#[doc = "Interrupt Priority Register 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip11(pub u32);
impl Nvicip11 {
    #[doc = "Priority of the INT_DMA11 interrupt 11"]
    #[inline(always)]
    pub const fn pri11(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA11 interrupt 11"]
    #[inline(always)]
    pub const fn set_pri11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip11 {
    #[inline(always)]
    fn default() -> Nvicip11 {
        Nvicip11(0)
    }
}
#[doc = "Interrupt Priority Register 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip12(pub u32);
impl Nvicip12 {
    #[doc = "Priority of the INT_DMA12 interrupt 12"]
    #[inline(always)]
    pub const fn pri12(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA12 interrupt 12"]
    #[inline(always)]
    pub const fn set_pri12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip12 {
    #[inline(always)]
    fn default() -> Nvicip12 {
        Nvicip12(0)
    }
}
#[doc = "Interrupt Priority Register 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip13(pub u32);
impl Nvicip13 {
    #[doc = "Priority of the INT_DMA13 interrupt 13"]
    #[inline(always)]
    pub const fn pri13(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA13 interrupt 13"]
    #[inline(always)]
    pub const fn set_pri13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip13 {
    #[inline(always)]
    fn default() -> Nvicip13 {
        Nvicip13(0)
    }
}
#[doc = "Interrupt Priority Register 14"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip14(pub u32);
impl Nvicip14 {
    #[doc = "Priority of the INT_DMA14 interrupt 14"]
    #[inline(always)]
    pub const fn pri14(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA14 interrupt 14"]
    #[inline(always)]
    pub const fn set_pri14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip14 {
    #[inline(always)]
    fn default() -> Nvicip14 {
        Nvicip14(0)
    }
}
#[doc = "Interrupt Priority Register 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip15(pub u32);
impl Nvicip15 {
    #[doc = "Priority of the INT_DMA15 interrupt 15"]
    #[inline(always)]
    pub const fn pri15(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA15 interrupt 15"]
    #[inline(always)]
    pub const fn set_pri15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip15 {
    #[inline(always)]
    fn default() -> Nvicip15 {
        Nvicip15(0)
    }
}
#[doc = "Interrupt Priority Register 16"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip16(pub u32);
impl Nvicip16 {
    #[doc = "Priority of the INT_DMA_ERROR interrupt 16"]
    #[inline(always)]
    pub const fn pri16(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA_ERROR interrupt 16"]
    #[inline(always)]
    pub const fn set_pri16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip16 {
    #[inline(always)]
    fn default() -> Nvicip16 {
        Nvicip16(0)
    }
}
#[doc = "Interrupt Priority Register 17"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip17(pub u32);
impl Nvicip17 {
    #[doc = "Priority of the INT_CTI0_ERROR interrupt 17"]
    #[inline(always)]
    pub const fn pri17(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CTI0_ERROR interrupt 17"]
    #[inline(always)]
    pub const fn set_pri17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip17 {
    #[inline(always)]
    fn default() -> Nvicip17 {
        Nvicip17(0)
    }
}
#[doc = "Interrupt Priority Register 18"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip18(pub u32);
impl Nvicip18 {
    #[doc = "Priority of the INT_CTI1_ERROR interrupt 18"]
    #[inline(always)]
    pub const fn pri18(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CTI1_ERROR interrupt 18"]
    #[inline(always)]
    pub const fn set_pri18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip18 {
    #[inline(always)]
    fn default() -> Nvicip18 {
        Nvicip18(0)
    }
}
#[doc = "Interrupt Priority Register 19"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip19(pub u32);
impl Nvicip19 {
    #[doc = "Priority of the INT_CORE interrupt 19"]
    #[inline(always)]
    pub const fn pri19(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CORE interrupt 19"]
    #[inline(always)]
    pub const fn set_pri19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip19 {
    #[inline(always)]
    fn default() -> Nvicip19 {
        Nvicip19(0)
    }
}
#[doc = "Interrupt Priority Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip2(pub u32);
impl Nvicip2 {
    #[doc = "Priority of the INT_DMA2 interrupt 2"]
    #[inline(always)]
    pub const fn pri2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA2 interrupt 2"]
    #[inline(always)]
    pub const fn set_pri2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip2 {
    #[inline(always)]
    fn default() -> Nvicip2 {
        Nvicip2(0)
    }
}
#[doc = "Interrupt Priority Register 20"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip20(pub u32);
impl Nvicip20 {
    #[doc = "Priority of the INT_LPUART1 interrupt 20"]
    #[inline(always)]
    pub const fn pri20(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART1 interrupt 20"]
    #[inline(always)]
    pub const fn set_pri20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip20 {
    #[inline(always)]
    fn default() -> Nvicip20 {
        Nvicip20(0)
    }
}
#[doc = "Interrupt Priority Register 21"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip21(pub u32);
impl Nvicip21 {
    #[doc = "Priority of the INT_LPUART2 interrupt 21"]
    #[inline(always)]
    pub const fn pri21(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART2 interrupt 21"]
    #[inline(always)]
    pub const fn set_pri21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip21 {
    #[inline(always)]
    fn default() -> Nvicip21 {
        Nvicip21(0)
    }
}
#[doc = "Interrupt Priority Register 22"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip22(pub u32);
impl Nvicip22 {
    #[doc = "Priority of the INT_LPUART3 interrupt 22"]
    #[inline(always)]
    pub const fn pri22(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART3 interrupt 22"]
    #[inline(always)]
    pub const fn set_pri22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip22 {
    #[inline(always)]
    fn default() -> Nvicip22 {
        Nvicip22(0)
    }
}
#[doc = "Interrupt Priority Register 23"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip23(pub u32);
impl Nvicip23 {
    #[doc = "Priority of the INT_LPUART4 interrupt 23"]
    #[inline(always)]
    pub const fn pri23(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPUART4 interrupt 23"]
    #[inline(always)]
    pub const fn set_pri23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip23 {
    #[inline(always)]
    fn default() -> Nvicip23 {
        Nvicip23(0)
    }
}
#[doc = "Interrupt Priority Register 24"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip24(pub u32);
impl Nvicip24 {
    #[doc = "Priority of the INT_PIT interrupt 24"]
    #[inline(always)]
    pub const fn pri24(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PIT interrupt 24"]
    #[inline(always)]
    pub const fn set_pri24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip24 {
    #[inline(always)]
    fn default() -> Nvicip24 {
        Nvicip24(0)
    }
}
#[doc = "Interrupt Priority Register 25"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip25(pub u32);
impl Nvicip25 {
    #[doc = "Priority of the INT_USB_OTG1 interrupt 25"]
    #[inline(always)]
    pub const fn pri25(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USB_OTG1 interrupt 25"]
    #[inline(always)]
    pub const fn set_pri25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip25 {
    #[inline(always)]
    fn default() -> Nvicip25 {
        Nvicip25(0)
    }
}
#[doc = "Interrupt Priority Register 26"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip26(pub u32);
impl Nvicip26 {
    #[doc = "Priority of the INT_FLEXSPI interrupt 26"]
    #[inline(always)]
    pub const fn pri26(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXSPI interrupt 26"]
    #[inline(always)]
    pub const fn set_pri26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip26 {
    #[inline(always)]
    fn default() -> Nvicip26 {
        Nvicip26(0)
    }
}
#[doc = "Interrupt Priority Register 27"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip27(pub u32);
impl Nvicip27 {
    #[doc = "Priority of the INT_FLEXRAM interrupt 27"]
    #[inline(always)]
    pub const fn pri27(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXRAM interrupt 27"]
    #[inline(always)]
    pub const fn set_pri27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip27 {
    #[inline(always)]
    fn default() -> Nvicip27 {
        Nvicip27(0)
    }
}
#[doc = "Interrupt Priority Register 28"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip28(pub u32);
impl Nvicip28 {
    #[doc = "Priority of the INT_LPI2C1 interrupt 28"]
    #[inline(always)]
    pub const fn pri28(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPI2C1 interrupt 28"]
    #[inline(always)]
    pub const fn set_pri28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip28 {
    #[inline(always)]
    fn default() -> Nvicip28 {
        Nvicip28(0)
    }
}
#[doc = "Interrupt Priority Register 29"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip29(pub u32);
impl Nvicip29 {
    #[doc = "Priority of the INT_LPI2C2 interrupt 29"]
    #[inline(always)]
    pub const fn pri29(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPI2C2 interrupt 29"]
    #[inline(always)]
    pub const fn set_pri29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip29 {
    #[inline(always)]
    fn default() -> Nvicip29 {
        Nvicip29(0)
    }
}
#[doc = "Interrupt Priority Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip3(pub u32);
impl Nvicip3 {
    #[doc = "Priority of the INT_DMA3 interrupt 3"]
    #[inline(always)]
    pub const fn pri3(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA3 interrupt 3"]
    #[inline(always)]
    pub const fn set_pri3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip3 {
    #[inline(always)]
    fn default() -> Nvicip3 {
        Nvicip3(0)
    }
}
#[doc = "Interrupt Priority Register 30"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip30(pub u32);
impl Nvicip30 {
    #[doc = "Priority of the INT_GPT1 interrupt 30"]
    #[inline(always)]
    pub const fn pri30(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPT1 interrupt 30"]
    #[inline(always)]
    pub const fn set_pri30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip30 {
    #[inline(always)]
    fn default() -> Nvicip30 {
        Nvicip30(0)
    }
}
#[doc = "Interrupt Priority Register 31"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip31(pub u32);
impl Nvicip31 {
    #[doc = "Priority of the INT_GPT2 interrupt 31"]
    #[inline(always)]
    pub const fn pri31(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPT2 interrupt 31"]
    #[inline(always)]
    pub const fn set_pri31(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip31 {
    #[inline(always)]
    fn default() -> Nvicip31 {
        Nvicip31(0)
    }
}
#[doc = "Interrupt Priority Register 32"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip32(pub u32);
impl Nvicip32 {
    #[doc = "Priority of the INT_LPSPI1 interrupt 32"]
    #[inline(always)]
    pub const fn pri32(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPSPI1 interrupt 32"]
    #[inline(always)]
    pub const fn set_pri32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip32 {
    #[inline(always)]
    fn default() -> Nvicip32 {
        Nvicip32(0)
    }
}
#[doc = "Interrupt Priority Register 33"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip33(pub u32);
impl Nvicip33 {
    #[doc = "Priority of the INT_LPSPI2 interrupt 33"]
    #[inline(always)]
    pub const fn pri33(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_LPSPI2 interrupt 33"]
    #[inline(always)]
    pub const fn set_pri33(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip33 {
    #[inline(always)]
    fn default() -> Nvicip33 {
        Nvicip33(0)
    }
}
#[doc = "Interrupt Priority Register 34"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip34(pub u32);
impl Nvicip34 {
    #[doc = "Priority of the INT_PWM1_0 interrupt 34"]
    #[inline(always)]
    pub const fn pri34(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_0 interrupt 34"]
    #[inline(always)]
    pub const fn set_pri34(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip34 {
    #[inline(always)]
    fn default() -> Nvicip34 {
        Nvicip34(0)
    }
}
#[doc = "Interrupt Priority Register 35"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip35(pub u32);
impl Nvicip35 {
    #[doc = "Priority of the INT_PWM1_1 interrupt 35"]
    #[inline(always)]
    pub const fn pri35(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_1 interrupt 35"]
    #[inline(always)]
    pub const fn set_pri35(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip35 {
    #[inline(always)]
    fn default() -> Nvicip35 {
        Nvicip35(0)
    }
}
#[doc = "Interrupt Priority Register 36"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip36(pub u32);
impl Nvicip36 {
    #[doc = "Priority of the INT_PWM1_2 interrupt 36"]
    #[inline(always)]
    pub const fn pri36(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_2 interrupt 36"]
    #[inline(always)]
    pub const fn set_pri36(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip36 {
    #[inline(always)]
    fn default() -> Nvicip36 {
        Nvicip36(0)
    }
}
#[doc = "Interrupt Priority Register 37"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip37(pub u32);
impl Nvicip37 {
    #[doc = "Priority of the INT_PWM1_3 interrupt 37"]
    #[inline(always)]
    pub const fn pri37(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_3 interrupt 37"]
    #[inline(always)]
    pub const fn set_pri37(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip37 {
    #[inline(always)]
    fn default() -> Nvicip37 {
        Nvicip37(0)
    }
}
#[doc = "Interrupt Priority Register 38"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip38(pub u32);
impl Nvicip38 {
    #[doc = "Priority of the INT_PWM1_FAULT interrupt 38"]
    #[inline(always)]
    pub const fn pri38(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PWM1_FAULT interrupt 38"]
    #[inline(always)]
    pub const fn set_pri38(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip38 {
    #[inline(always)]
    fn default() -> Nvicip38 {
        Nvicip38(0)
    }
}
#[doc = "Interrupt Priority Register 39"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip39(pub u32);
impl Nvicip39 {
    #[doc = "Priority of the INT_KPP interrupt 39"]
    #[inline(always)]
    pub const fn pri39(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_KPP interrupt 39"]
    #[inline(always)]
    pub const fn set_pri39(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip39 {
    #[inline(always)]
    fn default() -> Nvicip39 {
        Nvicip39(0)
    }
}
#[doc = "Interrupt Priority Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip4(pub u32);
impl Nvicip4 {
    #[doc = "Priority of the INT_DMA4 interrupt 4"]
    #[inline(always)]
    pub const fn pri4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA4 interrupt 4"]
    #[inline(always)]
    pub const fn set_pri4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip4 {
    #[inline(always)]
    fn default() -> Nvicip4 {
        Nvicip4(0)
    }
}
#[doc = "Interrupt Priority Register 40"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip40(pub u32);
impl Nvicip40 {
    #[doc = "Priority of the INT_SRC interrupt 40"]
    #[inline(always)]
    pub const fn pri40(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SRC interrupt 40"]
    #[inline(always)]
    pub const fn set_pri40(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip40 {
    #[inline(always)]
    fn default() -> Nvicip40 {
        Nvicip40(0)
    }
}
#[doc = "Interrupt Priority Register 41"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip41(pub u32);
impl Nvicip41 {
    #[doc = "Priority of the INT_GPR_IRQ interrupt 41"]
    #[inline(always)]
    pub const fn pri41(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPR_IRQ interrupt 41"]
    #[inline(always)]
    pub const fn set_pri41(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip41 {
    #[inline(always)]
    fn default() -> Nvicip41 {
        Nvicip41(0)
    }
}
#[doc = "Interrupt Priority Register 42"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip42(pub u32);
impl Nvicip42 {
    #[doc = "Priority of the INT_CCM_1 interrupt 42"]
    #[inline(always)]
    pub const fn pri42(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CCM_1 interrupt 42"]
    #[inline(always)]
    pub const fn set_pri42(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip42 {
    #[inline(always)]
    fn default() -> Nvicip42 {
        Nvicip42(0)
    }
}
#[doc = "Interrupt Priority Register 43"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip43(pub u32);
impl Nvicip43 {
    #[doc = "Priority of the INT_CCM_2 interrupt 43"]
    #[inline(always)]
    pub const fn pri43(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CCM_2 interrupt 43"]
    #[inline(always)]
    pub const fn set_pri43(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip43 {
    #[inline(always)]
    fn default() -> Nvicip43 {
        Nvicip43(0)
    }
}
#[doc = "Interrupt Priority Register 44"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip44(pub u32);
impl Nvicip44 {
    #[doc = "Priority of the INT_EWM interrupt 44"]
    #[inline(always)]
    pub const fn pri44(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_EWM interrupt 44"]
    #[inline(always)]
    pub const fn set_pri44(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip44 {
    #[inline(always)]
    fn default() -> Nvicip44 {
        Nvicip44(0)
    }
}
#[doc = "Interrupt Priority Register 45"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip45(pub u32);
impl Nvicip45 {
    #[doc = "Priority of the INT_WDOG2 interrupt 45"]
    #[inline(always)]
    pub const fn pri45(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_WDOG2 interrupt 45"]
    #[inline(always)]
    pub const fn set_pri45(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip45 {
    #[inline(always)]
    fn default() -> Nvicip45 {
        Nvicip45(0)
    }
}
#[doc = "Interrupt Priority Register 46"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip46(pub u32);
impl Nvicip46 {
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER interrupt 46"]
    #[inline(always)]
    pub const fn pri46(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER interrupt 46"]
    #[inline(always)]
    pub const fn set_pri46(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip46 {
    #[inline(always)]
    fn default() -> Nvicip46 {
        Nvicip46(0)
    }
}
#[doc = "Interrupt Priority Register 47"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip47(pub u32);
impl Nvicip47 {
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER_TZ interrupt 47"]
    #[inline(always)]
    pub const fn pri47(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SNVS_HP_WRAPPER_TZ interrupt 47"]
    #[inline(always)]
    pub const fn set_pri47(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip47 {
    #[inline(always)]
    fn default() -> Nvicip47 {
        Nvicip47(0)
    }
}
#[doc = "Interrupt Priority Register 48"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip48(pub u32);
impl Nvicip48 {
    #[doc = "Priority of the INT_SNVS_LP_WRAPPER interrupt 48"]
    #[inline(always)]
    pub const fn pri48(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SNVS_LP_WRAPPER interrupt 48"]
    #[inline(always)]
    pub const fn set_pri48(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip48 {
    #[inline(always)]
    fn default() -> Nvicip48 {
        Nvicip48(0)
    }
}
#[doc = "Interrupt Priority Register 49"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip49(pub u32);
impl Nvicip49 {
    #[doc = "Priority of the INT_CSU interrupt 49"]
    #[inline(always)]
    pub const fn pri49(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_CSU interrupt 49"]
    #[inline(always)]
    pub const fn set_pri49(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip49 {
    #[inline(always)]
    fn default() -> Nvicip49 {
        Nvicip49(0)
    }
}
#[doc = "Interrupt Priority Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip5(pub u32);
impl Nvicip5 {
    #[doc = "Priority of the INT_DMA5 interrupt 5"]
    #[inline(always)]
    pub const fn pri5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA5 interrupt 5"]
    #[inline(always)]
    pub const fn set_pri5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip5 {
    #[inline(always)]
    fn default() -> Nvicip5 {
        Nvicip5(0)
    }
}
#[doc = "Interrupt Priority Register 50"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip50(pub u32);
impl Nvicip50 {
    #[doc = "Priority of the INT_DCP interrupt 50"]
    #[inline(always)]
    pub const fn pri50(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DCP interrupt 50"]
    #[inline(always)]
    pub const fn set_pri50(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip50 {
    #[inline(always)]
    fn default() -> Nvicip50 {
        Nvicip50(0)
    }
}
#[doc = "Interrupt Priority Register 51"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip51(pub u32);
impl Nvicip51 {
    #[doc = "Priority of the INT_DCP_VMI interrupt 51"]
    #[inline(always)]
    pub const fn pri51(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DCP_VMI interrupt 51"]
    #[inline(always)]
    pub const fn set_pri51(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip51 {
    #[inline(always)]
    fn default() -> Nvicip51 {
        Nvicip51(0)
    }
}
#[doc = "Interrupt Priority Register 52"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip52(pub u32);
impl Nvicip52 {
    #[doc = "Priority of the INT_Reserved68 interrupt 52"]
    #[inline(always)]
    pub const fn pri52(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved68 interrupt 52"]
    #[inline(always)]
    pub const fn set_pri52(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip52 {
    #[inline(always)]
    fn default() -> Nvicip52 {
        Nvicip52(0)
    }
}
#[doc = "Interrupt Priority Register 53"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip53(pub u32);
impl Nvicip53 {
    #[doc = "Priority of the INT_TRNG interrupt 53"]
    #[inline(always)]
    pub const fn pri53(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TRNG interrupt 53"]
    #[inline(always)]
    pub const fn set_pri53(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip53 {
    #[inline(always)]
    fn default() -> Nvicip53 {
        Nvicip53(0)
    }
}
#[doc = "Interrupt Priority Register 54"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip54(pub u32);
impl Nvicip54 {
    #[doc = "Priority of the INT_Reserved70 interrupt 54"]
    #[inline(always)]
    pub const fn pri54(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved70 interrupt 54"]
    #[inline(always)]
    pub const fn set_pri54(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip54 {
    #[inline(always)]
    fn default() -> Nvicip54 {
        Nvicip54(0)
    }
}
#[doc = "Interrupt Priority Register 55"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip55(pub u32);
impl Nvicip55 {
    #[doc = "Priority of the INT_Reserved71 interrupt 55"]
    #[inline(always)]
    pub const fn pri55(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_Reserved71 interrupt 55"]
    #[inline(always)]
    pub const fn set_pri55(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip55 {
    #[inline(always)]
    fn default() -> Nvicip55 {
        Nvicip55(0)
    }
}
#[doc = "Interrupt Priority Register 56"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip56(pub u32);
impl Nvicip56 {
    #[doc = "Priority of the INT_SAI1 interrupt 56"]
    #[inline(always)]
    pub const fn pri56(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI1 interrupt 56"]
    #[inline(always)]
    pub const fn set_pri56(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip56 {
    #[inline(always)]
    fn default() -> Nvicip56 {
        Nvicip56(0)
    }
}
#[doc = "Interrupt Priority Register 57"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip57(pub u32);
impl Nvicip57 {
    #[doc = "Priority of the INT_RTWDOG interrupt 57"]
    #[inline(always)]
    pub const fn pri57(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_RTWDOG interrupt 57"]
    #[inline(always)]
    pub const fn set_pri57(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip57 {
    #[inline(always)]
    fn default() -> Nvicip57 {
        Nvicip57(0)
    }
}
#[doc = "Interrupt Priority Register 58"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip58(pub u32);
impl Nvicip58 {
    #[doc = "Priority of the INT_SAI3_RX interrupt 58"]
    #[inline(always)]
    pub const fn pri58(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI3_RX interrupt 58"]
    #[inline(always)]
    pub const fn set_pri58(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip58 {
    #[inline(always)]
    fn default() -> Nvicip58 {
        Nvicip58(0)
    }
}
#[doc = "Interrupt Priority Register 59"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip59(pub u32);
impl Nvicip59 {
    #[doc = "Priority of the INT_SAI3_TX interrupt 59"]
    #[inline(always)]
    pub const fn pri59(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SAI3_TX interrupt 59"]
    #[inline(always)]
    pub const fn set_pri59(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip59 {
    #[inline(always)]
    fn default() -> Nvicip59 {
        Nvicip59(0)
    }
}
#[doc = "Interrupt Priority Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip6(pub u32);
impl Nvicip6 {
    #[doc = "Priority of the INT_DMA6 interrupt 6"]
    #[inline(always)]
    pub const fn pri6(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA6 interrupt 6"]
    #[inline(always)]
    pub const fn set_pri6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip6 {
    #[inline(always)]
    fn default() -> Nvicip6 {
        Nvicip6(0)
    }
}
#[doc = "Interrupt Priority Register 60"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip60(pub u32);
impl Nvicip60 {
    #[doc = "Priority of the INT_SPDIF interrupt 60"]
    #[inline(always)]
    pub const fn pri60(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_SPDIF interrupt 60"]
    #[inline(always)]
    pub const fn set_pri60(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip60 {
    #[inline(always)]
    fn default() -> Nvicip60 {
        Nvicip60(0)
    }
}
#[doc = "Interrupt Priority Register 61"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip61(pub u32);
impl Nvicip61 {
    #[doc = "Priority of the INT_PMU interrupt 61"]
    #[inline(always)]
    pub const fn pri61(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_PMU interrupt 61"]
    #[inline(always)]
    pub const fn set_pri61(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip61 {
    #[inline(always)]
    fn default() -> Nvicip61 {
        Nvicip61(0)
    }
}
#[doc = "Interrupt Priority Register 62"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip62(pub u32);
impl Nvicip62 {
    #[doc = "Priority of the INT_XBAR1_IRQ_0_1_2_3 interrupt 62"]
    #[inline(always)]
    pub const fn pri62(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_XBAR1_IRQ_0_1_2_3 interrupt 62"]
    #[inline(always)]
    pub const fn set_pri62(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip62 {
    #[inline(always)]
    fn default() -> Nvicip62 {
        Nvicip62(0)
    }
}
#[doc = "Interrupt Priority Register 63"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip63(pub u32);
impl Nvicip63 {
    #[doc = "Priority of the INT_TEMP_LOW_HIGH interrupt 63"]
    #[inline(always)]
    pub const fn pri63(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TEMP_LOW_HIGH interrupt 63"]
    #[inline(always)]
    pub const fn set_pri63(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip63 {
    #[inline(always)]
    fn default() -> Nvicip63 {
        Nvicip63(0)
    }
}
#[doc = "Interrupt Priority Register 64"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip64(pub u32);
impl Nvicip64 {
    #[doc = "Priority of the INT_TEMP_PANIC interrupt 64"]
    #[inline(always)]
    pub const fn pri64(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_TEMP_PANIC interrupt 64"]
    #[inline(always)]
    pub const fn set_pri64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip64 {
    #[inline(always)]
    fn default() -> Nvicip64 {
        Nvicip64(0)
    }
}
#[doc = "Interrupt Priority Register 65"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip65(pub u32);
impl Nvicip65 {
    #[doc = "Priority of the INT_USB_PHY interrupt 65"]
    #[inline(always)]
    pub const fn pri65(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_USB_PHY interrupt 65"]
    #[inline(always)]
    pub const fn set_pri65(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip65 {
    #[inline(always)]
    fn default() -> Nvicip65 {
        Nvicip65(0)
    }
}
#[doc = "Interrupt Priority Register 66"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip66(pub u32);
impl Nvicip66 {
    #[doc = "Priority of the INT_GPC interrupt 66"]
    #[inline(always)]
    pub const fn pri66(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPC interrupt 66"]
    #[inline(always)]
    pub const fn set_pri66(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip66 {
    #[inline(always)]
    fn default() -> Nvicip66 {
        Nvicip66(0)
    }
}
#[doc = "Interrupt Priority Register 67"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip67(pub u32);
impl Nvicip67 {
    #[doc = "Priority of the INT_ADC1 interrupt 67"]
    #[inline(always)]
    pub const fn pri67(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC1 interrupt 67"]
    #[inline(always)]
    pub const fn set_pri67(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip67 {
    #[inline(always)]
    fn default() -> Nvicip67 {
        Nvicip67(0)
    }
}
#[doc = "Interrupt Priority Register 68"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip68(pub u32);
impl Nvicip68 {
    #[doc = "Priority of the INT_FLEXIO1 interrupt 68"]
    #[inline(always)]
    pub const fn pri68(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_FLEXIO1 interrupt 68"]
    #[inline(always)]
    pub const fn set_pri68(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip68 {
    #[inline(always)]
    fn default() -> Nvicip68 {
        Nvicip68(0)
    }
}
#[doc = "Interrupt Priority Register 69"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip69(pub u32);
impl Nvicip69 {
    #[doc = "Priority of the INT_DCDC interrupt 69"]
    #[inline(always)]
    pub const fn pri69(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DCDC interrupt 69"]
    #[inline(always)]
    pub const fn set_pri69(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip69 {
    #[inline(always)]
    fn default() -> Nvicip69 {
        Nvicip69(0)
    }
}
#[doc = "Interrupt Priority Register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip7(pub u32);
impl Nvicip7 {
    #[doc = "Priority of the INT_DMA7 interrupt 7"]
    #[inline(always)]
    pub const fn pri7(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA7 interrupt 7"]
    #[inline(always)]
    pub const fn set_pri7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip7 {
    #[inline(always)]
    fn default() -> Nvicip7 {
        Nvicip7(0)
    }
}
#[doc = "Interrupt Priority Register 70"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip70(pub u32);
impl Nvicip70 {
    #[doc = "Priority of the INT_GPIO1_Combined_0_15 interrupt 70"]
    #[inline(always)]
    pub const fn pri70(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_Combined_0_15 interrupt 70"]
    #[inline(always)]
    pub const fn set_pri70(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip70 {
    #[inline(always)]
    fn default() -> Nvicip70 {
        Nvicip70(0)
    }
}
#[doc = "Interrupt Priority Register 71"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip71(pub u32);
impl Nvicip71 {
    #[doc = "Priority of the INT_GPIO1_Combined_16_31 interrupt 71"]
    #[inline(always)]
    pub const fn pri71(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO1_Combined_16_31 interrupt 71"]
    #[inline(always)]
    pub const fn set_pri71(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip71 {
    #[inline(always)]
    fn default() -> Nvicip71 {
        Nvicip71(0)
    }
}
#[doc = "Interrupt Priority Register 72"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip72(pub u32);
impl Nvicip72 {
    #[doc = "Priority of the INT_GPIO2_Combined_0_15 interrupt 72"]
    #[inline(always)]
    pub const fn pri72(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO2_Combined_0_15 interrupt 72"]
    #[inline(always)]
    pub const fn set_pri72(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip72 {
    #[inline(always)]
    fn default() -> Nvicip72 {
        Nvicip72(0)
    }
}
#[doc = "Interrupt Priority Register 73"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip73(pub u32);
impl Nvicip73 {
    #[doc = "Priority of the INT_GPIO5_Combined_0_15 interrupt 73"]
    #[inline(always)]
    pub const fn pri73(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_GPIO5_Combined_0_15 interrupt 73"]
    #[inline(always)]
    pub const fn set_pri73(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip73 {
    #[inline(always)]
    fn default() -> Nvicip73 {
        Nvicip73(0)
    }
}
#[doc = "Interrupt Priority Register 74"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip74(pub u32);
impl Nvicip74 {
    #[doc = "Priority of the INT_WDOG1 interrupt 74"]
    #[inline(always)]
    pub const fn pri74(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_WDOG1 interrupt 74"]
    #[inline(always)]
    pub const fn set_pri74(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip74 {
    #[inline(always)]
    fn default() -> Nvicip74 {
        Nvicip74(0)
    }
}
#[doc = "Interrupt Priority Register 75"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip75(pub u32);
impl Nvicip75 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ0 interrupt 75"]
    #[inline(always)]
    pub const fn pri75(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ0 interrupt 75"]
    #[inline(always)]
    pub const fn set_pri75(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip75 {
    #[inline(always)]
    fn default() -> Nvicip75 {
        Nvicip75(0)
    }
}
#[doc = "Interrupt Priority Register 76"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip76(pub u32);
impl Nvicip76 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ1 interrupt 76"]
    #[inline(always)]
    pub const fn pri76(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ1 interrupt 76"]
    #[inline(always)]
    pub const fn set_pri76(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip76 {
    #[inline(always)]
    fn default() -> Nvicip76 {
        Nvicip76(0)
    }
}
#[doc = "Interrupt Priority Register 77"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip77(pub u32);
impl Nvicip77 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ2 interrupt 77"]
    #[inline(always)]
    pub const fn pri77(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ2 interrupt 77"]
    #[inline(always)]
    pub const fn set_pri77(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip77 {
    #[inline(always)]
    fn default() -> Nvicip77 {
        Nvicip77(0)
    }
}
#[doc = "Interrupt Priority Register 78"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip78(pub u32);
impl Nvicip78 {
    #[doc = "Priority of the INT_ADC_ETC_IRQ3 interrupt 78"]
    #[inline(always)]
    pub const fn pri78(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_IRQ3 interrupt 78"]
    #[inline(always)]
    pub const fn set_pri78(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip78 {
    #[inline(always)]
    fn default() -> Nvicip78 {
        Nvicip78(0)
    }
}
#[doc = "Interrupt Priority Register 79"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip79(pub u32);
impl Nvicip79 {
    #[doc = "Priority of the INT_ADC_ETC_ERROR_IRQ interrupt 79"]
    #[inline(always)]
    pub const fn pri79(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_ADC_ETC_ERROR_IRQ interrupt 79"]
    #[inline(always)]
    pub const fn set_pri79(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip79 {
    #[inline(always)]
    fn default() -> Nvicip79 {
        Nvicip79(0)
    }
}
#[doc = "Interrupt Priority Register 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip8(pub u32);
impl Nvicip8 {
    #[doc = "Priority of the INT_DMA8 interrupt 8"]
    #[inline(always)]
    pub const fn pri8(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA8 interrupt 8"]
    #[inline(always)]
    pub const fn set_pri8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip8 {
    #[inline(always)]
    fn default() -> Nvicip8 {
        Nvicip8(0)
    }
}
#[doc = "Interrupt Priority Register 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicip9(pub u32);
impl Nvicip9 {
    #[doc = "Priority of the INT_DMA9 interrupt 9"]
    #[inline(always)]
    pub const fn pri9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Priority of the INT_DMA9 interrupt 9"]
    #[inline(always)]
    pub const fn set_pri9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Nvicip9 {
    #[inline(always)]
    fn default() -> Nvicip9 {
        Nvicip9(0)
    }
}
#[doc = "Software Trigger Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Nvicstir(pub u32);
impl Nvicstir {
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
    #[inline(always)]
    pub const fn intid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
    #[inline(always)]
    pub const fn set_intid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Nvicstir {
    #[inline(always)]
    fn default() -> Nvicstir {
        Nvicstir(0)
    }
}
