#[doc = "Auxiliary Control Register,"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Actlr(pub u32);
impl Actlr {
    #[doc = "Disables folding of IT instructions."]
    #[inline(always)]
    pub const fn disfold(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disables folding of IT instructions."]
    #[inline(always)]
    pub const fn set_disfold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Disables FPU exception outputs."]
    #[inline(always)]
    pub const fn fpexcodis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Disables FPU exception outputs."]
    #[inline(always)]
    pub const fn set_fpexcodis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline(always)]
    pub const fn disramode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline(always)]
    pub const fn set_disramode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Disables ITM and DWT ATB flush."]
    #[inline(always)]
    pub const fn disitmatbflush(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Disables ITM and DWT ATB flush."]
    #[inline(always)]
    pub const fn set_disitmatbflush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Disables BTAC read."]
    #[inline(always)]
    pub const fn disbtacread(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Disables BTAC read."]
    #[inline(always)]
    pub const fn set_disbtacread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Disables BTAC allocate."]
    #[inline(always)]
    pub const fn disbtacalloc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Disables BTAC allocate."]
    #[inline(always)]
    pub const fn set_disbtacalloc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Disables critical AXI Read-Under-Read."]
    #[inline(always)]
    pub const fn discritaxirur(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Disables critical AXI Read-Under-Read."]
    #[inline(always)]
    pub const fn set_discritaxirur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Disables dual-issued."]
    #[inline(always)]
    pub const fn disdi(&self) -> super::vals::Disdi {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Disdi::from_bits(val as u8)
    }
    #[doc = "Disables dual-issued."]
    #[inline(always)]
    pub const fn set_disdi(&mut self, val: super::vals::Disdi) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Disables dual-issued."]
    #[inline(always)]
    pub const fn disissch1(&self) -> super::vals::Disissch1 {
        let val = (self.0 >> 21usize) & 0x1f;
        super::vals::Disissch1::from_bits(val as u8)
    }
    #[doc = "Disables dual-issued."]
    #[inline(always)]
    pub const fn set_disissch1(&mut self, val: super::vals::Disissch1) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val.to_bits() as u32) & 0x1f) << 21usize);
    }
    #[doc = "Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub const fn disdynadd(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub const fn set_disdynadd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Disables critical AXI read-under-write"]
    #[inline(always)]
    pub const fn discritaxiruw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Disables critical AXI read-under-write"]
    #[inline(always)]
    pub const fn set_discritaxiruw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Disables critical AXI read-under-write"]
    #[inline(always)]
    pub const fn disfpuissopt(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Disables critical AXI read-under-write"]
    #[inline(always)]
    pub const fn set_disfpuissopt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Actlr {
    #[inline(always)]
    fn default() -> Actlr {
        Actlr(0)
    }
}
#[doc = "Application Interrupt and Reset Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Aircr(pub u32);
impl Aircr {
    #[doc = "Writing 1 to this bit causes a local system reset"]
    #[inline(always)]
    pub const fn vectreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 to this bit causes a local system reset"]
    #[inline(always)]
    pub const fn set_vectreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing 1 to this bit clears all active state information for fixed and configurable exceptions."]
    #[inline(always)]
    pub const fn vectclractive(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 to this bit clears all active state information for fixed and configurable exceptions."]
    #[inline(always)]
    pub const fn set_vectclractive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System reset request"]
    #[inline(always)]
    pub const fn sysresetreq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "System reset request"]
    #[inline(always)]
    pub const fn set_sysresetreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub const fn prigroup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub const fn set_prigroup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Data endianness"]
    #[inline(always)]
    pub const fn endianness(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data endianness"]
    #[inline(always)]
    pub const fn set_endianness(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Register key"]
    #[inline(always)]
    pub const fn vectkey(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key"]
    #[inline(always)]
    pub const fn set_vectkey(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Aircr {
    #[inline(always)]
    fn default() -> Aircr {
        Aircr(0)
    }
}
#[doc = "Configuration and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Indicates how the processor enters Thread mode"]
    #[inline(always)]
    pub const fn nonbasethrdena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates how the processor enters Thread mode"]
    #[inline(always)]
    pub const fn set_nonbasethrdena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub const fn usersetmpend(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub const fn set_usersetmpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables unaligned access traps"]
    #[inline(always)]
    pub const fn unalign_trp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables unaligned access traps"]
    #[inline(always)]
    pub const fn set_unalign_trp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub const fn div_0_trp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub const fn set_div_0_trp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub const fn bfhfnmign(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub const fn set_bfhfnmign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub const fn stkalign(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub const fn set_stkalign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enables L1 data cache."]
    #[inline(always)]
    pub const fn dc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables L1 data cache."]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables L1 instruction cache."]
    #[inline(always)]
    pub const fn ic(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables L1 instruction cache."]
    #[inline(always)]
    pub const fn set_ic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Always reads-as-one. It indicates branch prediction is enabled."]
    #[inline(always)]
    pub const fn bp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Always reads-as-one. It indicates branch prediction is enabled."]
    #[inline(always)]
    pub const fn set_bp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
#[doc = "Cache Size ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccsidr(pub u32);
impl Ccsidr {
    #[doc = "(Log2(Number of words in cache line)) - 2."]
    #[inline(always)]
    pub const fn linesize(&self) -> super::vals::Linesize {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Linesize::from_bits(val as u8)
    }
    #[doc = "(Log2(Number of words in cache line)) - 2."]
    #[inline(always)]
    pub const fn set_linesize(&mut self, val: super::vals::Linesize) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "(Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1. The associativity does not have to be a power of 2."]
    #[inline(always)]
    pub const fn associativity(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x03ff;
        val as u16
    }
    #[doc = "(Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1. The associativity does not have to be a power of 2."]
    #[inline(always)]
    pub const fn set_associativity(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 3usize)) | (((val as u32) & 0x03ff) << 3usize);
    }
    #[doc = "(Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The number of sets does not have to be a power of 2."]
    #[inline(always)]
    pub const fn numsets(&self) -> u16 {
        let val = (self.0 >> 13usize) & 0x7fff;
        val as u16
    }
    #[doc = "(Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The number of sets does not have to be a power of 2."]
    #[inline(always)]
    pub const fn set_numsets(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 13usize)) | (((val as u32) & 0x7fff) << 13usize);
    }
    #[doc = "Indicates whether the cache level supports write-allocation"]
    #[inline(always)]
    pub const fn wa(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the cache level supports write-allocation"]
    #[inline(always)]
    pub const fn set_wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates whether the cache level supports read-allocation"]
    #[inline(always)]
    pub const fn ra(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the cache level supports read-allocation"]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Indicates whether the cache level supports write-back"]
    #[inline(always)]
    pub const fn wb(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the cache level supports write-back"]
    #[inline(always)]
    pub const fn set_wb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicates whether the cache level supports write-through"]
    #[inline(always)]
    pub const fn wt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the cache level supports write-through"]
    #[inline(always)]
    pub const fn set_wt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ccsidr {
    #[inline(always)]
    fn default() -> Ccsidr {
        Ccsidr(0)
    }
}
#[doc = "Configurable Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cfsr(pub u32);
impl Cfsr {
    #[doc = "Instruction access violation flag"]
    #[inline(always)]
    pub const fn iaccviol(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction access violation flag"]
    #[inline(always)]
    pub const fn set_iaccviol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data access violation flag"]
    #[inline(always)]
    pub const fn daccviol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data access violation flag"]
    #[inline(always)]
    pub const fn set_daccviol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub const fn munstkerr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub const fn set_munstkerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub const fn mstkerr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub const fn set_mstkerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub const fn mlsperr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub const fn set_mlsperr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub const fn mmarvalid(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub const fn set_mmarvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Instruction bus error"]
    #[inline(always)]
    pub const fn ibuserr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction bus error"]
    #[inline(always)]
    pub const fn set_ibuserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Precise data bus error"]
    #[inline(always)]
    pub const fn preciserr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Precise data bus error"]
    #[inline(always)]
    pub const fn set_preciserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Imprecise data bus error"]
    #[inline(always)]
    pub const fn impreciserr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Imprecise data bus error"]
    #[inline(always)]
    pub const fn set_impreciserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub const fn unstkerr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub const fn set_unstkerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BusFault on stacking for exception entry"]
    #[inline(always)]
    pub const fn stkerr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault on stacking for exception entry"]
    #[inline(always)]
    pub const fn set_stkerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub const fn lsperr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub const fn set_lsperr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub const fn bfarvalid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub const fn set_bfarvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Undefined instruction UsageFault"]
    #[inline(always)]
    pub const fn undefinstr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Undefined instruction UsageFault"]
    #[inline(always)]
    pub const fn set_undefinstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Invalid state UsageFault"]
    #[inline(always)]
    pub const fn invstate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid state UsageFault"]
    #[inline(always)]
    pub const fn set_invstate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    #[inline(always)]
    pub const fn invpc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    #[inline(always)]
    pub const fn set_invpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "No coprocessor UsageFault"]
    #[inline(always)]
    pub const fn nocp(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "No coprocessor UsageFault"]
    #[inline(always)]
    pub const fn set_nocp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Unaligned access UsageFault"]
    #[inline(always)]
    pub const fn unaligned(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Unaligned access UsageFault"]
    #[inline(always)]
    pub const fn set_unaligned(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Divide by zero UsageFault"]
    #[inline(always)]
    pub const fn divbyzero(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Divide by zero UsageFault"]
    #[inline(always)]
    pub const fn set_divbyzero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Cfsr {
    #[inline(always)]
    fn default() -> Cfsr {
        Cfsr(0)
    }
}
#[doc = "Cache Level ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clidr(pub u32);
impl Clidr {
    #[doc = "Indicate the type of cache implemented at level 1."]
    #[inline(always)]
    pub const fn cl1(&self) -> super::vals::Cl1 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cl1::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 1."]
    #[inline(always)]
    pub const fn set_cl1(&mut self, val: super::vals::Cl1) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Indicate the type of cache implemented at level 2."]
    #[inline(always)]
    pub const fn cl2(&self) -> super::vals::Cl2 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Cl2::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 2."]
    #[inline(always)]
    pub const fn set_cl2(&mut self, val: super::vals::Cl2) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Indicate the type of cache implemented at level 3."]
    #[inline(always)]
    pub const fn cl3(&self) -> super::vals::Cl3 {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Cl3::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 3."]
    #[inline(always)]
    pub const fn set_cl3(&mut self, val: super::vals::Cl3) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Indicate the type of cache implemented at level 4."]
    #[inline(always)]
    pub const fn cl4(&self) -> super::vals::Cl4 {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Cl4::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 4."]
    #[inline(always)]
    pub const fn set_cl4(&mut self, val: super::vals::Cl4) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Indicate the type of cache implemented at level 5."]
    #[inline(always)]
    pub const fn cl5(&self) -> super::vals::Cl5 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cl5::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 5."]
    #[inline(always)]
    pub const fn set_cl5(&mut self, val: super::vals::Cl5) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Indicate the type of cache implemented at level 6."]
    #[inline(always)]
    pub const fn cl6(&self) -> super::vals::Cl6 {
        let val = (self.0 >> 15usize) & 0x07;
        super::vals::Cl6::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 6."]
    #[inline(always)]
    pub const fn set_cl6(&mut self, val: super::vals::Cl6) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Indicate the type of cache implemented at level 7."]
    #[inline(always)]
    pub const fn cl7(&self) -> super::vals::Cl7 {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::Cl7::from_bits(val as u8)
    }
    #[doc = "Indicate the type of cache implemented at level 7."]
    #[inline(always)]
    pub const fn set_cl7(&mut self, val: super::vals::Cl7) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ."]
    #[inline(always)]
    pub const fn louis(&self) -> super::vals::Louis {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Louis::from_bits(val as u8)
    }
    #[doc = "Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ."]
    #[inline(always)]
    pub const fn set_louis(&mut self, val: super::vals::Louis) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Level of Coherency for the cache hierarchy"]
    #[inline(always)]
    pub const fn loc(&self) -> super::vals::Loc {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Loc::from_bits(val as u8)
    }
    #[doc = "Level of Coherency for the cache hierarchy"]
    #[inline(always)]
    pub const fn set_loc(&mut self, val: super::vals::Loc) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Level of Unification for the cache hierarchy"]
    #[inline(always)]
    pub const fn lou(&self) -> super::vals::Lou {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Lou::from_bits(val as u8)
    }
    #[doc = "Level of Unification for the cache hierarchy"]
    #[inline(always)]
    pub const fn set_lou(&mut self, val: super::vals::Lou) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
}
impl Default for Clidr {
    #[inline(always)]
    fn default() -> Clidr {
        Clidr(0)
    }
}
#[doc = "Auxiliary Bus Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7abfsr(pub u32);
impl Cm7abfsr {
    #[doc = "Asynchronous fault on ITCM interface."]
    #[inline(always)]
    pub const fn itcm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous fault on ITCM interface."]
    #[inline(always)]
    pub const fn set_itcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Asynchronous fault on DTCM interface."]
    #[inline(always)]
    pub const fn dtcm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous fault on DTCM interface."]
    #[inline(always)]
    pub const fn set_dtcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Asynchronous fault on AHBP interface."]
    #[inline(always)]
    pub const fn ahbp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous fault on AHBP interface."]
    #[inline(always)]
    pub const fn set_ahbp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Asynchronous fault on AXIM interface."]
    #[inline(always)]
    pub const fn axim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous fault on AXIM interface."]
    #[inline(always)]
    pub const fn set_axim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Asynchronous fault on EPPB interface."]
    #[inline(always)]
    pub const fn eppb(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous fault on EPPB interface."]
    #[inline(always)]
    pub const fn set_eppb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline(always)]
    pub const fn aximtype(&self) -> super::vals::Aximtype {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Aximtype::from_bits(val as u8)
    }
    #[doc = "Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline(always)]
    pub const fn set_aximtype(&mut self, val: super::vals::Aximtype) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Cm7abfsr {
    #[inline(always)]
    fn default() -> Cm7abfsr {
        Cm7abfsr(0)
    }
}
#[doc = "AHBP Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7ahbpcr(pub u32);
impl Cm7ahbpcr {
    #[doc = "AHBP enable."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AHBP enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AHBP size."]
    #[inline(always)]
    pub const fn sz(&self) -> super::vals::Cm7ahbpcrSz {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Cm7ahbpcrSz::from_bits(val as u8)
    }
    #[doc = "AHBP size."]
    #[inline(always)]
    pub const fn set_sz(&mut self, val: super::vals::Cm7ahbpcrSz) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Cm7ahbpcr {
    #[inline(always)]
    fn default() -> Cm7ahbpcr {
        Cm7ahbpcr(0)
    }
}
#[doc = "AHB Slave Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7ahbscr(pub u32);
impl Cm7ahbscr {
    #[doc = "AHBS prioritization control."]
    #[inline(always)]
    pub const fn ctl(&self) -> super::vals::Ctl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctl::from_bits(val as u8)
    }
    #[doc = "AHBS prioritization control."]
    #[inline(always)]
    pub const fn set_ctl(&mut self, val: super::vals::Ctl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Threshold execution priority for AHBS traffic demotion."]
    #[inline(always)]
    pub const fn tpri(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x01ff;
        val as u16
    }
    #[doc = "Threshold execution priority for AHBS traffic demotion."]
    #[inline(always)]
    pub const fn set_tpri(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
    }
    #[doc = "Fairness counter initialization value."]
    #[inline(always)]
    pub const fn initcount(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fairness counter initialization value."]
    #[inline(always)]
    pub const fn set_initcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for Cm7ahbscr {
    #[inline(always)]
    fn default() -> Cm7ahbscr {
        Cm7ahbscr(0)
    }
}
#[doc = "L1 Cache Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7cacr(pub u32);
impl Cm7cacr {
    #[doc = "Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline(always)]
    pub const fn siwt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline(always)]
    pub const fn set_siwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub const fn eccdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub const fn set_eccdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables Force Write-Through in the data cache."]
    #[inline(always)]
    pub const fn forcewt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables Force Write-Through in the data cache."]
    #[inline(always)]
    pub const fn set_forcewt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cm7cacr {
    #[inline(always)]
    fn default() -> Cm7cacr {
        Cm7cacr(0)
    }
}
#[doc = "Data Tightly-Coupled Memory Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7dtcmcr(pub u32);
impl Cm7dtcmcr {
    #[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub const fn rmw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub const fn set_rmw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub const fn reten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub const fn set_reten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TCM size. Indicates the size of the relevant TCM."]
    #[inline(always)]
    pub const fn sz(&self) -> super::vals::Cm7dtcmcrSz {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Cm7dtcmcrSz::from_bits(val as u8)
    }
    #[doc = "TCM size. Indicates the size of the relevant TCM."]
    #[inline(always)]
    pub const fn set_sz(&mut self, val: super::vals::Cm7dtcmcrSz) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
}
impl Default for Cm7dtcmcr {
    #[inline(always)]
    fn default() -> Cm7dtcmcr {
        Cm7dtcmcr(0)
    }
}
#[doc = "Instruction Tightly-Coupled Memory Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cm7itcmcr(pub u32);
impl Cm7itcmcr {
    #[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub const fn rmw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub const fn set_rmw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub const fn reten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub const fn set_reten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TCM size. Indicates the size of the relevant TCM."]
    #[inline(always)]
    pub const fn sz(&self) -> super::vals::Cm7itcmcrSz {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Cm7itcmcrSz::from_bits(val as u8)
    }
    #[doc = "TCM size. Indicates the size of the relevant TCM."]
    #[inline(always)]
    pub const fn set_sz(&mut self, val: super::vals::Cm7itcmcrSz) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
}
impl Default for Cm7itcmcr {
    #[inline(always)]
    fn default() -> Cm7itcmcr {
        Cm7itcmcr(0)
    }
}
#[doc = "Coprocessor Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cpacr(pub u32);
impl Cpacr {
    #[doc = "Access privileges for coprocessor 0."]
    #[inline(always)]
    pub const fn cp0(&self) -> super::vals::Cp0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cp0::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 0."]
    #[inline(always)]
    pub const fn set_cp0(&mut self, val: super::vals::Cp0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Access privileges for coprocessor 1."]
    #[inline(always)]
    pub const fn cp1(&self) -> super::vals::Cp1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Cp1::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 1."]
    #[inline(always)]
    pub const fn set_cp1(&mut self, val: super::vals::Cp1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Access privileges for coprocessor 2."]
    #[inline(always)]
    pub const fn cp2(&self) -> super::vals::Cp2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cp2::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 2."]
    #[inline(always)]
    pub const fn set_cp2(&mut self, val: super::vals::Cp2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Access privileges for coprocessor 3."]
    #[inline(always)]
    pub const fn cp3(&self) -> super::vals::Cp3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Cp3::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 3."]
    #[inline(always)]
    pub const fn set_cp3(&mut self, val: super::vals::Cp3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Access privileges for coprocessor 4."]
    #[inline(always)]
    pub const fn cp4(&self) -> super::vals::Cp4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Cp4::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 4."]
    #[inline(always)]
    pub const fn set_cp4(&mut self, val: super::vals::Cp4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Access privileges for coprocessor 5."]
    #[inline(always)]
    pub const fn cp5(&self) -> super::vals::Cp5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Cp5::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 5."]
    #[inline(always)]
    pub const fn set_cp5(&mut self, val: super::vals::Cp5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Access privileges for coprocessor 6."]
    #[inline(always)]
    pub const fn cp6(&self) -> super::vals::Cp6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Cp6::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 6."]
    #[inline(always)]
    pub const fn set_cp6(&mut self, val: super::vals::Cp6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Access privileges for coprocessor 7."]
    #[inline(always)]
    pub const fn cp7(&self) -> super::vals::Cp7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Cp7::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 7."]
    #[inline(always)]
    pub const fn set_cp7(&mut self, val: super::vals::Cp7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Access privileges for coprocessor 10."]
    #[inline(always)]
    pub const fn cp10(&self) -> super::vals::Cp10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Cp10::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 10."]
    #[inline(always)]
    pub const fn set_cp10(&mut self, val: super::vals::Cp10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Access privileges for coprocessor 11."]
    #[inline(always)]
    pub const fn cp11(&self) -> super::vals::Cp11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Cp11::from_bits(val as u8)
    }
    #[doc = "Access privileges for coprocessor 11."]
    #[inline(always)]
    pub const fn set_cp11(&mut self, val: super::vals::Cp11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
}
impl Default for Cpacr {
    #[inline(always)]
    fn default() -> Cpacr {
        Cpacr(0)
    }
}
#[doc = "CPUID Base Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cpuid(pub u32);
impl Cpuid {
    #[doc = "Indicates patch release: 0x0 = Patch 0"]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates patch release: 0x0 = Patch 0"]
    #[inline(always)]
    pub const fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates part number"]
    #[inline(always)]
    pub const fn partno(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Indicates part number"]
    #[inline(always)]
    pub const fn set_partno(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "ARCHITECTURE"]
    #[inline(always)]
    pub const fn architecture(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "ARCHITECTURE"]
    #[inline(always)]
    pub const fn set_architecture(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicates processor revision: 0x2 = Revision 2"]
    #[inline(always)]
    pub const fn variant(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates processor revision: 0x2 = Revision 2"]
    #[inline(always)]
    pub const fn set_variant(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Implementer code"]
    #[inline(always)]
    pub const fn implementer(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Implementer code"]
    #[inline(always)]
    pub const fn set_implementer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cpuid {
    #[inline(always)]
    fn default() -> Cpuid {
        Cpuid(0)
    }
}
#[doc = "Cache Size Selection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Csselr(pub u32);
impl Csselr {
    #[doc = "Instruction not data bit"]
    #[inline(always)]
    pub const fn ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction not data bit"]
    #[inline(always)]
    pub const fn set_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cache level of required cache"]
    #[inline(always)]
    pub const fn level(&self) -> super::vals::Level {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Level::from_bits(val as u8)
    }
    #[doc = "Cache level of required cache"]
    #[inline(always)]
    pub const fn set_level(&mut self, val: super::vals::Level) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Csselr {
    #[inline(always)]
    fn default() -> Csselr {
        Csselr(0)
    }
}
#[doc = "Cache Type register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctr(pub u32);
impl Ctr {
    #[doc = "Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
    #[inline(always)]
    pub const fn iminline(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
    #[inline(always)]
    pub const fn set_iminline(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
    #[inline(always)]
    pub const fn dminline(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
    #[inline(always)]
    pub const fn set_dminline(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub const fn erg(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub const fn set_erg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub const fn cwg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub const fn set_cwg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Indicates the implemented CTR format."]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::Format {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::Format::from_bits(val as u8)
    }
    #[doc = "Indicates the implemented CTR format."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::Format) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for Ctr {
    #[inline(always)]
    fn default() -> Ctr {
        Ctr(0)
    }
}
#[doc = "Debug Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dfsr(pub u32);
impl Dfsr {
    #[doc = "Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
    #[inline(always)]
    pub const fn halted(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
    #[inline(always)]
    pub const fn set_halted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    #[inline(always)]
    pub const fn bkpt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    #[inline(always)]
    pub const fn set_bkpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug event generated by the DWT"]
    #[inline(always)]
    pub const fn dwttrap(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug event generated by the DWT"]
    #[inline(always)]
    pub const fn set_dwttrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates triggering of a Vector catch"]
    #[inline(always)]
    pub const fn vcatch(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates triggering of a Vector catch"]
    #[inline(always)]
    pub const fn set_vcatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Debug event generated because of the assertion of an external debug request"]
    #[inline(always)]
    pub const fn external(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Debug event generated because of the assertion of an external debug request"]
    #[inline(always)]
    pub const fn set_external(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Dfsr {
    #[inline(always)]
    fn default() -> Dfsr {
        Dfsr(0)
    }
}
#[doc = "HardFault Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hfsr(pub u32);
impl Hfsr {
    #[doc = "Indicates a BusFault on a vector table read during exception processing."]
    #[inline(always)]
    pub const fn vecttbl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a BusFault on a vector table read during exception processing."]
    #[inline(always)]
    pub const fn set_vecttbl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    #[inline(always)]
    pub const fn forced(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    #[inline(always)]
    pub const fn set_forced(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline(always)]
    pub const fn debugevt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline(always)]
    pub const fn set_debugevt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hfsr {
    #[inline(always)]
    fn default() -> Hfsr {
        Hfsr(0)
    }
}
#[doc = "Interrupt Control and State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Icsr(pub u32);
impl Icsr {
    #[doc = "Active exception number"]
    #[inline(always)]
    pub const fn vectactive(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Active exception number"]
    #[inline(always)]
    pub const fn set_vectactive(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Indicates whether there are preempted active exceptions"]
    #[inline(always)]
    pub const fn rettobase(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether there are preempted active exceptions"]
    #[inline(always)]
    pub const fn set_rettobase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub const fn vectpending(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x01ff;
        val as u16
    }
    #[doc = "Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub const fn set_vectpending(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
    }
    #[doc = "Interrupt pending flag, excluding NMI and Faults"]
    #[inline(always)]
    pub const fn isrpending(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending flag, excluding NMI and Faults"]
    #[inline(always)]
    pub const fn set_isrpending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SysTick exception clear-pending bit"]
    #[inline(always)]
    pub const fn pendstclr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception clear-pending bit"]
    #[inline(always)]
    pub const fn set_pendstclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SysTick exception set-pending bit"]
    #[inline(always)]
    pub const fn pendstset(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception set-pending bit"]
    #[inline(always)]
    pub const fn set_pendstset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "PendSV clear-pending bit"]
    #[inline(always)]
    pub const fn pendsvclr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "PendSV clear-pending bit"]
    #[inline(always)]
    pub const fn set_pendsvclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "PendSV set-pending bit"]
    #[inline(always)]
    pub const fn pendsvset(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "PendSV set-pending bit"]
    #[inline(always)]
    pub const fn set_pendsvset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "NMI set-pending bit"]
    #[inline(always)]
    pub const fn nmipendset(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "NMI set-pending bit"]
    #[inline(always)]
    pub const fn set_nmipendset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Icsr {
    #[inline(always)]
    fn default() -> Icsr {
        Icsr(0)
    }
}
#[doc = "Auxiliary Feature Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdAfr0(pub u32);
impl IdAfr0 {
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn implementation_defined0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn set_implementation_defined0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn implementation_defined1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn set_implementation_defined1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn implementation_defined2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn set_implementation_defined2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn implementation_defined3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub const fn set_implementation_defined3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for IdAfr0 {
    #[inline(always)]
    fn default() -> IdAfr0 {
        IdAfr0(0)
    }
}
#[doc = "Debug Feature Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdDfr0(pub u32);
impl IdDfr0 {
    #[doc = "Support for memory-mapped debug model for M profile processors"]
    #[inline(always)]
    pub const fn debugmodel(&self) -> super::vals::Debugmodel {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Debugmodel::from_bits(val as u8)
    }
    #[doc = "Support for memory-mapped debug model for M profile processors"]
    #[inline(always)]
    pub const fn set_debugmodel(&mut self, val: super::vals::Debugmodel) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for IdDfr0 {
    #[inline(always)]
    fn default() -> IdDfr0 {
        IdDfr0(0)
    }
}
#[doc = "Instruction Set Attributes Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdIsar0(pub u32);
impl IdIsar0 {
    #[doc = "Indicates the supported Bit Counting instructions"]
    #[inline(always)]
    pub const fn bitcount_instrs(&self) -> super::vals::BitcountInstrs {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::BitcountInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Bit Counting instructions"]
    #[inline(always)]
    pub const fn set_bitcount_instrs(&mut self, val: super::vals::BitcountInstrs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the supported BitField instructions"]
    #[inline(always)]
    pub const fn bitfield_instrs(&self) -> super::vals::BitfieldInstrs {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::BitfieldInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported BitField instructions"]
    #[inline(always)]
    pub const fn set_bitfield_instrs(&mut self, val: super::vals::BitfieldInstrs) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicates the supported combined Compare and Branch instructions"]
    #[inline(always)]
    pub const fn cmpbranch_instrs(&self) -> super::vals::CmpbranchInstrs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::CmpbranchInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported combined Compare and Branch instructions"]
    #[inline(always)]
    pub const fn set_cmpbranch_instrs(&mut self, val: super::vals::CmpbranchInstrs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicates the supported Coprocessor instructions"]
    #[inline(always)]
    pub const fn coproc_instrs(&self) -> super::vals::CoprocInstrs {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CoprocInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Coprocessor instructions"]
    #[inline(always)]
    pub const fn set_coproc_instrs(&mut self, val: super::vals::CoprocInstrs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicates the supported Debug instructions"]
    #[inline(always)]
    pub const fn debug_instrs(&self) -> super::vals::DebugInstrs {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::DebugInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Debug instructions"]
    #[inline(always)]
    pub const fn set_debug_instrs(&mut self, val: super::vals::DebugInstrs) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the supported Divide instructions"]
    #[inline(always)]
    pub const fn divide_instrs(&self) -> super::vals::DivideInstrs {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::DivideInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Divide instructions"]
    #[inline(always)]
    pub const fn set_divide_instrs(&mut self, val: super::vals::DivideInstrs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for IdIsar0 {
    #[inline(always)]
    fn default() -> IdIsar0 {
        IdIsar0(0)
    }
}
#[doc = "Instruction Set Attributes Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdIsar1(pub u32);
impl IdIsar1 {
    #[doc = "Indicates the supported Extend instructions"]
    #[inline(always)]
    pub const fn extend_instrs(&self) -> super::vals::ExtendInstrs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::ExtendInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Extend instructions"]
    #[inline(always)]
    pub const fn set_extend_instrs(&mut self, val: super::vals::ExtendInstrs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicates the supported IfThen instructions"]
    #[inline(always)]
    pub const fn ifthen_instrs(&self) -> super::vals::IfthenInstrs {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::IfthenInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported IfThen instructions"]
    #[inline(always)]
    pub const fn set_ifthen_instrs(&mut self, val: super::vals::IfthenInstrs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicates the support for data-processing instructions with long immediate"]
    #[inline(always)]
    pub const fn immediate_instrs(&self) -> super::vals::ImmediateInstrs {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::ImmediateInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the support for data-processing instructions with long immediate"]
    #[inline(always)]
    pub const fn set_immediate_instrs(&mut self, val: super::vals::ImmediateInstrs) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the supported Interworking instructions"]
    #[inline(always)]
    pub const fn interwork_instrs(&self) -> super::vals::InterworkInstrs {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::InterworkInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Interworking instructions"]
    #[inline(always)]
    pub const fn set_interwork_instrs(&mut self, val: super::vals::InterworkInstrs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for IdIsar1 {
    #[inline(always)]
    fn default() -> IdIsar1 {
        IdIsar1(0)
    }
}
#[doc = "Instruction Set Attributes Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdIsar2(pub u32);
impl IdIsar2 {
    #[doc = "Indicates the supported additional load and store instructions"]
    #[inline(always)]
    pub const fn loadstore_instrs(&self) -> super::vals::LoadstoreInstrs {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LoadstoreInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported additional load and store instructions"]
    #[inline(always)]
    pub const fn set_loadstore_instrs(&mut self, val: super::vals::LoadstoreInstrs) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the supported Memory Hint instructions"]
    #[inline(always)]
    pub const fn memhint_instrs(&self) -> super::vals::MemhintInstrs {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::MemhintInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Memory Hint instructions"]
    #[inline(always)]
    pub const fn set_memhint_instrs(&mut self, val: super::vals::MemhintInstrs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the support for multi-access interruptible instructions"]
    #[inline(always)]
    pub const fn multiaccessint_instrs(&self) -> super::vals::MultiaccessintInstrs {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::MultiaccessintInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the support for multi-access interruptible instructions"]
    #[inline(always)]
    pub const fn set_multiaccessint_instrs(&mut self, val: super::vals::MultiaccessintInstrs) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicates the supported additional Multiply instructions"]
    #[inline(always)]
    pub const fn mult_instrs(&self) -> super::vals::MultInstrs {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::MultInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported additional Multiply instructions"]
    #[inline(always)]
    pub const fn set_mult_instrs(&mut self, val: super::vals::MultInstrs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicates the supported advanced signed Multiply instructions"]
    #[inline(always)]
    pub const fn mults_instrs(&self) -> super::vals::MultsInstrs {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::MultsInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported advanced signed Multiply instructions"]
    #[inline(always)]
    pub const fn set_mults_instrs(&mut self, val: super::vals::MultsInstrs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicates the supported advanced unsigned Multiply instructions"]
    #[inline(always)]
    pub const fn multu_instrs(&self) -> super::vals::MultuInstrs {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::MultuInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported advanced unsigned Multiply instructions"]
    #[inline(always)]
    pub const fn set_multu_instrs(&mut self, val: super::vals::MultuInstrs) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the supported Reversal instructions"]
    #[inline(always)]
    pub const fn reversal_instrs(&self) -> super::vals::ReversalInstrs {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::ReversalInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Reversal instructions"]
    #[inline(always)]
    pub const fn set_reversal_instrs(&mut self, val: super::vals::ReversalInstrs) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for IdIsar2 {
    #[inline(always)]
    fn default() -> IdIsar2 {
        IdIsar2(0)
    }
}
#[doc = "Instruction Set Attributes Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdIsar3(pub u32);
impl IdIsar3 {
    #[doc = "Indicates the supported Saturate instructions"]
    #[inline(always)]
    pub const fn saturate_instrs(&self) -> super::vals::SaturateInstrs {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SaturateInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Saturate instructions"]
    #[inline(always)]
    pub const fn set_saturate_instrs(&mut self, val: super::vals::SaturateInstrs) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the supported SIMD instructions"]
    #[inline(always)]
    pub const fn simd_instrs(&self) -> super::vals::SimdInstrs {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::SimdInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported SIMD instructions"]
    #[inline(always)]
    pub const fn set_simd_instrs(&mut self, val: super::vals::SimdInstrs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the supported SVC instructions"]
    #[inline(always)]
    pub const fn svc_instrs(&self) -> super::vals::SvcInstrs {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::SvcInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported SVC instructions"]
    #[inline(always)]
    pub const fn set_svc_instrs(&mut self, val: super::vals::SvcInstrs) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Together with the ID_ISAR4\\[SYNCHPRIM_INSTRS_FRAC\\] indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub const fn synchprim_instrs(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Together with the ID_ISAR4\\[SYNCHPRIM_INSTRS_FRAC\\] indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub const fn set_synchprim_instrs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicates the supported Table Branch instructions"]
    #[inline(always)]
    pub const fn tabbranch_instrs(&self) -> super::vals::TabbranchInstrs {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TabbranchInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Table Branch instructions"]
    #[inline(always)]
    pub const fn set_tabbranch_instrs(&mut self, val: super::vals::TabbranchInstrs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub const fn thumbcopy_instrs(&self) -> super::vals::ThumbcopyInstrs {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::ThumbcopyInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub const fn set_thumbcopy_instrs(&mut self, val: super::vals::ThumbcopyInstrs) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub const fn truenop_instrs(&self) -> super::vals::TruenopInstrs {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::TruenopInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub const fn set_truenop_instrs(&mut self, val: super::vals::TruenopInstrs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for IdIsar3 {
    #[inline(always)]
    fn default() -> IdIsar3 {
        IdIsar3(0)
    }
}
#[doc = "Instruction Set Attributes Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdIsar4(pub u32);
impl IdIsar4 {
    #[doc = "Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix."]
    #[inline(always)]
    pub const fn unpriv_instrs(&self) -> super::vals::UnprivInstrs {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::UnprivInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix."]
    #[inline(always)]
    pub const fn set_unpriv_instrs(&mut self, val: super::vals::UnprivInstrs) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the support for instructions with shifts"]
    #[inline(always)]
    pub const fn withshifts_instrs(&self) -> super::vals::WithshiftsInstrs {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::WithshiftsInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the support for instructions with shifts"]
    #[inline(always)]
    pub const fn set_withshifts_instrs(&mut self, val: super::vals::WithshiftsInstrs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the support for Writeback addressing modes"]
    #[inline(always)]
    pub const fn writeback_instrs(&self) -> super::vals::WritebackInstrs {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::WritebackInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the support for Writeback addressing modes"]
    #[inline(always)]
    pub const fn set_writeback_instrs(&mut self, val: super::vals::WritebackInstrs) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicates the supported Barrier instructions"]
    #[inline(always)]
    pub const fn barrier_instrs(&self) -> super::vals::BarrierInstrs {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::BarrierInstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported Barrier instructions"]
    #[inline(always)]
    pub const fn set_barrier_instrs(&mut self, val: super::vals::BarrierInstrs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Together with the ID_ISAR3\\[SYNCHPRIM_INSTRS\\] indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub const fn synchprim_instrs_frac(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Together with the ID_ISAR3\\[SYNCHPRIM_INSTRS\\] indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub const fn set_synchprim_instrs_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the supported M profile instructions to modify the PSRs"]
    #[inline(always)]
    pub const fn psr_m_instrs(&self) -> super::vals::PsrMinstrs {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::PsrMinstrs::from_bits(val as u8)
    }
    #[doc = "Indicates the supported M profile instructions to modify the PSRs"]
    #[inline(always)]
    pub const fn set_psr_m_instrs(&mut self, val: super::vals::PsrMinstrs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for IdIsar4 {
    #[inline(always)]
    fn default() -> IdIsar4 {
        IdIsar4(0)
    }
}
#[doc = "Memory Model Feature Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdMmfr0(pub u32);
impl IdMmfr0 {
    #[doc = "Indicates support for a PMSA"]
    #[inline(always)]
    pub const fn pmsasupport(&self) -> super::vals::Pmsasupport {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Pmsasupport::from_bits(val as u8)
    }
    #[doc = "Indicates support for a PMSA"]
    #[inline(always)]
    pub const fn set_pmsasupport(&mut self, val: super::vals::Pmsasupport) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the outermost shareability domain implemented"]
    #[inline(always)]
    pub const fn outermost_shareability(&self) -> super::vals::OutermostShareability {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::OutermostShareability::from_bits(val as u8)
    }
    #[doc = "Indicates the outermost shareability domain implemented"]
    #[inline(always)]
    pub const fn set_outermost_shareability(&mut self, val: super::vals::OutermostShareability) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicates the number of shareability levels implemented"]
    #[inline(always)]
    pub const fn shareability_levels(&self) -> super::vals::ShareabilityLevels {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::ShareabilityLevels::from_bits(val as u8)
    }
    #[doc = "Indicates the number of shareability levels implemented"]
    #[inline(always)]
    pub const fn set_shareability_levels(&mut self, val: super::vals::ShareabilityLevels) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicates the support for Tightly Coupled Memory"]
    #[inline(always)]
    pub const fn tcm_support(&self) -> super::vals::TcmSupport {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TcmSupport::from_bits(val as u8)
    }
    #[doc = "Indicates the support for Tightly Coupled Memory"]
    #[inline(always)]
    pub const fn set_tcm_support(&mut self, val: super::vals::TcmSupport) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicates the support for Auxiliary registers"]
    #[inline(always)]
    pub const fn auxiliary_registers(&self) -> super::vals::AuxiliaryRegisters {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::AuxiliaryRegisters::from_bits(val as u8)
    }
    #[doc = "Indicates the support for Auxiliary registers"]
    #[inline(always)]
    pub const fn set_auxiliary_registers(&mut self, val: super::vals::AuxiliaryRegisters) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for IdMmfr0 {
    #[inline(always)]
    fn default() -> IdMmfr0 {
        IdMmfr0(0)
    }
}
#[doc = "Memory Model Feature Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdMmfr2(pub u32);
impl IdMmfr2 {
    #[doc = "Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline(always)]
    pub const fn wfi_stall(&self) -> super::vals::WfiStall {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::WfiStall::from_bits(val as u8)
    }
    #[doc = "Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline(always)]
    pub const fn set_wfi_stall(&mut self, val: super::vals::WfiStall) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for IdMmfr2 {
    #[inline(always)]
    fn default() -> IdMmfr2 {
        IdMmfr2(0)
    }
}
#[doc = "Processor Feature Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdPfr0(pub u32);
impl IdPfr0 {
    #[doc = "ARM instruction set support"]
    #[inline(always)]
    pub const fn state0(&self) -> super::vals::State0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::State0::from_bits(val as u8)
    }
    #[doc = "ARM instruction set support"]
    #[inline(always)]
    pub const fn set_state0(&mut self, val: super::vals::State0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Thumb instruction set support"]
    #[inline(always)]
    pub const fn state1(&self) -> super::vals::State1 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::State1::from_bits(val as u8)
    }
    #[doc = "Thumb instruction set support"]
    #[inline(always)]
    pub const fn set_state1(&mut self, val: super::vals::State1) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "ARMv7-M unused"]
    #[inline(always)]
    pub const fn state2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "ARMv7-M unused"]
    #[inline(always)]
    pub const fn set_state2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "ARMv7-M unused"]
    #[inline(always)]
    pub const fn state3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "ARMv7-M unused"]
    #[inline(always)]
    pub const fn set_state3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for IdPfr0 {
    #[inline(always)]
    fn default() -> IdPfr0 {
        IdPfr0(0)
    }
}
#[doc = "Processor Feature Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IdPfr1(pub u32);
impl IdPfr1 {
    #[doc = "M profile programmers' model"]
    #[inline(always)]
    pub const fn progmodel(&self) -> super::vals::Progmodel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Progmodel::from_bits(val as u8)
    }
    #[doc = "M profile programmers' model"]
    #[inline(always)]
    pub const fn set_progmodel(&mut self, val: super::vals::Progmodel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for IdPfr1 {
    #[inline(always)]
    fn default() -> IdPfr1 {
        IdPfr1(0)
    }
}
#[doc = "System Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    pub const fn sleeponexit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    pub const fn set_sleeponexit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub const fn sleepdeep(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub const fn set_sleepdeep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Send Event on Pending bit"]
    #[inline(always)]
    pub const fn sevonpend(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Send Event on Pending bit"]
    #[inline(always)]
    pub const fn set_sevonpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
#[doc = "System Handler Control and State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shcsr(pub u32);
impl Shcsr {
    #[doc = "MemManage exception active bit"]
    #[inline(always)]
    pub const fn memfaultact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage exception active bit"]
    #[inline(always)]
    pub const fn set_memfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BusFault exception active bit"]
    #[inline(always)]
    pub const fn busfaultact(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault exception active bit"]
    #[inline(always)]
    pub const fn set_busfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UsageFault exception active bit"]
    #[inline(always)]
    pub const fn usgfaultact(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault exception active bit"]
    #[inline(always)]
    pub const fn set_usgfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SVCall active bit"]
    #[inline(always)]
    pub const fn svcallact(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SVCall active bit"]
    #[inline(always)]
    pub const fn set_svcallact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Debug monitor active bit"]
    #[inline(always)]
    pub const fn monitoract(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Debug monitor active bit"]
    #[inline(always)]
    pub const fn set_monitoract(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PendSV exception active bit"]
    #[inline(always)]
    pub const fn pendsvact(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PendSV exception active bit"]
    #[inline(always)]
    pub const fn set_pendsvact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SysTick exception active bit"]
    #[inline(always)]
    pub const fn systickact(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception active bit"]
    #[inline(always)]
    pub const fn set_systickact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "UsageFault exception pending bit"]
    #[inline(always)]
    pub const fn usgfaultpended(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault exception pending bit"]
    #[inline(always)]
    pub const fn set_usgfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "MemManage exception pending bit"]
    #[inline(always)]
    pub const fn memfaultpended(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage exception pending bit"]
    #[inline(always)]
    pub const fn set_memfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "BusFault exception pending bit"]
    #[inline(always)]
    pub const fn busfaultpended(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault exception pending bit"]
    #[inline(always)]
    pub const fn set_busfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SVCall pending bit"]
    #[inline(always)]
    pub const fn svcallpended(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SVCall pending bit"]
    #[inline(always)]
    pub const fn set_svcallpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "MemManage enable bit"]
    #[inline(always)]
    pub const fn memfaultena(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage enable bit"]
    #[inline(always)]
    pub const fn set_memfaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "BusFault enable bit"]
    #[inline(always)]
    pub const fn busfaultena(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault enable bit"]
    #[inline(always)]
    pub const fn set_busfaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "UsageFault enable bit"]
    #[inline(always)]
    pub const fn usgfaultena(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault enable bit"]
    #[inline(always)]
    pub const fn set_usgfaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Shcsr {
    #[inline(always)]
    fn default() -> Shcsr {
        Shcsr(0)
    }
}
#[doc = "System Handler Priority Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shpr1(pub u32);
impl Shpr1 {
    #[doc = "Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub const fn pri_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub const fn set_pri_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub const fn pri_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub const fn set_pri_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub const fn pri_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub const fn set_pri_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Shpr1 {
    #[inline(always)]
    fn default() -> Shpr1 {
        Shpr1(0)
    }
}
#[doc = "System Handler Priority Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shpr2(pub u32);
impl Shpr2 {
    #[doc = "Priority of system handler 11, SVCall"]
    #[inline(always)]
    pub const fn pri_11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Priority of system handler 11, SVCall"]
    #[inline(always)]
    pub const fn set_pri_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Shpr2 {
    #[inline(always)]
    fn default() -> Shpr2 {
        Shpr2(0)
    }
}
#[doc = "System Handler Priority Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Shpr3(pub u32);
impl Shpr3 {
    #[doc = "Priority of system handler 14, PendSV"]
    #[inline(always)]
    pub const fn pri_14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Priority of system handler 14, PendSV"]
    #[inline(always)]
    pub const fn set_pri_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Priority of system handler 15, SysTick exception"]
    #[inline(always)]
    pub const fn pri_15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Priority of system handler 15, SysTick exception"]
    #[inline(always)]
    pub const fn set_pri_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Shpr3 {
    #[inline(always)]
    fn default() -> Shpr3 {
        Shpr3(0)
    }
}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Stir(pub u32);
impl Stir {
    #[doc = "Indicates the interrupt to be triggered"]
    #[inline(always)]
    pub const fn intid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Indicates the interrupt to be triggered"]
    #[inline(always)]
    pub const fn set_intid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Stir {
    #[inline(always)]
    fn default() -> Stir {
        Stir(0)
    }
}
#[doc = "Vector Table Offset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vtor(pub u32);
impl Vtor {
    #[doc = "Vector table base offset"]
    #[inline(always)]
    pub const fn tbloff(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Vector table base offset"]
    #[inline(always)]
    pub const fn set_tbloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Vtor {
    #[inline(always)]
    fn default() -> Vtor {
        Vtor(0)
    }
}
