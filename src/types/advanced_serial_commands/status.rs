//! 90

use bitflags::bitflags;

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
