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

use clap::{App, SubCommand};

use errors::*;

quick_main!(|| -> Result<()> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let matches = App::new("roccat-tools")
        .author("Ash Lea <ashlea@protonmail.com>")
        .about("Controls Roccat devices")
        .args_from_usage("
            -l, --list            'List attached devices'
            -s, --script <script> 'Run a script'
        ")
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

    if matches.is_present("list") {
        for (i, device) in libroccat::find_devices()?.iter().enumerate() {
            println!("{}: {}", i, device.get_common_name());
        }
        std::process::exit(0);
    }

    if let Some(path) = matches.value_of("script") {
        libroccat_lua::run_script(path)?;
        std::process::exit(0);
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
