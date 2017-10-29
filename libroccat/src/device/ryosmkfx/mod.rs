use std::path::{Path, PathBuf};

use device::HidrawData;
use errors::*;

pub struct RyosMkFx {
    path: PathBuf,
}

impl RyosMkFx {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    /// Gets the current profile
    pub fn get_profile(&self) -> Result<u8> {
        // Numbering starts from 32 for some reason in the API
        Ok(Profile::read(&self.path)?.index - 31)
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<()> {
        // Numbering starts from 32 for some reason in the API
        Profile::write(&self.path, &Profile::new(index + 31))
    }

    pub fn get_info(&self) -> Result<DeviceInfo> {
        DeviceInfo::read(&self.path)
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Ryos MK FX"
    }
}

impl_hidraw! {
    readwrite, report_id: 0x05;
    pub struct Profile {
        pub index: u8,
    }
}

impl_hidraw! {
    read, report_id: 0x0f;
    pub struct DeviceInfo {
        pub firmware_version: u8,
        pub dfu_version: u8,
        pub led_firmware_version: u8,
        pub unknown: [u8; 2],
    }
}
