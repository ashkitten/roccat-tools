use std::fs::File;

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

#[derive(HidrawRead, HidrawWrite)]
#[repr(C, packed)]
pub struct Control {
    #[hidraw_constant = "0x04"]
    _report_id: u8,
    pub value: u8,
    pub request: u8,
}

impl Control {
    pub fn new(value: u8, request: u8) -> Self {
        Self {
            _report_id: 0x04,
            value,
            request,
        }
    }

    pub fn check_write(interface: &File) -> Result<()> {
        unsafe {
            loop {
                use std::thread::sleep;
                use std::time::Duration;

                sleep(Duration::from_millis(50));

                let control = Self::read(interface)?;
                match ::std::mem::transmute(control.value) {
                    ControlStatus::Ok => return Ok(()),
                    ControlStatus::Busy => (),
                    ControlStatus::Critical0 | ControlStatus::Critical1 => {
                        bail!("Got critical status")
                    }
                    ControlStatus::Invalid => bail!("Got unknown status"),
                }
            }
        }
    }
}
