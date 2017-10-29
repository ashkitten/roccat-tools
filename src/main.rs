extern crate clap;
extern crate libroccat;
#[macro_use]
extern crate dyon;

mod libroccat_dyon;

use libroccat::device::Device;
use clap::{App, Arg};

fn main() {
    let matches = App::new("roccat-tools")
        .author("Ash Lea <ashlea@protonmail.com>")
        .about("Controls Roccat devices")
        .args(&[
            Arg::with_name("list")
                .long("list")
                .help("List attached devices"),
            Arg::with_name("device")
                .long("device")
                .help("Operate on a specific device")
                .takes_value(true),
            Arg::with_name("get_profile")
                .long("get-profile")
                .help("Gets the current profile of the device"),
            Arg::with_name("set_profile")
                .long("set-profile")
                .help("Sets the current profile of the device")
                .takes_value(true),
            Arg::with_name("script")
                .long("script")
                .help("Runs a Dyon script")
                .takes_value(true),
        ])
        .get_matches();

    if matches.is_present("list") {
        for (i, device) in libroccat::find_devices().unwrap().iter().enumerate() {
            println!("{}: {}", i, device.get_common_name());
        }
        std::process::exit(0);
    }

    if let Some(path) = matches.value_of("script") {
        libroccat_dyon::run_dyon(path);
    }

    if let Some(device_index) = matches.value_of("device") {
        let device_index = device_index.parse::<usize>().unwrap();
        match libroccat::find_devices().unwrap()[device_index] {
            Device::RyosMkFx(ref device) => {
                if let Some(profile) = matches.value_of("set_profile") {
                    // Profile numbering starts from 1 in libroccat
                    device.set_profile(profile.parse::<u8>().unwrap()).unwrap();
                }

                if matches.is_present("get_profile") {
                    println!("{}", device.get_profile().unwrap());
                }
            }
            Device::Tyon(ref device) => {
                if let Some(profile) = matches.value_of("set_profile") {
                    // Profile numbering starts from 1 in libroccat
                    device.set_profile(profile.parse::<u8>().unwrap()).unwrap();
                }

                if matches.is_present("get_profile") {
                    println!("{}", device.get_profile().unwrap());
                }
            }
        }
    }
}
