//! This module handles the serial connection between the controller and
//! the RoboClaw.

use crate::Commands;
use crc16::{State, XMODEM};
use serialport::{ClearBuffer, SerialPort};

pub enum ConnectionError {
    Io(std::io::Error),
    Serial(serialport::Error),
    InvalidByteSize(u8),
    CRCMismatch,
}

impl From<std::io::Error> for ConnectionError {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::Io(err)
    }
}

impl From<serialport::Error> for ConnectionError {
    fn from(err: serialport::Error) -> Self {
        ConnectionError::Serial(err)
    }
}

/// Represents the serial connection to the RoboClaw motor controller.
pub struct Connection {
    port: Box<dyn SerialPort>, // The serial port for commjnication
    pub address: u8,           // The address of the RoboClaw device
    tries: u8,                 // Number of attempts to retry a failed operation
    crc: State<XMODEM>,        // CRC16 XMODEM state for the checksum calculation
    buffer: Vec<u8>,           // Buffer holding the data to be sent
}

impl Connection {
    /// Creates a new `Connection` instance with the specified parameters.
    /// Initializes the CRC State and prepares the buffer for communication.
    pub fn new(self, port: Box<dyn SerialPort>, address: u8, tries: u8) -> Self {
        let crc = self.initialize_crc();
        let buffer = Vec::new();
        Connection {
            port,
            address,
            tries,
            crc,
            buffer,
        }
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
    pub fn write(&mut self, command: Commands, values: &[u32]) -> Result<bool, ConnectionError> {
        for _ in 0..self.tries {
            self.reset_connection()?;
            self.send_command(command);

            for &val in values {
                match val {
                    0..=0xFF => {
                        self.crc.update(&[val as u8]);
                        self.buffer.extend_from_slice(&[val as u8]);
                    }
                    0x100..=0xFFFF => {
                        self.crc.update(&(val as u16).to_be_bytes());
                        self.buffer.extend_from_slice(&(val as u16).to_be_bytes());
                    }
                    _ => {
                        self.crc.update(&val.to_be_bytes());
                        self.buffer.extend_from_slice(&val.to_be_bytes());
                    }
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
    pub fn read<const N: usize>(
        &mut self,
        command: Commands,
        how: &[u8; N],
    ) -> Result<[u32; N], ConnectionError> {
        for _ in 0..self.tries {
            self.reset_connection()?;
            self.send_command(command);

            let mut data = [0u32; N];
            for (i, &byte_size) in how.iter().enumerate() {
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
