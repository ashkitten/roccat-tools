use failure::{bail, Error};
use hidraw_derive::{HidrawRead, HidrawWrite};
use std::fs::File;

#[repr(u8)]
pub enum ControlRequest {
    KeysPrimary = 0xa0,
    KeysFunction = 0xa1,
    KeysMacro = 0xa2,
    KeysThumbster = 0xa3,
    KeysEasyzone = 0xa5,
    KeyMask = 0xb0,
    Light = 0xb1,
    KeysExtra = 0xb2,
    StoredLightsAutomatic = 0xc0,
    StoredLightsManual = 0xd0,
    LightMacro = 0xe0,
    Request12 = 0xf0, // idk, from erazor_de's code
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
            value,
            request,
            ..unsafe { ::std::mem::uninitialized() }
        }
    }

    pub fn check_write(interface: &File) -> Result<(), Error> {
        unsafe {
            loop {
                use std::{thread::sleep, time::Duration};

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
