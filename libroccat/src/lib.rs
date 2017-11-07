#![feature(const_size_of)]
#![feature(try_from)]

extern crate bitfield;
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate hidraw_derive;
extern crate libudev;
#[macro_use]
extern crate nix;

pub mod device;
pub mod errors {
    error_chain! {
        foreign_links {
            UdevError(::libudev::Error);
            IoError(::std::io::Error);
            NixError(::nix::Error);
        }
    }
}

pub use device::Device;
pub use errors::*;
use device::ryosmkfx::RyosMkFx;
use device::tyon::Tyon;

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
                }.ok_or("Incompatible Roccat device".into())
            })
            .filter_map(|device| {
                if let Ok(device) = device {
                    Some(device)
                } else {
                    println!("{}", device.err().unwrap());
                    None
                }
            })
            .collect(),
    )
}
