//! This module handles the serial connection between the controller and
//! the RoboClaw.

use crate::{errors::ConnectionError, Commands};
use crc16::{State, XMODEM};
use serialport::{ClearBuffer, SerialPort};
use std::time::Duration;

/// Represents the serial connection to the RoboClaw motor controller.
pub(crate) struct Connection {
    port: Box<dyn SerialPort>, // The serial port for commjnication
    pub(crate) address: u8,    // The address of the RoboClaw device
    tries: u8,                 // Number of attempts to retry a failed operation
    crc: State<XMODEM>,        // CRC16 XMODEM state for the checksum calculation
    buffer: Vec<u8>,           // Buffer holding the data to be sent
}

impl Connection {
    /// Creates a new `Connection` instance with the specified parameters.
    /// Initializes the CRC State and prepares the buffer for communication.
    pub(crate) fn new(
        mut port: Box<dyn SerialPort>,
        address: u8,
        tries: u8,
    ) -> Result<Self, ConnectionError> {
        port.set_timeout(Duration::from_millis(5))?; // making sure that the timeout is under 10 milliseconds
        let crc = State::<XMODEM>::new();
        let buffer = Vec::new();
        Ok(Connection {
            port,
            address,
            tries,
            crc,
            buffer,
        })
    }

    /// Initializes a new CRC16 XMODEM state.
    fn initialize_crc(&self) -> State<XMODEM> {
        State::<XMODEM>::new()
    }

    /// Resets the connection by clearing the buffer and CRC state.
    fn reset_connection(&mut self) -> Result<(), ConnectionError> {
        self.port.clear(ClearBuffer::All)?;
        self.crc = self.initialize_crc();
        self.buffer.clear();
        Ok(())
    }

    /// Sends the address and command to the motor controller, updating the CRC
    fn send_command(&mut self, command: Commands) {
        self.crc.update(&[self.address, command as u8]);
        self.buffer.extend(&[self.address, command as u8]);
    }

    /// Writes the specified command and values to the RoboClaw.
    /// Attempts multiple retries on failure. Returns `true` if successful.
    pub(crate) fn write<const N: usize>(
        &mut self,
        command: Commands,
        values: &[u32; N],
        byte_sizes: &[u8; N],
    ) -> Result<bool, ConnectionError> {
        for _ in 0..self.tries {
            self.reset_connection()?;
            self.send_command(command);

            for (i, &byte_size) in byte_sizes.iter().enumerate() {
                let val = values[i];
                match byte_size {
                    1 => {
                        self.crc.update(&[val as u8]);
                        self.buffer.extend_from_slice(&[val as u8]);
                    }
                    2 => {
                        self.crc.update(&(val as u16).to_be_bytes());
                        self.buffer.extend_from_slice(&(val as u16).to_be_bytes());
                    }
                    4 => {
                        self.crc.update(&val.to_be_bytes());
                        self.buffer.extend_from_slice(&val.to_be_bytes());
                    }
                    _ => return Err(ConnectionError::InvalidByteSize(byte_size)),
                }
            }

            let crc_bytes = self.crc.get().to_be_bytes();
            self.buffer.extend_from_slice(&crc_bytes);

            self.port.write_all(&self.buffer)?;

            let mut ack = [0u8; 1];
            self.port.read_exact(&mut ack)?;
            let success: bool = match ack[0] {
                0xFF => true,
                _ => false,
            };
            if success {
                return Ok(true);
            }
        }

        Err(ConnectionError::CRCMismatch)
    }

    /// Reads data from the RoboClaw based on the provided command and expected sizes.
    /// Returns an array of values read from the device.
    pub(crate) fn read<const N: usize>(
        &mut self,
        command: Commands,
        byte_sizes: &[u8; N],
    ) -> Result<[u32; N], ConnectionError> {
        for _ in 0..self.tries {
            self.reset_connection()?;
            self.send_command(command);

            let mut data = [0u32; N];
            for (i, &byte_size) in byte_sizes.iter().enumerate() {
                data[i] = match byte_size {
                    1 => {
                        let mut buffer = [0u8; 1];
                        self.read_bytes(&mut buffer)?;
                        buffer[0] as u32
                    }
                    2 => {
                        let mut buffer = [0u8; 2];
                        self.read_bytes(&mut buffer)?;
                        u16::from_be_bytes(buffer) as u32
                    }
                    4 => {
                        let mut buffer = [0u8; 4];
                        self.read_bytes(&mut buffer)?;
                        u32::from_be_bytes(buffer)
                    }
                    _ => return Err(ConnectionError::InvalidByteSize(byte_size)),
                };
            }

            let mut crc = [0u8; 2];
            let _ = self.port.read_exact(&mut crc);
            if self.crc.get().to_be_bytes() == crc {
                return Ok(data);
            }
        }

        Err(ConnectionError::CRCMismatch)
    }

    /// Reads a specififc number of bytes from the serialport and updates the CRC state.
    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), ConnectionError> {
        self.port.read_exact(buffer)?;
        self.crc.update(&buffer);
        Ok(())
    }
}
