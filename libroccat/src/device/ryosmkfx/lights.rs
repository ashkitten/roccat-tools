#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum LightMode {
    Plain = 0x00,
    Layer = 0x01,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum LightLedFeedback {
    Off = 0x00,
    MacroExecution = 0x01,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum LightDimnessType {
    Off = 0x00,
    StarlitSky = 0x01,
    // TODO: find out what 0x02 is?
    FallAsleep = 0x03,
}

#[derive(HidrawRead, HidrawWrite, Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Lights {
    #[hidraw_constant = "0x0d"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub profile: u8,
    pub brightness: u8, // 0-5
    pub dimness: u8,    // 0-5
    pub timeout: u8,    // minutes
    pub mode: LightMode,
    pub effect: LightEffect,
    pub unknown0: u8,     // 0x00
    pub effect_speed: u8, // 1-3
    pub unknown1: u8,     // 0x00
    pub led_feedback: LightLedFeedback,
    pub dimness_type: LightDimnessType,
    pub unknown2: u8, // 0x1e
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub unused: [u8; 10],
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl Default for Lights {
    /// This is actually just arbitrary defaults I hand-picked
    fn default() -> Self {
        Self {
            profile: 1,
            brightness: 4,
            dimness: 1,
            timeout: 10,
            mode: LightMode::Plain,
            effect: LightEffect::FullyLit,
            unknown0: Default::default(),
            effect_speed: 1,
            unknown1: Default::default(),
            led_feedback: LightLedFeedback::Off,
            dimness_type: LightDimnessType::StarlitSky,
            unknown2: Default::default(),
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            unused: Default::default(),
            ..unsafe { ::std::mem::uninitialized() }
        }
    }
}
