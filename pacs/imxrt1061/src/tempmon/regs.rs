#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0(pub u32);
impl Tempsense0 {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tempsense0 {{ power_down: {=bool:?}, measure_temp: {=bool:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}" , self . power_down () , self . measure_temp () , self . finished () , self . temp_cnt () , self . alarm_value ())
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0clr(pub u32);
impl Tempsense0clr {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense0clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0clr")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0clr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tempsense0clr {{ power_down: {=bool:?}, measure_temp: {=bool:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}" , self . power_down () , self . measure_temp () , self . finished () , self . temp_cnt () , self . alarm_value ())
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0set(pub u32);
impl Tempsense0set {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense0set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0set")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0set {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tempsense0set {{ power_down: {=bool:?}, measure_temp: {=bool:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}" , self . power_down () , self . measure_temp () , self . finished () , self . temp_cnt () , self . alarm_value ())
    }
}
#[doc = "Tempsensor Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense0tog(pub u32);
impl Tempsense0tog {
    #[doc = "This bit powers down the temperature sensor."]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense0tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense0tog")
            .field("power_down", &self.power_down())
            .field("measure_temp", &self.measure_temp())
            .field("finished", &self.finished())
            .field("temp_cnt", &self.temp_cnt())
            .field("alarm_value", &self.alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense0tog {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tempsense0tog {{ power_down: {=bool:?}, measure_temp: {=bool:?}, finished: {=bool:?}, temp_cnt: {=u16:?}, alarm_value: {=u16:?} }}" , self . power_down () , self . measure_temp () , self . finished () , self . temp_cnt () , self . alarm_value ())
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1(pub u32);
impl Tempsense1 {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> super::vals::Tempsense1measureFreq {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Tempsense1measureFreq::from_bits(val as u16)
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: super::vals::Tempsense1measureFreq) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1 {
    #[inline(always)]
    fn default() -> Tempsense1 {
        Tempsense1(0)
    }
}
impl core::fmt::Debug for Tempsense1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1 {{ measure_freq: {:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1clr(pub u32);
impl Tempsense1clr {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> super::vals::Tempsense1clrMeasureFreq {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Tempsense1clrMeasureFreq::from_bits(val as u16)
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: super::vals::Tempsense1clrMeasureFreq) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1clr {
    #[inline(always)]
    fn default() -> Tempsense1clr {
        Tempsense1clr(0)
    }
}
impl core::fmt::Debug for Tempsense1clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1clr")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1clr {{ measure_freq: {:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1set(pub u32);
impl Tempsense1set {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> super::vals::Tempsense1setMeasureFreq {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Tempsense1setMeasureFreq::from_bits(val as u16)
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: super::vals::Tempsense1setMeasureFreq) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1set {
    #[inline(always)]
    fn default() -> Tempsense1set {
        Tempsense1set(0)
    }
}
impl core::fmt::Debug for Tempsense1set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1set")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1set {{ measure_freq: {:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense1tog(pub u32);
impl Tempsense1tog {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_freq(&self) -> super::vals::Tempsense1togMeasureFreq {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Tempsense1togMeasureFreq::from_bits(val as u16)
    }
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub const fn set_measure_freq(&mut self, val: super::vals::Tempsense1togMeasureFreq) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tempsense1tog {
    #[inline(always)]
    fn default() -> Tempsense1tog {
        Tempsense1tog(0)
    }
}
impl core::fmt::Debug for Tempsense1tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense1tog")
            .field("measure_freq", &self.measure_freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense1tog {{ measure_freq: {:?} }}",
            self.measure_freq()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2(pub u32);
impl Tempsense2 {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2 {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2clr(pub u32);
impl Tempsense2clr {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense2clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2clr")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2clr {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2set(pub u32);
impl Tempsense2set {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense2set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2set")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2set {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
#[doc = "Tempsensor Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsense2tog(pub u32);
impl Tempsense2tog {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[must_use]
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
    #[must_use]
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
impl core::fmt::Debug for Tempsense2tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsense2tog")
            .field("low_alarm_value", &self.low_alarm_value())
            .field("panic_alarm_value", &self.panic_alarm_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense2tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tempsense2tog {{ low_alarm_value: {=u16:?}, panic_alarm_value: {=u16:?} }}",
            self.low_alarm_value(),
            self.panic_alarm_value()
        )
    }
}
