//! This module defines the return types for RoboClaw motor controller commands.
//! It provides structured representations of the responses received from the device.

use bitflags::bitflags;

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum MotorsU32 {
    M1(u32),
    M2(u32),
}

/// Enum for storing unsigned 16-bit integers for both motors
#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum MotorsU16 {
    M1(u16),
    M2(u16),
}

pub type PWMValues = MotorsU16;
pub type MotorCurrents = MotorsU16;
pub type DefaultSpeedSettings = MotorsU16;
pub type DefaultDutyAccel = MotorsU32;
pub type SpeedErrorLimits = MotorsU32;
pub type PositionErrorLimits = MotorsU32;
pub type BlankingPercentage = MotorsU16;
pub type ISpeedCounters = MotorsU32;
pub type AverageSpeed = MotorsU32;
pub type SpeedErrors = MotorsU32;
pub type PositionErrors = MotorsU32;

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum Current {
    Min(u32),
    Max(u32),
}

pub type M1CurrentLimit = Current;
pub type M2CurrentLimit = Current;

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum BatteryVoltageSetting {
    Min(u16),
    Max(u16),
}

pub type MainBatteryVoltageSetting = BatteryVoltageSetting;
pub type LogicBatteryVoltageSetting = BatteryVoltageSetting;

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum BatteryVoltageOffset {
    MainBatteryOffset(u8),
    LogicBatteryOffset(u8),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum S345Modes {
    S3Mode(u8),
    S4Mode(u8),
    S5Mode(u8),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum DeadBand {
    Reverse(u8),
    SForward(u8),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum CTRLModes {
    CTRL1Mode(u8),
    CTRL2Mode(u8),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum CTRL {
    CTRL1(u16),
    CTRL2(u16),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum Homing {
    Percentage(u16),
    Timeout(u32),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum BufferStatus {
    NotEmpty(u8),
    Empty,
    LastCommandExecuting,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum MotorBuffers {
    M1(BufferStatus),
    M2(BufferStatus),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum PIDQPPS {
    P(u32),
    I(u32),
    D(u32),
    QPPS(u32),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum PositionPIDConst {
    P(u32),
    I(u32),
    D(u32),
    MaxI(u32),
    Deadzone(u32),
    MinPos(u32),
    MaxPos(u32),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum Encoders {
    Enc1(u32),
    Enc2(u32),
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum EncoderMode {
    Enc1(u8),
    Enc2(u8),
}

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
