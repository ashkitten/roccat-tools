extern crate clap;
extern crate libroccat;

use libroccat::device::Device;
use clap::{App, Arg};

fn main() {
    let matches = App::new("roccat-tools")
        .author("Ash Lea <ashlea@protonmail.com>")
        .about("Controls Roccat devices")
        .args(&[
            Arg::with_name("profile")
                .long("profile")
                .help("Sets the current profile of your device")
                .takes_value(true),
        ])
        .get_matches();

    for device in libroccat::find_devices().unwrap() {
        if let Device::RyosMkFx(device) = device {
            if let Some(profile) = matches.value_of("profile") {
                // Profile numbering starts from 1 in libroccat
                device.set_profile(profile.parse::<u8>().unwrap()).unwrap();
            }
        }
    }
}
