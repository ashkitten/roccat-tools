extern crate libroccat;

use libroccat::device::Device;

#[test]
fn finds_devices() {
    for device in libroccat::find_devices().unwrap() {
        println!("found device: {:?}", device.get_path());
    }
}

#[test]
fn ryosmkfx() {
    for device in libroccat::find_devices().unwrap() {
        if let Device::RyosMkFx(device) = device {
            println!("Found Ryos MK FX: {:?}", device.get_path());
            println!("Profile index: {}", device.get_profile().unwrap());
            println!("{:?}", device.get_info().unwrap());
        }
    }
}

#[test]
fn tyon() {
    for device in libroccat::find_devices().unwrap() {
        if let Device::Tyon(device) = device {
            println!("Found Tyon: {:?}", device.get_path());
        }
    }
}
