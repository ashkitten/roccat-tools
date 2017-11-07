use std::fs::File;

use errors::*;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightControlState {
    Stored = 0x00,
    Custom = 0x01,
}

impl Default for LightControlState {
    fn default() -> Self {
        LightControlState::Stored
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightControlWriteCheck {
    Ok = 0x01,
    Invalid = 0x02,
    Busy = 0x03,
}

impl Default for LightControlWriteCheck {
    fn default() -> Self {
        LightControlWriteCheck::Ok
    }
}

#[derive(HidrawRead, HidrawWrite, Debug)]
#[repr(C, packed)]
pub struct LightControl {
    #[hidraw_constant = "0x13"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self> as u8"]
    _size: u8,
    pub state: LightControlState,
    pub unknown0: [u8; 3],
    pub write_check: LightControlWriteCheck,
    pub unknown1: u8,
}

impl LightControl {
    pub fn new(state: LightControlState) -> Self {
        Self {
            _report_id: 0x13,
            _size: ::std::mem::size_of::<Self> as u8,
            state,
            unknown0: Default::default(),
            write_check: Default::default(),
            unknown1: Default::default(),
        }
    }

    pub fn check_write(file: &File) -> Result<()> {
        unsafe {
            loop {
                use std::thread::sleep;
                use std::time::Duration;

                sleep(Duration::from_millis(50));

                let control = Self::read(file)?;
                match ::std::mem::transmute(control.write_check) {
                    LightControlWriteCheck::Ok => return Ok(()),
                    LightControlWriteCheck::Busy => (),
                    err => bail!("Write check returned {:?}", err),
                }
            }
        }
    }
}
