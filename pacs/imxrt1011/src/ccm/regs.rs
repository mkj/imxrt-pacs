#[doc = "CCM Bus Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cbcdr(pub u32);
impl Cbcdr {
    #[doc = "Divider for ipg podf."]
    #[inline(always)]
    pub const fn ipg_podf(&self) -> super::vals::IpgPodf {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IpgPodf::from_bits(val as u8)
    }
    #[doc = "Divider for ipg podf."]
    #[inline(always)]
    pub const fn set_ipg_podf(&mut self, val: super::vals::IpgPodf) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Divider for AHB PODF"]
    #[inline(always)]
    pub const fn ahb_podf(&self) -> super::vals::AhbPodf {
        let val = (self.0 >> 10usize) & 0x07;
        super::vals::AhbPodf::from_bits(val as u8)
    }
    #[doc = "Divider for AHB PODF"]
    #[inline(always)]
    pub const fn set_ahb_podf(&mut self, val: super::vals::AhbPodf) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
    }
    #[doc = "Selector for peripheral main clock"]
    #[inline(always)]
    pub const fn periph_clk_sel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Selector for peripheral main clock"]
    #[inline(always)]
    pub const fn set_periph_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Cbcdr {
    #[inline(always)]
    fn default() -> Cbcdr {
        Cbcdr(0)
    }
}
#[doc = "CCM Bus Clock Multiplexer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cbcmr(pub u32);
impl Cbcmr {
    #[doc = "Selector for lpspi clock multiplexer"]
    #[inline(always)]
    pub const fn lpspi_clk_sel(&self) -> super::vals::LpspiClkSel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LpspiClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for lpspi clock multiplexer"]
    #[inline(always)]
    pub const fn set_lpspi_clk_sel(&mut self, val: super::vals::LpspiClkSel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    pub const fn periph_clk2_sel(&self) -> super::vals::PeriphClk2sel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::PeriphClk2sel::from_bits(val as u8)
    }
    #[doc = "Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    pub const fn set_periph_clk2_sel(&mut self, val: super::vals::PeriphClk2sel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Selector for Trace clock multiplexer"]
    #[inline(always)]
    pub const fn trace_clk_sel(&self) -> super::vals::TraceClkSel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::TraceClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for Trace clock multiplexer"]
    #[inline(always)]
    pub const fn set_trace_clk_sel(&mut self, val: super::vals::TraceClkSel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    pub const fn pre_periph_clk_sel(&self) -> super::vals::PrePeriphClkSel {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::PrePeriphClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    pub const fn set_pre_periph_clk_sel(&mut self, val: super::vals::PrePeriphClkSel) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn lpspi_podf(&self) -> super::vals::LpspiPodf {
        let val = (self.0 >> 26usize) & 0x0f;
        super::vals::LpspiPodf::from_bits(val as u8)
    }
    #[doc = "Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_lpspi_podf(&mut self, val: super::vals::LpspiPodf) {
        self.0 = (self.0 & !(0x0f << 26usize)) | (((val.to_bits() as u32) & 0x0f) << 26usize);
    }
}
impl Default for Cbcmr {
    #[inline(always)]
    fn default() -> Cbcmr {
        Cbcmr(0)
    }
}
#[doc = "CCM Clock Gating Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr0(pub u32);
impl Ccgr0 {
    #[doc = "aips_tz1 clocks (aips_tz1_clk_enable)"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "aips_tz1 clocks (aips_tz1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "aips_tz2 clocks (aips_tz2_clk_enable)"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "aips_tz2 clocks (aips_tz2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "mqs clock ( mqs_hmclk_clock_enable)"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "mqs clock ( mqs_hmclk_clock_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "flexspi_exsc clock (flexspi_exsc_clk_enable)"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "flexspi_exsc clock (flexspi_exsc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "sim_m_clk_r_clk_enable"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m_clk_r_clk_enable"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "dcp clock (dcp_clk_enable)"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "dcp clock (dcp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "lpuart3 clock (lpuart3_clk_enable)"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart3 clock (lpuart3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "trace clock (trace_clk_enable)"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "trace clock (trace_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "gpt2 bus clocks (gpt2_bus_clk_enable)"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "gpt2 bus clocks (gpt2_bus_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "gpt2 serial clocks (gpt2_serial_clk_enable)"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "gpt2 serial clocks (gpt2_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "lpuart2 clock (lpuart2_clk_enable)"]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart2 clock (lpuart2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "gpio2_clocks (gpio2_clk_enable)"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "gpio2_clocks (gpio2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr0 {
    #[inline(always)]
    fn default() -> Ccgr0 {
        Ccgr0(0)
    }
}
#[doc = "CCM Clock Gating Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr1(pub u32);
impl Ccgr1 {
    #[doc = "lpspi1 clocks (lpspi1_clk_enable)"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "lpspi1 clocks (lpspi1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "lpspi2 clocks (lpspi2_clk_enable)"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "lpspi2 clocks (lpspi2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "pit clocks (pit_clk_enable)"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "pit clocks (pit_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "adc1 clock (adc1_clk_enable)"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "adc1 clock (adc1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "gpt1 bus clock (gpt_clk_enable)"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "gpt1 bus clock (gpt_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "gpt1 serial clock (gpt_serial_clk_enable)"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "gpt1 serial clock (gpt_serial_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "lpuart4 clock (lpuart4_clk_enable)"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart4 clock (lpuart4_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "gpio1 clock (gpio1_clk_enable)"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "gpio1 clock (gpio1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "csu clock (csu_clk_enable)"]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "csu clock (csu_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "gpio5 clock (gpio5_clk_enable)"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "gpio5 clock (gpio5_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr1 {
    #[inline(always)]
    fn default() -> Ccgr1 {
        Ccgr1(0)
    }
}
#[doc = "CCM Clock Gating Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr2(pub u32);
impl Ccgr2 {
    #[doc = "ocram_exsc clock (ocram_exsc_clk_enable)"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ocram_exsc clock (ocram_exsc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "iomuxc_snvs clock (iomuxc_snvs_clk_enable)"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc_snvs clock (iomuxc_snvs_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "lpi2c1 clock (lpi2c1_clk_enable)"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "lpi2c1 clock (lpi2c1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "lpi2c2 clock (lpi2c2_clk_enable)"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "lpi2c2 clock (lpi2c2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "OCOTP_CTRL clock (ocotp_clk_enable)"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "OCOTP_CTRL clock (ocotp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "xbar1 clock (xbar1_clk_enable)"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "xbar1 clock (xbar1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr2 {
    #[inline(always)]
    fn default() -> Ccgr2 {
        Ccgr2(0)
    }
}
#[doc = "CCM Clock Gating Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr3(pub u32);
impl Ccgr3 {
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "aoi1 clock (aoi1_clk_enable)"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "aoi1 clock (aoi1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "ewm clocks (ewm_clk_enable)"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "ewm clocks (ewm_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "wdog1 clock (wdog1_clk_enable)"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "wdog1 clock (wdog1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "flexram clock (flexram_clk_enable)"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "flexram clock (flexram_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "The OCRAM clock cannot be turned off when the CM cache is running on this device."]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc_snvs_gpr clock (iomuxc_snvs_gpr_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr3 {
    #[inline(always)]
    fn default() -> Ccgr3 {
        Ccgr3(0)
    }
}
#[doc = "CCM Clock Gating Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr4(pub u32);
impl Ccgr4 {
    #[doc = "sim_m7_clk_r_enable"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m7_clk_r_enable"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "iomuxc clock (iomuxc_clk_enable)"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc clock (iomuxc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "sim_m7 clock (sim_m7_clk_enable)"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m7 clock (sim_m7_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "sim_m clocks (sim_m_clk_enable)"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "sim_m clocks (sim_m_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "sim_ems clocks (sim_ems_clk_enable)"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "sim_ems clocks (sim_ems_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "pwm1 clocks (pwm1_clk_enable)"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "pwm1 clocks (pwm1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "dma_ps clocks (dma_ps_clk_enable)"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "dma_ps clocks (dma_ps_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr4 {
    #[inline(always)]
    fn default() -> Ccgr4 {
        Ccgr4(0)
    }
}
#[doc = "CCM Clock Gating Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr5(pub u32);
impl Ccgr5 {
    #[doc = "rom clock (rom_clk_enable)"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "rom clock (rom_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "flexio1 clock (flexio1_clk_enable)"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "flexio1 clock (flexio1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "wdog3 clock (wdog3_clk_enable)"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "wdog3 clock (wdog3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "dma clock (dma_clk_enable)"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "dma clock (dma_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "kpp clock (kpp_clk_enable)"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "kpp clock (kpp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "wdog2 clock (wdog2_clk_enable)"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "wdog2 clock (wdog2_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "spdif clock (spdif_clk_enable)"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "spdif clock (spdif_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "sai1 clock (sai1_clk_enable)"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "sai1 clock (sai1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "sai3 clock (sai3_clk_enable)"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "sai3 clock (sai3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "lpuart1 clock (lpuart1_clk_enable)"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "lpuart1 clock (lpuart1_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "snvs_hp clock (snvs_hp_clk_enable)"]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "snvs_hp clock (snvs_hp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "snvs_lp clock (snvs_lp_clk_enable)"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "snvs_lp clock (snvs_lp_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr5 {
    #[inline(always)]
    fn default() -> Ccgr5 {
        Ccgr5(0)
    }
}
#[doc = "CCM Clock Gating Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccgr6(pub u32);
impl Ccgr6 {
    #[doc = "usboh3 clock (usboh3_clk_enable)"]
    #[inline(always)]
    pub const fn cg0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "usboh3 clock (usboh3_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "dcdc clocks (dcdc_clk_enable)"]
    #[inline(always)]
    pub const fn cg3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "dcdc clocks (dcdc_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "flexspi clocks (flexspi_clk_enable) sim_ems_clk_enable must also be cleared, when flexspi_clk_enable is cleared"]
    #[inline(always)]
    pub const fn cg5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "flexspi clocks (flexspi_clk_enable) sim_ems_clk_enable must also be cleared, when flexspi_clk_enable is cleared"]
    #[inline(always)]
    pub const fn set_cg5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "trng clock (trng_clk_enable)"]
    #[inline(always)]
    pub const fn cg6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "trng clock (trng_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "sim_per clock (sim_per_clk_enable)"]
    #[inline(always)]
    pub const fn cg10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "sim_per clock (sim_per_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "anadig clocks (anadig_clk_enable)"]
    #[inline(always)]
    pub const fn cg11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "anadig clocks (anadig_clk_enable)"]
    #[inline(always)]
    pub const fn set_cg11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn cg15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_cg15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccgr6 {
    #[inline(always)]
    fn default() -> Ccgr6 {
        Ccgr6(0)
    }
}
#[doc = "CCM Clock Output Source Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccosr(pub u32);
impl Ccosr {
    #[doc = "Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    pub const fn clko1_sel(&self) -> super::vals::Clko1sel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Clko1sel::from_bits(val as u8)
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    pub const fn set_clko1_sel(&mut self, val: super::vals::Clko1sel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    pub const fn clko1_div(&self) -> super::vals::Clko1div {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Clko1div::from_bits(val as u8)
    }
    #[doc = "Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    pub const fn set_clko1_div(&mut self, val: super::vals::Clko1div) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    pub const fn clko1_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    pub const fn set_clko1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    pub const fn clk_out_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    pub const fn set_clk_out_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    pub const fn clko2_sel(&self) -> super::vals::Clko2sel {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::Clko2sel::from_bits(val as u8)
    }
    #[doc = "Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    pub const fn set_clko2_sel(&mut self, val: super::vals::Clko2sel) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    pub const fn clko2_div(&self) -> super::vals::Clko2div {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Clko2div::from_bits(val as u8)
    }
    #[doc = "Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    pub const fn set_clko2_div(&mut self, val: super::vals::Clko2div) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    pub const fn clko2_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    pub const fn set_clko2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccosr {
    #[inline(always)]
    fn default() -> Ccosr {
        Ccosr(0)
    }
}
#[doc = "CCM Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    pub const fn oscnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    pub const fn set_oscnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    pub const fn cosc_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    pub const fn set_cosc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    pub const fn reg_bypass_count(&self) -> super::vals::RegBypassCount {
        let val = (self.0 >> 21usize) & 0x3f;
        super::vals::RegBypassCount::from_bits(val as u8)
    }
    #[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    pub const fn set_reg_bypass_count(&mut self, val: super::vals::RegBypassCount) {
        self.0 = (self.0 & !(0x3f << 21usize)) | (((val.to_bits() as u32) & 0x3f) << 21usize);
    }
    #[doc = "Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    pub const fn rbc_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    pub const fn set_rbc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
#[doc = "CCM Clock Switcher Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ccsr(pub u32);
impl Ccsr {
    #[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    pub const fn pll3_sw_clk_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    pub const fn set_pll3_sw_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ccsr {
    #[inline(always)]
    fn default() -> Ccsr {
        Ccsr(0)
    }
}
#[doc = "CCM D1 Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cdcdr(pub u32);
impl Cdcdr {
    #[doc = "Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    pub const fn spdif0_clk_sel(&self) -> super::vals::Spdif0clkSel {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Spdif0clkSel::from_bits(val as u8)
    }
    #[doc = "Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    pub const fn set_spdif0_clk_sel(&mut self, val: super::vals::Spdif0clkSel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn spdif0_clk_podf(&self) -> super::vals::Spdif0clkPodf {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Spdif0clkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_spdif0_clk_podf(&mut self, val: super::vals::Spdif0clkPodf) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn spdif0_clk_pred(&self) -> super::vals::Spdif0clkPred {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Spdif0clkPred::from_bits(val as u8)
    }
    #[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_spdif0_clk_pred(&mut self, val: super::vals::Spdif0clkPred) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
}
impl Default for Cdcdr {
    #[inline(always)]
    fn default() -> Cdcdr {
        Cdcdr(0)
    }
}
#[doc = "CCM Divider Handshake In-Process Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cdhipr(pub u32);
impl Cdhipr {
    #[doc = "Busy indicator for ahb_podf."]
    #[inline(always)]
    pub const fn ahb_podf_busy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Busy indicator for ahb_podf."]
    #[inline(always)]
    pub const fn set_ahb_podf_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Busy indicator for flexspi_podf."]
    #[inline(always)]
    pub const fn flexspi_podf_busy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Busy indicator for flexspi_podf."]
    #[inline(always)]
    pub const fn set_flexspi_podf_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Busy indicator for perclk_podf."]
    #[inline(always)]
    pub const fn perclk_podf_busy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Busy indicator for perclk_podf."]
    #[inline(always)]
    pub const fn set_perclk_podf_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Busy indicator for periph_clk_sel mux control."]
    #[inline(always)]
    pub const fn periph_clk_sel_busy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Busy indicator for periph_clk_sel mux control."]
    #[inline(always)]
    pub const fn set_periph_clk_sel_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Cdhipr {
    #[inline(always)]
    fn default() -> Cdhipr {
        Cdhipr(0)
    }
}
#[doc = "CCM General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cgpr(pub u32);
impl Cgpr {
    #[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    pub const fn pmic_delay_scaler(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    pub const fn set_pmic_delay_scaler(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    pub const fn efuse_prog_supply_gate(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    pub const fn set_efuse_prog_supply_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "System memory DS control"]
    #[inline(always)]
    pub const fn sys_mem_ds_ctrl(&self) -> super::vals::SysMemDsCtrl {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SysMemDsCtrl::from_bits(val as u8)
    }
    #[doc = "System memory DS control"]
    #[inline(always)]
    pub const fn set_sys_mem_ds_ctrl(&mut self, val: super::vals::SysMemDsCtrl) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Fast PLL enable."]
    #[inline(always)]
    pub const fn fpl(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Fast PLL enable."]
    #[inline(always)]
    pub const fn set_fpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
    #[inline(always)]
    pub const fn int_mem_clk_lpm(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
    #[inline(always)]
    pub const fn set_int_mem_clk_lpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Cgpr {
    #[inline(always)]
    fn default() -> Cgpr {
        Cgpr(0)
    }
}
#[doc = "CCM Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cimr(pub u32);
impl Cimr {
    #[doc = "mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    pub const fn mask_lrf_pll(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    pub const fn set_mask_lrf_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    pub const fn mask_cosc_ready(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    pub const fn set_mask_cosc_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "mask interrupt generation due to update of flexspi_podf"]
    #[inline(always)]
    pub const fn mask_flexspi_podf_loaded(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "mask interrupt generation due to update of flexspi_podf"]
    #[inline(always)]
    pub const fn set_mask_flexspi_podf_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "mask interrupt generation due to update of perclk_podf"]
    #[inline(always)]
    pub const fn mask_perclk_podf_loaded(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "mask interrupt generation due to update of perclk_podf"]
    #[inline(always)]
    pub const fn set_mask_perclk_podf_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    pub const fn mask_ahb_podf_loaded(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    pub const fn set_mask_ahb_podf_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    pub const fn mask_periph_clk_sel_loaded(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    pub const fn set_mask_periph_clk_sel_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Cimr {
    #[inline(always)]
    fn default() -> Cimr {
        Cimr(0)
    }
}
#[doc = "CCM Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cisr(pub u32);
impl Cisr {
    #[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub const fn lrf_pll(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub const fn set_lrf_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    pub const fn cosc_ready(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    pub const fn set_cosc_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of flexspi_podf"]
    #[inline(always)]
    pub const fn flexspi_podf_loaded(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of flexspi_podf"]
    #[inline(always)]
    pub const fn set_flexspi_podf_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of perclk_podf"]
    #[inline(always)]
    pub const fn perclk_podf_loaded(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of perclk_podf"]
    #[inline(always)]
    pub const fn set_perclk_podf_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub const fn ahb_podf_loaded(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub const fn set_ahb_podf_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub const fn periph_clk_sel_loaded(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub const fn set_periph_clk_sel_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Cisr {
    #[inline(always)]
    fn default() -> Cisr {
        Cisr(0)
    }
}
#[doc = "CCM Low Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Clpcr(pub u32);
impl Clpcr {
    #[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    pub const fn lpm(&self) -> super::vals::Lpm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpm::from_bits(val as u8)
    }
    #[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    pub const fn set_lpm(&mut self, val: super::vals::Lpm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    pub const fn arm_clk_dis_on_lpm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    pub const fn set_arm_clk_dis_on_lpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Standby clock oscillator bit"]
    #[inline(always)]
    pub const fn sbyos(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Standby clock oscillator bit"]
    #[inline(always)]
    pub const fn set_sbyos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    pub const fn dis_ref_osc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    pub const fn set_dis_ref_osc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Voltage standby request bit"]
    #[inline(always)]
    pub const fn vstby(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage standby request bit"]
    #[inline(always)]
    pub const fn set_vstby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Standby counter definition"]
    #[inline(always)]
    pub const fn stby_count(&self) -> super::vals::StbyCount {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::StbyCount::from_bits(val as u8)
    }
    #[doc = "Standby counter definition"]
    #[inline(always)]
    pub const fn set_stby_count(&mut self, val: super::vals::StbyCount) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    pub const fn cosc_pwrdown(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    pub const fn set_cosc_pwrdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub const fn mask_core0_wfi(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub const fn set_mask_core0_wfi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub const fn mask_scu_idle(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub const fn set_mask_scu_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    pub const fn mask_l2cc_idle(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    pub const fn set_mask_l2cc_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Clpcr {
    #[inline(always)]
    fn default() -> Clpcr {
        Clpcr(0)
    }
}
#[doc = "CCM Module Enable Overide Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cmeor(pub u32);
impl Cmeor {
    #[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub const fn mod_en_ov_gpt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_gpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub const fn mod_en_ov_pit(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub const fn set_mod_en_ov_pit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Overide clock enable signal from TRNG"]
    #[inline(always)]
    pub const fn mod_en_ov_trng(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Overide clock enable signal from TRNG"]
    #[inline(always)]
    pub const fn set_mod_en_ov_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Cmeor {
    #[inline(always)]
    fn default() -> Cmeor {
        Cmeor(0)
    }
}
#[doc = "CCM Clock Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cs1cdr(pub u32);
impl Cs1cdr {
    #[doc = "Divider for sai1 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn sai1_clk_podf(&self) -> super::vals::Sai1clkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sai1clkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for sai1 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_sai1_clk_podf(&mut self, val: super::vals::Sai1clkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Divider for sai1 clock pred."]
    #[inline(always)]
    pub const fn sai1_clk_pred(&self) -> super::vals::Sai1clkPred {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Sai1clkPred::from_bits(val as u8)
    }
    #[doc = "Divider for sai1 clock pred."]
    #[inline(always)]
    pub const fn set_sai1_clk_pred(&mut self, val: super::vals::Sai1clkPred) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Divider for flexio1 clock."]
    #[inline(always)]
    pub const fn flexio1_clk_pred(&self) -> super::vals::Flexio1clkPred {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Flexio1clkPred::from_bits(val as u8)
    }
    #[doc = "Divider for flexio1 clock."]
    #[inline(always)]
    pub const fn set_flexio1_clk_pred(&mut self, val: super::vals::Flexio1clkPred) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Divider for sai3 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn sai3_clk_podf(&self) -> super::vals::Sai3clkPodf {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Sai3clkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for sai3 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_sai3_clk_podf(&mut self, val: super::vals::Sai3clkPodf) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "Divider for sai3 clock pred."]
    #[inline(always)]
    pub const fn sai3_clk_pred(&self) -> super::vals::Sai3clkPred {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Sai3clkPred::from_bits(val as u8)
    }
    #[doc = "Divider for sai3 clock pred."]
    #[inline(always)]
    pub const fn set_sai3_clk_pred(&mut self, val: super::vals::Sai3clkPred) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Divider for flexio1 clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn flexio1_clk_podf(&self) -> super::vals::Flexio1clkPodf {
        let val = (self.0 >> 25usize) & 0x0f;
        super::vals::Flexio1clkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for flexio1 clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_flexio1_clk_podf(&mut self, val: super::vals::Flexio1clkPodf) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
    }
}
impl Default for Cs1cdr {
    #[inline(always)]
    fn default() -> Cs1cdr {
        Cs1cdr(0)
    }
}
#[doc = "CCM Serial Clock Divider Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cscdr1(pub u32);
impl Cscdr1 {
    #[doc = "Divider for uart clock podf."]
    #[inline(always)]
    pub const fn uart_clk_podf(&self) -> super::vals::UartClkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::UartClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for uart clock podf."]
    #[inline(always)]
    pub const fn set_uart_clk_podf(&mut self, val: super::vals::UartClkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Selector for the UART clock multiplexor"]
    #[inline(always)]
    pub const fn uart_clk_sel(&self) -> super::vals::UartClkSel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::UartClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for the UART clock multiplexor"]
    #[inline(always)]
    pub const fn set_uart_clk_sel(&mut self, val: super::vals::UartClkSel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn trace_podf(&self) -> super::vals::TracePodf {
        let val = (self.0 >> 25usize) & 0x0f;
        super::vals::TracePodf::from_bits(val as u8)
    }
    #[doc = "Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub const fn set_trace_podf(&mut self, val: super::vals::TracePodf) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
    }
}
impl Default for Cscdr1 {
    #[inline(always)]
    fn default() -> Cscdr1 {
        Cscdr1(0)
    }
}
#[doc = "CCM Serial Clock Divider Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cscdr2(pub u32);
impl Cscdr2 {
    #[doc = "Selector for the LPI2C clock multiplexor"]
    #[inline(always)]
    pub const fn lpi2c_clk_sel(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Selector for the LPI2C clock multiplexor"]
    #[inline(always)]
    pub const fn set_lpi2c_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Divider for lpi2c clock podf. Divider should be updated when output clock is gated. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn lpi2c_clk_podf(&self) -> super::vals::Lpi2cClkPodf {
        let val = (self.0 >> 19usize) & 0x3f;
        super::vals::Lpi2cClkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for lpi2c clock podf. Divider should be updated when output clock is gated. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub const fn set_lpi2c_clk_podf(&mut self, val: super::vals::Lpi2cClkPodf) {
        self.0 = (self.0 & !(0x3f << 19usize)) | (((val.to_bits() as u32) & 0x3f) << 19usize);
    }
}
impl Default for Cscdr2 {
    #[inline(always)]
    fn default() -> Cscdr2 {
        Cscdr2(0)
    }
}
#[doc = "CCM Serial Clock Multiplexer Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cscmr1(pub u32);
impl Cscmr1 {
    #[doc = "Divider for perclk podf."]
    #[inline(always)]
    pub const fn perclk_podf(&self) -> super::vals::PerclkPodf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::PerclkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for perclk podf."]
    #[inline(always)]
    pub const fn set_perclk_podf(&mut self, val: super::vals::PerclkPodf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Selector for the perclk clock multiplexor"]
    #[inline(always)]
    pub const fn perclk_clk_sel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Selector for the perclk clock multiplexor"]
    #[inline(always)]
    pub const fn set_perclk_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Selector for sai1 clock multiplexer"]
    #[inline(always)]
    pub const fn sai1_clk_sel(&self) -> super::vals::Sai1clkSel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sai1clkSel::from_bits(val as u8)
    }
    #[doc = "Selector for sai1 clock multiplexer"]
    #[inline(always)]
    pub const fn set_sai1_clk_sel(&mut self, val: super::vals::Sai1clkSel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Selector for sai3 clock multiplexer"]
    #[inline(always)]
    pub const fn sai3_clk_sel(&self) -> super::vals::Sai3clkSel {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sai3clkSel::from_bits(val as u8)
    }
    #[doc = "Selector for sai3 clock multiplexer"]
    #[inline(always)]
    pub const fn set_sai3_clk_sel(&mut self, val: super::vals::Sai3clkSel) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Divider for flexspi clock root."]
    #[inline(always)]
    pub const fn flexspi_podf(&self) -> super::vals::FlexspiPodf {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::FlexspiPodf::from_bits(val as u8)
    }
    #[doc = "Divider for flexspi clock root."]
    #[inline(always)]
    pub const fn set_flexspi_podf(&mut self, val: super::vals::FlexspiPodf) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Selector for flexspi clock multiplexer"]
    #[inline(always)]
    pub const fn flexspi_clk_sel(&self) -> super::vals::FlexspiClkSel {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::FlexspiClkSel::from_bits(val as u8)
    }
    #[doc = "Selector for flexspi clock multiplexer"]
    #[inline(always)]
    pub const fn set_flexspi_clk_sel(&mut self, val: super::vals::FlexspiClkSel) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "Select for source of flexspi_clk_root"]
    #[inline(always)]
    pub const fn flexspi_clk_src(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Select for source of flexspi_clk_root"]
    #[inline(always)]
    pub const fn set_flexspi_clk_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cscmr1 {
    #[inline(always)]
    fn default() -> Cscmr1 {
        Cscmr1(0)
    }
}
#[doc = "CCM Serial Clock Multiplexer Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cscmr2(pub u32);
impl Cscmr2 {
    #[doc = "Selector for flexio1 clock multiplexer"]
    #[inline(always)]
    pub const fn flexio1_clk_sel(&self) -> super::vals::Flexio1clkSel {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::Flexio1clkSel::from_bits(val as u8)
    }
    #[doc = "Selector for flexio1 clock multiplexer"]
    #[inline(always)]
    pub const fn set_flexio1_clk_sel(&mut self, val: super::vals::Flexio1clkSel) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "Divider for ADC alt_clk, as the list below (other values reserved)."]
    #[inline(always)]
    pub const fn adc_aclk_podf(&self) -> super::vals::AdcAclkPodf {
        let val = (self.0 >> 27usize) & 0x0f;
        super::vals::AdcAclkPodf::from_bits(val as u8)
    }
    #[doc = "Divider for ADC alt_clk, as the list below (other values reserved)."]
    #[inline(always)]
    pub const fn set_adc_aclk_podf(&mut self, val: super::vals::AdcAclkPodf) {
        self.0 = (self.0 & !(0x0f << 27usize)) | (((val.to_bits() as u32) & 0x0f) << 27usize);
    }
    #[doc = "Enable ADC alt_clk, so that ADC alt_clk can be driven be divided pll3_sw_clk."]
    #[inline(always)]
    pub const fn adc_aclk_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ADC alt_clk, so that ADC alt_clk can be driven be divided pll3_sw_clk."]
    #[inline(always)]
    pub const fn set_adc_aclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cscmr2 {
    #[inline(always)]
    fn default() -> Cscmr2 {
        Cscmr2(0)
    }
}
#[doc = "CCM Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Status of the value of CCM_REF_EN_B output of ccm"]
    #[inline(always)]
    pub const fn ref_en_b(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Status of the value of CCM_REF_EN_B output of ccm"]
    #[inline(always)]
    pub const fn set_ref_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Status indication of CAMP2."]
    #[inline(always)]
    pub const fn camp2_ready(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Status indication of CAMP2."]
    #[inline(always)]
    pub const fn set_camp2_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Status indication of on board oscillator"]
    #[inline(always)]
    pub const fn cosc_ready(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status indication of on board oscillator"]
    #[inline(always)]
    pub const fn set_cosc_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
