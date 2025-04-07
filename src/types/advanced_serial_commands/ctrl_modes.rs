//! 100, 101, 102, 103, 104

#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::TryFrom;
use core::fmt;

use crate::common::as_int::AsInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CTRLMode {
    Disable = 0,
    User = 1,
    VoltageClamp = 2,
    Brake = 3,
}

impl TryFrom<u8> for CTRLMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CTRLMode::Disable),
            1 => Ok(CTRLMode::User),
            2 => Ok(CTRLMode::VoltageClamp),
            3 => Ok(CTRLMode::Brake),
            _ => Err(()),
        }
    }
}

impl From<CTRLMode> for u8 {
    fn from(value: CTRLMode) -> Self {
        value.as_u8()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidCTRLMode(u8);

impl fmt::Display for InvalidCTRLMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid ctrl mode: {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidCTRLMode {}
