mod byte_operations;
mod commands;

use bitflags::bitflags;
use byte_operations::*;
use commands::Commands;
use crc16;
use serialport::SerialPort;

bitflags! {
    pub struct ConfigFlags: u16 {
        const RC_MODE = 0x0000;
        const ANALOG_MODE = 0x0001;
        const SIMPLE_SERIAL_MODE = 0x0002;
        const PACKET_SERIAL_MODE = 0x0003;
        const BATTERY_MODE_OFF = 0x0000;
        const BATTERY_MODE_AUTO = 0x0004;
        const BATTERY_MODE_2_CELL = 0x0008;
        const BATTERY_MODE_3_CELL = 0x000C;
        const BATTERY_MODE_4_CELL = 0x0010;
        const BATTERY_MODE_5_CELL = 0x0014;
        const BATTERY_MODE_6_CELL = 0x0018;
        const BATTERY_MODE_7_CELL = 0x001C;
        const MIXING = 0x0020;
        const EXPONENTIAL = 0x0040;
        const MCU = 0x0080;
        const BAUDRATE_2400 = 0x0000;
        const BAUDRATE_9600 = 0x0020;
        const BAUDRATE_19200 = 0x0040;
        const BAUDRATE_38400 = 0x0060;
        const BAUDRATE_57600 = 0x0080;
        const BAUDRATE_115200 = 0x00A0;
        const BAUDRATE_230400 = 0x00C0;
        const BAUDRATE_460800 = 0x00E0;
        const FLIPSWITCH = 0x0100;
        const PACKET_ADDRESS_0X80 = 0x0000;
        const PACKET_ADDRESS_0X81 = 0x0100;
        const PACKET_ADDRESS_0X82 = 0x0200;
        const PACKET_ADDRESS_0X83 = 0x0300;
        const PACKET_ADDRESS_0X84 = 0x0400;
        const PACKET_ADDRESS_0X85 = 0x0500;
        const PACKET_ADDRESS_0X86 = 0x0600;
        const PACKET_ADDRESS_0X87 = 0x0700;
        const SLAVE_MODE = 0x0800;
        const RELAY_MODE = 0x1000;
        const SWAP_ENCODERS = 0x2000;
        const SWAP_BUTTONS = 0x4000;
        const MULTI_UNIT_MODE = 0x8000;
    }
}

bitflags! {
    pub struct StatusFlags: u16 {
        const NORMAL = 0x0000;
        const M1_OVERCURRENT_WARNING = 0x0001;
        const M2_OVERCURRENT_WARNING = 0x0002;
        const E_STOP = 0x0004;
        const TEMPERATURE_ERROR = 0x0008;
        const TEMPERATURE2_ERROR = 0x0010;
        const MAIN_BATTERY_HIGH_ERROR = 0x0020;
        const LOGIC_BATTERY_HIGH_ERROR = 0x0040;
        const LOGIC_BATTERY_LOW_ERROR = 0x0080;
        const M1_DRIVER_FAULT = 0x0100;
        const M2_DRIVER_FAULT = 0x0200;
        const MAIN_BATTERY_HIGH_WARNING = 0x0400;
        const MAIN_BATTERY_LOW_WARNING = 0x0800;
        const TERMPERATURE_WARNING = 0x1000;
        const TEMPERATURE2_WARNING = 0x2000;
        const M1_HOME = 0x4000;
        const M2_HOME = 0x8000;
    }
}

#[derive(PartialEq, Debug)]
pub enum BufferStatus {
    NotEmpty(u8),
    Empty,
    LastCommandExecuting,
}

fn crc(buf: &Vec<u8>) -> Vec<u8> {
    let crc = crc16::State::<crc16::XMODEM>::calculate(&buf);
    split_u16_u8(crc).to_vec()
}

pub struct Roboclaw {
    port: Box<dyn SerialPort>,
    address: u8,
}

impl Roboclaw {
    pub fn new(port: Box<dyn SerialPort>, address: u8) -> Self {
        Roboclaw { port, address }
    }

    fn read_command(&mut self, command_code: u8, num_bytes: usize) -> std::io::Result<Vec<u8>> {
        const CRC_SIZE: usize = 2;
        let command = vec![self.address, command_code];
        self.port.write(&command[..])?;
        let mut buf = vec![0; num_bytes + CRC_SIZE];
        self.port.read(&mut buf)?;
        let crc = buf.split_off(num_bytes);
        let crc_read = join_u8(crc[0], crc[1]);
        let crc_calc = crc16::State::<crc16::XMODEM>::calculate(&[&command[..], &buf].concat());
        if crc_read == crc_calc {
            Ok(buf)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "crc error"))
        }
    }

    fn write_simple_command(&mut self, command_code: u8) -> std::io::Result<()> {
        let command = vec![self.address, command_code];
        let crc = crc(&command);
        let command_bytes = [&[self.address], &command[..], &crc[..]].concat();
        self.port.write(&command_bytes)?;
        let mut buf = vec![0; 1];
        self.port.read(&mut buf)?;
        if buf[0] == 0xFF {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "return value error",
            ))
        }
    }

    fn write_command(&mut self, command_code: u8, data: &Vec<u8>) -> std::io::Result<()> {
        let mut command = vec![self.address, command_code];
        let mut data_copy = data.clone();
        command.append(&mut data_copy);
        let crc = crc(&command);
        let command_bytes = [&[self.address], &command[..], &crc[..]].concat();
        self.port.write(&command_bytes)?;
        let mut buf = vec![0; 1];
        self.port.read(&mut buf)?;
        if buf[0] == 0xFF {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "return value error",
            ))
        }
    }

    pub fn forward_m1(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::M1Forward as u8, &vec![speed])
    }

    pub fn backward_m1(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::M1Backward as u8, &vec![speed])
    }

    pub fn set_min_voltage_main_battery(_voltage: u8) {
        unimplemented!()
    }

    pub fn set_max_voltage_main_battery(_voltage: u8) {
        unimplemented!()
    }

    pub fn forward_m2(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::M2Forward as u8, &vec![speed])
    }

    pub fn backward_m2(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::M2Backward as u8, &vec![speed])
    }

    pub fn forward_backward_m1(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::M1Drive as u8, &vec![speed])
    }

    pub fn forward_backward_m2(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::M2Drive as u8, &vec![speed])
    }

    pub fn forward_mixed(&mut self, speed: u8) -> Result<(), std::io::Error> {
        self.write_command(Commands::MixDriveForward as u8, &vec![speed])
    }

    pub fn backward_mixed(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::MixDriveBackward as u8, &vec![speed])
    }

    pub fn turn_right_mixed(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::MixTurnRight as u8, &vec![speed])
    }

    pub fn turn_left_mixed(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::MixTurnLeft as u8, &vec![speed])
    }

    pub fn forward_backward_mixed(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::MixDrive as u8, &vec![speed])
    }

    pub fn left_right_mixed(&mut self, speed: u8) -> std::io::Result<()> {
        self.write_command(Commands::TurnLeftRight as u8, &vec![speed])
    }

    pub fn read_enc_m1(&mut self) -> Result<u32, &str> {
        unimplemented!()
    }

    pub fn read_enc_m2(&mut self) -> Result<u32, &str> {
        unimplemented!()
    }

    pub fn set_enc_m1(&mut self, _value: i32) -> Result<(), &str> {
        unimplemented!()
    }

    pub fn set_enc_m2(&mut self, _value: i32) -> Result<(), &str> {
        unimplemented!()
    }

    pub fn reset_encoders(&mut self) -> Result<(), std::io::Error> {
        self.write_simple_command(Commands::ResetEncoders as u8)
    }

    pub fn read_main_battery_voltage(&mut self) -> Result<f32, std::io::Error> {
        self.read_command(Commands::ReadMainBatVoltage as u8, 2)
            .map(|data| (join_u8(data[0], data[1]) as f32) / 10.0)
    }

    pub fn read_logic_battery_voltage(&mut self) -> Result<f32, std::io::Error> {
        self.read_command(Commands::ReadLogicBatVoltage as u8, 2)
            .map(|data| (join_u8(data[0], data[1]) as f32) / 10.0)
    }

    pub fn duty_m1(&mut self, duty: i16) -> std::io::Result<()> {
        self.write_command(
            Commands::M1DriveSignedDutyCycle as u8,
            &split_i16_u8(duty).to_vec(),
        )
    }

    pub fn duty_m2(&mut self, duty: i16) -> std::io::Result<()> {
        self.write_command(
            Commands::M2DriveSignedDutyCycle as u8,
            &split_i16_u8(duty).to_vec(),
        )
    }

    pub fn duty_m1_m2(&mut self, duty1: i16, duty2: i16) -> std::io::Result<()> {
        self.write_command(
            Commands::MixDriveSignedDutyCycle as u8,
            &[&split_i16_u8(duty1)[..], &split_i16_u8(duty2)[..]].concat(),
        )
    }

    pub fn speed_m1_m2(&mut self, speed_1: i32, speed_2: i32) -> Result<(), std::io::Error> {
        let speed_1_bytes = split_i32_u8(speed_1);
        let speed_2_bytes = split_i32_u8(speed_2);
        let data = [&speed_1_bytes[..], &speed_2_bytes[..]].concat();
        self.write_command(Commands::MixDriveSignedSpeed as u8, &data)
    }

    pub fn speed_distance_m1(&mut self, speed: i32, distance: u32) -> Result<(), std::io::Error> {
        let speed_bytes = split_i32_u8(speed);
        let distance_bytes = split_u32_u8(distance);
        let data = [&speed_bytes[..], &distance_bytes[..], &vec![1u8]].concat();
        self.write_command(Commands::M1DriveSignedSpeedDistanceBuffered as u8, &data)
    }

    pub fn speed_distance_m2(&mut self, speed: i32, distance: u32) -> Result<(), std::io::Error> {
        let speed_bytes = split_i32_u8(speed);
        let distance_bytes = split_u32_u8(distance);
        let data = [&speed_bytes[..], &distance_bytes[..], &vec![1u8]].concat();
        self.write_command(Commands::M2DriveSignedSpeedDistanceBuffered as u8, &data)
    }

    pub fn speed_distance_m1_m2(
        &mut self,
        speed_1: i32,
        distance_1: u32,
        speed_2: i32,
        distance_2: u32,
    ) -> Result<(), std::io::Error> {
        let speed_1_bytes = split_i32_u8(speed_1);
        let distance_1_bytes = split_u32_u8(distance_1);
        let speed_2_bytes = split_i32_u8(speed_2);
        let distance_2_bytes = split_u32_u8(distance_2);
        let data = [
            &speed_1_bytes[..],
            &distance_1_bytes[..],
            &speed_2_bytes[..],
            &distance_2_bytes[..],
            &vec![1u8],
        ]
        .concat();
        self.write_command(Commands::MixDriveSignedSpeedDistanceBuffered as u8, &data)
    }

    pub fn speed_accel_distance_m1_m2(
        &mut self,
        accel: u32,
        speed_1: i32,
        distance_1: u32,
        speed_2: i32,
        distance_2: u32,
    ) -> Result<(), std::io::Error> {
        let accel_bytes = split_u32_u8(accel);
        let speed_1_bytes = split_i32_u8(speed_1);
        let distance_1_bytes = split_u32_u8(distance_1);
        let speed_2_bytes = split_i32_u8(speed_2);
        let distance_2_bytes = split_u32_u8(distance_2);
        let data = [
            &accel_bytes[..],
            &speed_1_bytes[..],
            &distance_1_bytes[..],
            &speed_2_bytes[..],
            &distance_2_bytes[..],
            &vec![1u8],
        ]
        .concat();
        self.write_command(
            Commands::MixDriveSignedSpeedAccelDistanceBuffered as u8,
            &data,
        )
    }

    pub fn read_buffers(&mut self) -> std::io::Result<(BufferStatus, BufferStatus)> {
        self.read_command(Commands::ReadBufferLength as u8, 2)
            .map(|data| {
                (
                    match data[0] {
                        0x0 => BufferStatus::LastCommandExecuting,
                        0x80 => BufferStatus::Empty,
                        num => BufferStatus::NotEmpty(num),
                    },
                    match data[1] {
                        0x0 => BufferStatus::LastCommandExecuting,
                        0x80 => BufferStatus::Empty,
                        num => BufferStatus::NotEmpty(num),
                    },
                )
            })
    }

    pub fn read_min_max_main_voltages(&mut self) -> Result<(f32, f32), std::io::Error> {
        self.read_command(Commands::ReadMainBatVoltageSettings as u8, 4)
            .map(|data| {
                (
                    join_u8(data[0], data[1]) as f32 / 10.0,
                    join_u8(data[2], data[3]) as f32 / 10.0,
                )
            })
    }

    pub fn speed_accel_deccel_position_m1_m2(
        &mut self,
        accel_1: u32,
        speed_1: i32,
        deccel_1: u32,
        position_1: u32,
        accel_2: u32,
        speed_2: i32,
        deccel_2: u32,
        position_2: u32,
    ) -> Result<(), std::io::Error> {
        let accel_1_bytes = split_u32_u8(accel_1);
        let speed_1_bytes = split_i32_u8(speed_1);
        let deccel_1_bytes = split_u32_u8(deccel_1);
        let position_1_bytes = split_u32_u8(position_1);

        let accel_2_bytes = split_u32_u8(accel_2);
        let speed_2_bytes = split_i32_u8(speed_2);
        let deccel_2_bytes = split_u32_u8(deccel_2);
        let position_2_bytes = split_u32_u8(position_2);

        let data = [
            &accel_1_bytes[..],
            &speed_1_bytes[..],
            &deccel_1_bytes[..],
            &position_1_bytes[..],
            &accel_2_bytes[..],
            &speed_2_bytes[..],
            &deccel_2_bytes[..],
            &position_2_bytes[..],
            &vec![1u8],
        ]
        .concat();
        self.write_command(Commands::MixDriveSpeedAccelDeccelPosition as u8, &data)
    }

    pub fn read_encoders(&mut self) -> Result<(u32, u32), std::io::Error> {
        self.read_command(Commands::ReadEncoderCounts as u8, 8)
            .map(|data| {
                (
                    join_u8_u32(data[0], data[1], data[2], data[3]),
                    join_u8_u32(data[4], data[5], data[6], data[7]),
                )
            })
    }

    pub fn read_error(&mut self) -> Result<StatusFlags, std::io::Error> {
        self.read_command(Commands::ReadStatus as u8, 2)
            .map(|data| StatusFlags::from_bits(join_u8(data[0], data[1])).unwrap())
    }

    pub fn get_config(&mut self) -> Result<ConfigFlags, std::io::Error> {
        self.read_command(Commands::ReadStandardConfigSettings as u8, 2)
            .map(|data| ConfigFlags::from_bits(join_u8(data[0], data[1])).unwrap())
    }
}
