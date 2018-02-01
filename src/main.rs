#![feature(type_ascription)]

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate libroccat;
#[macro_use]
extern crate log;
extern crate rlua;

mod libroccat_lua;

use clap::{App, SubCommand};
use failure::{Error, ResultExt};
use std::thread;

fn run() -> Result<(), Error> {
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
            .context("Device must be an integer")?;
        let device = libroccat::find_devices()
            .context("Device index out of range")?
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
            .context("Device must be an integer")?;
        let device = libroccat::find_devices()
            .context("Device index out of range")?
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
}

fn main() {
    use std::io::Write;

    env_logger::try_init().expect("Failed to initialize logger");

    std::process::exit(match run() {
        Ok(()) => 0,
        Err(ref error) => {
            let mut causes = error.causes();

            error!(
                "{}",
                causes
                    .next()
                    .expect("`causes` should contain at least one error")
            );
            for cause in causes {
                error!("Caused by: {}", cause);
            }

            let backtrace = format!("{}", error.backtrace());
            if backtrace.is_empty() {
                writeln!(
                    ::std::io::stderr(),
                    "Set RUST_BACKTRACE=1 to see a backtrace"
                ).expect("Could not write to stderr");
            } else {
                writeln!(::std::io::stderr(), "{}", error.backtrace())
                    .expect("Could not write to stderr");
            }

            1
        }
    });
}
