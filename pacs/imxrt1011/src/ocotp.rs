#[doc = "no description available"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ocotp {
    ptr: *mut u8,
}
unsafe impl Send for Ocotp {}
unsafe impl Sync for Ocotp {}
impl Ocotp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl(self) -> crate::common::Reg<regs::HwOcotpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl_set(
        self,
    ) -> crate::common::Reg<regs::HwOcotpCtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl_clr(
        self,
    ) -> crate::common::Reg<regs::HwOcotpCtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl_tog(
        self,
    ) -> crate::common::Reg<regs::HwOcotpCtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "OTP Controller Timing Register"]
    #[inline(always)]
    pub const fn hw_ocotp_timing(
        self,
    ) -> crate::common::Reg<regs::HwOcotpTiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "OTP Controller Write Data Register"]
    #[inline(always)]
    pub const fn hw_ocotp_data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "OTP Controller Write Data Register"]
    #[inline(always)]
    pub const fn hw_ocotp_read_ctrl(
        self,
    ) -> crate::common::Reg<regs::HwOcotpReadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "OTP Controller Read Data Register"]
    #[inline(always)]
    pub const fn hw_ocotp_read_fuse_data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Sticky bit Register"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_sticky(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwSticky, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs(self) -> crate::common::Reg<regs::HwOcotpScs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs_set(
        self,
    ) -> crate::common::Reg<regs::HwOcotpScsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs_clr(
        self,
    ) -> crate::common::Reg<regs::HwOcotpScsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs_tog(
        self,
    ) -> crate::common::Reg<regs::HwOcotpScsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "OTP Controller Version Register"]
    #[inline(always)]
    pub const fn hw_ocotp_version(
        self,
    ) -> crate::common::Reg<regs::HwOcotpVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "OTP Controller Timing Register 2"]
    #[inline(always)]
    pub const fn hw_ocotp_timing2(
        self,
    ) -> crate::common::Reg<regs::HwOcotpTiming2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
    #[inline(always)]
    pub const fn hw_ocotp_lock(self) -> crate::common::Reg<regs::HwOcotpLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1040usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1056usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1072usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1088usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1104usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1120usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1136usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1152usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1168usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1184usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1200usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 4 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1216usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 5 (Analog Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_ana0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1232usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 6 (Analog Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_ana1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1248usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 7 (Analog Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_ana2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1264usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1408usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1424usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1440usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1456usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1472usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1488usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1504usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1520usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    #[inline(always)]
    pub const fn hw_ocotp_sjc_resp0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1536usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    #[inline(always)]
    pub const fn hw_ocotp_sjc_resp1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1552usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word2 (OTFAD)"]
    #[inline(always)]
    pub const fn hw_ocotp_otfad_cfg0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1568usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word3 (OTFAD)"]
    #[inline(always)]
    pub const fn hw_ocotp_otfad_cfg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1584usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word4 (OTFAD)"]
    #[inline(always)]
    pub const fn hw_ocotp_gp3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1600usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    #[inline(always)]
    pub const fn hw_ocotp_gp1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1632usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    #[inline(always)]
    pub const fn hw_ocotp_gp2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1648usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1664usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp20(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1680usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp21(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1696usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp22(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1712usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp23(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1728usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
    #[inline(always)]
    pub const fn hw_ocotp_misc_conf0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1744usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
    #[inline(always)]
    pub const fn hw_ocotp_misc_conf1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1760usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk_revoke(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1776usize) as _) }
    }
}
pub mod regs;
pub mod vals;
