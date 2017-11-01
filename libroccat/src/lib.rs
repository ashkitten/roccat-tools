#![feature(const_size_of)]

#[macro_use]
extern crate error_chain;
extern crate libudev;
#[macro_use]
extern crate nix;

pub mod errors {
    error_chain! {
        foreign_links {
            UdevError(::libudev::Error);
            IoError(::std::io::Error);
            NixError(::nix::Error);
        }
    }
}

#[macro_use]
mod macros;

pub mod device;

use device::Device;
use device::ryosmkfx::RyosMkFx;
use device::tyon::Tyon;
use errors::*;

pub fn find_devices() -> Result<Vec<Device>> {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("usb")?;
    enumerator.match_attribute("idVendor", "1e7d")?;
    Ok(
        enumerator
            .scan_devices()?
            .filter(|parent| parent.attribute_value("idProduct").is_some())
            .map(|parent| -> Result<Device> {
                let mut enumerator = libudev::Enumerator::new(&context)?;
                enumerator.match_subsystem("hidraw")?;
                enumerator.match_parent(&parent)?;
                enumerator
                    .scan_devices()
                    .unwrap()
                    .filter_map(|device| {
                        match parent.attribute_value("idProduct")?.to_str()? {
                            // Ryos MK FX
                            "2fda" => Some(Device::RyosMkFx(RyosMkFx::new(device.devnode()?))),
                            // Tyon Black
                            "2e4a" => Some(Device::Tyon(Tyon::new(device.devnode()?))),
                            // Tyon White
                            "2e4b" => Some(Device::Tyon(Tyon::new(device.devnode()?))),
                            _ => None,
                        }
                    })
                    .nth(0)
                    .ok_or("incompatible Roccat device".into())
            })
            .filter_map(|device| device.ok())
            .collect(),
    )
}
