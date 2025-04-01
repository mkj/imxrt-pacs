#[doc = "DCP capability 0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Capability0(pub u32);
impl Capability0 {
    #[doc = "Encoded value indicating the number of key-storage locations implemented in the design"]
    #[inline(always)]
    pub const fn num_keys(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Encoded value indicating the number of key-storage locations implemented in the design"]
    #[inline(always)]
    pub const fn set_num_keys(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Encoded value indicating the number of channels implemented in the design"]
    #[inline(always)]
    pub const fn num_channels(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Encoded value indicating the number of channels implemented in the design"]
    #[inline(always)]
    pub const fn set_num_channels(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write to a 1 to disable the per-device unique key"]
    #[inline(always)]
    pub const fn disable_unique_key(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write to a 1 to disable the per-device unique key"]
    #[inline(always)]
    pub const fn set_disable_unique_key(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Write to 1 to disable the decryption"]
    #[inline(always)]
    pub const fn disable_decrypt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write to 1 to disable the decryption"]
    #[inline(always)]
    pub const fn set_disable_decrypt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Capability0 {
    #[inline(always)]
    fn default() -> Capability0 {
        Capability0(0)
    }
}
#[doc = "DCP capability 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Capability1(pub u32);
impl Capability1 {
    #[doc = "One-hot field indicating which cipher algorithms are available"]
    #[inline(always)]
    pub const fn cipher_algorithms(&self) -> super::vals::CipherAlgorithms {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::CipherAlgorithms::from_bits(val as u16)
    }
    #[doc = "One-hot field indicating which cipher algorithms are available"]
    #[inline(always)]
    pub const fn set_cipher_algorithms(&mut self, val: super::vals::CipherAlgorithms) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "One-hot field indicating which hashing features are implemented in the hardware"]
    #[inline(always)]
    pub const fn hash_algorithms(&self) -> super::vals::HashAlgorithms {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::HashAlgorithms::from_bits(val as u16)
    }
    #[doc = "One-hot field indicating which hashing features are implemented in the hardware"]
    #[inline(always)]
    pub const fn set_hash_algorithms(&mut self, val: super::vals::HashAlgorithms) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capability1 {
    #[inline(always)]
    fn default() -> Capability1 {
        Capability1(0)
    }
}
#[doc = "DCP channel 0 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0opts(pub u32);
impl Ch0opts {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch0opts {
    #[inline(always)]
    fn default() -> Ch0opts {
        Ch0opts(0)
    }
}
#[doc = "DCP channel 0 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0optsClr(pub u32);
impl Ch0optsClr {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch0optsClr {
    #[inline(always)]
    fn default() -> Ch0optsClr {
        Ch0optsClr(0)
    }
}
#[doc = "DCP channel 0 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0optsSet(pub u32);
impl Ch0optsSet {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch0optsSet {
    #[inline(always)]
    fn default() -> Ch0optsSet {
        Ch0optsSet(0)
    }
}
#[doc = "DCP channel 0 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0optsTog(pub u32);
impl Ch0optsTog {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch0optsTog {
    #[inline(always)]
    fn default() -> Ch0optsTog {
        Ch0optsTog(0)
    }
}
#[doc = "DCP channel 0 semaphore register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0sema(pub u32);
impl Ch0sema {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn increment(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The value written to this field is added to the semaphore count in an atomic way such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn set_increment(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ch0sema {
    #[inline(always)]
    fn default() -> Ch0sema {
        Ch0sema(0)
    }
}
#[doc = "DCP channel 0 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0stat(pub u32);
impl Ch0stat {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch0statErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch0statErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch0statErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch0stat {
    #[inline(always)]
    fn default() -> Ch0stat {
        Ch0stat(0)
    }
}
#[doc = "DCP channel 0 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0statClr(pub u32);
impl Ch0statClr {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch0statClrErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch0statClrErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch0statClrErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch0statClr {
    #[inline(always)]
    fn default() -> Ch0statClr {
        Ch0statClr(0)
    }
}
#[doc = "DCP channel 0 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0statSet(pub u32);
impl Ch0statSet {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch0statSetErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch0statSetErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch0statSetErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch0statSet {
    #[inline(always)]
    fn default() -> Ch0statSet {
        Ch0statSet(0)
    }
}
#[doc = "DCP channel 0 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch0statTog(pub u32);
impl Ch0statTog {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch0statTogErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch0statTogErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch0statTogErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch0statTog {
    #[inline(always)]
    fn default() -> Ch0statTog {
        Ch0statTog(0)
    }
}
#[doc = "DCP channel 1 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1opts(pub u32);
impl Ch1opts {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch1opts {
    #[inline(always)]
    fn default() -> Ch1opts {
        Ch1opts(0)
    }
}
#[doc = "DCP channel 1 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1optsClr(pub u32);
impl Ch1optsClr {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch1optsClr {
    #[inline(always)]
    fn default() -> Ch1optsClr {
        Ch1optsClr(0)
    }
}
#[doc = "DCP channel 1 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1optsSet(pub u32);
impl Ch1optsSet {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch1optsSet {
    #[inline(always)]
    fn default() -> Ch1optsSet {
        Ch1optsSet(0)
    }
}
#[doc = "DCP channel 1 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1optsTog(pub u32);
impl Ch1optsTog {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch1optsTog {
    #[inline(always)]
    fn default() -> Ch1optsTog {
        Ch1optsTog(0)
    }
}
#[doc = "DCP channel 1 semaphore register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1sema(pub u32);
impl Ch1sema {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and the DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn increment(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and the DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn set_increment(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ch1sema {
    #[inline(always)]
    fn default() -> Ch1sema {
        Ch1sema(0)
    }
}
#[doc = "DCP channel 1 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1stat(pub u32);
impl Ch1stat {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch1statErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch1statErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch1statErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch1stat {
    #[inline(always)]
    fn default() -> Ch1stat {
        Ch1stat(0)
    }
}
#[doc = "DCP channel 1 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1statClr(pub u32);
impl Ch1statClr {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch1statClrErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch1statClrErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch1statClrErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch1statClr {
    #[inline(always)]
    fn default() -> Ch1statClr {
        Ch1statClr(0)
    }
}
#[doc = "DCP channel 1 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1statSet(pub u32);
impl Ch1statSet {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch1statSetErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch1statSetErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch1statSetErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch1statSet {
    #[inline(always)]
    fn default() -> Ch1statSet {
        Ch1statSet(0)
    }
}
#[doc = "DCP channel 1 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch1statTog(pub u32);
impl Ch1statTog {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch1statTogErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch1statTogErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch1statTogErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch1statTog {
    #[inline(always)]
    fn default() -> Ch1statTog {
        Ch1statTog(0)
    }
}
#[doc = "DCP channel 2 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2opts(pub u32);
impl Ch2opts {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch2opts {
    #[inline(always)]
    fn default() -> Ch2opts {
        Ch2opts(0)
    }
}
#[doc = "DCP channel 2 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2optsClr(pub u32);
impl Ch2optsClr {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch2optsClr {
    #[inline(always)]
    fn default() -> Ch2optsClr {
        Ch2optsClr(0)
    }
}
#[doc = "DCP channel 2 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2optsSet(pub u32);
impl Ch2optsSet {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch2optsSet {
    #[inline(always)]
    fn default() -> Ch2optsSet {
        Ch2optsSet(0)
    }
}
#[doc = "DCP channel 2 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2optsTog(pub u32);
impl Ch2optsTog {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch2optsTog {
    #[inline(always)]
    fn default() -> Ch2optsTog {
        Ch2optsTog(0)
    }
}
#[doc = "DCP channel 2 semaphore register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2sema(pub u32);
impl Ch2sema {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn increment(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn set_increment(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ch2sema {
    #[inline(always)]
    fn default() -> Ch2sema {
        Ch2sema(0)
    }
}
#[doc = "DCP channel 2 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2stat(pub u32);
impl Ch2stat {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch2statErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch2statErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch2statErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch2stat {
    #[inline(always)]
    fn default() -> Ch2stat {
        Ch2stat(0)
    }
}
#[doc = "DCP channel 2 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2statClr(pub u32);
impl Ch2statClr {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch2statClrErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch2statClrErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch2statClrErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch2statClr {
    #[inline(always)]
    fn default() -> Ch2statClr {
        Ch2statClr(0)
    }
}
#[doc = "DCP channel 2 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2statSet(pub u32);
impl Ch2statSet {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch2statSetErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch2statSetErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch2statSetErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch2statSet {
    #[inline(always)]
    fn default() -> Ch2statSet {
        Ch2statSet(0)
    }
}
#[doc = "DCP channel 2 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch2statTog(pub u32);
impl Ch2statTog {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch2statTogErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch2statTogErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch2statTogErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch2statTog {
    #[inline(always)]
    fn default() -> Ch2statTog {
        Ch2statTog(0)
    }
}
#[doc = "DCP channel 3 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3opts(pub u32);
impl Ch3opts {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch3opts {
    #[inline(always)]
    fn default() -> Ch3opts {
        Ch3opts(0)
    }
}
#[doc = "DCP channel 3 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3optsClr(pub u32);
impl Ch3optsClr {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch3optsClr {
    #[inline(always)]
    fn default() -> Ch3optsClr {
        Ch3optsClr(0)
    }
}
#[doc = "DCP channel 3 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3optsSet(pub u32);
impl Ch3optsSet {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch3optsSet {
    #[inline(always)]
    fn default() -> Ch3optsSet {
        Ch3optsSet(0)
    }
}
#[doc = "DCP channel 3 options register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3optsTog(pub u32);
impl Ch3optsTog {
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn recovery_timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub const fn set_recovery_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch3optsTog {
    #[inline(always)]
    fn default() -> Ch3optsTog {
        Ch3optsTog(0)
    }
}
#[doc = "DCP channel 3 semaphore register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3sema(pub u32);
impl Ch3sema {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn increment(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub const fn set_increment(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ch3sema {
    #[inline(always)]
    fn default() -> Ch3sema {
        Ch3sema(0)
    }
}
#[doc = "DCP channel 3 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3stat(pub u32);
impl Ch3stat {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch3statErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch3statErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch3statErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch3stat {
    #[inline(always)]
    fn default() -> Ch3stat {
        Ch3stat(0)
    }
}
#[doc = "DCP channel 3 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3statClr(pub u32);
impl Ch3statClr {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch3statClrErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch3statClrErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch3statClrErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch3statClr {
    #[inline(always)]
    fn default() -> Ch3statClr {
        Ch3statClr(0)
    }
}
#[doc = "DCP channel 3 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3statSet(pub u32);
impl Ch3statSet {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch3statSetErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch3statSetErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch3statSetErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch3statSet {
    #[inline(always)]
    fn default() -> Ch3statSet {
        Ch3statSet(0)
    }
}
#[doc = "DCP channel 3 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ch3statTog(pub u32);
impl Ch3statTog {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn hash_mismatch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub const fn set_hash_mismatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn error_setup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub const fn set_error_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn error_packet(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub const fn set_error_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn error_src(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub const fn set_error_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn error_dst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub const fn set_error_dst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn error_pagefault(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub const fn set_error_pagefault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn error_code(&self) -> super::vals::Ch3statTogErrorCode {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Ch3statTogErrorCode::from_bits(val as u8)
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub const fn set_error_code(&mut self, val: super::vals::Ch3statTogErrorCode) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ch3statTog {
    #[inline(always)]
    fn default() -> Ch3statTog {
        Ch3statTog(0)
    }
}
#[doc = "DCP channel control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Channelctrl(pub u32);
impl Channelctrl {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn enable_channel(&self) -> super::vals::ChannelctrlEnableChannel {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ChannelctrlEnableChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn set_enable_channel(&mut self, val: super::vals::ChannelctrlEnableChannel) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn high_priority_channel(&self) -> super::vals::ChannelctrlHighPriorityChannel {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::ChannelctrlHighPriorityChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn set_high_priority_channel(
        &mut self,
        val: super::vals::ChannelctrlHighPriorityChannel,
    ) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn ch0_irq_merged(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn set_ch0_irq_merged(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Channelctrl {
    #[inline(always)]
    fn default() -> Channelctrl {
        Channelctrl(0)
    }
}
#[doc = "DCP channel control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ChannelctrlClr(pub u32);
impl ChannelctrlClr {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn enable_channel(&self) -> super::vals::ChannelctrlClrEnableChannel {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ChannelctrlClrEnableChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn set_enable_channel(&mut self, val: super::vals::ChannelctrlClrEnableChannel) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn high_priority_channel(&self) -> super::vals::ChannelctrlClrHighPriorityChannel {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::ChannelctrlClrHighPriorityChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn set_high_priority_channel(
        &mut self,
        val: super::vals::ChannelctrlClrHighPriorityChannel,
    ) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn ch0_irq_merged(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn set_ch0_irq_merged(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for ChannelctrlClr {
    #[inline(always)]
    fn default() -> ChannelctrlClr {
        ChannelctrlClr(0)
    }
}
#[doc = "DCP channel control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ChannelctrlSet(pub u32);
impl ChannelctrlSet {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn enable_channel(&self) -> super::vals::ChannelctrlSetEnableChannel {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ChannelctrlSetEnableChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn set_enable_channel(&mut self, val: super::vals::ChannelctrlSetEnableChannel) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn high_priority_channel(&self) -> super::vals::ChannelctrlSetHighPriorityChannel {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::ChannelctrlSetHighPriorityChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn set_high_priority_channel(
        &mut self,
        val: super::vals::ChannelctrlSetHighPriorityChannel,
    ) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn ch0_irq_merged(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn set_ch0_irq_merged(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for ChannelctrlSet {
    #[inline(always)]
    fn default() -> ChannelctrlSet {
        ChannelctrlSet(0)
    }
}
#[doc = "DCP channel control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ChannelctrlTog(pub u32);
impl ChannelctrlTog {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn enable_channel(&self) -> super::vals::ChannelctrlTogEnableChannel {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ChannelctrlTogEnableChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub const fn set_enable_channel(&mut self, val: super::vals::ChannelctrlTogEnableChannel) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn high_priority_channel(&self) -> super::vals::ChannelctrlTogHighPriorityChannel {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::ChannelctrlTogHighPriorityChannel::from_bits(val as u8)
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub const fn set_high_priority_channel(
        &mut self,
        val: super::vals::ChannelctrlTogHighPriorityChannel,
    ) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn ch0_irq_merged(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub const fn set_ch0_irq_merged(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for ChannelctrlTog {
    #[inline(always)]
    fn default() -> ChannelctrlTog {
        ChannelctrlTog(0)
    }
}
#[doc = "DCP control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn channel_interrupt_enable(&self) -> super::vals::CtrlChannelInterruptEnable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::CtrlChannelInterruptEnable::from_bits(val as u8)
    }
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn set_channel_interrupt_enable(
        &mut self,
        val: super::vals::CtrlChannelInterruptEnable,
    ) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn enable_context_switching(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn set_enable_context_switching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn enable_context_caching(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn set_enable_context_caching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn gather_residual_writes(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn set_gather_residual_writes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn present_sha(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn set_present_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn present_crypto(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn set_present_crypto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
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
#[doc = "DCP control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn channel_interrupt_enable(&self) -> super::vals::CtrlClrChannelInterruptEnable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::CtrlClrChannelInterruptEnable::from_bits(val as u8)
    }
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn set_channel_interrupt_enable(
        &mut self,
        val: super::vals::CtrlClrChannelInterruptEnable,
    ) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn enable_context_switching(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn set_enable_context_switching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn enable_context_caching(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn set_enable_context_caching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn gather_residual_writes(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn set_gather_residual_writes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn present_sha(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn set_present_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn present_crypto(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn set_present_crypto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
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
#[doc = "DCP control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn channel_interrupt_enable(&self) -> super::vals::CtrlSetChannelInterruptEnable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::CtrlSetChannelInterruptEnable::from_bits(val as u8)
    }
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn set_channel_interrupt_enable(
        &mut self,
        val: super::vals::CtrlSetChannelInterruptEnable,
    ) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn enable_context_switching(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn set_enable_context_switching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn enable_context_caching(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn set_enable_context_caching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn gather_residual_writes(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn set_gather_residual_writes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn present_sha(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn set_present_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn present_crypto(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn set_present_crypto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
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
#[doc = "DCP control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn channel_interrupt_enable(&self) -> super::vals::CtrlTogChannelInterruptEnable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::CtrlTogChannelInterruptEnable::from_bits(val as u8)
    }
    #[doc = "Per-channel interrupt enable bit"]
    #[inline(always)]
    pub const fn set_channel_interrupt_enable(
        &mut self,
        val: super::vals::CtrlTogChannelInterruptEnable,
    ) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn enable_context_switching(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable automatic context switching for the channels"]
    #[inline(always)]
    pub const fn set_enable_context_switching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn enable_context_caching(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub const fn set_enable_context_caching(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn gather_residual_writes(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub const fn set_gather_residual_writes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn present_sha(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub const fn set_present_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn present_crypto(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub const fn set_present_crypto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
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
#[doc = "DCP debug select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dbgselect(pub u32);
impl Dbgselect {
    #[doc = "Selects a value to read via the debug data register."]
    #[inline(always)]
    pub const fn index(&self) -> super::vals::Index {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Index::from_bits(val as u8)
    }
    #[doc = "Selects a value to read via the debug data register."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: super::vals::Index) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Dbgselect {
    #[inline(always)]
    fn default() -> Dbgselect {
        Dbgselect(0)
    }
}
#[doc = "DCP key index"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Key(pub u32);
impl Key {
    #[doc = "Key subword pointer"]
    #[inline(always)]
    pub const fn subword(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Key subword pointer"]
    #[inline(always)]
    pub const fn set_subword(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Key {
    #[inline(always)]
    fn default() -> Key {
        Key(0)
    }
}
#[doc = "DCP work packet 1 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Packet1(pub u32);
impl Packet1 {
    #[doc = "Reflects whether the channel must issue an interrupt upon the completion of the packet."]
    #[inline(always)]
    pub const fn interrupt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the channel must issue an interrupt upon the completion of the packet."]
    #[inline(always)]
    pub const fn set_interrupt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reflects whether the channel's semaphore must be decremented at the end of the current operation"]
    #[inline(always)]
    pub const fn decr_semaphore(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the channel's semaphore must be decremented at the end of the current operation"]
    #[inline(always)]
    pub const fn set_decr_semaphore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reflects whether the next command pointer register must be loaded into the channel's current descriptor pointer"]
    #[inline(always)]
    pub const fn chain(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the next command pointer register must be loaded into the channel's current descriptor pointer"]
    #[inline(always)]
    pub const fn set_chain(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reflects whether the next packet's address is located following this packet's payload."]
    #[inline(always)]
    pub const fn chain_contiguous(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the next packet's address is located following this packet's payload."]
    #[inline(always)]
    pub const fn set_chain_contiguous(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reflects whether the selected memory-copy (memcopy) function should be enabled for this operation."]
    #[inline(always)]
    pub const fn enable_memcopy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the selected memory-copy (memcopy) function should be enabled for this operation."]
    #[inline(always)]
    pub const fn set_enable_memcopy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Reflects whether the selected cipher function must be enabled for this operation."]
    #[inline(always)]
    pub const fn enable_cipher(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the selected cipher function must be enabled for this operation."]
    #[inline(always)]
    pub const fn set_enable_cipher(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reflects whether the selected hashing function must be enabled for this operation."]
    #[inline(always)]
    pub const fn enable_hash(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the selected hashing function must be enabled for this operation."]
    #[inline(always)]
    pub const fn set_enable_hash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reflects whether the DCP must perform a blit operation"]
    #[inline(always)]
    pub const fn enable_blit(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP must perform a blit operation"]
    #[inline(always)]
    pub const fn set_enable_blit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption"]
    #[inline(always)]
    pub const fn cipher_encrypt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption"]
    #[inline(always)]
    pub const fn set_cipher_encrypt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reflects whether the cipher block must load the initialization vector from the payload for this operation"]
    #[inline(always)]
    pub const fn cipher_init(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the cipher block must load the initialization vector from the payload for this operation"]
    #[inline(always)]
    pub const fn set_cipher_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reflects whether a hardware-based key must be used"]
    #[inline(always)]
    pub const fn otp_key(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether a hardware-based key must be used"]
    #[inline(always)]
    pub const fn set_otp_key(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When set, it indicates the payload contains the key"]
    #[inline(always)]
    pub const fn payload_key(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When set, it indicates the payload contains the key"]
    #[inline(always)]
    pub const fn set_payload_key(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reflects whether the current hashing block is the initial block in the hashing operation, so the hash registers must be initialized before the operation"]
    #[inline(always)]
    pub const fn hash_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the current hashing block is the initial block in the hashing operation, so the hash registers must be initialized before the operation"]
    #[inline(always)]
    pub const fn set_hash_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reflects whether the current hashing block is the final block in the hashing operation, so the hash padding must be applied by the hardware"]
    #[inline(always)]
    pub const fn hash_term(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the current hashing block is the final block in the hashing operation, so the hash padding must be applied by the hardware"]
    #[inline(always)]
    pub const fn set_hash_term(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Reflects whether the calculated hash value must be compared to the hash provided in the payload."]
    #[inline(always)]
    pub const fn check_hash(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the calculated hash value must be compared to the hash provided in the payload."]
    #[inline(always)]
    pub const fn set_check_hash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When the hashing is enabled, this bit controls whether the input or output data is hashed."]
    #[inline(always)]
    pub const fn hash_output(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When the hashing is enabled, this bit controls whether the input or output data is hashed."]
    #[inline(always)]
    pub const fn set_hash_output(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When this bit is set (MEMCOPY and BLIT modes only), the DCP simply fills the destination buffer with the value found in the source address field"]
    #[inline(always)]
    pub const fn constant_fill(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set (MEMCOPY and BLIT modes only), the DCP simply fills the destination buffer with the value found in the source address field"]
    #[inline(always)]
    pub const fn set_constant_fill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit is used to test the channel semaphore transition to 0. FOR TEST USE ONLY!"]
    #[inline(always)]
    pub const fn test_sema_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to test the channel semaphore transition to 0. FOR TEST USE ONLY!"]
    #[inline(always)]
    pub const fn set_test_sema_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Reflects whether the DCP engine swaps the key bytes (big-endian key)."]
    #[inline(always)]
    pub const fn key_byteswap(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP engine swaps the key bytes (big-endian key)."]
    #[inline(always)]
    pub const fn set_key_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Reflects whether the DCP engine swaps the key words (big-endian key)."]
    #[inline(always)]
    pub const fn key_wordswap(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP engine swaps the key words (big-endian key)."]
    #[inline(always)]
    pub const fn set_key_wordswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Reflects whether the DCP engine byteswaps the input data (big-endian data)."]
    #[inline(always)]
    pub const fn input_byteswap(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP engine byteswaps the input data (big-endian data)."]
    #[inline(always)]
    pub const fn set_input_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reflects whether the DCP engine wordswaps the input data (big-endian data)."]
    #[inline(always)]
    pub const fn input_wordswap(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP engine wordswaps the input data (big-endian data)."]
    #[inline(always)]
    pub const fn set_input_wordswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Reflects whether the DCP engine byteswaps the output data (big-endian data)."]
    #[inline(always)]
    pub const fn output_byteswap(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP engine byteswaps the output data (big-endian data)."]
    #[inline(always)]
    pub const fn set_output_byteswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reflects whether the DCP engine wordswaps the output data (big-endian data)."]
    #[inline(always)]
    pub const fn output_wordswap(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects whether the DCP engine wordswaps the output data (big-endian data)."]
    #[inline(always)]
    pub const fn set_output_wordswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Packet Tag"]
    #[inline(always)]
    pub const fn tag(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Packet Tag"]
    #[inline(always)]
    pub const fn set_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Packet1 {
    #[inline(always)]
    fn default() -> Packet1 {
        Packet1(0)
    }
}
#[doc = "DCP work packet 2 status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Packet2(pub u32);
impl Packet2 {
    #[doc = "Cipher selection field"]
    #[inline(always)]
    pub const fn cipher_select(&self) -> super::vals::CipherSelect {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::CipherSelect::from_bits(val as u8)
    }
    #[doc = "Cipher selection field"]
    #[inline(always)]
    pub const fn set_cipher_select(&mut self, val: super::vals::CipherSelect) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
    #[inline(always)]
    pub const fn cipher_mode(&self) -> super::vals::CipherMode {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::CipherMode::from_bits(val as u8)
    }
    #[doc = "Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
    #[inline(always)]
    pub const fn set_cipher_mode(&mut self, val: super::vals::CipherMode) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Key selection field"]
    #[inline(always)]
    pub const fn key_select(&self) -> super::vals::KeySelect {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::KeySelect::from_bits(val as u8)
    }
    #[doc = "Key selection field"]
    #[inline(always)]
    pub const fn set_key_select(&mut self, val: super::vals::KeySelect) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Hash Selection Field"]
    #[inline(always)]
    pub const fn hash_select(&self) -> super::vals::HashSelect {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::HashSelect::from_bits(val as u8)
    }
    #[doc = "Hash Selection Field"]
    #[inline(always)]
    pub const fn set_hash_select(&mut self, val: super::vals::HashSelect) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Cipher configuration bits. Optional configuration bits are required for the ciphers."]
    #[inline(always)]
    pub const fn cipher_cfg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Cipher configuration bits. Optional configuration bits are required for the ciphers."]
    #[inline(always)]
    pub const fn set_cipher_cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Packet2 {
    #[inline(always)]
    fn default() -> Packet2 {
        Packet2(0)
    }
}
#[doc = "DCP page table register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pagetable(pub u32);
impl Pagetable {
    #[doc = "Page table enable control"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Page table enable control"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
    #[inline(always)]
    pub const fn flush(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
    #[inline(always)]
    pub const fn set_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Page table base address"]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Page table base address"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Pagetable {
    #[inline(always)]
    fn default() -> Pagetable {
        Pagetable(0)
    }
}
#[doc = "DCP status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn ready_channels(&self) -> super::vals::StatReadyChannels {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::StatReadyChannels::from_bits(val as u8)
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn set_ready_channels(&mut self, val: super::vals::StatReadyChannels) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn cur_channel(&self) -> super::vals::StatCurChannel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::StatCurChannel::from_bits(val as u8)
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn set_cur_channel(&mut self, val: super::vals::StatCurChannel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn otp_key_ready(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn set_otp_key_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "DCP status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StatClr(pub u32);
impl StatClr {
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn ready_channels(&self) -> super::vals::StatClrReadyChannels {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::StatClrReadyChannels::from_bits(val as u8)
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn set_ready_channels(&mut self, val: super::vals::StatClrReadyChannels) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn cur_channel(&self) -> super::vals::StatClrCurChannel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::StatClrCurChannel::from_bits(val as u8)
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn set_cur_channel(&mut self, val: super::vals::StatClrCurChannel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn otp_key_ready(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn set_otp_key_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for StatClr {
    #[inline(always)]
    fn default() -> StatClr {
        StatClr(0)
    }
}
#[doc = "DCP status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StatSet(pub u32);
impl StatSet {
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn ready_channels(&self) -> super::vals::StatSetReadyChannels {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::StatSetReadyChannels::from_bits(val as u8)
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn set_ready_channels(&mut self, val: super::vals::StatSetReadyChannels) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn cur_channel(&self) -> super::vals::StatSetCurChannel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::StatSetCurChannel::from_bits(val as u8)
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn set_cur_channel(&mut self, val: super::vals::StatSetCurChannel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn otp_key_ready(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn set_otp_key_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for StatSet {
    #[inline(always)]
    fn default() -> StatSet {
        StatSet(0)
    }
}
#[doc = "DCP status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StatTog(pub u32);
impl StatTog {
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn ready_channels(&self) -> super::vals::StatTogReadyChannels {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::StatTogReadyChannels::from_bits(val as u8)
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub const fn set_ready_channels(&mut self, val: super::vals::StatTogReadyChannels) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn cur_channel(&self) -> super::vals::StatTogCurChannel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::StatTogCurChannel::from_bits(val as u8)
    }
    #[doc = "Current (active) channel (encoded)"]
    #[inline(always)]
    pub const fn set_cur_channel(&mut self, val: super::vals::StatTogCurChannel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn otp_key_ready(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub const fn set_otp_key_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for StatTog {
    #[inline(always)]
    fn default() -> StatTog {
        StatTog(0)
    }
}
#[doc = "DCP version register"]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Fixed read-only value reflecting the stepping of the version of the design implementation."]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fixed read-only value reflecting the stepping of the version of the design implementation."]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fixed read-only value reflecting the MINOR version of the design implementation."]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MINOR version of the design implementation."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Fixed read-only value reflecting the MAJOR version of the design implementation."]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MAJOR version of the design implementation."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
