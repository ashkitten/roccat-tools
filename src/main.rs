#![feature(type_ascription)]

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate libroccat;
extern crate rlua;

mod libroccat_lua;

mod errors {
    error_chain! {
        links {
            LibroccatError(::libroccat::errors::Error, ::libroccat::errors::ErrorKind);
        }

        foreign_links {
            RLuaError(::rlua::Error);
            StdIoError(::std::io::Error);
            StdParseIntError(::std::num::ParseIntError);
        }
    }

    unsafe impl Sync for Error {}
}

use clap::{App, Arg};

use errors::*;

quick_main!(|| -> Result<()> {
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
                .help("Runs a script")
                .takes_value(true),
        ])
        .get_matches();

    if matches.is_present("list") {
        for (i, device) in libroccat::find_devices()?.iter().enumerate() {
            println!("{}: {}", i, device.get_common_name());
        }
        std::process::exit(0);
    }

    if let Some(device_index) = matches.value_of("device") {
        let device_index = device_index.parse::<usize>()?;
        let device = libroccat::find_devices()?.remove(device_index);

        if matches.is_present("get_profile") {
            println!("{}", device.get_profile()?);
            return Ok(());
        }

        if let Some(profile) = matches.value_of("set_profile") {
            device.set_profile(profile.parse::<u8>()?)?;
            return Ok(());
        }
    }

    if let Some(path) = matches.value_of("script") {
        libroccat_lua::run_script(path)?;
    }

    Ok(())
});
