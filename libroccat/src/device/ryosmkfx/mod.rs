mod profile;
mod deviceinfo;

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

    /// Gets the current profile of the keyboard
    pub fn get_profile(&self) -> Result<u8> {
        // Numbering starts from 32 for some reason in the API
        Ok(Profile::read(&self.path)?.index)
    }

    /// Sets the current profile of the keyboard
    pub fn set_profile(&self, index: u8) -> Result<()> {
        // Numbering starts from 32 for some reason in the API
        Profile::write(&self.path, &Profile::new(index + 31))?;
        Ok(())
    }

    pub fn get_info(&self) -> Result<DeviceInfo> {
        Ok(DeviceInfo::read(&self.path)?)
    }
}
