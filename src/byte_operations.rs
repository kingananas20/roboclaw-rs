//! This module provides functions for splitting and joining integers into bytes
//!
//! It includes utilities to split 16-bit and 32-bit signed/unsigned integers into
//! 8-bit byte arrays, and to join multiple 8-bit bytes back into 16-bit or 32-bit
//! unsigned integers.

/// Splits an unsigned 16-bit integer into 2 unsigned 8-bit integers
pub fn split_u16_u8(x: u16) -> [u8; 2] {
    [(x >> 8) as u8, x as u8]
}

/// Splits a signed 16-bit integer into 2 unsigned 8-bit integers
pub fn split_i16_u8(x: i16) -> [u8; 2] {
    [(x >> 8) as u8, x as u8]
}

/// Splits an unsigned 32-bit integer into 4 unsigned 8-bit integers
pub fn split_u32_u8(x: u32) -> [u8; 4] {
    [(x >> 24) as u8, (x >> 16) as u8, (x >> 8) as u8, x as u8]
}

/// Splits a signed 32-bit integer into 4 unsigned 8-bit integers
pub fn split_i32_u8(x: i32) -> [u8; 4] {
    [(x >> 24) as u8, (x >> 16) as u8, (x >> 8) as u8, x as u8]
}

/// Joins 2 unsigned 8-bit integers into an unsigned 16-bit integer
pub fn join_u8(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | low as u16
}

/// Joins 4 unsigned 8-bit integers into an unsigned 32-bit integer
pub fn join_u8_u32(byte0: u8, byte1: u8, byte2: u8, byte3: u8) -> u32 {
    ((byte0 as u32) << 24) | ((byte1 as u32) << 16) | ((byte2 as u32) << 8) | (byte3 as u32)
}
