use std::path::Path;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std;

use device::DeviceData;
use errors::*;

#[derive(Clone)]
pub struct Profile {
    _report_id: u8,
    _size: u8,
    pub index: u8,
}

impl Profile {
    impl_hidraw!();

    pub fn new(index: u8) -> Self {
        Self {
            _report_id: 0x05,
            _size: std::mem::size_of::<Self>() as u8,
            index: index,
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new(0)
    }
}

impl DeviceData for Profile {
    fn read(path: &Path) -> Result<Profile> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut data = Self::default();
        unsafe {
            Self::hidraw_read(file.as_raw_fd(), &mut data as *mut Self)?;
        }
        Ok(data)
    }

    fn write(path: &Path, data: &Self) -> Result<()> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut data = data.clone();
        unsafe {
            Self::hidraw_write(file.as_raw_fd(), &mut data as *mut Self)?;
        }
        Ok(())
    }
}
