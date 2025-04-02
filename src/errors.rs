//! This module defines error types for the RoboClaw motor controller and
//! connection-related issues.
//! It contains two primary error enums:
//! - `RoboClawError`: A wrapper for various error types related to RoboClaw
//! motor controller interactions.
//! - `ConnectionError`: Represents errors that occur during the connection setup
//! or communication with the RoboClaw device.
//!
//! The module also includes conversion implementations to allow seamless error
//! handling from other error types.

/// `RoboClawError` represents errors encountered while communicating with the
/// RoboClaw motor controller.
/// This enum encapsulates different error types related to RoboClaw's connection
/// or I/O issues.
pub enum RoboClawError {
    Connection(ConnectionError), // Represents a connection-related error, wrapping a `ConnectionError`.
    Io(std::io::Error),          // Represents an I/O error, wrapping a `std::io::Error`
}

impl From<ConnectionError> for RoboClawError {
    /// Converts from `ConnectionError` to `RoboClawError::Connection`
    fn from(value: ConnectionError) -> Self {
        RoboClawError::Connection(value)
    }
}

impl From<std::io::Error> for RoboClawError {
    /// Converts from `std::io::Error` to `RoboClawError::Io`
    fn from(value: std::io::Error) -> Self {
        RoboClawError::Io(value)
    }
}

/// `ConnectionError` represents errors that occure when trying to maintain or communicate
/// with the RoboClaw motor controller. It covers different kinds of connection failures.
pub enum ConnectionError {
    Io(std::io::Error), // Represents a generic I/O error that occurs during connection handling.
    Serial(serialport::Error), // Represents an error specific to the serial port connection.
    InvalidByteSize(u8), // Represents an error where the byte size value is invalid
    CRCMismatch,        // Represents a CRC (Cyclic Redundancy Check) mismatch error.
}

impl From<std::io::Error> for ConnectionError {
    /// Converts `std::io::Error` to `ConnectionError::Io`
    fn from(value: std::io::Error) -> Self {
        ConnectionError::Io(value)
    }
}

impl From<serialport::Error> for ConnectionError {
    /// Converts `serialport::Error` to `ConnectionError::Serial`
    fn from(value: serialport::Error) -> Self {
        ConnectionError::Serial(value)
    }
}
