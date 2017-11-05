use std::fs::File;
use std::path::PathBuf;

use errors::*;

pub struct Tyon {
    interfaces: Vec<File>,
}

impl Tyon {
    pub fn new(paths: Vec<PathBuf>) -> Result<Self> {
        let mut interfaces = Vec::new();
        for path in paths {
            interfaces.push(File::open(path)?);
        }

        Ok(Self {
            interfaces: interfaces,
        })
    }

    pub fn get_interface(&self, interface: Interface) -> &File {
        &self.interfaces[interface as usize]
    }

    /// Gets the current profile
    pub fn get_profile(&self) -> Result<u8> {
        Ok(Profile::read(&self.get_interface(Interface::Mouse))?.index + 1)
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<()> {
        ensure!(index <= 5, "Profile {} is out of range", index);
        Profile::write(
            &self.get_interface(Interface::Mouse),
            &Profile::new(index - 1),
        )
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Tyon"
    }
}

pub enum Interface {
    Mouse = 0,
    Keyboard = 1,
    Joystick = 2,
    Misc = 3,
}

impl_hidraw! {
    readwrite;
    #[derive(Debug)]
    Profile {
        @constant _report_id: u8 = 0x05,
        @constant _size: u8 = ::std::mem::size_of::<Self>() as u8,
        index: u8,
    }
}
