//! This module handles the serial connection between the controller and
//! the RoboClaw.

use std::error::Error;

use crate::Commands;
use crc16::{State, XMODEM};
use serialport::{ClearBuffer, SerialPort};

pub struct Connection {
    port: Box<dyn SerialPort>,
    pub address: u8,
    crc: State<XMODEM>,
}

impl Connection {
    pub fn new(port: Box<dyn SerialPort>, address: u8) -> Self {
        let crc = State::<XMODEM>::new();
        Connection { port, address, crc }
    }

    fn initialize_crc(&mut self) -> State<XMODEM> {
        State::<XMODEM>::new()
    }

    fn reset_connection(&mut self) {
        let _ = self.port.clear(ClearBuffer::Input);
        self.crc = self.initialize_crc();
    }

    fn send_command(&mut self, command: Commands) {
        self.crc.update(&[self.address, command as u8]);
        let _ = self.port.write_all(&[self.address, command as u8]);
    }
}
