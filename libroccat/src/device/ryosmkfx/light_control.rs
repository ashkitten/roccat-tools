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

impl_hidraw! {
    readwrite;
    #[derive(Debug)]
    LightControl {
        @constant _report_id: u8 = 0x13,
        @constant _size: u8 = ::std::mem::size_of::<Self> as u8,
        state: LightControlState,
        unknown0: [u8; 3],
        write_check: LightControlWriteCheck,
        unknown1: u8,
    }
}

impl LightControl {
    pub fn check_write(file: &File) -> Result<()> {
        loop {
            use std::thread::sleep;
            use std::time::Duration;

            sleep(Duration::from_millis(50));

            let control = Self::read(file)?;
            match unsafe { ::std::mem::transmute(control.write_check) } {
                LightControlWriteCheck::Ok => return Ok(()),
                LightControlWriteCheck::Busy => (),
                err => bail!("Write check returned {:?}", err),
            }
        }
    }
}
