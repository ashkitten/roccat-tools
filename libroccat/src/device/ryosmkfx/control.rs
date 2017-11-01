use std::path::Path;

use errors::*;

#[repr(u8)]
pub enum ControlRequest {
    Light = 0xb1,
    B3 = 0xb3, // TODO: ??
}

#[repr(u8)]
pub enum ControlStatus {
    Critical0 = 0x00,
    Ok = 0x01,
    Invalid = 0x02,
    Busy = 0x03,
    Critical1 = 0x04, // used by Ryos MK
}

impl_hidraw! {
    readwrite;
    Control {
        @constant _report_id: u8 = 0x04,
        value: u8,
        request: u8,
    }
}

impl Control {
    pub fn check_write(path: &Path) -> Result<()> {
        loop {
            use std::thread::sleep;
            use std::time::Duration;

            sleep(Duration::from_millis(50));

            let control = Control::read(path)?;
            match unsafe { ::std::mem::transmute(control.value) } {
                ControlStatus::Ok => return Ok(()),
                ControlStatus::Busy => (),
                ControlStatus::Critical0 | ControlStatus::Critical1 => bail!("Got critical status"),
                ControlStatus::Invalid => bail!("Got unknown status"),
            }
        }
    }
}
