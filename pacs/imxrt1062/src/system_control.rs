#[doc = "System Control Block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemControl {
    ptr: *mut u8,
}
unsafe impl Send for SystemControl {}
unsafe impl Sync for SystemControl {}
impl SystemControl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Auxiliary Control Register,"]
    #[inline(always)]
    pub const fn actlr(self) -> crate::common::Reg<regs::Actlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CPUID Base Register"]
    #[inline(always)]
    pub const fn cpuid(self) -> crate::common::Reg<regs::Cpuid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d00usize) as _) }
    }
    #[doc = "Interrupt Control and State Register"]
    #[inline(always)]
    pub const fn icsr(self) -> crate::common::Reg<regs::Icsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d04usize) as _) }
    }
    #[doc = "Vector Table Offset Register"]
    #[inline(always)]
    pub const fn vtor(self) -> crate::common::Reg<regs::Vtor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d08usize) as _) }
    }
    #[doc = "Application Interrupt and Reset Control Register"]
    #[inline(always)]
    pub const fn aircr(self) -> crate::common::Reg<regs::Aircr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d0cusize) as _) }
    }
    #[doc = "System Control Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d10usize) as _) }
    }
    #[doc = "Configuration and Control Register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d14usize) as _) }
    }
    #[doc = "System Handler Priority Register 1"]
    #[inline(always)]
    pub const fn shpr1(self) -> crate::common::Reg<regs::Shpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d18usize) as _) }
    }
    #[doc = "System Handler Priority Register 2"]
    #[inline(always)]
    pub const fn shpr2(self) -> crate::common::Reg<regs::Shpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d1cusize) as _) }
    }
    #[doc = "System Handler Priority Register 3"]
    #[inline(always)]
    pub const fn shpr3(self) -> crate::common::Reg<regs::Shpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d20usize) as _) }
    }
    #[doc = "System Handler Control and State Register"]
    #[inline(always)]
    pub const fn shcsr(self) -> crate::common::Reg<regs::Shcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d24usize) as _) }
    }
    #[doc = "Configurable Fault Status Register"]
    #[inline(always)]
    pub const fn cfsr(self) -> crate::common::Reg<regs::Cfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d28usize) as _) }
    }
    #[doc = "HardFault Status register"]
    #[inline(always)]
    pub const fn hfsr(self) -> crate::common::Reg<regs::Hfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d2cusize) as _) }
    }
    #[doc = "Debug Fault Status Register"]
    #[inline(always)]
    pub const fn dfsr(self) -> crate::common::Reg<regs::Dfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d30usize) as _) }
    }
    #[doc = "MemManage Fault Address Register"]
    #[inline(always)]
    pub const fn mmfar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d34usize) as _) }
    }
    #[doc = "BusFault Address Register"]
    #[inline(always)]
    pub const fn bfar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d38usize) as _) }
    }
    #[doc = "Processor Feature Register 0"]
    #[inline(always)]
    pub const fn id_pfr0(self) -> crate::common::Reg<regs::IdPfr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d40usize) as _) }
    }
    #[doc = "Processor Feature Register 1"]
    #[inline(always)]
    pub const fn id_pfr1(self) -> crate::common::Reg<regs::IdPfr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d44usize) as _) }
    }
    #[doc = "Debug Feature Register"]
    #[inline(always)]
    pub const fn id_dfr0(self) -> crate::common::Reg<regs::IdDfr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d48usize) as _) }
    }
    #[doc = "Auxiliary Feature Register"]
    #[inline(always)]
    pub const fn id_afr0(self) -> crate::common::Reg<regs::IdAfr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d4cusize) as _) }
    }
    #[doc = "Memory Model Feature Register 0"]
    #[inline(always)]
    pub const fn id_mmfr0(self) -> crate::common::Reg<regs::IdMmfr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d50usize) as _) }
    }
    #[doc = "Memory Model Feature Register 1"]
    #[inline(always)]
    pub const fn id_mmfr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d54usize) as _) }
    }
    #[doc = "Memory Model Feature Register 2"]
    #[inline(always)]
    pub const fn id_mmfr2(self) -> crate::common::Reg<regs::IdMmfr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d58usize) as _) }
    }
    #[doc = "Memory Model Feature Register 3"]
    #[inline(always)]
    pub const fn id_mmfr3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d5cusize) as _) }
    }
    #[doc = "Instruction Set Attributes Register 0"]
    #[inline(always)]
    pub const fn id_isar0(self) -> crate::common::Reg<regs::IdIsar0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d60usize) as _) }
    }
    #[doc = "Instruction Set Attributes Register 1"]
    #[inline(always)]
    pub const fn id_isar1(self) -> crate::common::Reg<regs::IdIsar1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d64usize) as _) }
    }
    #[doc = "Instruction Set Attributes Register 2"]
    #[inline(always)]
    pub const fn id_isar2(self) -> crate::common::Reg<regs::IdIsar2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d68usize) as _) }
    }
    #[doc = "Instruction Set Attributes Register 3"]
    #[inline(always)]
    pub const fn id_isar3(self) -> crate::common::Reg<regs::IdIsar3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d6cusize) as _) }
    }
    #[doc = "Instruction Set Attributes Register 4"]
    #[inline(always)]
    pub const fn id_isar4(self) -> crate::common::Reg<regs::IdIsar4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d70usize) as _) }
    }
    #[doc = "Cache Level ID register"]
    #[inline(always)]
    pub const fn clidr(self) -> crate::common::Reg<regs::Clidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d78usize) as _) }
    }
    #[doc = "Cache Type register"]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d7cusize) as _) }
    }
    #[doc = "Cache Size ID Register"]
    #[inline(always)]
    pub const fn ccsidr(self) -> crate::common::Reg<regs::Ccsidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d80usize) as _) }
    }
    #[doc = "Cache Size Selection Register"]
    #[inline(always)]
    pub const fn csselr(self) -> crate::common::Reg<regs::Csselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d84usize) as _) }
    }
    #[doc = "Coprocessor Access Control Register"]
    #[inline(always)]
    pub const fn cpacr(self) -> crate::common::Reg<regs::Cpacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d88usize) as _) }
    }
    #[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
    #[inline(always)]
    pub const fn stir(self) -> crate::common::Reg<regs::Stir, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f00usize) as _) }
    }
    #[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
    #[inline(always)]
    pub const fn iciallu(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f50usize) as _) }
    }
    #[doc = "Instruction cache invalidate by address to PoU"]
    #[inline(always)]
    pub const fn icimvau(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f58usize) as _) }
    }
    #[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
    #[inline(always)]
    pub const fn dcimvac(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f5cusize) as _) }
    }
    #[doc = "Data cache invalidate by set/way"]
    #[inline(always)]
    pub const fn dcisw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f60usize) as _) }
    }
    #[doc = "Data cache by address to PoU"]
    #[inline(always)]
    pub const fn dccmvau(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f64usize) as _) }
    }
    #[doc = "Data cache clean by address to PoC"]
    #[inline(always)]
    pub const fn dccmvac(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f68usize) as _) }
    }
    #[doc = "Data cache clean by set/way"]
    #[inline(always)]
    pub const fn dccsw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f6cusize) as _) }
    }
    #[doc = "Data cache clean and invalidate by address to PoC"]
    #[inline(always)]
    pub const fn dccimvac(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f70usize) as _) }
    }
    #[doc = "Data cache clean and invalidate by set/way"]
    #[inline(always)]
    pub const fn dccisw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f74usize) as _) }
    }
    #[doc = "Instruction Tightly-Coupled Memory Control Register"]
    #[inline(always)]
    pub const fn cm7_itcmcr(self) -> crate::common::Reg<regs::Cm7itcmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f90usize) as _) }
    }
    #[doc = "Data Tightly-Coupled Memory Control Register"]
    #[inline(always)]
    pub const fn cm7_dtcmcr(self) -> crate::common::Reg<regs::Cm7dtcmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f94usize) as _) }
    }
    #[doc = "AHBP Control Register"]
    #[inline(always)]
    pub const fn cm7_ahbpcr(self) -> crate::common::Reg<regs::Cm7ahbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f98usize) as _) }
    }
    #[doc = "L1 Cache Control Register"]
    #[inline(always)]
    pub const fn cm7_cacr(self) -> crate::common::Reg<regs::Cm7cacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f9cusize) as _) }
    }
    #[doc = "AHB Slave Control Register"]
    #[inline(always)]
    pub const fn cm7_ahbscr(self) -> crate::common::Reg<regs::Cm7ahbscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa0usize) as _) }
    }
    #[doc = "Auxiliary Bus Fault Status Register"]
    #[inline(always)]
    pub const fn cm7_abfsr(self) -> crate::common::Reg<regs::Cm7abfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
