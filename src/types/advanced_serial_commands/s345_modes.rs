//! 74, 75

pub enum S3Mode {
    Default = 0x00,
    EStop = 0x01,
    EStopLatching = 0x81,
    VoltageClamp = 0x14,
    RS485Direction = 0x24,
    EncoderToggle = 0x84,
}

pub enum S4Mode {
    Default = 0x00,
    EStop = 0x01,
    EStopLatching = 0x81,
    VoltageClamp = 0x14,
    Brake = 0x04,
    HomeAuto = 0xE2,
    HomeUser = 0x62,
    HomeAutoLimitFwd = 0xF2,
    HomeUserLimitFwd = 0x72,
    LimitFwd = 0x12,
    LimitRev = 0x22,
    LimitBoth = 0x32,
}

pub enum S5Mode {
    Default = 0x00,
    EStop = 0x01,
    EStopLatching = 0x81,
    VoltageClamp = 0x14,
    Brake = 0x04,
    HomeUser = 0x62,
    HomeAutoLimitFwd = 0xF2,
    HomeUserLimitFwd = 0x72,
    LimitFwd = 0x12,
    LimitRev = 0x22,
    LimitBoth = 0x32,
}

// IMPLEMENT FROM AND OTHER THINGS

pub struct S345Modes {
    s3: S3Mode,
    s4: S4Mode,
    s5: S5Mode,
}
