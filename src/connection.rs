//! This module handles the serial connection between the controller and
//! the RoboClaw.

use crate::Commands;
use crc16::{State, XMODEM};
use serialport::{ClearBuffer, SerialPort};

pub struct Connection {
    port: Box<dyn SerialPort>,
    pub address: u8,
    tries: u8,
    crc: State<XMODEM>,
    buffer: Vec<u8>,
}

impl Connection {
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

    /// initializes a new State for CRC16 XMODEM
    fn initialize_crc(&self) -> State<XMODEM> {
        State::<XMODEM>::new()
    }

    /// clears the input buffer of the connectio
    fn reset_connection(&mut self) {
        let _ = self.port.clear(ClearBuffer::All);
        self.crc = self.initialize_crc();
        self.buffer.clear();
    }

    /// sends the address and command and updates the crc
    fn send_command(&mut self, command: Commands) {
        self.crc.update(&[self.address, command as u8]);
        self.buffer.extend(&[self.address, command as u8]);
    }

    pub fn write(&mut self, command: Commands, values: &[u32]) -> bool {
        for _ in 0..self.tries {
            self.reset_connection();
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

            let _ = self.port.write_all(&self.buffer);

            let mut ack = [0u8; 1];
            let success: bool = match self.port.read_exact(&mut ack) {
                Ok(_) => ack[0] == 0xFF,
                Err(_) => false,
            };

            return success;
        }

        false
    }

    pub fn read<const N: usize>(&mut self, command: Commands, how: &[u8; N]) -> [u32; N] {
        for _ in 0..self.tries {
            self.reset_connection();
            self.send_command(command);

            let mut data = [0u32; N];
            for (i, &byte_size) in how.iter().enumerate() {
                data[i] = match byte_size {
                    1 => {
                        let mut buffer = [0u8; 1];
                        self.read_bytes(&mut buffer);
                        buffer[0] as u32
                    }
                    2 => {
                        let mut buffer = [0u8; 2];
                        self.read_bytes(&mut buffer);
                        u16::from_be_bytes(buffer) as u32
                    }
                    4 => {
                        let mut buffer = [0u8; 4];
                        self.read_bytes(&mut buffer);
                        u32::from_be_bytes(buffer)
                    }
                    _ => panic!(
                        "value cannot be {}!!\nonly values 1, 2 and 4 are allowd",
                        byte_size
                    ),
                };
            }

            let mut crc = [0u8; 2];
            let _ = self.port.read_exact(&mut crc);
            if self.crc.get().to_be_bytes() == crc {
                return data;
            }
        }

        [0; N]
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) {
        let _ = self.port.read_exact(buffer);
        self.crc.update(&buffer);
    }
}
