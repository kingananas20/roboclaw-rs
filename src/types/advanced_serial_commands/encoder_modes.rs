//! 91, 92, 93, 95

use bitflags::bitflags;

bitflags! {
    pub struct EncoderMode: u8 {
        const RC_ANALOG_ENCODER_SUPPORT = 0b10000000;
        const REVERSE_ENCODER_RELATIVE_DIRECTION = 0b01000000;
        const REVERSE_MOTOR_RELATIVE_DIRECTION = 0b00100000;
        const ABSOLUTE_MODE = 0b00000001;
    }
}
