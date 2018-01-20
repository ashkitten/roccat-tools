#[repr(u8)]
pub enum ButtonModifier {
    None = 0,
    Shift = 1,
    Ctrl = 2,
    Alt = 4,
    Super = 8,
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
