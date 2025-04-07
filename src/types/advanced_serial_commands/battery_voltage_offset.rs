//! 115, 116

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BatteryVoltageOffset {
    main_battery_offset: u8,
    logic_battery_offset: u8,
}
