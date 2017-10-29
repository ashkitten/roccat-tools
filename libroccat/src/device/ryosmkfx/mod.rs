pub mod profile;
pub mod deviceinfo;

use std::path::{Path, PathBuf};

use self::profile::Profile;
use self::deviceinfo::DeviceInfo;
use device::DeviceData;
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
