mod commands;
pub mod connection;
pub mod errors;

use bitflags::bitflags;
use commands::Commands;
use connection::Connection;
use errors::RoboClawError;
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

pub struct Roboclaw {
    connection: Connection,
}

impl Roboclaw {
    pub fn new(
        port: Box<dyn SerialPort>,
        address: u8,
        tries: Option<u8>,
    ) -> Result<Self, RoboClawError> {
        let tries: u8 = tries.unwrap_or_else(|| 3);
        let connection: Connection = Connection::new(port, address, tries)?;
        Ok(Roboclaw { connection })
    }

    pub fn forward_m1(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M1Forward, &[speed as u32], &[1])?)
    }

    pub fn backward_m1(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M1Backward, &[speed as u32], &[1])?)
    }

    pub fn set_min_voltage_main_battery(_voltage: u8) {
        unimplemented!()
    }

    pub fn set_max_voltage_main_battery(_voltage: u8) {
        unimplemented!()
    }

    pub fn forward_m2(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M2Forward, &[speed as u32], &[1])?)
    }

    pub fn backward_m2(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M2Backward, &[speed as u32], &[1])?)
    }

    pub fn forward_backward_m1(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M1Drive, &[speed as u32], &[1])?)
    }

    pub fn forward_backward_m2(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M2Drive, &[speed as u32], &[1])?)
    }

    pub fn forward_mixed(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::MixDriveForward, &[speed as u32], &[1])?)
    }

    pub fn backward_mixed(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::MixDriveBackward, &[speed as u32], &[1])?)
    }

    pub fn turn_right_mixed(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::MixTurnRight, &[speed as u32], &[1])?)
    }

    pub fn turn_left_mixed(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::MixTurnLeft, &[speed as u32], &[1])?)
    }

    pub fn forward_backward_mixed(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::MixDrive, &[speed as u32], &[1])?)
    }

    pub fn left_right_mixed(&mut self, speed: u8) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::TurnLeftRight, &[speed as u32], &[1])?)
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

    pub fn reset_encoders(&mut self) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(Commands::ResetEncoders, &[], &[])?)
    }

    pub fn read_main_battery_voltage(&mut self) -> Result<u32, RoboClawError> {
        Ok(self.connection.read(Commands::ReadMainBatVoltage, &[2])?[0])
    }

    pub fn read_logic_battery_voltage(&mut self) -> Result<u32, RoboClawError> {
        Ok(self.connection.read(Commands::ReadLogicBatVoltage, &[2])?[0])
    }

    pub fn duty_m1(&mut self, duty: i16) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M1DriveSignedDutyCycle, &[duty as u32], &[2])?)
    }

    pub fn duty_m2(&mut self, duty: i16) -> Result<bool, RoboClawError> {
        Ok(self
            .connection
            .write(Commands::M2DriveSignedDutyCycle, &[duty as u32], &[2])?)
    }

    pub fn duty_m1_m2(&mut self, duty1: i16, duty2: i16) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::MixDriveSignedDutyCycle,
            &[duty1 as u32, duty2 as u32],
            &[2, 2],
        )?)
    }

    pub fn speed_m1_m2(&mut self, speed_1: i32, speed_2: i32) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::MixDriveSignedSpeed,
            &[speed_1 as u32, speed_2 as u32],
            &[4, 4],
        )?)
    }

    pub fn speed_distance_m1(
        &mut self,
        speed: i32,
        distance: u32,
        execute_directly: bool,
    ) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::M1DriveSignedSpeedDistanceBuffered,
            &[speed as u32, distance as u32, execute_directly as u32],
            &[4, 4, 1],
        )?)
    }

    pub fn speed_distance_m2(
        &mut self,
        speed: i32,
        distance: u32,
        execute_directly: bool,
    ) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::M2DriveSignedSpeedDistanceBuffered,
            &[speed as u32, distance as u32, execute_directly as u32],
            &[4, 4, 1],
        )?)
    }

    pub fn speed_distance_m1_m2(
        &mut self,
        speed_1: i32,
        distance_1: u32,
        speed_2: i32,
        distance_2: u32,
        execute_directly: bool,
    ) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::MixDriveSignedSpeedDistanceBuffered,
            &[
                speed_1 as u32,
                distance_1 as u32,
                speed_2 as u32,
                distance_2 as u32,
                execute_directly as u32,
            ],
            &[4, 4, 4, 4, 1],
        )?)
    }

    pub fn speed_accel_distance_m1_m2(
        &mut self,
        accel: u32,
        speed_1: i32,
        distance_1: u32,
        speed_2: i32,
        distance_2: u32,
        execute_directly: bool,
    ) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::MixDriveSignedSpeedAccelDistanceBuffered,
            &[
                accel as u32,
                speed_1 as u32,
                distance_1 as u32,
                speed_2 as u32,
                distance_2 as u32,
                execute_directly as u32,
            ],
            &[4, 4, 4, 4, 4, 1],
        )?)
    }

    pub fn read_buffers(&mut self) -> Result<[BufferStatus; 2], RoboClawError> {
        let values = self.connection.read(Commands::ReadBufferLength, &[1, 1])?;
        Ok(values.map(|data| match data {
            0x0 => BufferStatus::LastCommandExecuting,
            0x80 => BufferStatus::Empty,
            num => BufferStatus::NotEmpty(num as u8),
        }))
    }

    pub fn read_min_max_main_voltages(&mut self) -> Result<[u32; 2], RoboClawError> {
        Ok(self
            .connection
            .read(Commands::ReadMainBatVoltageSettings, &[2, 2])?)
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
        execute_directly: bool,
    ) -> Result<bool, RoboClawError> {
        Ok(self.connection.write(
            Commands::MixDriveSpeedAccelDeccelPosition,
            &[
                accel_1 as u32,
                speed_1 as u32,
                deccel_1 as u32,
                position_1 as u32,
                accel_2 as u32,
                speed_2 as u32,
                deccel_2 as u32,
                position_2 as u32,
                execute_directly as u32,
            ],
            &[4, 4, 4, 4, 4, 4, 4, 4, 1],
        )?)
    }

    pub fn read_encoders(&mut self) -> Result<[u32; 2], RoboClawError> {
        Ok(self.connection.read(Commands::ReadEncoderCounts, &[4, 4])?)
    }

    pub fn read_error(&mut self) -> Result<StatusFlags, RoboClawError> {
        let value = self.connection.read(Commands::ReadStatus, &[1])?;
        Ok(value.map(|data| StatusFlags::from_bits(data as u16))[0].unwrap())
    }

    pub fn get_config(&mut self) -> Result<ConfigFlags, RoboClawError> {
        let value = self
            .connection
            .read(Commands::ReadStandardConfigSettings, &[1])?;
        Ok(value.map(|data| ConfigFlags::from_bits(data as u16))[0].unwrap())
    }
}
