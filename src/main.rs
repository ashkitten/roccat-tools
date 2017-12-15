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
            LibroccatError(::libroccat::Error, ::libroccat::ErrorKind);
        }

        foreign_links {
            RLuaError(::rlua::Error);
            StdIoError(::std::io::Error);
            StdParseIntError(::std::num::ParseIntError);
        }
    }

    unsafe impl Sync for Error {}
}

use clap::{App, SubCommand};
use std::thread;

use errors::*;

quick_main!(|| -> Result<()> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let matches = App::new("roccat-tools")
        .author("Ash Lea <ashlea@protonmail.com>")
        .about("Controls Roccat devices")
        .subcommand(SubCommand::with_name("list")
            .about("List attached devices")
        )
        .subcommand(SubCommand::with_name("run")
            .about("Run scripts")
            .args_from_usage("
                <script>...
            ")
        )
        .subcommand(SubCommand::with_name("get")
            .about("Get a property of a device")
            .args_from_usage("
                <device>   'Device to get from'
                <property> 'Property to get'
            ")
        )
        .subcommand(SubCommand::with_name("set")
            .about("Set a property of a device")
            .args_from_usage("
                <device>   'Device to set on'
                <property> 'Property to set'
                <value>    'Value of property'
            ")
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("list") {
        for (i, device) in libroccat::find_devices()?.iter().enumerate() {
            println!("{}: {}", i, device.get_common_name());
        }
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        let mut join_handles = Vec::new();

        for path in matches.values_of("script").unwrap() {
            let path = path.to_string();
            join_handles.push(thread::spawn(move || {
                libroccat_lua::run_script(&path).unwrap()
            }));
        }

        for handle in join_handles {
            handle.join().unwrap()
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let device_index = matches
            .value_of("device")
            .unwrap()
            .parse::<usize>()
            .chain_err(|| "Device must be an integer")?;
        let device = libroccat::find_devices()
            .chain_err(|| "Device index out of range")?
            .remove(device_index);

        println!(
            "{}",
            match matches.value_of("property") {
                Some("profile") => device.get_profile()?,
                Some(_) => bail!("Invalid property"),
                None => unreachable!(),
            }
        );
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        let device_index = matches
            .value_of("device")
            .unwrap()
            .parse::<usize>()
            .chain_err(|| "Device must be an integer")?;
        let device = libroccat::find_devices()
            .chain_err(|| "Device index out of range")?
            .remove(device_index);
        let value = matches.value_of("value").unwrap();

        match matches.value_of("property") {
            Some("profile") => {
                device.set_profile(value.parse::<u8>()?)?;
            }
            Some(_) => bail!("Invalid property"),
            None => unreachable!(),
        }
    }

    Ok(())
});
