//! 148, 149

#![cfg_attr(not(feature = "std"), no_std)]

use crate::common::as_int::AsInt;
use core::convert::TryFrom;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PWMMode {
    Antiphase = 0,
    SignMagnitude = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidPWMMode(u8);

impl fmt::Display for InvalidPWMMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid pwm mode: {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidPWMMode {}

impl TryFrom<u8> for PWMMode {
    type Error = InvalidPWMMode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PWMMode::Antiphase),
            1 => Ok(PWMMode::SignMagnitude),
            _ => Err(InvalidPWMMode(value)),
        }
    }
}

impl From<PWMMode> for u8 {
    fn from(value: PWMMode) -> Self {
        value.as_u8()
    }
}
