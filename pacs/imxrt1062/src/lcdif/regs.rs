#[doc = "LCDIF General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_24_bit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[inline(always)]
    pub const fn set_data_format_24_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_18_bit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline(always)]
    pub const fn set_data_format_18_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_16_bit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline(always)]
    pub const fn set_data_format_16_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pxp_handshake(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[inline(always)]
    pub const fn set_enable_pxp_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input data format."]
    #[must_use]
    #[inline(always)]
    pub const fn word_length(&self) -> super::vals::CtrlWordLength {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlWordLength::from_bits(val as u8)
    }
    #[doc = "Input data format."]
    #[inline(always)]
    pub const fn set_word_length(&mut self, val: super::vals::CtrlWordLength) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_databus_width(&self) -> super::vals::CtrlLcdDatabusWidth {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::CtrlLcdDatabusWidth::from_bits(val as u8)
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[inline(always)]
    pub const fn set_lcd_databus_width(&mut self, val: super::vals::CtrlLcdDatabusWidth) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[must_use]
    #[inline(always)]
    pub const fn csc_data_swizzle(&self) -> super::vals::CtrlCscDataSwizzle {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::CtrlCscDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline(always)]
    pub const fn set_csc_data_swizzle(&mut self, val: super::vals::CtrlCscDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[must_use]
    #[inline(always)]
    pub const fn input_data_swizzle(&self) -> super::vals::CtrlInputDataSwizzle {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::CtrlInputDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline(always)]
    pub const fn set_input_data_swizzle(&mut self, val: super::vals::CtrlInputDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_mode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline(always)]
    pub const fn set_dotclk_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_count(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline(always)]
    pub const fn set_bypass_count(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[must_use]
    #[inline(always)]
    pub const fn shift_num_bits(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[inline(always)]
    pub const fn set_shift_num_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn data_shift_dir(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[inline(always)]
    pub const fn set_data_shift_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
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
            .field("run", &self.run())
            .field("data_format_24_bit", &self.data_format_24_bit())
            .field("data_format_18_bit", &self.data_format_18_bit())
            .field("data_format_16_bit", &self.data_format_16_bit())
            .field("master", &self.master())
            .field("enable_pxp_handshake", &self.enable_pxp_handshake())
            .field("word_length", &self.word_length())
            .field("lcd_databus_width", &self.lcd_databus_width())
            .field("csc_data_swizzle", &self.csc_data_swizzle())
            .field("input_data_swizzle", &self.input_data_swizzle())
            .field("dotclk_mode", &self.dotclk_mode())
            .field("bypass_count", &self.bypass_count())
            .field("shift_num_bits", &self.shift_num_bits())
            .field("data_shift_dir", &self.data_shift_dir())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ run: {=bool:?}, data_format_24_bit: {=bool:?}, data_format_18_bit: {=bool:?}, data_format_16_bit: {=bool:?}, master: {=bool:?}, enable_pxp_handshake: {=bool:?}, word_length: {:?}, lcd_databus_width: {:?}, csc_data_swizzle: {:?}, input_data_swizzle: {:?}, dotclk_mode: {=bool:?}, bypass_count: {=bool:?}, shift_num_bits: {=u8:?}, data_shift_dir: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}" , self . run () , self . data_format_24_bit () , self . data_format_18_bit () , self . data_format_16_bit () , self . master () , self . enable_pxp_handshake () , self . word_length () , self . lcd_databus_width () , self . csc_data_swizzle () , self . input_data_swizzle () , self . dotclk_mode () , self . bypass_count () , self . shift_num_bits () , self . data_shift_dir () , self . clkgate () , self . sftrst ())
    }
}
#[doc = "LCDIF General Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_underflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_overflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_underflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_overflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_packing_format(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline(always)]
    pub const fn set_byte_packing_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_on_alternate_fields(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline(always)]
    pub const fn set_irq_on_alternate_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_clear(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline(always)]
    pub const fn set_fifo_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[must_use]
    #[inline(always)]
    pub const fn start_interlace_from_second_field(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[inline(always)]
    pub const fn set_start_interlace_from_second_field(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[must_use]
    #[inline(always)]
    pub const fn interlace_fields(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline(always)]
    pub const fn set_interlace_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[must_use]
    #[inline(always)]
    pub const fn recover_on_underflow(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline(always)]
    pub const fn set_recover_on_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_bm_error_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline(always)]
    pub const fn set_bm_error_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_out_select(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[inline(always)]
    pub const fn set_cs_out_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn image_data_select(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[inline(always)]
    pub const fn set_image_data_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("vsync_edge_irq", &self.vsync_edge_irq())
            .field("cur_frame_done_irq", &self.cur_frame_done_irq())
            .field("underflow_irq", &self.underflow_irq())
            .field("overflow_irq", &self.overflow_irq())
            .field("vsync_edge_irq_en", &self.vsync_edge_irq_en())
            .field("cur_frame_done_irq_en", &self.cur_frame_done_irq_en())
            .field("underflow_irq_en", &self.underflow_irq_en())
            .field("overflow_irq_en", &self.overflow_irq_en())
            .field("byte_packing_format", &self.byte_packing_format())
            .field("irq_on_alternate_fields", &self.irq_on_alternate_fields())
            .field("fifo_clear", &self.fifo_clear())
            .field(
                "start_interlace_from_second_field",
                &self.start_interlace_from_second_field(),
            )
            .field("interlace_fields", &self.interlace_fields())
            .field("recover_on_underflow", &self.recover_on_underflow())
            .field("bm_error_irq", &self.bm_error_irq())
            .field("bm_error_irq_en", &self.bm_error_irq_en())
            .field("cs_out_select", &self.cs_out_select())
            .field("image_data_select", &self.image_data_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl1 {{ vsync_edge_irq: {=bool:?}, cur_frame_done_irq: {=bool:?}, underflow_irq: {=bool:?}, overflow_irq: {=bool:?}, vsync_edge_irq_en: {=bool:?}, cur_frame_done_irq_en: {=bool:?}, underflow_irq_en: {=bool:?}, overflow_irq_en: {=bool:?}, byte_packing_format: {=u8:?}, irq_on_alternate_fields: {=bool:?}, fifo_clear: {=bool:?}, start_interlace_from_second_field: {=bool:?}, interlace_fields: {=bool:?}, recover_on_underflow: {=bool:?}, bm_error_irq: {=bool:?}, bm_error_irq_en: {=bool:?}, cs_out_select: {=bool:?}, image_data_select: {=bool:?} }}" , self . vsync_edge_irq () , self . cur_frame_done_irq () , self . underflow_irq () , self . overflow_irq () , self . vsync_edge_irq_en () , self . cur_frame_done_irq_en () , self . underflow_irq_en () , self . overflow_irq_en () , self . byte_packing_format () , self . irq_on_alternate_fields () , self . fifo_clear () , self . start_interlace_from_second_field () , self . interlace_fields () , self . recover_on_underflow () , self . bm_error_irq () , self . bm_error_irq_en () , self . cs_out_select () , self . image_data_select ())
    }
}
#[doc = "LCDIF General Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1clr(pub u32);
impl Ctrl1clr {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_underflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_overflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_underflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_overflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_packing_format(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline(always)]
    pub const fn set_byte_packing_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_on_alternate_fields(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline(always)]
    pub const fn set_irq_on_alternate_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_clear(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline(always)]
    pub const fn set_fifo_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[must_use]
    #[inline(always)]
    pub const fn start_interlace_from_second_field(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[inline(always)]
    pub const fn set_start_interlace_from_second_field(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[must_use]
    #[inline(always)]
    pub const fn interlace_fields(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline(always)]
    pub const fn set_interlace_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[must_use]
    #[inline(always)]
    pub const fn recover_on_underflow(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline(always)]
    pub const fn set_recover_on_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_bm_error_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline(always)]
    pub const fn set_bm_error_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_out_select(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[inline(always)]
    pub const fn set_cs_out_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn image_data_select(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[inline(always)]
    pub const fn set_image_data_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1clr {
    #[inline(always)]
    fn default() -> Ctrl1clr {
        Ctrl1clr(0)
    }
}
impl core::fmt::Debug for Ctrl1clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1clr")
            .field("vsync_edge_irq", &self.vsync_edge_irq())
            .field("cur_frame_done_irq", &self.cur_frame_done_irq())
            .field("underflow_irq", &self.underflow_irq())
            .field("overflow_irq", &self.overflow_irq())
            .field("vsync_edge_irq_en", &self.vsync_edge_irq_en())
            .field("cur_frame_done_irq_en", &self.cur_frame_done_irq_en())
            .field("underflow_irq_en", &self.underflow_irq_en())
            .field("overflow_irq_en", &self.overflow_irq_en())
            .field("byte_packing_format", &self.byte_packing_format())
            .field("irq_on_alternate_fields", &self.irq_on_alternate_fields())
            .field("fifo_clear", &self.fifo_clear())
            .field(
                "start_interlace_from_second_field",
                &self.start_interlace_from_second_field(),
            )
            .field("interlace_fields", &self.interlace_fields())
            .field("recover_on_underflow", &self.recover_on_underflow())
            .field("bm_error_irq", &self.bm_error_irq())
            .field("bm_error_irq_en", &self.bm_error_irq_en())
            .field("cs_out_select", &self.cs_out_select())
            .field("image_data_select", &self.image_data_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1clr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl1clr {{ vsync_edge_irq: {=bool:?}, cur_frame_done_irq: {=bool:?}, underflow_irq: {=bool:?}, overflow_irq: {=bool:?}, vsync_edge_irq_en: {=bool:?}, cur_frame_done_irq_en: {=bool:?}, underflow_irq_en: {=bool:?}, overflow_irq_en: {=bool:?}, byte_packing_format: {=u8:?}, irq_on_alternate_fields: {=bool:?}, fifo_clear: {=bool:?}, start_interlace_from_second_field: {=bool:?}, interlace_fields: {=bool:?}, recover_on_underflow: {=bool:?}, bm_error_irq: {=bool:?}, bm_error_irq_en: {=bool:?}, cs_out_select: {=bool:?}, image_data_select: {=bool:?} }}" , self . vsync_edge_irq () , self . cur_frame_done_irq () , self . underflow_irq () , self . overflow_irq () , self . vsync_edge_irq_en () , self . cur_frame_done_irq_en () , self . underflow_irq_en () , self . overflow_irq_en () , self . byte_packing_format () , self . irq_on_alternate_fields () , self . fifo_clear () , self . start_interlace_from_second_field () , self . interlace_fields () , self . recover_on_underflow () , self . bm_error_irq () , self . bm_error_irq_en () , self . cs_out_select () , self . image_data_select ())
    }
}
#[doc = "LCDIF General Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1set(pub u32);
impl Ctrl1set {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_underflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_overflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_underflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_overflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_packing_format(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline(always)]
    pub const fn set_byte_packing_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_on_alternate_fields(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline(always)]
    pub const fn set_irq_on_alternate_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_clear(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline(always)]
    pub const fn set_fifo_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[must_use]
    #[inline(always)]
    pub const fn start_interlace_from_second_field(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[inline(always)]
    pub const fn set_start_interlace_from_second_field(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[must_use]
    #[inline(always)]
    pub const fn interlace_fields(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline(always)]
    pub const fn set_interlace_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[must_use]
    #[inline(always)]
    pub const fn recover_on_underflow(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline(always)]
    pub const fn set_recover_on_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_bm_error_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline(always)]
    pub const fn set_bm_error_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_out_select(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[inline(always)]
    pub const fn set_cs_out_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn image_data_select(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[inline(always)]
    pub const fn set_image_data_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1set {
    #[inline(always)]
    fn default() -> Ctrl1set {
        Ctrl1set(0)
    }
}
impl core::fmt::Debug for Ctrl1set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1set")
            .field("vsync_edge_irq", &self.vsync_edge_irq())
            .field("cur_frame_done_irq", &self.cur_frame_done_irq())
            .field("underflow_irq", &self.underflow_irq())
            .field("overflow_irq", &self.overflow_irq())
            .field("vsync_edge_irq_en", &self.vsync_edge_irq_en())
            .field("cur_frame_done_irq_en", &self.cur_frame_done_irq_en())
            .field("underflow_irq_en", &self.underflow_irq_en())
            .field("overflow_irq_en", &self.overflow_irq_en())
            .field("byte_packing_format", &self.byte_packing_format())
            .field("irq_on_alternate_fields", &self.irq_on_alternate_fields())
            .field("fifo_clear", &self.fifo_clear())
            .field(
                "start_interlace_from_second_field",
                &self.start_interlace_from_second_field(),
            )
            .field("interlace_fields", &self.interlace_fields())
            .field("recover_on_underflow", &self.recover_on_underflow())
            .field("bm_error_irq", &self.bm_error_irq())
            .field("bm_error_irq_en", &self.bm_error_irq_en())
            .field("cs_out_select", &self.cs_out_select())
            .field("image_data_select", &self.image_data_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1set {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl1set {{ vsync_edge_irq: {=bool:?}, cur_frame_done_irq: {=bool:?}, underflow_irq: {=bool:?}, overflow_irq: {=bool:?}, vsync_edge_irq_en: {=bool:?}, cur_frame_done_irq_en: {=bool:?}, underflow_irq_en: {=bool:?}, overflow_irq_en: {=bool:?}, byte_packing_format: {=u8:?}, irq_on_alternate_fields: {=bool:?}, fifo_clear: {=bool:?}, start_interlace_from_second_field: {=bool:?}, interlace_fields: {=bool:?}, recover_on_underflow: {=bool:?}, bm_error_irq: {=bool:?}, bm_error_irq_en: {=bool:?}, cs_out_select: {=bool:?}, image_data_select: {=bool:?} }}" , self . vsync_edge_irq () , self . cur_frame_done_irq () , self . underflow_irq () , self . overflow_irq () , self . vsync_edge_irq_en () , self . cur_frame_done_irq_en () , self . underflow_irq_en () , self . overflow_irq_en () , self . byte_packing_format () , self . irq_on_alternate_fields () , self . fifo_clear () , self . start_interlace_from_second_field () , self . interlace_fields () , self . recover_on_underflow () , self . bm_error_irq () , self . bm_error_irq_en () , self . cs_out_select () , self . image_data_select ())
    }
}
#[doc = "LCDIF General Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1tog(pub u32);
impl Ctrl1tog {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_underflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_overflow_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_edge_irq_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline(always)]
    pub const fn set_vsync_edge_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_frame_done_irq_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline(always)]
    pub const fn set_cur_frame_done_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn underflow_irq_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_underflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_irq_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub const fn set_overflow_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_packing_format(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline(always)]
    pub const fn set_byte_packing_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_on_alternate_fields(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline(always)]
    pub const fn set_irq_on_alternate_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_clear(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline(always)]
    pub const fn set_fifo_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[must_use]
    #[inline(always)]
    pub const fn start_interlace_from_second_field(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    #[inline(always)]
    pub const fn set_start_interlace_from_second_field(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[must_use]
    #[inline(always)]
    pub const fn interlace_fields(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline(always)]
    pub const fn set_interlace_fields(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[must_use]
    #[inline(always)]
    pub const fn recover_on_underflow(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline(always)]
    pub const fn set_recover_on_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub const fn set_bm_error_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bm_error_irq_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline(always)]
    pub const fn set_bm_error_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_out_select(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    #[inline(always)]
    pub const fn set_cs_out_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn image_data_select(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Command Mode MIPI image data select bit"]
    #[inline(always)]
    pub const fn set_image_data_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1tog {
    #[inline(always)]
    fn default() -> Ctrl1tog {
        Ctrl1tog(0)
    }
}
impl core::fmt::Debug for Ctrl1tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1tog")
            .field("vsync_edge_irq", &self.vsync_edge_irq())
            .field("cur_frame_done_irq", &self.cur_frame_done_irq())
            .field("underflow_irq", &self.underflow_irq())
            .field("overflow_irq", &self.overflow_irq())
            .field("vsync_edge_irq_en", &self.vsync_edge_irq_en())
            .field("cur_frame_done_irq_en", &self.cur_frame_done_irq_en())
            .field("underflow_irq_en", &self.underflow_irq_en())
            .field("overflow_irq_en", &self.overflow_irq_en())
            .field("byte_packing_format", &self.byte_packing_format())
            .field("irq_on_alternate_fields", &self.irq_on_alternate_fields())
            .field("fifo_clear", &self.fifo_clear())
            .field(
                "start_interlace_from_second_field",
                &self.start_interlace_from_second_field(),
            )
            .field("interlace_fields", &self.interlace_fields())
            .field("recover_on_underflow", &self.recover_on_underflow())
            .field("bm_error_irq", &self.bm_error_irq())
            .field("bm_error_irq_en", &self.bm_error_irq_en())
            .field("cs_out_select", &self.cs_out_select())
            .field("image_data_select", &self.image_data_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1tog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl1tog {{ vsync_edge_irq: {=bool:?}, cur_frame_done_irq: {=bool:?}, underflow_irq: {=bool:?}, overflow_irq: {=bool:?}, vsync_edge_irq_en: {=bool:?}, cur_frame_done_irq_en: {=bool:?}, underflow_irq_en: {=bool:?}, overflow_irq_en: {=bool:?}, byte_packing_format: {=u8:?}, irq_on_alternate_fields: {=bool:?}, fifo_clear: {=bool:?}, start_interlace_from_second_field: {=bool:?}, interlace_fields: {=bool:?}, recover_on_underflow: {=bool:?}, bm_error_irq: {=bool:?}, bm_error_irq_en: {=bool:?}, cs_out_select: {=bool:?}, image_data_select: {=bool:?} }}" , self . vsync_edge_irq () , self . cur_frame_done_irq () , self . underflow_irq () , self . overflow_irq () , self . vsync_edge_irq_en () , self . cur_frame_done_irq_en () , self . underflow_irq_en () , self . overflow_irq_en () , self . byte_packing_format () , self . irq_on_alternate_fields () , self . fifo_clear () , self . start_interlace_from_second_field () , self . interlace_fields () , self . recover_on_underflow () , self . bm_error_irq () , self . bm_error_irq_en () , self . cs_out_select () , self . image_data_select ())
    }
}
#[doc = "LCDIF General Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[must_use]
    #[inline(always)]
    pub const fn even_line_pattern(&self) -> super::vals::Ctrl2evenLinePattern {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Ctrl2evenLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline(always)]
    pub const fn set_even_line_pattern(&mut self, val: super::vals::Ctrl2evenLinePattern) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[must_use]
    #[inline(always)]
    pub const fn odd_line_pattern(&self) -> super::vals::Ctrl2oddLinePattern {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Ctrl2oddLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline(always)]
    pub const fn set_odd_line_pattern(&mut self, val: super::vals::Ctrl2oddLinePattern) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_len_8(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline(always)]
    pub const fn set_burst_len_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn outstanding_reqs(&self) -> super::vals::Ctrl2outstandingReqs {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Ctrl2outstandingReqs::from_bits(val as u8)
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline(always)]
    pub const fn set_outstanding_reqs(&mut self, val: super::vals::Ctrl2outstandingReqs) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("even_line_pattern", &self.even_line_pattern())
            .field("odd_line_pattern", &self.odd_line_pattern())
            .field("burst_len_8", &self.burst_len_8())
            .field("outstanding_reqs", &self.outstanding_reqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl2 {{ even_line_pattern: {:?}, odd_line_pattern: {:?}, burst_len_8: {=bool:?}, outstanding_reqs: {:?} }}" , self . even_line_pattern () , self . odd_line_pattern () , self . burst_len_8 () , self . outstanding_reqs ())
    }
}
#[doc = "LCDIF General Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2clr(pub u32);
impl Ctrl2clr {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[must_use]
    #[inline(always)]
    pub const fn even_line_pattern(&self) -> super::vals::Ctrl2clrEvenLinePattern {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Ctrl2clrEvenLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline(always)]
    pub const fn set_even_line_pattern(&mut self, val: super::vals::Ctrl2clrEvenLinePattern) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[must_use]
    #[inline(always)]
    pub const fn odd_line_pattern(&self) -> super::vals::Ctrl2clrOddLinePattern {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Ctrl2clrOddLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline(always)]
    pub const fn set_odd_line_pattern(&mut self, val: super::vals::Ctrl2clrOddLinePattern) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_len_8(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline(always)]
    pub const fn set_burst_len_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn outstanding_reqs(&self) -> super::vals::Ctrl2clrOutstandingReqs {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Ctrl2clrOutstandingReqs::from_bits(val as u8)
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline(always)]
    pub const fn set_outstanding_reqs(&mut self, val: super::vals::Ctrl2clrOutstandingReqs) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for Ctrl2clr {
    #[inline(always)]
    fn default() -> Ctrl2clr {
        Ctrl2clr(0)
    }
}
impl core::fmt::Debug for Ctrl2clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2clr")
            .field("even_line_pattern", &self.even_line_pattern())
            .field("odd_line_pattern", &self.odd_line_pattern())
            .field("burst_len_8", &self.burst_len_8())
            .field("outstanding_reqs", &self.outstanding_reqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2clr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl2clr {{ even_line_pattern: {:?}, odd_line_pattern: {:?}, burst_len_8: {=bool:?}, outstanding_reqs: {:?} }}" , self . even_line_pattern () , self . odd_line_pattern () , self . burst_len_8 () , self . outstanding_reqs ())
    }
}
#[doc = "LCDIF General Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2set(pub u32);
impl Ctrl2set {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[must_use]
    #[inline(always)]
    pub const fn even_line_pattern(&self) -> super::vals::Ctrl2setEvenLinePattern {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Ctrl2setEvenLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline(always)]
    pub const fn set_even_line_pattern(&mut self, val: super::vals::Ctrl2setEvenLinePattern) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[must_use]
    #[inline(always)]
    pub const fn odd_line_pattern(&self) -> super::vals::Ctrl2setOddLinePattern {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Ctrl2setOddLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline(always)]
    pub const fn set_odd_line_pattern(&mut self, val: super::vals::Ctrl2setOddLinePattern) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_len_8(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline(always)]
    pub const fn set_burst_len_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn outstanding_reqs(&self) -> super::vals::Ctrl2setOutstandingReqs {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Ctrl2setOutstandingReqs::from_bits(val as u8)
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline(always)]
    pub const fn set_outstanding_reqs(&mut self, val: super::vals::Ctrl2setOutstandingReqs) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for Ctrl2set {
    #[inline(always)]
    fn default() -> Ctrl2set {
        Ctrl2set(0)
    }
}
impl core::fmt::Debug for Ctrl2set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2set")
            .field("even_line_pattern", &self.even_line_pattern())
            .field("odd_line_pattern", &self.odd_line_pattern())
            .field("burst_len_8", &self.burst_len_8())
            .field("outstanding_reqs", &self.outstanding_reqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2set {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl2set {{ even_line_pattern: {:?}, odd_line_pattern: {:?}, burst_len_8: {=bool:?}, outstanding_reqs: {:?} }}" , self . even_line_pattern () , self . odd_line_pattern () , self . burst_len_8 () , self . outstanding_reqs ())
    }
}
#[doc = "LCDIF General Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2tog(pub u32);
impl Ctrl2tog {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[must_use]
    #[inline(always)]
    pub const fn even_line_pattern(&self) -> super::vals::Ctrl2togEvenLinePattern {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Ctrl2togEvenLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline(always)]
    pub const fn set_even_line_pattern(&mut self, val: super::vals::Ctrl2togEvenLinePattern) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[must_use]
    #[inline(always)]
    pub const fn odd_line_pattern(&self) -> super::vals::Ctrl2togOddLinePattern {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Ctrl2togOddLinePattern::from_bits(val as u8)
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline(always)]
    pub const fn set_odd_line_pattern(&mut self, val: super::vals::Ctrl2togOddLinePattern) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_len_8(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline(always)]
    pub const fn set_burst_len_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn outstanding_reqs(&self) -> super::vals::Ctrl2togOutstandingReqs {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Ctrl2togOutstandingReqs::from_bits(val as u8)
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline(always)]
    pub const fn set_outstanding_reqs(&mut self, val: super::vals::Ctrl2togOutstandingReqs) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for Ctrl2tog {
    #[inline(always)]
    fn default() -> Ctrl2tog {
        Ctrl2tog(0)
    }
}
impl core::fmt::Debug for Ctrl2tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2tog")
            .field("even_line_pattern", &self.even_line_pattern())
            .field("odd_line_pattern", &self.odd_line_pattern())
            .field("burst_len_8", &self.burst_len_8())
            .field("outstanding_reqs", &self.outstanding_reqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2tog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl2tog {{ even_line_pattern: {:?}, odd_line_pattern: {:?}, burst_len_8: {=bool:?}, outstanding_reqs: {:?} }}" , self . even_line_pattern () , self . odd_line_pattern () , self . burst_len_8 () , self . outstanding_reqs ())
    }
}
#[doc = "LCDIF General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_24_bit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[inline(always)]
    pub const fn set_data_format_24_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_18_bit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline(always)]
    pub const fn set_data_format_18_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_16_bit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline(always)]
    pub const fn set_data_format_16_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pxp_handshake(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[inline(always)]
    pub const fn set_enable_pxp_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input data format."]
    #[must_use]
    #[inline(always)]
    pub const fn word_length(&self) -> super::vals::CtrlClrWordLength {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlClrWordLength::from_bits(val as u8)
    }
    #[doc = "Input data format."]
    #[inline(always)]
    pub const fn set_word_length(&mut self, val: super::vals::CtrlClrWordLength) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_databus_width(&self) -> super::vals::CtrlClrLcdDatabusWidth {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::CtrlClrLcdDatabusWidth::from_bits(val as u8)
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[inline(always)]
    pub const fn set_lcd_databus_width(&mut self, val: super::vals::CtrlClrLcdDatabusWidth) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[must_use]
    #[inline(always)]
    pub const fn csc_data_swizzle(&self) -> super::vals::CtrlClrCscDataSwizzle {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::CtrlClrCscDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline(always)]
    pub const fn set_csc_data_swizzle(&mut self, val: super::vals::CtrlClrCscDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[must_use]
    #[inline(always)]
    pub const fn input_data_swizzle(&self) -> super::vals::CtrlClrInputDataSwizzle {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::CtrlClrInputDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline(always)]
    pub const fn set_input_data_swizzle(&mut self, val: super::vals::CtrlClrInputDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_mode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline(always)]
    pub const fn set_dotclk_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_count(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline(always)]
    pub const fn set_bypass_count(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[must_use]
    #[inline(always)]
    pub const fn shift_num_bits(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[inline(always)]
    pub const fn set_shift_num_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn data_shift_dir(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[inline(always)]
    pub const fn set_data_shift_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
impl core::fmt::Debug for CtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlClr")
            .field("run", &self.run())
            .field("data_format_24_bit", &self.data_format_24_bit())
            .field("data_format_18_bit", &self.data_format_18_bit())
            .field("data_format_16_bit", &self.data_format_16_bit())
            .field("master", &self.master())
            .field("enable_pxp_handshake", &self.enable_pxp_handshake())
            .field("word_length", &self.word_length())
            .field("lcd_databus_width", &self.lcd_databus_width())
            .field("csc_data_swizzle", &self.csc_data_swizzle())
            .field("input_data_swizzle", &self.input_data_swizzle())
            .field("dotclk_mode", &self.dotclk_mode())
            .field("bypass_count", &self.bypass_count())
            .field("shift_num_bits", &self.shift_num_bits())
            .field("data_shift_dir", &self.data_shift_dir())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlClr {{ run: {=bool:?}, data_format_24_bit: {=bool:?}, data_format_18_bit: {=bool:?}, data_format_16_bit: {=bool:?}, master: {=bool:?}, enable_pxp_handshake: {=bool:?}, word_length: {:?}, lcd_databus_width: {:?}, csc_data_swizzle: {:?}, input_data_swizzle: {:?}, dotclk_mode: {=bool:?}, bypass_count: {=bool:?}, shift_num_bits: {=u8:?}, data_shift_dir: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}" , self . run () , self . data_format_24_bit () , self . data_format_18_bit () , self . data_format_16_bit () , self . master () , self . enable_pxp_handshake () , self . word_length () , self . lcd_databus_width () , self . csc_data_swizzle () , self . input_data_swizzle () , self . dotclk_mode () , self . bypass_count () , self . shift_num_bits () , self . data_shift_dir () , self . clkgate () , self . sftrst ())
    }
}
#[doc = "LCDIF General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_24_bit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[inline(always)]
    pub const fn set_data_format_24_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_18_bit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline(always)]
    pub const fn set_data_format_18_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_16_bit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline(always)]
    pub const fn set_data_format_16_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pxp_handshake(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[inline(always)]
    pub const fn set_enable_pxp_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input data format."]
    #[must_use]
    #[inline(always)]
    pub const fn word_length(&self) -> super::vals::CtrlSetWordLength {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlSetWordLength::from_bits(val as u8)
    }
    #[doc = "Input data format."]
    #[inline(always)]
    pub const fn set_word_length(&mut self, val: super::vals::CtrlSetWordLength) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_databus_width(&self) -> super::vals::CtrlSetLcdDatabusWidth {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::CtrlSetLcdDatabusWidth::from_bits(val as u8)
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[inline(always)]
    pub const fn set_lcd_databus_width(&mut self, val: super::vals::CtrlSetLcdDatabusWidth) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[must_use]
    #[inline(always)]
    pub const fn csc_data_swizzle(&self) -> super::vals::CtrlSetCscDataSwizzle {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::CtrlSetCscDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline(always)]
    pub const fn set_csc_data_swizzle(&mut self, val: super::vals::CtrlSetCscDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[must_use]
    #[inline(always)]
    pub const fn input_data_swizzle(&self) -> super::vals::CtrlSetInputDataSwizzle {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::CtrlSetInputDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline(always)]
    pub const fn set_input_data_swizzle(&mut self, val: super::vals::CtrlSetInputDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_mode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline(always)]
    pub const fn set_dotclk_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_count(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline(always)]
    pub const fn set_bypass_count(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[must_use]
    #[inline(always)]
    pub const fn shift_num_bits(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[inline(always)]
    pub const fn set_shift_num_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn data_shift_dir(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[inline(always)]
    pub const fn set_data_shift_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
impl core::fmt::Debug for CtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlSet")
            .field("run", &self.run())
            .field("data_format_24_bit", &self.data_format_24_bit())
            .field("data_format_18_bit", &self.data_format_18_bit())
            .field("data_format_16_bit", &self.data_format_16_bit())
            .field("master", &self.master())
            .field("enable_pxp_handshake", &self.enable_pxp_handshake())
            .field("word_length", &self.word_length())
            .field("lcd_databus_width", &self.lcd_databus_width())
            .field("csc_data_swizzle", &self.csc_data_swizzle())
            .field("input_data_swizzle", &self.input_data_swizzle())
            .field("dotclk_mode", &self.dotclk_mode())
            .field("bypass_count", &self.bypass_count())
            .field("shift_num_bits", &self.shift_num_bits())
            .field("data_shift_dir", &self.data_shift_dir())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlSet {{ run: {=bool:?}, data_format_24_bit: {=bool:?}, data_format_18_bit: {=bool:?}, data_format_16_bit: {=bool:?}, master: {=bool:?}, enable_pxp_handshake: {=bool:?}, word_length: {:?}, lcd_databus_width: {:?}, csc_data_swizzle: {:?}, input_data_swizzle: {:?}, dotclk_mode: {=bool:?}, bypass_count: {=bool:?}, shift_num_bits: {=u8:?}, data_shift_dir: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}" , self . run () , self . data_format_24_bit () , self . data_format_18_bit () , self . data_format_16_bit () , self . master () , self . enable_pxp_handshake () , self . word_length () , self . lcd_databus_width () , self . csc_data_swizzle () , self . input_data_swizzle () , self . dotclk_mode () , self . bypass_count () , self . shift_num_bits () , self . data_shift_dir () , self . clkgate () , self . sftrst ())
    }
}
#[doc = "LCDIF General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_24_bit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    #[inline(always)]
    pub const fn set_data_format_24_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_18_bit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline(always)]
    pub const fn set_data_format_18_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format_16_bit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline(always)]
    pub const fn set_data_format_16_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pxp_handshake(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[inline(always)]
    pub const fn set_enable_pxp_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input data format."]
    #[must_use]
    #[inline(always)]
    pub const fn word_length(&self) -> super::vals::CtrlTogWordLength {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlTogWordLength::from_bits(val as u8)
    }
    #[doc = "Input data format."]
    #[inline(always)]
    pub const fn set_word_length(&mut self, val: super::vals::CtrlTogWordLength) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_databus_width(&self) -> super::vals::CtrlTogLcdDatabusWidth {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::CtrlTogLcdDatabusWidth::from_bits(val as u8)
    }
    #[doc = "LCD Data bus transfer width. When LUT enabled, this field should be set to 0x01."]
    #[inline(always)]
    pub const fn set_lcd_databus_width(&mut self, val: super::vals::CtrlTogLcdDatabusWidth) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[must_use]
    #[inline(always)]
    pub const fn csc_data_swizzle(&self) -> super::vals::CtrlTogCscDataSwizzle {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::CtrlTogCscDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline(always)]
    pub const fn set_csc_data_swizzle(&mut self, val: super::vals::CtrlTogCscDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[must_use]
    #[inline(always)]
    pub const fn input_data_swizzle(&self) -> super::vals::CtrlTogInputDataSwizzle {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::CtrlTogInputDataSwizzle::from_bits(val as u8)
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline(always)]
    pub const fn set_input_data_swizzle(&mut self, val: super::vals::CtrlTogInputDataSwizzle) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_mode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline(always)]
    pub const fn set_dotclk_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_count(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline(always)]
    pub const fn set_bypass_count(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[must_use]
    #[inline(always)]
    pub const fn shift_num_bits(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    #[inline(always)]
    pub const fn set_shift_num_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[must_use]
    #[inline(always)]
    pub const fn data_shift_dir(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    #[inline(always)]
    pub const fn set_data_shift_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
impl core::fmt::Debug for CtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTog")
            .field("run", &self.run())
            .field("data_format_24_bit", &self.data_format_24_bit())
            .field("data_format_18_bit", &self.data_format_18_bit())
            .field("data_format_16_bit", &self.data_format_16_bit())
            .field("master", &self.master())
            .field("enable_pxp_handshake", &self.enable_pxp_handshake())
            .field("word_length", &self.word_length())
            .field("lcd_databus_width", &self.lcd_databus_width())
            .field("csc_data_swizzle", &self.csc_data_swizzle())
            .field("input_data_swizzle", &self.input_data_swizzle())
            .field("dotclk_mode", &self.dotclk_mode())
            .field("bypass_count", &self.bypass_count())
            .field("shift_num_bits", &self.shift_num_bits())
            .field("data_shift_dir", &self.data_shift_dir())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlTog {{ run: {=bool:?}, data_format_24_bit: {=bool:?}, data_format_18_bit: {=bool:?}, data_format_16_bit: {=bool:?}, master: {=bool:?}, enable_pxp_handshake: {=bool:?}, word_length: {:?}, lcd_databus_width: {:?}, csc_data_swizzle: {:?}, input_data_swizzle: {:?}, dotclk_mode: {=bool:?}, bypass_count: {=bool:?}, shift_num_bits: {=u8:?}, data_shift_dir: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}" , self . run () , self . data_format_24_bit () , self . data_format_18_bit () , self . data_format_16_bit () , self . master () , self . enable_pxp_handshake () , self . word_length () , self . lcd_databus_width () , self . csc_data_swizzle () , self . input_data_swizzle () , self . dotclk_mode () , self . bypass_count () , self . shift_num_bits () , self . data_shift_dir () , self . clkgate () , self . sftrst ())
    }
}
#[doc = "Lookup Table 0 Index Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut0addr(pub u32);
impl Lut0addr {
    #[doc = "LUT indexed address pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LUT indexed address pointer"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Lut0addr {
    #[inline(always)]
    fn default() -> Lut0addr {
        Lut0addr(0)
    }
}
impl core::fmt::Debug for Lut0addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lut0addr")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lut0addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lut0addr {{ addr: {=u8:?} }}", self.addr())
    }
}
#[doc = "Lookup Table 1 Index Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut1addr(pub u32);
impl Lut1addr {
    #[doc = "LUT indexed address pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LUT indexed address pointer"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Lut1addr {
    #[inline(always)]
    fn default() -> Lut1addr {
        Lut1addr(0)
    }
}
impl core::fmt::Debug for Lut1addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lut1addr")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lut1addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lut1addr {{ addr: {=u8:?} }}", self.addr())
    }
}
#[doc = "Look Up Table Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutCtrl(pub u32);
impl LutCtrl {
    #[doc = "Setting this bit will bypass the LUT memory resource completely"]
    #[must_use]
    #[inline(always)]
    pub const fn lut_bypass(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit will bypass the LUT memory resource completely"]
    #[inline(always)]
    pub const fn set_lut_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LutCtrl {
    #[inline(always)]
    fn default() -> LutCtrl {
        LutCtrl(0)
    }
}
impl core::fmt::Debug for LutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LutCtrl")
            .field("lut_bypass", &self.lut_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LutCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LutCtrl {{ lut_bypass: {=bool:?} }}", self.lut_bypass())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon00(pub u32);
impl Pigeon00 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon00incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon00incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon00incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon00maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon00maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon00maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon00stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon00stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon00stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon00 {
    #[inline(always)]
    fn default() -> Pigeon00 {
        Pigeon00(0)
    }
}
impl core::fmt::Debug for Pigeon00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon00")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon00 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon00 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon01(pub u32);
impl Pigeon01 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon01setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon01setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon01setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon01clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon01clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon01clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon01 {
    #[inline(always)]
    fn default() -> Pigeon01 {
        Pigeon01(0)
    }
}
impl core::fmt::Debug for Pigeon01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon01")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon01 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon02(pub u32);
impl Pigeon02 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon02sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon02sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon02sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon02sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon02sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon02sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon02 {
    #[inline(always)]
    fn default() -> Pigeon02 {
        Pigeon02(0)
    }
}
impl core::fmt::Debug for Pigeon02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon02")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon02 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon10(pub u32);
impl Pigeon10 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon10incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon10incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon10incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon10maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon10maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon10maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon10stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon10stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon10stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon10 {
    #[inline(always)]
    fn default() -> Pigeon10 {
        Pigeon10(0)
    }
}
impl core::fmt::Debug for Pigeon10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon10")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon10 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon100(pub u32);
impl Pigeon100 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon100incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon100incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon100incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon100maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon100maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon100maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon100stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon100stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon100stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon100 {
    #[inline(always)]
    fn default() -> Pigeon100 {
        Pigeon100(0)
    }
}
impl core::fmt::Debug for Pigeon100 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon100")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon100 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon100 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon101(pub u32);
impl Pigeon101 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon101setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon101setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon101setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon101clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon101clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon101clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon101 {
    #[inline(always)]
    fn default() -> Pigeon101 {
        Pigeon101(0)
    }
}
impl core::fmt::Debug for Pigeon101 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon101")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon101 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon101 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon102(pub u32);
impl Pigeon102 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon102sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon102sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon102sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon102sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon102sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon102sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon102 {
    #[inline(always)]
    fn default() -> Pigeon102 {
        Pigeon102(0)
    }
}
impl core::fmt::Debug for Pigeon102 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon102")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon102 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon102 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon11(pub u32);
impl Pigeon11 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon11setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon11setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon11setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon11clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon11clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon11clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon11 {
    #[inline(always)]
    fn default() -> Pigeon11 {
        Pigeon11(0)
    }
}
impl core::fmt::Debug for Pigeon11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon11")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon11 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon110(pub u32);
impl Pigeon110 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon110incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon110incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon110incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon110maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon110maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon110maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon110stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon110stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon110stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon110 {
    #[inline(always)]
    fn default() -> Pigeon110 {
        Pigeon110(0)
    }
}
impl core::fmt::Debug for Pigeon110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon110")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon110 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon110 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon111(pub u32);
impl Pigeon111 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon111setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon111setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon111setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon111clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon111clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon111clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon111 {
    #[inline(always)]
    fn default() -> Pigeon111 {
        Pigeon111(0)
    }
}
impl core::fmt::Debug for Pigeon111 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon111")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon111 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon111 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon112(pub u32);
impl Pigeon112 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon112sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon112sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon112sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon112sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon112sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon112sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon112 {
    #[inline(always)]
    fn default() -> Pigeon112 {
        Pigeon112(0)
    }
}
impl core::fmt::Debug for Pigeon112 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon112")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon112 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon112 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon12(pub u32);
impl Pigeon12 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon12sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon12sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon12sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon12sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon12sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon12sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon12 {
    #[inline(always)]
    fn default() -> Pigeon12 {
        Pigeon12(0)
    }
}
impl core::fmt::Debug for Pigeon12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon12")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon12 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon20(pub u32);
impl Pigeon20 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon20incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon20incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon20incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon20maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon20maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon20maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon20stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon20stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon20stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon20 {
    #[inline(always)]
    fn default() -> Pigeon20 {
        Pigeon20(0)
    }
}
impl core::fmt::Debug for Pigeon20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon20")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon20 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon20 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon21(pub u32);
impl Pigeon21 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon21setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon21setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon21setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon21clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon21clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon21clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon21 {
    #[inline(always)]
    fn default() -> Pigeon21 {
        Pigeon21(0)
    }
}
impl core::fmt::Debug for Pigeon21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon21")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon21 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon22(pub u32);
impl Pigeon22 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon22sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon22sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon22sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon22sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon22sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon22sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon22 {
    #[inline(always)]
    fn default() -> Pigeon22 {
        Pigeon22(0)
    }
}
impl core::fmt::Debug for Pigeon22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon22")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon22 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon30(pub u32);
impl Pigeon30 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon30incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon30incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon30incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon30maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon30maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon30maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon30stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon30stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon30stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon30 {
    #[inline(always)]
    fn default() -> Pigeon30 {
        Pigeon30(0)
    }
}
impl core::fmt::Debug for Pigeon30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon30")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon30 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon30 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon31(pub u32);
impl Pigeon31 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon31setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon31setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon31setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon31clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon31clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon31clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon31 {
    #[inline(always)]
    fn default() -> Pigeon31 {
        Pigeon31(0)
    }
}
impl core::fmt::Debug for Pigeon31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon31")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon31 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon32(pub u32);
impl Pigeon32 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon32sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon32sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon32sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon32sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon32sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon32sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon32 {
    #[inline(always)]
    fn default() -> Pigeon32 {
        Pigeon32(0)
    }
}
impl core::fmt::Debug for Pigeon32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon32")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon32 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon40(pub u32);
impl Pigeon40 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon40incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon40incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon40incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon40maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon40maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon40maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon40stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon40stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon40stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon40 {
    #[inline(always)]
    fn default() -> Pigeon40 {
        Pigeon40(0)
    }
}
impl core::fmt::Debug for Pigeon40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon40")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon40 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon40 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon41(pub u32);
impl Pigeon41 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon41setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon41setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon41setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon41clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon41clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon41clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon41 {
    #[inline(always)]
    fn default() -> Pigeon41 {
        Pigeon41(0)
    }
}
impl core::fmt::Debug for Pigeon41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon41")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon41 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon42(pub u32);
impl Pigeon42 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon42sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon42sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon42sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon42sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon42sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon42sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon42 {
    #[inline(always)]
    fn default() -> Pigeon42 {
        Pigeon42(0)
    }
}
impl core::fmt::Debug for Pigeon42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon42")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon42 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon42 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon50(pub u32);
impl Pigeon50 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon50incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon50incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon50incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon50maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon50maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon50maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon50stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon50stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon50stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon50 {
    #[inline(always)]
    fn default() -> Pigeon50 {
        Pigeon50(0)
    }
}
impl core::fmt::Debug for Pigeon50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon50")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon50 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon50 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon51(pub u32);
impl Pigeon51 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon51setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon51setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon51setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon51clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon51clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon51clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon51 {
    #[inline(always)]
    fn default() -> Pigeon51 {
        Pigeon51(0)
    }
}
impl core::fmt::Debug for Pigeon51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon51")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon51 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon51 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon52(pub u32);
impl Pigeon52 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon52sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon52sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon52sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon52sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon52sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon52sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon52 {
    #[inline(always)]
    fn default() -> Pigeon52 {
        Pigeon52(0)
    }
}
impl core::fmt::Debug for Pigeon52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon52")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon52 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon52 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon60(pub u32);
impl Pigeon60 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon60incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon60incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon60incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon60maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon60maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon60maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon60stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon60stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon60stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon60 {
    #[inline(always)]
    fn default() -> Pigeon60 {
        Pigeon60(0)
    }
}
impl core::fmt::Debug for Pigeon60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon60")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon60 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon60 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon61(pub u32);
impl Pigeon61 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon61setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon61setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon61setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon61clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon61clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon61clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon61 {
    #[inline(always)]
    fn default() -> Pigeon61 {
        Pigeon61(0)
    }
}
impl core::fmt::Debug for Pigeon61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon61")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon61 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon61 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon62(pub u32);
impl Pigeon62 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon62sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon62sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon62sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon62sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon62sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon62sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon62 {
    #[inline(always)]
    fn default() -> Pigeon62 {
        Pigeon62(0)
    }
}
impl core::fmt::Debug for Pigeon62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon62")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon62 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon62 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon70(pub u32);
impl Pigeon70 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon70incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon70incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon70incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon70maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon70maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon70maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon70stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon70stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon70stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon70 {
    #[inline(always)]
    fn default() -> Pigeon70 {
        Pigeon70(0)
    }
}
impl core::fmt::Debug for Pigeon70 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon70")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon70 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon70 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon71(pub u32);
impl Pigeon71 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon71setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon71setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon71setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon71clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon71clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon71clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon71 {
    #[inline(always)]
    fn default() -> Pigeon71 {
        Pigeon71(0)
    }
}
impl core::fmt::Debug for Pigeon71 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon71")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon71 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon71 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon72(pub u32);
impl Pigeon72 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon72sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon72sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon72sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon72sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon72sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon72sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon72 {
    #[inline(always)]
    fn default() -> Pigeon72 {
        Pigeon72(0)
    }
}
impl core::fmt::Debug for Pigeon72 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon72")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon72 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon72 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon80(pub u32);
impl Pigeon80 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon80incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon80incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon80incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon80maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon80maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon80maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon80stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon80stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon80stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon80 {
    #[inline(always)]
    fn default() -> Pigeon80 {
        Pigeon80(0)
    }
}
impl core::fmt::Debug for Pigeon80 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon80")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon80 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon80 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon81(pub u32);
impl Pigeon81 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon81setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon81setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon81setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon81clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon81clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon81clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon81 {
    #[inline(always)]
    fn default() -> Pigeon81 {
        Pigeon81(0)
    }
}
impl core::fmt::Debug for Pigeon81 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon81")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon81 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon81 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon82(pub u32);
impl Pigeon82 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon82sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon82sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon82sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon82sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon82sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon82sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon82 {
    #[inline(always)]
    fn default() -> Pigeon82 {
        Pigeon82(0)
    }
}
impl core::fmt::Debug for Pigeon82 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon82")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon82 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon82 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon90(pub u32);
impl Pigeon90 {
    #[doc = "Enable pigeon Mode on this signal"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polarity of signal output"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of signal output"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event to incrment local counter"]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sel(&self) -> super::vals::Pigeon90incSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pigeon90incSel::from_bits(val as u8)
    }
    #[doc = "Event to incrment local counter"]
    #[inline(always)]
    pub const fn set_inc_sel(&mut self, val: super::vals::Pigeon90incSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "offset on pclk unit"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "offset on pclk unit"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt_sel(&self) -> super::vals::Pigeon90maskCntSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pigeon90maskCntSel::from_bits(val as u8)
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub const fn set_mask_cnt_sel(&mut self, val: super::vals::Pigeon90maskCntSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_cnt(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub const fn set_mask_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[must_use]
    #[inline(always)]
    pub const fn state_mask(&self) -> super::vals::Pigeon90stateMask {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pigeon90stateMask::from_bits(val as u8)
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub const fn set_state_mask(&mut self, val: super::vals::Pigeon90stateMask) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pigeon90 {
    #[inline(always)]
    fn default() -> Pigeon90 {
        Pigeon90(0)
    }
}
impl core::fmt::Debug for Pigeon90 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon90")
            .field("en", &self.en())
            .field("pol", &self.pol())
            .field("inc_sel", &self.inc_sel())
            .field("offset", &self.offset())
            .field("mask_cnt_sel", &self.mask_cnt_sel())
            .field("mask_cnt", &self.mask_cnt())
            .field("state_mask", &self.state_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon90 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pigeon90 {{ en: {=bool:?}, pol: {=bool:?}, inc_sel: {:?}, offset: {=u8:?}, mask_cnt_sel: {:?}, mask_cnt: {=u16:?}, state_mask: {:?} }}" , self . en () , self . pol () , self . inc_sel () , self . offset () , self . mask_cnt_sel () , self . mask_cnt () , self . state_mask ())
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon91(pub u32);
impl Pigeon91 {
    #[doc = "Assert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn set_cnt(&self) -> super::vals::Pigeon91setCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Pigeon91setCnt::from_bits(val as u16)
    }
    #[doc = "Assert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_set_cnt(&mut self, val: super::vals::Pigeon91setCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_cnt(&self) -> super::vals::Pigeon91clrCnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Pigeon91clrCnt::from_bits(val as u16)
    }
    #[doc = "Deassert signal output when counter match this value"]
    #[inline(always)]
    pub const fn set_clr_cnt(&mut self, val: super::vals::Pigeon91clrCnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pigeon91 {
    #[inline(always)]
    fn default() -> Pigeon91 {
        Pigeon91(0)
    }
}
impl core::fmt::Debug for Pigeon91 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon91")
            .field("set_cnt", &self.set_cnt())
            .field("clr_cnt", &self.clr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon91 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon91 {{ set_cnt: {:?}, clr_cnt: {:?} }}",
            self.set_cnt(),
            self.clr_cnt()
        )
    }
}
#[doc = "Panel Interface Signal Generator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeon92(pub u32);
impl Pigeon92 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_logic(&self) -> super::vals::Pigeon92sigLogic {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pigeon92sigLogic::from_bits(val as u8)
    }
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub const fn set_sig_logic(&mut self, val: super::vals::Pigeon92sigLogic) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[must_use]
    #[inline(always)]
    pub const fn sig_another(&self) -> super::vals::Pigeon92sigAnother {
        let val = (self.0 >> 4usize) & 0x1f;
        super::vals::Pigeon92sigAnother::from_bits(val as u8)
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub const fn set_sig_another(&mut self, val: super::vals::Pigeon92sigAnother) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
    }
}
impl Default for Pigeon92 {
    #[inline(always)]
    fn default() -> Pigeon92 {
        Pigeon92(0)
    }
}
impl core::fmt::Debug for Pigeon92 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeon92")
            .field("sig_logic", &self.sig_logic())
            .field("sig_another", &self.sig_another())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon92 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeon92 {{ sig_logic: {:?}, sig_another: {:?} }}",
            self.sig_logic(),
            self.sig_another()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl0(pub u32);
impl Pigeonctrl0 {
    #[doc = "Period of line counter during FD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn fd_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of line counter during FD phase"]
    #[inline(always)]
    pub const fn set_fd_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn ld_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[inline(always)]
    pub const fn set_ld_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl0 {
    #[inline(always)]
    fn default() -> Pigeonctrl0 {
        Pigeonctrl0(0)
    }
}
impl core::fmt::Debug for Pigeonctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl0")
            .field("fd_period", &self.fd_period())
            .field("ld_period", &self.ld_period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl0 {{ fd_period: {=u16:?}, ld_period: {=u16:?} }}",
            self.fd_period(),
            self.ld_period()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl0clr(pub u32);
impl Pigeonctrl0clr {
    #[doc = "Period of line counter during FD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn fd_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of line counter during FD phase"]
    #[inline(always)]
    pub const fn set_fd_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn ld_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[inline(always)]
    pub const fn set_ld_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl0clr {
    #[inline(always)]
    fn default() -> Pigeonctrl0clr {
        Pigeonctrl0clr(0)
    }
}
impl core::fmt::Debug for Pigeonctrl0clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl0clr")
            .field("fd_period", &self.fd_period())
            .field("ld_period", &self.ld_period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl0clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl0clr {{ fd_period: {=u16:?}, ld_period: {=u16:?} }}",
            self.fd_period(),
            self.ld_period()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl0set(pub u32);
impl Pigeonctrl0set {
    #[doc = "Period of line counter during FD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn fd_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of line counter during FD phase"]
    #[inline(always)]
    pub const fn set_fd_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn ld_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[inline(always)]
    pub const fn set_ld_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl0set {
    #[inline(always)]
    fn default() -> Pigeonctrl0set {
        Pigeonctrl0set(0)
    }
}
impl core::fmt::Debug for Pigeonctrl0set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl0set")
            .field("fd_period", &self.fd_period())
            .field("ld_period", &self.ld_period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl0set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl0set {{ fd_period: {=u16:?}, ld_period: {=u16:?} }}",
            self.fd_period(),
            self.ld_period()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl0tog(pub u32);
impl Pigeonctrl0tog {
    #[doc = "Period of line counter during FD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn fd_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of line counter during FD phase"]
    #[inline(always)]
    pub const fn set_fd_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[must_use]
    #[inline(always)]
    pub const fn ld_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of pclk counter during LD phase"]
    #[inline(always)]
    pub const fn set_ld_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl0tog {
    #[inline(always)]
    fn default() -> Pigeonctrl0tog {
        Pigeonctrl0tog(0)
    }
}
impl core::fmt::Debug for Pigeonctrl0tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl0tog")
            .field("fd_period", &self.fd_period())
            .field("ld_period", &self.ld_period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl0tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl0tog {{ fd_period: {=u16:?}, ld_period: {=u16:?} }}",
            self.fd_period(),
            self.ld_period()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl1(pub u32);
impl Pigeonctrl1 {
    #[doc = "Period of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Max cycles of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_cycles(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Max cycles of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl1 {
    #[inline(always)]
    fn default() -> Pigeonctrl1 {
        Pigeonctrl1(0)
    }
}
impl core::fmt::Debug for Pigeonctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl1")
            .field("frame_cnt_period", &self.frame_cnt_period())
            .field("frame_cnt_cycles", &self.frame_cnt_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl1 {{ frame_cnt_period: {=u16:?}, frame_cnt_cycles: {=u16:?} }}",
            self.frame_cnt_period(),
            self.frame_cnt_cycles()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl1clr(pub u32);
impl Pigeonctrl1clr {
    #[doc = "Period of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Max cycles of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_cycles(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Max cycles of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl1clr {
    #[inline(always)]
    fn default() -> Pigeonctrl1clr {
        Pigeonctrl1clr(0)
    }
}
impl core::fmt::Debug for Pigeonctrl1clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl1clr")
            .field("frame_cnt_period", &self.frame_cnt_period())
            .field("frame_cnt_cycles", &self.frame_cnt_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl1clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl1clr {{ frame_cnt_period: {=u16:?}, frame_cnt_cycles: {=u16:?} }}",
            self.frame_cnt_period(),
            self.frame_cnt_cycles()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl1set(pub u32);
impl Pigeonctrl1set {
    #[doc = "Period of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Max cycles of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_cycles(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Max cycles of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl1set {
    #[inline(always)]
    fn default() -> Pigeonctrl1set {
        Pigeonctrl1set(0)
    }
}
impl core::fmt::Debug for Pigeonctrl1set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl1set")
            .field("frame_cnt_period", &self.frame_cnt_period())
            .field("frame_cnt_cycles", &self.frame_cnt_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl1set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl1set {{ frame_cnt_period: {=u16:?}, frame_cnt_cycles: {=u16:?} }}",
            self.frame_cnt_period(),
            self.frame_cnt_cycles()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl1tog(pub u32);
impl Pigeonctrl1tog {
    #[doc = "Period of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Period of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Max cycles of frame counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frame_cnt_cycles(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Max cycles of frame counter"]
    #[inline(always)]
    pub const fn set_frame_cnt_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pigeonctrl1tog {
    #[inline(always)]
    fn default() -> Pigeonctrl1tog {
        Pigeonctrl1tog(0)
    }
}
impl core::fmt::Debug for Pigeonctrl1tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl1tog")
            .field("frame_cnt_period", &self.frame_cnt_period())
            .field("frame_cnt_cycles", &self.frame_cnt_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl1tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl1tog {{ frame_cnt_period: {=u16:?}, frame_cnt_cycles: {=u16:?} }}",
            self.frame_cnt_period(),
            self.frame_cnt_cycles()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl2(pub u32);
impl Pigeonctrl2 {
    #[doc = "Pigeon mode data enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_data_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode data enable"]
    #[inline(always)]
    pub const fn set_pigeon_data_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_clk_gate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[inline(always)]
    pub const fn set_pigeon_clk_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pigeonctrl2 {
    #[inline(always)]
    fn default() -> Pigeonctrl2 {
        Pigeonctrl2(0)
    }
}
impl core::fmt::Debug for Pigeonctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl2")
            .field("pigeon_data_en", &self.pigeon_data_en())
            .field("pigeon_clk_gate", &self.pigeon_clk_gate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl2 {{ pigeon_data_en: {=bool:?}, pigeon_clk_gate: {=bool:?} }}",
            self.pigeon_data_en(),
            self.pigeon_clk_gate()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl2clr(pub u32);
impl Pigeonctrl2clr {
    #[doc = "Pigeon mode data enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_data_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode data enable"]
    #[inline(always)]
    pub const fn set_pigeon_data_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_clk_gate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[inline(always)]
    pub const fn set_pigeon_clk_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pigeonctrl2clr {
    #[inline(always)]
    fn default() -> Pigeonctrl2clr {
        Pigeonctrl2clr(0)
    }
}
impl core::fmt::Debug for Pigeonctrl2clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl2clr")
            .field("pigeon_data_en", &self.pigeon_data_en())
            .field("pigeon_clk_gate", &self.pigeon_clk_gate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl2clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl2clr {{ pigeon_data_en: {=bool:?}, pigeon_clk_gate: {=bool:?} }}",
            self.pigeon_data_en(),
            self.pigeon_clk_gate()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl2set(pub u32);
impl Pigeonctrl2set {
    #[doc = "Pigeon mode data enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_data_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode data enable"]
    #[inline(always)]
    pub const fn set_pigeon_data_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_clk_gate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[inline(always)]
    pub const fn set_pigeon_clk_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pigeonctrl2set {
    #[inline(always)]
    fn default() -> Pigeonctrl2set {
        Pigeonctrl2set(0)
    }
}
impl core::fmt::Debug for Pigeonctrl2set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl2set")
            .field("pigeon_data_en", &self.pigeon_data_en())
            .field("pigeon_clk_gate", &self.pigeon_clk_gate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl2set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl2set {{ pigeon_data_en: {=bool:?}, pigeon_clk_gate: {=bool:?} }}",
            self.pigeon_data_en(),
            self.pigeon_clk_gate()
        )
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pigeonctrl2tog(pub u32);
impl Pigeonctrl2tog {
    #[doc = "Pigeon mode data enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_data_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode data enable"]
    #[inline(always)]
    pub const fn set_pigeon_data_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pigeon_clk_gate(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    #[inline(always)]
    pub const fn set_pigeon_clk_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pigeonctrl2tog {
    #[inline(always)]
    fn default() -> Pigeonctrl2tog {
        Pigeonctrl2tog(0)
    }
}
impl core::fmt::Debug for Pigeonctrl2tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pigeonctrl2tog")
            .field("pigeon_data_en", &self.pigeon_data_en())
            .field("pigeon_clk_gate", &self.pigeon_clk_gate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeonctrl2tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pigeonctrl2tog {{ pigeon_data_en: {=bool:?}, pigeon_clk_gate: {=bool:?} }}",
            self.pigeon_data_en(),
            self.pigeon_clk_gate()
        )
    }
}
#[doc = "LCD Interface Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Read only view of the current count in Latency buffer (LFIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn lfifo_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read only view of the current count in Latency buffer (LFIFO)."]
    #[inline(always)]
    pub const fn set_lfifo_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Read only view of the signals that indicates LCD TXFIFO is empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifo_empty(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Read only view of the signals that indicates LCD TXFIFO is empty."]
    #[inline(always)]
    pub const fn set_txfifo_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Read only view of the signals that indicates LCD TXFIFO is full."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifo_full(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Read only view of the signals that indicates LCD TXFIFO is full."]
    #[inline(always)]
    pub const fn set_txfifo_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Read only view of the signals that indicates LCD LFIFO is empty."]
    #[must_use]
    #[inline(always)]
    pub const fn lfifo_empty(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Read only view of the signals that indicates LCD LFIFO is empty."]
    #[inline(always)]
    pub const fn set_lfifo_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Read only view of the signals that indicates LCD LFIFO is full."]
    #[must_use]
    #[inline(always)]
    pub const fn lfifo_full(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Read only view of the signals that indicates LCD LFIFO is full."]
    #[inline(always)]
    pub const fn set_lfifo_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Reflects the current state of the DMA Request line for the LCDIF"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects the current state of the DMA Request line for the LCDIF"]
    #[inline(always)]
    pub const fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "0: LCDIF not present on this product 1: LCDIF is present."]
    #[must_use]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0: LCDIF not present on this product 1: LCDIF is present."]
    #[inline(always)]
    pub const fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("lfifo_count", &self.lfifo_count())
            .field("txfifo_empty", &self.txfifo_empty())
            .field("txfifo_full", &self.txfifo_full())
            .field("lfifo_empty", &self.lfifo_empty())
            .field("lfifo_full", &self.lfifo_full())
            .field("dma_req", &self.dma_req())
            .field("present", &self.present())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stat {{ lfifo_count: {=u16:?}, txfifo_empty: {=bool:?}, txfifo_full: {=bool:?}, lfifo_empty: {=bool:?}, lfifo_full: {=bool:?}, dma_req: {=bool:?}, present: {=bool:?} }}" , self . lfifo_count () , self . txfifo_empty () , self . txfifo_full () , self . lfifo_empty () , self . lfifo_full () , self . dma_req () , self . present ())
    }
}
#[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TransferCount(pub u32);
impl TransferCount {
    #[doc = "Total valid data (pixels) in each horizontal line"]
    #[must_use]
    #[inline(always)]
    pub const fn h_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total valid data (pixels) in each horizontal line"]
    #[inline(always)]
    pub const fn set_h_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Number of horizontal lines per frame which contain valid data"]
    #[must_use]
    #[inline(always)]
    pub const fn v_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of horizontal lines per frame which contain valid data"]
    #[inline(always)]
    pub const fn set_v_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TransferCount {
    #[inline(always)]
    fn default() -> TransferCount {
        TransferCount(0)
    }
}
impl core::fmt::Debug for TransferCount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TransferCount")
            .field("h_count", &self.h_count())
            .field("v_count", &self.v_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TransferCount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TransferCount {{ h_count: {=u16:?}, v_count: {=u16:?} }}",
            self.h_count(),
            self.v_count()
        )
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl0(pub u32);
impl Vdctrl0 {
    #[doc = "Number of units for which VSYNC signal is active"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Number of units for which VSYNC signal is active"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline(always)]
    pub const fn set_half_line_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline(always)]
    pub const fn set_half_line(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width_unit(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_period_unit(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_period_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[inline(always)]
    pub const fn set_enable_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_pol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline(always)]
    pub const fn set_dotclk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_pol(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline(always)]
    pub const fn set_hsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pol(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline(always)]
    pub const fn set_vsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_present(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline(always)]
    pub const fn set_enable_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Vdctrl0 {
    #[inline(always)]
    fn default() -> Vdctrl0 {
        Vdctrl0(0)
    }
}
impl core::fmt::Debug for Vdctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl0")
            .field("vsync_pulse_width", &self.vsync_pulse_width())
            .field("half_line_mode", &self.half_line_mode())
            .field("half_line", &self.half_line())
            .field("vsync_pulse_width_unit", &self.vsync_pulse_width_unit())
            .field("vsync_period_unit", &self.vsync_period_unit())
            .field("enable_pol", &self.enable_pol())
            .field("dotclk_pol", &self.dotclk_pol())
            .field("hsync_pol", &self.hsync_pol())
            .field("vsync_pol", &self.vsync_pol())
            .field("enable_present", &self.enable_present())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vdctrl0 {{ vsync_pulse_width: {=u32:?}, half_line_mode: {=bool:?}, half_line: {=bool:?}, vsync_pulse_width_unit: {=bool:?}, vsync_period_unit: {=bool:?}, enable_pol: {=bool:?}, dotclk_pol: {=bool:?}, hsync_pol: {=bool:?}, vsync_pol: {=bool:?}, enable_present: {=bool:?} }}" , self . vsync_pulse_width () , self . half_line_mode () , self . half_line () , self . vsync_pulse_width_unit () , self . vsync_period_unit () , self . enable_pol () , self . dotclk_pol () , self . hsync_pol () , self . vsync_pol () , self . enable_present ())
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl0clr(pub u32);
impl Vdctrl0clr {
    #[doc = "Number of units for which VSYNC signal is active"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Number of units for which VSYNC signal is active"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline(always)]
    pub const fn set_half_line_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline(always)]
    pub const fn set_half_line(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width_unit(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_period_unit(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_period_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[inline(always)]
    pub const fn set_enable_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_pol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline(always)]
    pub const fn set_dotclk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_pol(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline(always)]
    pub const fn set_hsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pol(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline(always)]
    pub const fn set_vsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_present(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline(always)]
    pub const fn set_enable_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Vdctrl0clr {
    #[inline(always)]
    fn default() -> Vdctrl0clr {
        Vdctrl0clr(0)
    }
}
impl core::fmt::Debug for Vdctrl0clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl0clr")
            .field("vsync_pulse_width", &self.vsync_pulse_width())
            .field("half_line_mode", &self.half_line_mode())
            .field("half_line", &self.half_line())
            .field("vsync_pulse_width_unit", &self.vsync_pulse_width_unit())
            .field("vsync_period_unit", &self.vsync_period_unit())
            .field("enable_pol", &self.enable_pol())
            .field("dotclk_pol", &self.dotclk_pol())
            .field("hsync_pol", &self.hsync_pol())
            .field("vsync_pol", &self.vsync_pol())
            .field("enable_present", &self.enable_present())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl0clr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vdctrl0clr {{ vsync_pulse_width: {=u32:?}, half_line_mode: {=bool:?}, half_line: {=bool:?}, vsync_pulse_width_unit: {=bool:?}, vsync_period_unit: {=bool:?}, enable_pol: {=bool:?}, dotclk_pol: {=bool:?}, hsync_pol: {=bool:?}, vsync_pol: {=bool:?}, enable_present: {=bool:?} }}" , self . vsync_pulse_width () , self . half_line_mode () , self . half_line () , self . vsync_pulse_width_unit () , self . vsync_period_unit () , self . enable_pol () , self . dotclk_pol () , self . hsync_pol () , self . vsync_pol () , self . enable_present ())
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl0set(pub u32);
impl Vdctrl0set {
    #[doc = "Number of units for which VSYNC signal is active"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Number of units for which VSYNC signal is active"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline(always)]
    pub const fn set_half_line_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline(always)]
    pub const fn set_half_line(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width_unit(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_period_unit(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_period_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[inline(always)]
    pub const fn set_enable_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_pol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline(always)]
    pub const fn set_dotclk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_pol(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline(always)]
    pub const fn set_hsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pol(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline(always)]
    pub const fn set_vsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_present(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline(always)]
    pub const fn set_enable_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Vdctrl0set {
    #[inline(always)]
    fn default() -> Vdctrl0set {
        Vdctrl0set(0)
    }
}
impl core::fmt::Debug for Vdctrl0set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl0set")
            .field("vsync_pulse_width", &self.vsync_pulse_width())
            .field("half_line_mode", &self.half_line_mode())
            .field("half_line", &self.half_line())
            .field("vsync_pulse_width_unit", &self.vsync_pulse_width_unit())
            .field("vsync_period_unit", &self.vsync_period_unit())
            .field("enable_pol", &self.enable_pol())
            .field("dotclk_pol", &self.dotclk_pol())
            .field("hsync_pol", &self.hsync_pol())
            .field("vsync_pol", &self.vsync_pol())
            .field("enable_present", &self.enable_present())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl0set {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vdctrl0set {{ vsync_pulse_width: {=u32:?}, half_line_mode: {=bool:?}, half_line: {=bool:?}, vsync_pulse_width_unit: {=bool:?}, vsync_period_unit: {=bool:?}, enable_pol: {=bool:?}, dotclk_pol: {=bool:?}, hsync_pol: {=bool:?}, vsync_pol: {=bool:?}, enable_present: {=bool:?} }}" , self . vsync_pulse_width () , self . half_line_mode () , self . half_line () , self . vsync_pulse_width_unit () , self . vsync_period_unit () , self . enable_pol () , self . dotclk_pol () , self . hsync_pol () , self . vsync_pol () , self . enable_present ())
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl0tog(pub u32);
impl Vdctrl0tog {
    #[doc = "Number of units for which VSYNC signal is active"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Number of units for which VSYNC signal is active"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline(always)]
    pub const fn set_half_line_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[must_use]
    #[inline(always)]
    pub const fn half_line(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline(always)]
    pub const fn set_half_line(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pulse_width_unit(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_pulse_width_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_period_unit(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub const fn set_vsync_period_unit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    #[inline(always)]
    pub const fn set_enable_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_pol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline(always)]
    pub const fn set_dotclk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_pol(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline(always)]
    pub const fn set_hsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_pol(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline(always)]
    pub const fn set_vsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_present(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline(always)]
    pub const fn set_enable_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Vdctrl0tog {
    #[inline(always)]
    fn default() -> Vdctrl0tog {
        Vdctrl0tog(0)
    }
}
impl core::fmt::Debug for Vdctrl0tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl0tog")
            .field("vsync_pulse_width", &self.vsync_pulse_width())
            .field("half_line_mode", &self.half_line_mode())
            .field("half_line", &self.half_line())
            .field("vsync_pulse_width_unit", &self.vsync_pulse_width_unit())
            .field("vsync_period_unit", &self.vsync_period_unit())
            .field("enable_pol", &self.enable_pol())
            .field("dotclk_pol", &self.dotclk_pol())
            .field("hsync_pol", &self.hsync_pol())
            .field("vsync_pol", &self.vsync_pol())
            .field("enable_present", &self.enable_present())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl0tog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vdctrl0tog {{ vsync_pulse_width: {=u32:?}, half_line_mode: {=bool:?}, half_line: {=bool:?}, vsync_pulse_width_unit: {=bool:?}, vsync_period_unit: {=bool:?}, enable_pol: {=bool:?}, dotclk_pol: {=bool:?}, hsync_pol: {=bool:?}, vsync_pol: {=bool:?}, enable_present: {=bool:?} }}" , self . vsync_pulse_width () , self . half_line_mode () , self . half_line () , self . vsync_pulse_width_unit () , self . vsync_period_unit () , self . enable_pol () , self . dotclk_pol () , self . hsync_pol () , self . vsync_pol () , self . enable_present ())
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl2(pub u32);
impl Vdctrl2 {
    #[doc = "Total number of DISPLAY CLOCK (pix_clk) cycles between two positive or two negative edges of the HSYNC signal"]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_period(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Total number of DISPLAY CLOCK (pix_clk) cycles between two positive or two negative edges of the HSYNC signal"]
    #[inline(always)]
    pub const fn set_hsync_period(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Number of DISPLAY CLOCK (pix_clk) cycles for which HSYNC signal is active."]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_pulse_width(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "Number of DISPLAY CLOCK (pix_clk) cycles for which HSYNC signal is active."]
    #[inline(always)]
    pub const fn set_hsync_pulse_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for Vdctrl2 {
    #[inline(always)]
    fn default() -> Vdctrl2 {
        Vdctrl2(0)
    }
}
impl core::fmt::Debug for Vdctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl2")
            .field("hsync_period", &self.hsync_period())
            .field("hsync_pulse_width", &self.hsync_pulse_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vdctrl2 {{ hsync_period: {=u32:?}, hsync_pulse_width: {=u16:?} }}",
            self.hsync_period(),
            self.hsync_pulse_width()
        )
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl3(pub u32);
impl Vdctrl3 {
    #[doc = "In the VSYNC interface mode, wait for this number of DISPLAY CLOCK (pix_clk) cycles from the falling VSYNC edge (or rising if VSYNC_POL is 1) before starting LCD transactions and is applicable only if WAIT_FOR_VSYNC_EDGE is set"]
    #[must_use]
    #[inline(always)]
    pub const fn vertical_wait_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "In the VSYNC interface mode, wait for this number of DISPLAY CLOCK (pix_clk) cycles from the falling VSYNC edge (or rising if VSYNC_POL is 1) before starting LCD transactions and is applicable only if WAIT_FOR_VSYNC_EDGE is set"]
    #[inline(always)]
    pub const fn set_vertical_wait_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "In the DOTCLK mode, wait for this number of clocks from falling edge (or rising if HSYNC_POL is 1) of HSYNC signal to account for horizontal back porch plus the number of DOTCLKs before the moving picture information begins"]
    #[must_use]
    #[inline(always)]
    pub const fn horizontal_wait_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "In the DOTCLK mode, wait for this number of clocks from falling edge (or rising if HSYNC_POL is 1) of HSYNC signal to account for horizontal back porch plus the number of DOTCLKs before the moving picture information begins"]
    #[inline(always)]
    pub const fn set_horizontal_wait_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "This bit must be set to 1 in the VSYNC mode of operation, and 0 in the DOTCLK mode of operation."]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_only(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to 1 in the VSYNC mode of operation, and 0 in the DOTCLK mode of operation."]
    #[inline(always)]
    pub const fn set_vsync_only(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "When this bit is set, the LCDIF block will internally mux HSYNC with LCD_D14, DOTCLK with LCD_D13 and ENABLE with LCD_D12, otherwise these signals will go out on separate pins"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_sync_signals(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set, the LCDIF block will internally mux HSYNC with LCD_D14, DOTCLK with LCD_D13 and ENABLE with LCD_D12, otherwise these signals will go out on separate pins"]
    #[inline(always)]
    pub const fn set_mux_sync_signals(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Vdctrl3 {
    #[inline(always)]
    fn default() -> Vdctrl3 {
        Vdctrl3(0)
    }
}
impl core::fmt::Debug for Vdctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl3")
            .field("vertical_wait_cnt", &self.vertical_wait_cnt())
            .field("horizontal_wait_cnt", &self.horizontal_wait_cnt())
            .field("vsync_only", &self.vsync_only())
            .field("mux_sync_signals", &self.mux_sync_signals())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vdctrl3 {{ vertical_wait_cnt: {=u16:?}, horizontal_wait_cnt: {=u16:?}, vsync_only: {=bool:?}, mux_sync_signals: {=bool:?} }}" , self . vertical_wait_cnt () , self . horizontal_wait_cnt () , self . vsync_only () , self . mux_sync_signals ())
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vdctrl4(pub u32);
impl Vdctrl4 {
    #[doc = "Total number of DISPLAY CLOCK (pix_clk) cycles on each horizontal line that carry valid data in DOTCLK mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_h_valid_data_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Total number of DISPLAY CLOCK (pix_clk) cycles on each horizontal line that carry valid data in DOTCLK mode"]
    #[inline(always)]
    pub const fn set_dotclk_h_valid_data_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Set this field to 1 if the LCD controller requires that the VSYNC or VSYNC/HSYNC/DOTCLK control signals should be active at least one frame before the data transfers actually start and remain active at least one frame after the data transfers end"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_signals_on(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set this field to 1 if the LCD controller requires that the VSYNC or VSYNC/HSYNC/DOTCLK control signals should be active at least one frame before the data transfers actually start and remain active at least one frame after the data transfers end"]
    #[inline(always)]
    pub const fn set_sync_signals_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "This bitfield selects the amount of time by which the DOTCLK signal should be delayed before coming out of the LCD_DOTCK pin"]
    #[must_use]
    #[inline(always)]
    pub const fn dotclk_dly_sel(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "This bitfield selects the amount of time by which the DOTCLK signal should be delayed before coming out of the LCD_DOTCK pin"]
    #[inline(always)]
    pub const fn set_dotclk_dly_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Vdctrl4 {
    #[inline(always)]
    fn default() -> Vdctrl4 {
        Vdctrl4(0)
    }
}
impl core::fmt::Debug for Vdctrl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vdctrl4")
            .field("dotclk_h_valid_data_cnt", &self.dotclk_h_valid_data_cnt())
            .field("sync_signals_on", &self.sync_signals_on())
            .field("dotclk_dly_sel", &self.dotclk_dly_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vdctrl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vdctrl4 {{ dotclk_h_valid_data_cnt: {=u32:?}, sync_signals_on: {=bool:?}, dotclk_dly_sel: {=u8:?} }}" , self . dotclk_h_valid_data_cnt () , self . sync_signals_on () , self . dotclk_dly_sel ())
    }
}
