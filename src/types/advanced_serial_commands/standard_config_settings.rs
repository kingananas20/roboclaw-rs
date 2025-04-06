//! 98, 99

use bitflags::bitflags;

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
