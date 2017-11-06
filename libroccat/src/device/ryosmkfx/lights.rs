use errors::*;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightMode {
    Plain = 0x00,
    Layer = 0x01,
}

impl Default for LightMode {
    fn default() -> Self {
        LightMode::Plain
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightEffect {
    Off = 0x00,
    FullyLit = 0x01,
    Blinking = 0x02,
    Breathing = 0x03,
    Heartbeat = 0x04,
    Equalizer = 0x05,  // Client side
    Ripple = 0x06,     // Client side
    Wave = 0x07,       // Client side
    Heatmap = 0x08,    // Client side
    GamePreset = 0x09, // Client side
    Fade = 0x10,
}

impl Default for LightEffect {
    fn default() -> Self {
        LightEffect::Off
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightLedFeedback {
    Off = 0x00,
    MacroExecution = 0x01,
}

impl Default for LightLedFeedback {
    fn default() -> Self {
        LightLedFeedback::Off
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightDimnessType {
    Off = 0x00,
    StarlitSky = 0x01,
    // TODO: find out what 0x02 is?
    FallAsleep = 0x03,
}

impl Default for LightDimnessType {
    fn default() -> Self {
        LightDimnessType::Off
    }
}

#[derive(HidrawRead, HidrawWrite, Debug, Clone)]
#[repr(C, packed)]
pub struct Lights {
    #[hidraw_constant = "0x0d"] _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"] _size: u8,
    profile: u8,
    brightness: u8, // 0-5
    dimness: u8,    // 0-5
    timeout: u8,    // minutes
    mode: LightMode,
    effect: LightEffect,
    unknown0: u8,     // 0x00
    effect_speed: u8, // 1-3
    unknown1: u8,     // 0x00
    led_feedback: LightLedFeedback,
    dimness_type: LightDimnessType,
    unknown2: u8, // 0x1e
    red: u16,
    green: u16,
    blue: u16,
    unused: [u8; 10],
    pub bytesum: u16,
}
