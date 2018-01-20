use failure::Error;
use std::fs::File;
use std::path::PathBuf;

pub struct Tyon {
    interfaces: Vec<File>,
}

impl Tyon {
    pub fn new(paths: Vec<PathBuf>) -> Result<Self, Error> {
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
    pub fn get_profile(&self) -> Result<u8, Error> {
        unsafe { Ok(Profile::read(self.get_interface(Interface::Primary))?.index + 1) }
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<(), Error> {
        unsafe {
            ensure!(
                index >= 1 && index <= 5,
                "Profile {} is out of range",
                index
            );
            Profile::new(index - 1).write(self.get_interface(Interface::Primary))
        }
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Tyon"
    }
}

pub enum Interface {
    Primary = 0,
    Events = 1,
}

#[derive(HidrawRead, HidrawWrite, Debug)]
#[repr(C, packed)]
pub struct Profile {
    #[hidraw_constant = "0x05"] _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"] _size: u8,
    pub index: u8,
}

impl Profile {
    fn new(index: u8) -> Self {
        Profile {
            index: index,
            ..unsafe { ::std::mem::uninitialized() }
        }
    }
}
