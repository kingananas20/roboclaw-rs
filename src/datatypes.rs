pub enum MotorsU32 {
    M1(u32),
    M2(u32),
}

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

pub enum Current {
    Min(u32),
    Max(u32),
}

pub type M1CurrentLimit = Current;
pub type M2CurrentLimit = Current;

pub enum BatteryVoltageSetting {
    Min(u16),
    Max(u16),
}

pub type MainBatteryVoltageSetting = BatteryVoltageSetting;
pub type LogicBatteryVoltageSetting = BatteryVoltageSetting;

pub enum BatteryVoltageOffset {
    MainBatteryOffset(u8),
    LogicBatteryOffset(u8),
}

pub enum S345Modes {
    S3Mode(u8),
    S4Mode(u8),
    S5Mode(u8),
}

pub enum DeadBand {
    Reverse(u8),
    SForward(u8),
}

pub enum CTRLModes {
    CTRL1Mode(u8),
    CTRL2Mode(u8),
}

pub enum CTRL {
    CTRL1(u16),
    CTRL2(u16),
}

pub enum Homing {
    Percentage(u16),
    Timeout(u32),
}

#[derive(PartialEq, Debug)]
pub enum BufferStatus {
    NotEmpty(u8),
    Empty,
    LastCommandExecuting,
}

pub enum MotorBuffers {
    M1(BufferStatus),
    M2(BufferStatus),
}

pub enum PIDQPPS {
    P(u32),
    I(u32),
    D(u32),
    QPPS(u32),
}

pub enum PositionPIDConst {
    P(u32),
    I(u32),
    D(u32),
    MaxI(u32),
    Deadzone(u32),
    MinPos(u32),
    MaxPos(u32),
}

// For Encoders !TODO
// 91, 95, 78, 79, 108,
