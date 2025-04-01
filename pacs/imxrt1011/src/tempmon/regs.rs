#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense0(pub u32);
impl Tempsense0 {
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn measure_temp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0 {
    #[inline(always)]
    fn default() -> Tempsense0 {
        Tempsense0(0)
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense0clr(pub u32);
impl Tempsense0clr {
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn measure_temp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0clr {
    #[inline(always)]
    fn default() -> Tempsense0clr {
        Tempsense0clr(0)
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense0set(pub u32);
impl Tempsense0set {
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn measure_temp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0set {
    #[inline(always)]
    fn default() -> Tempsense0set {
        Tempsense0set(0)
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense0tog(pub u32);
impl Tempsense0tog {
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit powers down the temperature sensor."]
    #[inline(always)]
    pub const fn set_power_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn measure_temp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Starts the measurement process"]
    #[inline(always)]
    pub const fn set_measure_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn finished(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the latest temp is valid"]
    #[inline(always)]
    pub const fn set_finished(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn temp_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub const fn set_temp_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn alarm_value(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Tempsense0tog {
    #[inline(always)]
    fn default() -> Tempsense0tog {
        Tempsense0tog(0)
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense1(pub u32);
impl Tempsense1 {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1 {
    #[inline(always)]
    fn default() -> Tempsense1 {
        Tempsense1(0)
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense1clr(pub u32);
impl Tempsense1clr {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1clr {
    #[inline(always)]
    fn default() -> Tempsense1clr {
        Tempsense1clr(0)
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense1set(pub u32);
impl Tempsense1set {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1set {
    #[inline(always)]
    fn default() -> Tempsense1set {
        Tempsense1set(0)
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense1tog(pub u32);
impl Tempsense1tog {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn measure_freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1tog {
    #[inline(always)]
    fn default() -> Tempsense1tog {
        Tempsense1tog(0)
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense2(pub u32);
impl Tempsense2 {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2 {
    #[inline(always)]
    fn default() -> Tempsense2 {
        Tempsense2(0)
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense2clr(pub u32);
impl Tempsense2clr {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2clr {
    #[inline(always)]
    fn default() -> Tempsense2clr {
        Tempsense2clr(0)
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense2set(pub u32);
impl Tempsense2set {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2set {
    #[inline(always)]
    fn default() -> Tempsense2set {
        Tempsense2set(0)
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tempsense2tog(pub u32);
impl Tempsense2tog {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn low_alarm_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub const fn set_low_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn panic_alarm_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub const fn set_panic_alarm_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Tempsense2tog {
    #[inline(always)]
    fn default() -> Tempsense2tog {
        Tempsense2tog(0)
    }
}
