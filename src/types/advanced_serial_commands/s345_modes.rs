//! 74, 75

#![cfg_attr(not(feature = "std"), no_std)]

use crate::common::as_int::AsInt;
use core::convert::TryFrom;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum S3Mode {
    Default = 0x00,
    EStop = 0x01,
    EStopLatching = 0x81,
    VoltageClamp = 0x14,
    RS485Direction = 0x24,
    EncoderToggle = 0x84,
}

impl TryFrom<u8> for S3Mode {
    type Error = InvalidS345Mode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(S3Mode::Default),
            0x01 => Ok(S3Mode::EStop),
            0x81 => Ok(S3Mode::EStopLatching),
            0x14 => Ok(S3Mode::VoltageClamp),
            0x24 => Ok(S3Mode::RS485Direction),
            0x84 => Ok(S3Mode::EncoderToggle),
            _ => Err(InvalidS345Mode::new("S3", value)),
        }
    }
}

impl From<S3Mode> for u8 {
    fn from(value: S3Mode) -> Self {
        value.as_u8()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum S4Mode {
    Default = 0x00,
    EStop = 0x01,
    EStopLatching = 0x81,
    VoltageClamp = 0x14,
    Brake = 0x04,
    HomeAuto = 0xE2,
    HomeUser = 0x62,
    HomeAutoLimitFwd = 0xF2,
    HomeUserLimitFwd = 0x72,
    LimitFwd = 0x12,
    LimitRev = 0x22,
    LimitBoth = 0x32,
}

impl TryFrom<u8> for S4Mode {
    type Error = InvalidS345Mode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(S4Mode::Default),
            0x01 => Ok(S4Mode::EStop),
            0x81 => Ok(S4Mode::EStopLatching),
            0x14 => Ok(S4Mode::VoltageClamp),
            0x04 => Ok(S4Mode::Brake),
            0xE2 => Ok(S4Mode::HomeAuto),
            0x62 => Ok(S4Mode::HomeUser),
            0xF2 => Ok(S4Mode::HomeAutoLimitFwd),
            0x72 => Ok(S4Mode::HomeUserLimitFwd),
            0x12 => Ok(S4Mode::LimitFwd),
            0x22 => Ok(S4Mode::LimitRev),
            0x32 => Ok(S4Mode::LimitBoth),
            _ => Err(InvalidS345Mode::new("S4", value)),
        }
    }
}

impl From<S4Mode> for u8 {
    fn from(value: S4Mode) -> Self {
        value.as_u8()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum S5Mode {
    Default = 0x00,
    EStop = 0x01,
    EStopLatching = 0x81,
    VoltageClamp = 0x14,
    Brake = 0x04,
    HomeUser = 0x62,
    HomeAutoLimitFwd = 0xF2,
    HomeUserLimitFwd = 0x72,
    LimitFwd = 0x12,
    LimitRev = 0x22,
    LimitBoth = 0x32,
}

impl TryFrom<u8> for S5Mode {
    type Error = InvalidS345Mode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(S5Mode::Default),
            0x01 => Ok(S5Mode::EStop),
            0x81 => Ok(S5Mode::EStopLatching),
            0x14 => Ok(S5Mode::VoltageClamp),
            0x04 => Ok(S5Mode::Brake),
            0x62 => Ok(S5Mode::HomeUser),
            0xF2 => Ok(S5Mode::HomeAutoLimitFwd),
            0x72 => Ok(S5Mode::HomeUserLimitFwd),
            0x12 => Ok(S5Mode::LimitFwd),
            0x22 => Ok(S5Mode::LimitRev),
            0x32 => Ok(S5Mode::LimitBoth),
            _ => Err(InvalidS345Mode::new("S5", value)),
        }
    }
}

impl From<S5Mode> for u8 {
    fn from(value: S5Mode) -> Self {
        value.as_u8()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidS345Mode {
    mode_type: &'static str,
    value: u8,
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidS345Mode {}

impl fmt::Display for InvalidS345Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {} mode: {}", self.mode_type, self.value)
    }
}

impl InvalidS345Mode {
    pub fn new(mode_type: &'static str, value: u8) -> Self {
        InvalidS345Mode { mode_type, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct S345Modes {
    s3: S3Mode,
    s4: S4Mode,
    s5: S5Mode,
}
