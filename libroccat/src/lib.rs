#![feature(const_size_of, try_from, type_ascription)]

extern crate bitfield;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate hidraw_derive;
extern crate libudev;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nix;

pub mod device;

use failure::Error;

use device::Device;
use device::ryosmkfx::RyosMkFx;
use device::tyon::Tyon;

pub fn find_devices() -> Result<Vec<Device>, Error> {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("usb")?;
    enumerator.match_attribute("idVendor", "1e7d")?;
    Ok(enumerator
        .scan_devices()?
        .filter(|parent| parent.attribute_value("idProduct").is_some())
        .map(|parent| -> Result<Device, Error> {
            let mut enumerator = libudev::Enumerator::new(&context)?;
            enumerator.match_subsystem("hidraw")?;
            enumerator.match_parent(&parent)?;
            match parent
                .attribute_value("idProduct")
                .unwrap()
                .to_str()
                .unwrap()
            {
                // Ryos MK FX
                "2fda" => Some(Device::RyosMkFx(RyosMkFx::new(
                    enumerator
                        .scan_devices()
                        .unwrap()
                        .filter_map(|device| {
                            if let Some(devnode) = device.devnode() {
                                Some(devnode.to_path_buf())
                            } else {
                                None
                            }
                        })
                        .collect(),
                )?)),
                // Tyon Black
                "2e4a" => Some(Device::Tyon(Tyon::new(
                    enumerator
                        .scan_devices()
                        .unwrap()
                        .filter_map(|device| {
                            if let Some(devnode) = device.devnode() {
                                Some(devnode.to_path_buf())
                            } else {
                                None
                            }
                        })
                        .collect(),
                )?)),
                // Tyon White
                "2e4b" => Some(Device::Tyon(Tyon::new(
                    enumerator
                        .scan_devices()
                        .unwrap()
                        .filter_map(|device| {
                            if let Some(devnode) = device.devnode() {
                                Some(devnode.to_path_buf())
                            } else {
                                None
                            }
                        })
                        .collect(),
                )?)),
                _ => None,
            }.ok_or(format_err!("Incompatible Roccat device"))
        })
        .filter_map(|device| {
            if let Ok(device) = device {
                Some(device)
            } else {
                println!("{}", device.err().unwrap());
                None
            }
        })
        .collect())
}
