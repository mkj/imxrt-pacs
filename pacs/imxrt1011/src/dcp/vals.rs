#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statClrErrorCode(pub u8);
impl Ch0statClrErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch0statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statClrErrorCode {
        Ch0statClrErrorCode::from_bits(val)
    }
}
impl From<Ch0statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statClrErrorCode) -> u8 {
        Ch0statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statErrorCode(pub u8);
impl Ch0statErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch0statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statErrorCode {
        Ch0statErrorCode::from_bits(val)
    }
}
impl From<Ch0statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statErrorCode) -> u8 {
        Ch0statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statSetErrorCode(pub u8);
impl Ch0statSetErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch0statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statSetErrorCode {
        Ch0statSetErrorCode::from_bits(val)
    }
}
impl From<Ch0statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statSetErrorCode) -> u8 {
        Ch0statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch0statTogErrorCode(pub u8);
impl Ch0statTogErrorCode {
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch0statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch0statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch0statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch0statTogErrorCode {
        Ch0statTogErrorCode::from_bits(val)
    }
}
impl From<Ch0statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch0statTogErrorCode) -> u8 {
        Ch0statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statClrErrorCode(pub u8);
impl Ch1statClrErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch1statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statClrErrorCode {
        Ch1statClrErrorCode::from_bits(val)
    }
}
impl From<Ch1statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statClrErrorCode) -> u8 {
        Ch1statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statErrorCode(pub u8);
impl Ch1statErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch1statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statErrorCode {
        Ch1statErrorCode::from_bits(val)
    }
}
impl From<Ch1statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statErrorCode) -> u8 {
        Ch1statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statSetErrorCode(pub u8);
impl Ch1statSetErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch1statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statSetErrorCode {
        Ch1statSetErrorCode::from_bits(val)
    }
}
impl From<Ch1statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statSetErrorCode) -> u8 {
        Ch1statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch1statTogErrorCode(pub u8);
impl Ch1statTogErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch1statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch1statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch1statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch1statTogErrorCode {
        Ch1statTogErrorCode::from_bits(val)
    }
}
impl From<Ch1statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch1statTogErrorCode) -> u8 {
        Ch1statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statClrErrorCode(pub u8);
impl Ch2statClrErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch2statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statClrErrorCode {
        Ch2statClrErrorCode::from_bits(val)
    }
}
impl From<Ch2statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statClrErrorCode) -> u8 {
        Ch2statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statErrorCode(pub u8);
impl Ch2statErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch2statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statErrorCode {
        Ch2statErrorCode::from_bits(val)
    }
}
impl From<Ch2statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statErrorCode) -> u8 {
        Ch2statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statSetErrorCode(pub u8);
impl Ch2statSetErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch2statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statSetErrorCode {
        Ch2statSetErrorCode::from_bits(val)
    }
}
impl From<Ch2statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statSetErrorCode) -> u8 {
        Ch2statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch2statTogErrorCode(pub u8);
impl Ch2statTogErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch2statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch2statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch2statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch2statTogErrorCode {
        Ch2statTogErrorCode::from_bits(val)
    }
}
impl From<Ch2statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch2statTogErrorCode) -> u8 {
        Ch2statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statClrErrorCode(pub u8);
impl Ch3statClrErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statClrErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statClrErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch3statClrErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statClrErrorCode {
        Ch3statClrErrorCode::from_bits(val)
    }
}
impl From<Ch3statClrErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statClrErrorCode) -> u8 {
        Ch3statClrErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statErrorCode(pub u8);
impl Ch3statErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch3statErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statErrorCode {
        Ch3statErrorCode::from_bits(val)
    }
}
impl From<Ch3statErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statErrorCode) -> u8 {
        Ch3statErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statSetErrorCode(pub u8);
impl Ch3statSetErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statSetErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statSetErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch3statSetErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statSetErrorCode {
        Ch3statSetErrorCode::from_bits(val)
    }
}
impl From<Ch3statSetErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statSetErrorCode) -> u8 {
        Ch3statSetErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ch3statTogErrorCode(pub u8);
impl Ch3statTogErrorCode {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    pub const NEXT_CHAIN_IS_0: Self = Self(0x01);
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    pub const NO_CHAIN: Self = Self(0x02);
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    pub const CONTEXT_ERROR: Self = Self(0x03);
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    pub const PAYLOAD_ERROR: Self = Self(0x04);
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    pub const INVALID_MODE: Self = Self(0x05);
}
impl Ch3statTogErrorCode {
    pub const fn from_bits(val: u8) -> Ch3statTogErrorCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Ch3statTogErrorCode {
    #[inline(always)]
    fn from(val: u8) -> Ch3statTogErrorCode {
        Ch3statTogErrorCode::from_bits(val)
    }
}
impl From<Ch3statTogErrorCode> for u8 {
    #[inline(always)]
    fn from(val: Ch3statTogErrorCode) -> u8 {
        Ch3statTogErrorCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlClrEnableChannel(pub u8);
impl ChannelctrlClrEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlClrEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlClrEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlClrEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlClrEnableChannel {
        ChannelctrlClrEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlClrEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlClrEnableChannel) -> u8 {
        ChannelctrlClrEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlClrHighPriorityChannel(pub u8);
impl ChannelctrlClrHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlClrHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlClrHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlClrHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlClrHighPriorityChannel {
        ChannelctrlClrHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlClrHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlClrHighPriorityChannel) -> u8 {
        ChannelctrlClrHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlEnableChannel(pub u8);
impl ChannelctrlEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlEnableChannel {
        ChannelctrlEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlEnableChannel) -> u8 {
        ChannelctrlEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlHighPriorityChannel(pub u8);
impl ChannelctrlHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlHighPriorityChannel {
        ChannelctrlHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlHighPriorityChannel) -> u8 {
        ChannelctrlHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlSetEnableChannel(pub u8);
impl ChannelctrlSetEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlSetEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlSetEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlSetEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlSetEnableChannel {
        ChannelctrlSetEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlSetEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlSetEnableChannel) -> u8 {
        ChannelctrlSetEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlSetHighPriorityChannel(pub u8);
impl ChannelctrlSetHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlSetHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlSetHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlSetHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlSetHighPriorityChannel {
        ChannelctrlSetHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlSetHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlSetHighPriorityChannel) -> u8 {
        ChannelctrlSetHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlTogEnableChannel(pub u8);
impl ChannelctrlTogEnableChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlTogEnableChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlTogEnableChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlTogEnableChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlTogEnableChannel {
        ChannelctrlTogEnableChannel::from_bits(val)
    }
}
impl From<ChannelctrlTogEnableChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlTogEnableChannel) -> u8 {
        ChannelctrlTogEnableChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChannelctrlTogHighPriorityChannel(pub u8);
impl ChannelctrlTogHighPriorityChannel {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl ChannelctrlTogHighPriorityChannel {
    pub const fn from_bits(val: u8) -> ChannelctrlTogHighPriorityChannel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for ChannelctrlTogHighPriorityChannel {
    #[inline(always)]
    fn from(val: u8) -> ChannelctrlTogHighPriorityChannel {
        ChannelctrlTogHighPriorityChannel::from_bits(val)
    }
}
impl From<ChannelctrlTogHighPriorityChannel> for u8 {
    #[inline(always)]
    fn from(val: ChannelctrlTogHighPriorityChannel) -> u8 {
        ChannelctrlTogHighPriorityChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CipherAlgorithms(pub u16);
impl CipherAlgorithms {
    #[doc = "AES128"]
    pub const AES128: Self = Self(0x01);
}
impl CipherAlgorithms {
    pub const fn from_bits(val: u16) -> CipherAlgorithms {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for CipherAlgorithms {
    #[inline(always)]
    fn from(val: u16) -> CipherAlgorithms {
        CipherAlgorithms::from_bits(val)
    }
}
impl From<CipherAlgorithms> for u16 {
    #[inline(always)]
    fn from(val: CipherAlgorithms) -> u16 {
        CipherAlgorithms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CipherMode {
    #[doc = "ECB"]
    ECB = 0,
    #[doc = "CBC"]
    CBC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CipherMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CipherMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CipherMode {
    #[inline(always)]
    fn from(val: u8) -> CipherMode {
        CipherMode::from_bits(val)
    }
}
impl From<CipherMode> for u8 {
    #[inline(always)]
    fn from(val: CipherMode) -> u8 {
        CipherMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CipherSelect {
    #[doc = "AES128"]
    AES128 = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CipherSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CipherSelect {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CipherSelect {
    #[inline(always)]
    fn from(val: u8) -> CipherSelect {
        CipherSelect::from_bits(val)
    }
}
impl From<CipherSelect> for u8 {
    #[inline(always)]
    fn from(val: CipherSelect) -> u8 {
        CipherSelect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlChannelInterruptEnable(pub u8);
impl CtrlChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for CtrlChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlChannelInterruptEnable {
        CtrlChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlChannelInterruptEnable) -> u8 {
        CtrlChannelInterruptEnable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlClrChannelInterruptEnable(pub u8);
impl CtrlClrChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlClrChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlClrChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for CtrlClrChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrChannelInterruptEnable {
        CtrlClrChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlClrChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrChannelInterruptEnable) -> u8 {
        CtrlClrChannelInterruptEnable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlSetChannelInterruptEnable(pub u8);
impl CtrlSetChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlSetChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlSetChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for CtrlSetChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetChannelInterruptEnable {
        CtrlSetChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlSetChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetChannelInterruptEnable) -> u8 {
        CtrlSetChannelInterruptEnable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlTogChannelInterruptEnable(pub u8);
impl CtrlTogChannelInterruptEnable {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl CtrlTogChannelInterruptEnable {
    pub const fn from_bits(val: u8) -> CtrlTogChannelInterruptEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for CtrlTogChannelInterruptEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogChannelInterruptEnable {
        CtrlTogChannelInterruptEnable::from_bits(val)
    }
}
impl From<CtrlTogChannelInterruptEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogChannelInterruptEnable) -> u8 {
        CtrlTogChannelInterruptEnable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HashAlgorithms(pub u16);
impl HashAlgorithms {
    #[doc = "SHA1"]
    pub const SHA1: Self = Self(0x01);
    #[doc = "CRC32"]
    pub const CRC32: Self = Self(0x02);
    #[doc = "SHA256"]
    pub const SHA256: Self = Self(0x04);
}
impl HashAlgorithms {
    pub const fn from_bits(val: u16) -> HashAlgorithms {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for HashAlgorithms {
    #[inline(always)]
    fn from(val: u16) -> HashAlgorithms {
        HashAlgorithms::from_bits(val)
    }
}
impl From<HashAlgorithms> for u16 {
    #[inline(always)]
    fn from(val: HashAlgorithms) -> u16 {
        HashAlgorithms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HashSelect {
    #[doc = "SHA1"]
    SHA1 = 0,
    #[doc = "CRC32"]
    CRC32 = 0x01,
    #[doc = "SHA256"]
    SHA256 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl HashSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashSelect {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashSelect {
    #[inline(always)]
    fn from(val: u8) -> HashSelect {
        HashSelect::from_bits(val)
    }
}
impl From<HashSelect> for u8 {
    #[inline(always)]
    fn from(val: HashSelect) -> u8 {
        HashSelect::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Index(pub u8);
impl Index {
    #[doc = "CONTROL"]
    pub const CONTROL: Self = Self(0x01);
    #[doc = "OTPKEY0"]
    pub const OTPKEY0: Self = Self(0x10);
    #[doc = "OTPKEY1"]
    pub const OTPKEY1: Self = Self(0x11);
    #[doc = "OTPKEY2"]
    pub const OTPKEY2: Self = Self(0x12);
    #[doc = "OTPKEY3"]
    pub const OTPKEY3: Self = Self(0x13);
}
impl Index {
    pub const fn from_bits(val: u8) -> Index {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Index {
    #[inline(always)]
    fn from(val: u8) -> Index {
        Index::from_bits(val)
    }
}
impl From<Index> for u8 {
    #[inline(always)]
    fn from(val: Index) -> u8 {
        Index::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KeySelect {
    #[doc = "KEY0"]
    KEY0 = 0,
    #[doc = "KEY1"]
    KEY1 = 0x01,
    #[doc = "KEY2"]
    KEY2 = 0x02,
    #[doc = "KEY3"]
    KEY3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    #[doc = "UNIQUE_KEY"]
    UNIQUE_KEY = 0xfe,
    #[doc = "OTP_KEY"]
    OTP_KEY = 0xff,
}
impl KeySelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeySelect {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeySelect {
    #[inline(always)]
    fn from(val: u8) -> KeySelect {
        KeySelect::from_bits(val)
    }
}
impl From<KeySelect> for u8 {
    #[inline(always)]
    fn from(val: KeySelect) -> u8 {
        KeySelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatClrCurChannel {
    #[doc = "None"]
    NONE = 0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatClrCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatClrCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatClrCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatClrCurChannel {
        StatClrCurChannel::from_bits(val)
    }
}
impl From<StatClrCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatClrCurChannel) -> u8 {
        StatClrCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatClrReadyChannels(pub u8);
impl StatClrReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatClrReadyChannels {
    pub const fn from_bits(val: u8) -> StatClrReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for StatClrReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatClrReadyChannels {
        StatClrReadyChannels::from_bits(val)
    }
}
impl From<StatClrReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatClrReadyChannels) -> u8 {
        StatClrReadyChannels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatCurChannel {
    #[doc = "None"]
    NONE = 0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatCurChannel {
        StatCurChannel::from_bits(val)
    }
}
impl From<StatCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatCurChannel) -> u8 {
        StatCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatReadyChannels(pub u8);
impl StatReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatReadyChannels {
    pub const fn from_bits(val: u8) -> StatReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for StatReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatReadyChannels {
        StatReadyChannels::from_bits(val)
    }
}
impl From<StatReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatReadyChannels) -> u8 {
        StatReadyChannels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatSetCurChannel {
    #[doc = "None"]
    NONE = 0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatSetCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatSetCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatSetCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatSetCurChannel {
        StatSetCurChannel::from_bits(val)
    }
}
impl From<StatSetCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatSetCurChannel) -> u8 {
        StatSetCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatSetReadyChannels(pub u8);
impl StatSetReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatSetReadyChannels {
    pub const fn from_bits(val: u8) -> StatSetReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for StatSetReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatSetReadyChannels {
        StatSetReadyChannels::from_bits(val)
    }
}
impl From<StatSetReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatSetReadyChannels) -> u8 {
        StatSetReadyChannels::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatTogCurChannel {
    #[doc = "None"]
    NONE = 0,
    #[doc = "CH0"]
    CH0 = 0x01,
    #[doc = "CH1"]
    CH1 = 0x02,
    #[doc = "CH2"]
    CH2 = 0x03,
    #[doc = "CH3"]
    CH3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl StatTogCurChannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatTogCurChannel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatTogCurChannel {
    #[inline(always)]
    fn from(val: u8) -> StatTogCurChannel {
        StatTogCurChannel::from_bits(val)
    }
}
impl From<StatTogCurChannel> for u8 {
    #[inline(always)]
    fn from(val: StatTogCurChannel) -> u8 {
        StatTogCurChannel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatTogReadyChannels(pub u8);
impl StatTogReadyChannels {
    #[doc = "CH0"]
    pub const CH0: Self = Self(0x01);
    #[doc = "CH1"]
    pub const CH1: Self = Self(0x02);
    #[doc = "CH2"]
    pub const CH2: Self = Self(0x04);
    #[doc = "CH3"]
    pub const CH3: Self = Self(0x08);
}
impl StatTogReadyChannels {
    pub const fn from_bits(val: u8) -> StatTogReadyChannels {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for StatTogReadyChannels {
    #[inline(always)]
    fn from(val: u8) -> StatTogReadyChannels {
        StatTogReadyChannels::from_bits(val)
    }
}
impl From<StatTogReadyChannels> for u8 {
    #[inline(always)]
    fn from(val: StatTogReadyChannels) -> u8 {
        StatTogReadyChannels::to_bits(val)
    }
}
