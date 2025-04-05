mod commands;
mod connection;
pub mod errors;
pub mod returntypes;

use commands::Commands;
use connection::Connection;
use errors::RoboClawError;
use returntypes::*;
use serialport::SerialPort;

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
