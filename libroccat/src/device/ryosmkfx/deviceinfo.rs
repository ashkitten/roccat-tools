use std;
use std::fs::OpenOptions;
use std::path::Path;
use std::os::unix::io::AsRawFd;

use device::DeviceData;
use errors::*;

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    _report_id: u8,
    _size: u8,
    firmware_version: u8,
    dfu_version: u8,
    led_firmware_version: u8,
    unknown: [u8; 2],
}

impl DeviceInfo {
    impl_hidraw!();
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            _report_id: 0x0f,
            _size: std::mem::size_of::<Self>() as u8,
            firmware_version: 0,
            dfu_version: 0,
            led_firmware_version: 0,
            unknown: [0; 2],
        }
    }
}

impl DeviceData for DeviceInfo {
    fn read(path: &Path) -> Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut data = Self::default();
        unsafe {
            Self::hidraw_read(file.as_raw_fd(), &mut data as *mut Self)?;
        }
        Ok(data)
    }

    fn write(_path: &Path, _data: &Self) -> Result<()> {
        bail!("DeviceInfo is read-only");
    }
}
