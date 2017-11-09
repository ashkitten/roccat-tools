#[repr(u8)]
pub enum ButtonModifier {
    None = 0,
    Shift = 1 << 0,
    Ctrl = 1 << 1,
    Alt = 1 << 2,
    Super = 1 << 3,
}

#[derive(Clone, Debug)]
#[repr(C, packed)]
pub struct ButtonConfig {
    pub type_: u8,
    pub modifier: u8, // enum ButtonModifier
    pub key: u8,
}

impl ButtonConfig {
    pub fn new(type_: u8, modifier: u8, key: u8) -> Self {
        Self {
            type_,
            modifier,
            key,
        }
    }
}
