#[doc = "Clear DONE Status Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdne(pub u8);
impl Cdne {
    #[doc = "Clear DONE field"]
    #[must_use]
    #[inline(always)]
    pub const fn cdne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear DONE field"]
    #[inline(always)]
    pub const fn set_cdne(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clears All DONE fields"]
    #[must_use]
    #[inline(always)]
    pub const fn cadn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clears All DONE fields"]
    #[inline(always)]
    pub const fn set_cadn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Cdne {
    #[inline(always)]
    fn default() -> Cdne {
        Cdne(0)
    }
}
impl core::fmt::Debug for Cdne {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdne")
            .field("cdne", &self.cdne())
            .field("cadn", &self.cadn())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdne {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdne {{ cdne: {=u8:?}, cadn: {=bool:?}, nop: {=bool:?} }}",
            self.cdne(),
            self.cadn(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceei(pub u8);
impl Ceei {
    #[doc = "Clear Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ceei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_ceei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Enable Error Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn caee(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub const fn set_caee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ceei {
    #[inline(always)]
    fn default() -> Ceei {
        Ceei(0)
    }
}
impl core::fmt::Debug for Ceei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ceei")
            .field("ceei", &self.ceei())
            .field("caee", &self.caee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ceei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ceei {{ ceei: {=u8:?}, caee: {=bool:?}, nop: {=bool:?} }}",
            self.ceei(),
            self.caee(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerq(pub u8);
impl Cerq {
    #[doc = "Clear Enable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cerq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Enable Request"]
    #[inline(always)]
    pub const fn set_cerq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Enable Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn caer(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear All Enable Requests"]
    #[inline(always)]
    pub const fn set_caer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerq {
    #[inline(always)]
    fn default() -> Cerq {
        Cerq(0)
    }
}
impl core::fmt::Debug for Cerq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerq")
            .field("cerq", &self.cerq())
            .field("caer", &self.caer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerq {{ cerq: {=u8:?}, caer: {=bool:?}, nop: {=bool:?} }}",
            self.cerq(),
            self.caer(),
            self.nop()
        )
    }
}
#[doc = "Clear Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerr(pub u8);
impl Cerr {
    #[doc = "Clear Error Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn cerr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Error Indicator"]
    #[inline(always)]
    pub const fn set_cerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Error Indicators"]
    #[must_use]
    #[inline(always)]
    pub const fn caei(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear All Error Indicators"]
    #[inline(always)]
    pub const fn set_caei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerr {
    #[inline(always)]
    fn default() -> Cerr {
        Cerr(0)
    }
}
impl core::fmt::Debug for Cerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerr")
            .field("cerr", &self.cerr())
            .field("caei", &self.caei())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerr {{ cerr: {=u8:?}, caei: {=bool:?}, nop: {=bool:?} }}",
            self.cerr(),
            self.caei(),
            self.nop()
        )
    }
}
#[doc = "Clear Interrupt Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cint(pub u8);
impl Cint {
    #[doc = "Clear Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Interrupt Request"]
    #[inline(always)]
    pub const fn set_cint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Interrupt Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn cair(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clear All Interrupt Requests"]
    #[inline(always)]
    pub const fn set_cair(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Cint {
    #[inline(always)]
    fn default() -> Cint {
        Cint(0)
    }
}
impl core::fmt::Debug for Cint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cint")
            .field("cint", &self.cint())
            .field("cair", &self.cair())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cint {{ cint: {=u8:?}, cair: {=bool:?}, nop: {=bool:?} }}",
            self.cint(),
            self.cair(),
            self.nop()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Enable Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug"]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Round Robin Group Arbitration"]
    #[must_use]
    #[inline(always)]
    pub const fn erga(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Group Arbitration"]
    #[inline(always)]
    pub const fn set_erga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Halt On Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hoe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Halt On Error"]
    #[inline(always)]
    pub const fn set_hoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt eDMA Operations"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Halt eDMA Operations"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Continuous Link Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn clm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Link Mode"]
    #[inline(always)]
    pub const fn set_clm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Minor Loop Mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn emlm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Minor Loop Mapping"]
    #[inline(always)]
    pub const fn set_emlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel Group 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn grp0pri(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Group 0 Priority"]
    #[inline(always)]
    pub const fn set_grp0pri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel Group 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn grp1pri(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Group 1 Priority"]
    #[inline(always)]
    pub const fn set_grp1pri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error Cancel Transfer"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Cancel Transfer"]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "eDMA Active Status"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "eDMA Active Status"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("erga", &self.erga())
            .field("hoe", &self.hoe())
            .field("halt", &self.halt())
            .field("clm", &self.clm())
            .field("emlm", &self.emlm())
            .field("grp0pri", &self.grp0pri())
            .field("grp1pri", &self.grp1pri())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr {{ edbg: {=bool:?}, erca: {=bool:?}, erga: {=bool:?}, hoe: {=bool:?}, halt: {=bool:?}, clm: {=bool:?}, emlm: {=bool:?}, grp0pri: {=bool:?}, grp1pri: {=bool:?}, ecx: {=bool:?}, cx: {=bool:?}, active: {=bool:?} }}" , self . edbg () , self . erca () , self . erga () , self . hoe () , self . halt () , self . clm () , self . emlm () , self . grp0pri () , self . grp1pri () , self . ecx () , self . cx () , self . active ())
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DchpriX(pub u8);
impl DchpriX {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority"]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for DchpriX {
    #[inline(always)]
    fn default() -> DchpriX {
        DchpriX(0)
    }
}
impl core::fmt::Debug for DchpriX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DchpriX")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DchpriX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DchpriX {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {=bool:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Enable Asynchronous Request in Stop"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ears(pub u32);
impl Ears {
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub const fn set_edreq_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub const fn set_edreq_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub const fn set_edreq_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub const fn set_edreq_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    #[inline(always)]
    pub const fn set_edreq_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    #[inline(always)]
    pub const fn set_edreq_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    #[inline(always)]
    pub const fn set_edreq_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    #[inline(always)]
    pub const fn set_edreq_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    #[inline(always)]
    pub const fn set_edreq_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    #[inline(always)]
    pub const fn set_edreq_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    #[inline(always)]
    pub const fn set_edreq_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    #[inline(always)]
    pub const fn set_edreq_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    #[inline(always)]
    pub const fn set_edreq_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    #[inline(always)]
    pub const fn set_edreq_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    #[inline(always)]
    pub const fn set_edreq_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    #[inline(always)]
    pub const fn set_edreq_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 16."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 16."]
    #[inline(always)]
    pub const fn set_edreq_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 17."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 17."]
    #[inline(always)]
    pub const fn set_edreq_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 18."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 18."]
    #[inline(always)]
    pub const fn set_edreq_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 19."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 19."]
    #[inline(always)]
    pub const fn set_edreq_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 20."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 20."]
    #[inline(always)]
    pub const fn set_edreq_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 21."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 21."]
    #[inline(always)]
    pub const fn set_edreq_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 22."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 22."]
    #[inline(always)]
    pub const fn set_edreq_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 23."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 23."]
    #[inline(always)]
    pub const fn set_edreq_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 24."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 24."]
    #[inline(always)]
    pub const fn set_edreq_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 25."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 25."]
    #[inline(always)]
    pub const fn set_edreq_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 26."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 26."]
    #[inline(always)]
    pub const fn set_edreq_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 27."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 27."]
    #[inline(always)]
    pub const fn set_edreq_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 28."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 28."]
    #[inline(always)]
    pub const fn set_edreq_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 29."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 29."]
    #[inline(always)]
    pub const fn set_edreq_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 30."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 30."]
    #[inline(always)]
    pub const fn set_edreq_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 31."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 31."]
    #[inline(always)]
    pub const fn set_edreq_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ears {
    #[inline(always)]
    fn default() -> Ears {
        Ears(0)
    }
}
impl core::fmt::Debug for Ears {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ears")
            .field("edreq_0", &self.edreq_0())
            .field("edreq_1", &self.edreq_1())
            .field("edreq_2", &self.edreq_2())
            .field("edreq_3", &self.edreq_3())
            .field("edreq_4", &self.edreq_4())
            .field("edreq_5", &self.edreq_5())
            .field("edreq_6", &self.edreq_6())
            .field("edreq_7", &self.edreq_7())
            .field("edreq_8", &self.edreq_8())
            .field("edreq_9", &self.edreq_9())
            .field("edreq_10", &self.edreq_10())
            .field("edreq_11", &self.edreq_11())
            .field("edreq_12", &self.edreq_12())
            .field("edreq_13", &self.edreq_13())
            .field("edreq_14", &self.edreq_14())
            .field("edreq_15", &self.edreq_15())
            .field("edreq_16", &self.edreq_16())
            .field("edreq_17", &self.edreq_17())
            .field("edreq_18", &self.edreq_18())
            .field("edreq_19", &self.edreq_19())
            .field("edreq_20", &self.edreq_20())
            .field("edreq_21", &self.edreq_21())
            .field("edreq_22", &self.edreq_22())
            .field("edreq_23", &self.edreq_23())
            .field("edreq_24", &self.edreq_24())
            .field("edreq_25", &self.edreq_25())
            .field("edreq_26", &self.edreq_26())
            .field("edreq_27", &self.edreq_27())
            .field("edreq_28", &self.edreq_28())
            .field("edreq_29", &self.edreq_29())
            .field("edreq_30", &self.edreq_30())
            .field("edreq_31", &self.edreq_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ears {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ears {{ edreq_0: {=bool:?}, edreq_1: {=bool:?}, edreq_2: {=bool:?}, edreq_3: {=bool:?}, edreq_4: {=bool:?}, edreq_5: {=bool:?}, edreq_6: {=bool:?}, edreq_7: {=bool:?}, edreq_8: {=bool:?}, edreq_9: {=bool:?}, edreq_10: {=bool:?}, edreq_11: {=bool:?}, edreq_12: {=bool:?}, edreq_13: {=bool:?}, edreq_14: {=bool:?}, edreq_15: {=bool:?}, edreq_16: {=bool:?}, edreq_17: {=bool:?}, edreq_18: {=bool:?}, edreq_19: {=bool:?}, edreq_20: {=bool:?}, edreq_21: {=bool:?}, edreq_22: {=bool:?}, edreq_23: {=bool:?}, edreq_24: {=bool:?}, edreq_25: {=bool:?}, edreq_26: {=bool:?}, edreq_27: {=bool:?}, edreq_28: {=bool:?}, edreq_29: {=bool:?}, edreq_30: {=bool:?}, edreq_31: {=bool:?} }}" , self . edreq_0 () , self . edreq_1 () , self . edreq_2 () , self . edreq_3 () , self . edreq_4 () , self . edreq_5 () , self . edreq_6 () , self . edreq_7 () , self . edreq_8 () , self . edreq_9 () , self . edreq_10 () , self . edreq_11 () , self . edreq_12 () , self . edreq_13 () , self . edreq_14 () , self . edreq_15 () , self . edreq_16 () , self . edreq_17 () , self . edreq_18 () , self . edreq_19 () , self . edreq_20 () , self . edreq_21 () , self . edreq_22 () , self . edreq_23 () , self . edreq_24 () , self . edreq_25 () , self . edreq_26 () , self . edreq_27 () , self . edreq_28 () , self . edreq_29 () , self . edreq_30 () , self . edreq_31 ())
    }
}
#[doc = "Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eei(pub u32);
impl Eei {
    #[doc = "Enable Error Interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn eei0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 0"]
    #[inline(always)]
    pub const fn set_eei0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Error Interrupt 1"]
    #[must_use]
    #[inline(always)]
    pub const fn eei1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 1"]
    #[inline(always)]
    pub const fn set_eei1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Error Interrupt 2"]
    #[must_use]
    #[inline(always)]
    pub const fn eei2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 2"]
    #[inline(always)]
    pub const fn set_eei2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Error Interrupt 3"]
    #[must_use]
    #[inline(always)]
    pub const fn eei3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 3"]
    #[inline(always)]
    pub const fn set_eei3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Error Interrupt 4"]
    #[must_use]
    #[inline(always)]
    pub const fn eei4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 4"]
    #[inline(always)]
    pub const fn set_eei4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable Error Interrupt 5"]
    #[must_use]
    #[inline(always)]
    pub const fn eei5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 5"]
    #[inline(always)]
    pub const fn set_eei5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Error Interrupt 6"]
    #[must_use]
    #[inline(always)]
    pub const fn eei6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 6"]
    #[inline(always)]
    pub const fn set_eei6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Error Interrupt 7"]
    #[must_use]
    #[inline(always)]
    pub const fn eei7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 7"]
    #[inline(always)]
    pub const fn set_eei7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable Error Interrupt 8"]
    #[must_use]
    #[inline(always)]
    pub const fn eei8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 8"]
    #[inline(always)]
    pub const fn set_eei8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable Error Interrupt 9"]
    #[must_use]
    #[inline(always)]
    pub const fn eei9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 9"]
    #[inline(always)]
    pub const fn set_eei9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Error Interrupt 10"]
    #[must_use]
    #[inline(always)]
    pub const fn eei10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 10"]
    #[inline(always)]
    pub const fn set_eei10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Error Interrupt 11"]
    #[must_use]
    #[inline(always)]
    pub const fn eei11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 11"]
    #[inline(always)]
    pub const fn set_eei11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Error Interrupt 12"]
    #[must_use]
    #[inline(always)]
    pub const fn eei12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 12"]
    #[inline(always)]
    pub const fn set_eei12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable Error Interrupt 13"]
    #[must_use]
    #[inline(always)]
    pub const fn eei13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 13"]
    #[inline(always)]
    pub const fn set_eei13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable Error Interrupt 14"]
    #[must_use]
    #[inline(always)]
    pub const fn eei14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 14"]
    #[inline(always)]
    pub const fn set_eei14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Error Interrupt 15"]
    #[must_use]
    #[inline(always)]
    pub const fn eei15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 15"]
    #[inline(always)]
    pub const fn set_eei15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable Error Interrupt 16"]
    #[must_use]
    #[inline(always)]
    pub const fn eei16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 16"]
    #[inline(always)]
    pub const fn set_eei16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable Error Interrupt 17"]
    #[must_use]
    #[inline(always)]
    pub const fn eei17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 17"]
    #[inline(always)]
    pub const fn set_eei17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable Error Interrupt 18"]
    #[must_use]
    #[inline(always)]
    pub const fn eei18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 18"]
    #[inline(always)]
    pub const fn set_eei18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable Error Interrupt 19"]
    #[must_use]
    #[inline(always)]
    pub const fn eei19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 19"]
    #[inline(always)]
    pub const fn set_eei19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable Error Interrupt 20"]
    #[must_use]
    #[inline(always)]
    pub const fn eei20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 20"]
    #[inline(always)]
    pub const fn set_eei20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable Error Interrupt 21"]
    #[must_use]
    #[inline(always)]
    pub const fn eei21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 21"]
    #[inline(always)]
    pub const fn set_eei21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable Error Interrupt 22"]
    #[must_use]
    #[inline(always)]
    pub const fn eei22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 22"]
    #[inline(always)]
    pub const fn set_eei22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable Error Interrupt 23"]
    #[must_use]
    #[inline(always)]
    pub const fn eei23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 23"]
    #[inline(always)]
    pub const fn set_eei23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable Error Interrupt 24"]
    #[must_use]
    #[inline(always)]
    pub const fn eei24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 24"]
    #[inline(always)]
    pub const fn set_eei24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable Error Interrupt 25"]
    #[must_use]
    #[inline(always)]
    pub const fn eei25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 25"]
    #[inline(always)]
    pub const fn set_eei25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable Error Interrupt 26"]
    #[must_use]
    #[inline(always)]
    pub const fn eei26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 26"]
    #[inline(always)]
    pub const fn set_eei26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable Error Interrupt 27"]
    #[must_use]
    #[inline(always)]
    pub const fn eei27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 27"]
    #[inline(always)]
    pub const fn set_eei27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable Error Interrupt 28"]
    #[must_use]
    #[inline(always)]
    pub const fn eei28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 28"]
    #[inline(always)]
    pub const fn set_eei28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable Error Interrupt 29"]
    #[must_use]
    #[inline(always)]
    pub const fn eei29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 29"]
    #[inline(always)]
    pub const fn set_eei29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable Error Interrupt 30"]
    #[must_use]
    #[inline(always)]
    pub const fn eei30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 30"]
    #[inline(always)]
    pub const fn set_eei30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable Error Interrupt 31"]
    #[must_use]
    #[inline(always)]
    pub const fn eei31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 31"]
    #[inline(always)]
    pub const fn set_eei31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eei {
    #[inline(always)]
    fn default() -> Eei {
        Eei(0)
    }
}
impl core::fmt::Debug for Eei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eei")
            .field("eei0", &self.eei0())
            .field("eei1", &self.eei1())
            .field("eei2", &self.eei2())
            .field("eei3", &self.eei3())
            .field("eei4", &self.eei4())
            .field("eei5", &self.eei5())
            .field("eei6", &self.eei6())
            .field("eei7", &self.eei7())
            .field("eei8", &self.eei8())
            .field("eei9", &self.eei9())
            .field("eei10", &self.eei10())
            .field("eei11", &self.eei11())
            .field("eei12", &self.eei12())
            .field("eei13", &self.eei13())
            .field("eei14", &self.eei14())
            .field("eei15", &self.eei15())
            .field("eei16", &self.eei16())
            .field("eei17", &self.eei17())
            .field("eei18", &self.eei18())
            .field("eei19", &self.eei19())
            .field("eei20", &self.eei20())
            .field("eei21", &self.eei21())
            .field("eei22", &self.eei22())
            .field("eei23", &self.eei23())
            .field("eei24", &self.eei24())
            .field("eei25", &self.eei25())
            .field("eei26", &self.eei26())
            .field("eei27", &self.eei27())
            .field("eei28", &self.eei28())
            .field("eei29", &self.eei29())
            .field("eei30", &self.eei30())
            .field("eei31", &self.eei31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eei {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Eei {{ eei0: {=bool:?}, eei1: {=bool:?}, eei2: {=bool:?}, eei3: {=bool:?}, eei4: {=bool:?}, eei5: {=bool:?}, eei6: {=bool:?}, eei7: {=bool:?}, eei8: {=bool:?}, eei9: {=bool:?}, eei10: {=bool:?}, eei11: {=bool:?}, eei12: {=bool:?}, eei13: {=bool:?}, eei14: {=bool:?}, eei15: {=bool:?}, eei16: {=bool:?}, eei17: {=bool:?}, eei18: {=bool:?}, eei19: {=bool:?}, eei20: {=bool:?}, eei21: {=bool:?}, eei22: {=bool:?}, eei23: {=bool:?}, eei24: {=bool:?}, eei25: {=bool:?}, eei26: {=bool:?}, eei27: {=bool:?}, eei28: {=bool:?}, eei29: {=bool:?}, eei30: {=bool:?}, eei31: {=bool:?} }}" , self . eei0 () , self . eei1 () , self . eei2 () , self . eei3 () , self . eei4 () , self . eei5 () , self . eei6 () , self . eei7 () , self . eei8 () , self . eei9 () , self . eei10 () , self . eei11 () , self . eei12 () , self . eei13 () , self . eei14 () , self . eei15 () , self . eei16 () , self . eei17 () , self . eei18 () , self . eei19 () , self . eei20 () , self . eei21 () , self . eei22 () , self . eei23 () , self . eei24 () , self . eei25 () , self . eei26 () , self . eei27 () , self . eei28 () , self . eei29 () , self . eei30 () , self . eei31 ())
    }
}
#[doc = "Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erq(pub u32);
impl Erq {
    #[doc = "Enable DMA Request 0"]
    #[must_use]
    #[inline(always)]
    pub const fn erq(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 0"]
    #[inline(always)]
    pub const fn set_erq(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Erq {
    #[inline(always)]
    fn default() -> Erq {
        Erq(0)
    }
}
impl core::fmt::Debug for Erq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erq")
            .field("erq[0]", &self.erq(0usize))
            .field("erq[1]", &self.erq(1usize))
            .field("erq[2]", &self.erq(2usize))
            .field("erq[3]", &self.erq(3usize))
            .field("erq[4]", &self.erq(4usize))
            .field("erq[5]", &self.erq(5usize))
            .field("erq[6]", &self.erq(6usize))
            .field("erq[7]", &self.erq(7usize))
            .field("erq[8]", &self.erq(8usize))
            .field("erq[9]", &self.erq(9usize))
            .field("erq[10]", &self.erq(10usize))
            .field("erq[11]", &self.erq(11usize))
            .field("erq[12]", &self.erq(12usize))
            .field("erq[13]", &self.erq(13usize))
            .field("erq[14]", &self.erq(14usize))
            .field("erq[15]", &self.erq(15usize))
            .field("erq[16]", &self.erq(16usize))
            .field("erq[17]", &self.erq(17usize))
            .field("erq[18]", &self.erq(18usize))
            .field("erq[19]", &self.erq(19usize))
            .field("erq[20]", &self.erq(20usize))
            .field("erq[21]", &self.erq(21usize))
            .field("erq[22]", &self.erq(22usize))
            .field("erq[23]", &self.erq(23usize))
            .field("erq[24]", &self.erq(24usize))
            .field("erq[25]", &self.erq(25usize))
            .field("erq[26]", &self.erq(26usize))
            .field("erq[27]", &self.erq(27usize))
            .field("erq[28]", &self.erq(28usize))
            .field("erq[29]", &self.erq(29usize))
            .field("erq[30]", &self.erq(30usize))
            .field("erq[31]", &self.erq(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Erq {{ erq[0]: {=bool:?}, erq[1]: {=bool:?}, erq[2]: {=bool:?}, erq[3]: {=bool:?}, erq[4]: {=bool:?}, erq[5]: {=bool:?}, erq[6]: {=bool:?}, erq[7]: {=bool:?}, erq[8]: {=bool:?}, erq[9]: {=bool:?}, erq[10]: {=bool:?}, erq[11]: {=bool:?}, erq[12]: {=bool:?}, erq[13]: {=bool:?}, erq[14]: {=bool:?}, erq[15]: {=bool:?}, erq[16]: {=bool:?}, erq[17]: {=bool:?}, erq[18]: {=bool:?}, erq[19]: {=bool:?}, erq[20]: {=bool:?}, erq[21]: {=bool:?}, erq[22]: {=bool:?}, erq[23]: {=bool:?}, erq[24]: {=bool:?}, erq[25]: {=bool:?}, erq[26]: {=bool:?}, erq[27]: {=bool:?}, erq[28]: {=bool:?}, erq[29]: {=bool:?}, erq[30]: {=bool:?}, erq[31]: {=bool:?} }}" , self . erq (0usize) , self . erq (1usize) , self . erq (2usize) , self . erq (3usize) , self . erq (4usize) , self . erq (5usize) , self . erq (6usize) , self . erq (7usize) , self . erq (8usize) , self . erq (9usize) , self . erq (10usize) , self . erq (11usize) , self . erq (12usize) , self . erq (13usize) , self . erq (14usize) , self . erq (15usize) , self . erq (16usize) , self . erq (17usize) , self . erq (18usize) , self . erq (19usize) , self . erq (20usize) , self . erq (21usize) , self . erq (22usize) , self . erq (23usize) , self . erq (24usize) , self . erq (25usize) , self . erq (26usize) , self . erq (27usize) , self . erq (28usize) , self . erq (29usize) , self . erq (30usize) , self . erq (31usize))
    }
}
#[doc = "Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Err(pub u32);
impl Err {
    #[doc = "Error In Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 0"]
    #[inline(always)]
    pub const fn set_err(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Err {
    #[inline(always)]
    fn default() -> Err {
        Err(0)
    }
}
impl core::fmt::Debug for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Err")
            .field("err[0]", &self.err(0usize))
            .field("err[1]", &self.err(1usize))
            .field("err[2]", &self.err(2usize))
            .field("err[3]", &self.err(3usize))
            .field("err[4]", &self.err(4usize))
            .field("err[5]", &self.err(5usize))
            .field("err[6]", &self.err(6usize))
            .field("err[7]", &self.err(7usize))
            .field("err[8]", &self.err(8usize))
            .field("err[9]", &self.err(9usize))
            .field("err[10]", &self.err(10usize))
            .field("err[11]", &self.err(11usize))
            .field("err[12]", &self.err(12usize))
            .field("err[13]", &self.err(13usize))
            .field("err[14]", &self.err(14usize))
            .field("err[15]", &self.err(15usize))
            .field("err[16]", &self.err(16usize))
            .field("err[17]", &self.err(17usize))
            .field("err[18]", &self.err(18usize))
            .field("err[19]", &self.err(19usize))
            .field("err[20]", &self.err(20usize))
            .field("err[21]", &self.err(21usize))
            .field("err[22]", &self.err(22usize))
            .field("err[23]", &self.err(23usize))
            .field("err[24]", &self.err(24usize))
            .field("err[25]", &self.err(25usize))
            .field("err[26]", &self.err(26usize))
            .field("err[27]", &self.err(27usize))
            .field("err[28]", &self.err(28usize))
            .field("err[29]", &self.err(29usize))
            .field("err[30]", &self.err(30usize))
            .field("err[31]", &self.err(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Err {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Err {{ err[0]: {=bool:?}, err[1]: {=bool:?}, err[2]: {=bool:?}, err[3]: {=bool:?}, err[4]: {=bool:?}, err[5]: {=bool:?}, err[6]: {=bool:?}, err[7]: {=bool:?}, err[8]: {=bool:?}, err[9]: {=bool:?}, err[10]: {=bool:?}, err[11]: {=bool:?}, err[12]: {=bool:?}, err[13]: {=bool:?}, err[14]: {=bool:?}, err[15]: {=bool:?}, err[16]: {=bool:?}, err[17]: {=bool:?}, err[18]: {=bool:?}, err[19]: {=bool:?}, err[20]: {=bool:?}, err[21]: {=bool:?}, err[22]: {=bool:?}, err[23]: {=bool:?}, err[24]: {=bool:?}, err[25]: {=bool:?}, err[26]: {=bool:?}, err[27]: {=bool:?}, err[28]: {=bool:?}, err[29]: {=bool:?}, err[30]: {=bool:?}, err[31]: {=bool:?} }}" , self . err (0usize) , self . err (1usize) , self . err (2usize) , self . err (3usize) , self . err (4usize) , self . err (5usize) , self . err (6usize) , self . err (7usize) , self . err (8usize) , self . err (9usize) , self . err (10usize) , self . err (11usize) , self . err (12usize) , self . err (13usize) , self . err (14usize) , self . err (15usize) , self . err (16usize) , self . err (17usize) , self . err (18usize) , self . err (19usize) , self . err (20usize) , self . err (21usize) , self . err (22usize) , self . err (23usize) , self . err (24usize) , self . err (25usize) , self . err (26usize) , self . err (27usize) , self . err (28usize) , self . err (29usize) , self . err (30usize) , self . err (31usize))
    }
}
#[doc = "Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Es(pub u32);
impl Es {
    #[doc = "Destination Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Bus Error"]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Bus Error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Offset Error"]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Error"]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source Offset Error"]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Error"]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Channel Priority Error"]
    #[must_use]
    #[inline(always)]
    pub const fn cpe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Priority Error"]
    #[inline(always)]
    pub const fn set_cpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Group Priority Error"]
    #[must_use]
    #[inline(always)]
    pub const fn gpe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Group Priority Error"]
    #[inline(always)]
    pub const fn set_gpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transfer Canceled"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Canceled"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Logical OR of all ERR status fields"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Logical OR of all ERR status fields"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Es {
    #[inline(always)]
    fn default() -> Es {
        Es(0)
    }
}
impl core::fmt::Debug for Es {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Es")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("errchn", &self.errchn())
            .field("cpe", &self.cpe())
            .field("gpe", &self.gpe())
            .field("ecx", &self.ecx())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Es {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Es {{ dbe: {=bool:?}, sbe: {=bool:?}, sge: {=bool:?}, nce: {=bool:?}, doe: {=bool:?}, dae: {=bool:?}, soe: {=bool:?}, sae: {=bool:?}, errchn: {=u8:?}, cpe: {=bool:?}, gpe: {=bool:?}, ecx: {=bool:?}, vld: {=bool:?} }}" , self . dbe () , self . sbe () , self . sge () , self . nce () , self . doe () , self . dae () , self . soe () , self . sae () , self . errchn () , self . cpe () , self . gpe () , self . ecx () , self . vld ())
    }
}
#[doc = "Hardware Request Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrs(pub u32);
impl Hrs {
    #[doc = "Hardware Request Status Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 0"]
    #[inline(always)]
    pub const fn set_hrs(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Hrs {
    #[inline(always)]
    fn default() -> Hrs {
        Hrs(0)
    }
}
impl core::fmt::Debug for Hrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrs")
            .field("hrs[0]", &self.hrs(0usize))
            .field("hrs[1]", &self.hrs(1usize))
            .field("hrs[2]", &self.hrs(2usize))
            .field("hrs[3]", &self.hrs(3usize))
            .field("hrs[4]", &self.hrs(4usize))
            .field("hrs[5]", &self.hrs(5usize))
            .field("hrs[6]", &self.hrs(6usize))
            .field("hrs[7]", &self.hrs(7usize))
            .field("hrs[8]", &self.hrs(8usize))
            .field("hrs[9]", &self.hrs(9usize))
            .field("hrs[10]", &self.hrs(10usize))
            .field("hrs[11]", &self.hrs(11usize))
            .field("hrs[12]", &self.hrs(12usize))
            .field("hrs[13]", &self.hrs(13usize))
            .field("hrs[14]", &self.hrs(14usize))
            .field("hrs[15]", &self.hrs(15usize))
            .field("hrs[16]", &self.hrs(16usize))
            .field("hrs[17]", &self.hrs(17usize))
            .field("hrs[18]", &self.hrs(18usize))
            .field("hrs[19]", &self.hrs(19usize))
            .field("hrs[20]", &self.hrs(20usize))
            .field("hrs[21]", &self.hrs(21usize))
            .field("hrs[22]", &self.hrs(22usize))
            .field("hrs[23]", &self.hrs(23usize))
            .field("hrs[24]", &self.hrs(24usize))
            .field("hrs[25]", &self.hrs(25usize))
            .field("hrs[26]", &self.hrs(26usize))
            .field("hrs[27]", &self.hrs(27usize))
            .field("hrs[28]", &self.hrs(28usize))
            .field("hrs[29]", &self.hrs(29usize))
            .field("hrs[30]", &self.hrs(30usize))
            .field("hrs[31]", &self.hrs(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrs {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hrs {{ hrs[0]: {=bool:?}, hrs[1]: {=bool:?}, hrs[2]: {=bool:?}, hrs[3]: {=bool:?}, hrs[4]: {=bool:?}, hrs[5]: {=bool:?}, hrs[6]: {=bool:?}, hrs[7]: {=bool:?}, hrs[8]: {=bool:?}, hrs[9]: {=bool:?}, hrs[10]: {=bool:?}, hrs[11]: {=bool:?}, hrs[12]: {=bool:?}, hrs[13]: {=bool:?}, hrs[14]: {=bool:?}, hrs[15]: {=bool:?}, hrs[16]: {=bool:?}, hrs[17]: {=bool:?}, hrs[18]: {=bool:?}, hrs[19]: {=bool:?}, hrs[20]: {=bool:?}, hrs[21]: {=bool:?}, hrs[22]: {=bool:?}, hrs[23]: {=bool:?}, hrs[24]: {=bool:?}, hrs[25]: {=bool:?}, hrs[26]: {=bool:?}, hrs[27]: {=bool:?}, hrs[28]: {=bool:?}, hrs[29]: {=bool:?}, hrs[30]: {=bool:?}, hrs[31]: {=bool:?} }}" , self . hrs (0usize) , self . hrs (1usize) , self . hrs (2usize) , self . hrs (3usize) , self . hrs (4usize) , self . hrs (5usize) , self . hrs (6usize) , self . hrs (7usize) , self . hrs (8usize) , self . hrs (9usize) , self . hrs (10usize) , self . hrs (11usize) , self . hrs (12usize) , self . hrs (13usize) , self . hrs (14usize) , self . hrs (15usize) , self . hrs (16usize) , self . hrs (17usize) , self . hrs (18usize) , self . hrs (19usize) , self . hrs (20usize) , self . hrs (21usize) , self . hrs (22usize) , self . hrs (23usize) , self . hrs (24usize) , self . hrs (25usize) , self . hrs (26usize) , self . hrs (27usize) , self . hrs (28usize) , self . hrs (29usize) , self . hrs (30usize) , self . hrs (31usize))
    }
}
#[doc = "Interrupt Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Interrupt Request 0"]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 0"]
    #[inline(always)]
    pub const fn set_int(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("int[0]", &self.int(0usize))
            .field("int[1]", &self.int(1usize))
            .field("int[2]", &self.int(2usize))
            .field("int[3]", &self.int(3usize))
            .field("int[4]", &self.int(4usize))
            .field("int[5]", &self.int(5usize))
            .field("int[6]", &self.int(6usize))
            .field("int[7]", &self.int(7usize))
            .field("int[8]", &self.int(8usize))
            .field("int[9]", &self.int(9usize))
            .field("int[10]", &self.int(10usize))
            .field("int[11]", &self.int(11usize))
            .field("int[12]", &self.int(12usize))
            .field("int[13]", &self.int(13usize))
            .field("int[14]", &self.int(14usize))
            .field("int[15]", &self.int(15usize))
            .field("int[16]", &self.int(16usize))
            .field("int[17]", &self.int(17usize))
            .field("int[18]", &self.int(18usize))
            .field("int[19]", &self.int(19usize))
            .field("int[20]", &self.int(20usize))
            .field("int[21]", &self.int(21usize))
            .field("int[22]", &self.int(22usize))
            .field("int[23]", &self.int(23usize))
            .field("int[24]", &self.int(24usize))
            .field("int[25]", &self.int(25usize))
            .field("int[26]", &self.int(26usize))
            .field("int[27]", &self.int(27usize))
            .field("int[28]", &self.int(28usize))
            .field("int[29]", &self.int(29usize))
            .field("int[30]", &self.int(30usize))
            .field("int[31]", &self.int(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int {{ int[0]: {=bool:?}, int[1]: {=bool:?}, int[2]: {=bool:?}, int[3]: {=bool:?}, int[4]: {=bool:?}, int[5]: {=bool:?}, int[6]: {=bool:?}, int[7]: {=bool:?}, int[8]: {=bool:?}, int[9]: {=bool:?}, int[10]: {=bool:?}, int[11]: {=bool:?}, int[12]: {=bool:?}, int[13]: {=bool:?}, int[14]: {=bool:?}, int[15]: {=bool:?}, int[16]: {=bool:?}, int[17]: {=bool:?}, int[18]: {=bool:?}, int[19]: {=bool:?}, int[20]: {=bool:?}, int[21]: {=bool:?}, int[22]: {=bool:?}, int[23]: {=bool:?}, int[24]: {=bool:?}, int[25]: {=bool:?}, int[26]: {=bool:?}, int[27]: {=bool:?}, int[28]: {=bool:?}, int[29]: {=bool:?}, int[30]: {=bool:?}, int[31]: {=bool:?} }}" , self . int (0usize) , self . int (1usize) , self . int (2usize) , self . int (3usize) , self . int (4usize) , self . int (5usize) , self . int (6usize) , self . int (7usize) , self . int (8usize) , self . int (9usize) , self . int (10usize) , self . int (11usize) , self . int (12usize) , self . int (13usize) , self . int (14usize) , self . int (15usize) , self . int (16usize) , self . int (17usize) , self . int (18usize) , self . int (19usize) , self . int (20usize) , self . int (21usize) , self . int (22usize) , self . int (23usize) , self . int (24usize) , self . int (25usize) , self . int (26usize) , self . int (27usize) , self . int (28usize) , self . int (29usize) , self . int (30usize) , self . int (31usize))
    }
}
#[doc = "Set Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seei(pub u8);
impl Seei {
    #[doc = "Set Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn seei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_seei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All Enable Error Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn saee(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set All Enable Error Interrupts"]
    #[inline(always)]
    pub const fn set_saee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Seei {
    #[inline(always)]
    fn default() -> Seei {
        Seei(0)
    }
}
impl core::fmt::Debug for Seei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seei")
            .field("seei", &self.seei())
            .field("saee", &self.saee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seei {{ seei: {=u8:?}, saee: {=bool:?}, nop: {=bool:?} }}",
            self.seei(),
            self.saee(),
            self.nop()
        )
    }
}
#[doc = "Set Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serq(pub u8);
impl Serq {
    #[doc = "Set Enable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn serq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set Enable Request"]
    #[inline(always)]
    pub const fn set_serq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All Enable Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn saer(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set All Enable Requests"]
    #[inline(always)]
    pub const fn set_saer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Serq {
    #[inline(always)]
    fn default() -> Serq {
        Serq(0)
    }
}
impl core::fmt::Debug for Serq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serq")
            .field("serq", &self.serq())
            .field("saer", &self.saer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Serq {{ serq: {=u8:?}, saer: {=bool:?}, nop: {=bool:?} }}",
            self.serq(),
            self.saer(),
            self.nop()
        )
    }
}
#[doc = "Set START Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssrt(pub u8);
impl Ssrt {
    #[doc = "Set START field"]
    #[must_use]
    #[inline(always)]
    pub const fn ssrt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set START field"]
    #[inline(always)]
    pub const fn set_ssrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All START fields (activates all channels)"]
    #[must_use]
    #[inline(always)]
    pub const fn sast(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set All START fields (activates all channels)"]
    #[inline(always)]
    pub const fn set_sast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ssrt {
    #[inline(always)]
    fn default() -> Ssrt {
        Ssrt(0)
    }
}
impl core::fmt::Debug for Ssrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssrt")
            .field("ssrt", &self.ssrt())
            .field("sast", &self.sast())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrt {{ ssrt: {=u8:?}, sast: {=bool:?}, nop: {=bool:?} }}",
            self.ssrt(),
            self.sast(),
            self.nop()
        )
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::TcdAttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::TcdAttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::TcdAttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for TcdAttr {
    #[inline(always)]
    fn default() -> TcdAttr {
        TcdAttr(0)
    }
}
impl core::fmt::Debug for TcdAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdAttr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdAttr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdAttr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkno {
    #[inline(always)]
    fn default() -> TcdBiterElinkno {
        TcdBiterElinkno(0)
    }
}
impl core::fmt::Debug for TcdBiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkyes {
    #[inline(always)]
    fn default() -> TcdBiterElinkyes {
        TcdBiterElinkyes(0)
    }
}
impl core::fmt::Debug for TcdBiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkno {
    #[inline(always)]
    fn default() -> TcdCiterElinkno {
        TcdCiterElinkno(0)
    }
}
impl core::fmt::Debug for TcdCiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkyes {
    #[inline(always)]
    fn default() -> TcdCiterElinkyes {
        TcdCiterElinkyes(0)
    }
}
impl core::fmt::Debug for TcdCiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::TcdCsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::TcdCsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::TcdCsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for TcdCsr {
    #[inline(always)]
    fn default() -> TcdCsr {
        TcdCsr(0)
    }
}
impl core::fmt::Debug for TcdCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCsr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TcdCsr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {=bool:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}" , self . start () , self . intmajor () , self . inthalf () , self . dreq () , self . esg () , self . majorelink () , self . active () , self . done () , self . majorlinkch () , self . bwc ())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffno {
    #[inline(always)]
    fn default() -> TcdNbytesMloffno {
        TcdNbytesMloffno(0)
    }
}
impl core::fmt::Debug for TcdNbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffyes {
    #[inline(always)]
    fn default() -> TcdNbytesMloffyes {
        TcdNbytesMloffyes(0)
    }
}
impl core::fmt::Debug for TcdNbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TcdNbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}" , self . nbytes () , self . mloff () , self . dmloe () , self . smloe ())
    }
}
