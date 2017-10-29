use std::path::{Path, PathBuf};

use device::HidrawData;
use errors::*;

pub struct Tyon {
    path: PathBuf,
}

impl Tyon {
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
        Ok(Profile::read(&self.path)?.index + 1)
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<()> {
        Profile::write(&self.path, &Profile::new(index - 1))
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Tyon"
    }
}

impl_hidraw! {
    readwrite, report_id: 0x05;
    pub struct Profile {
        pub index: u8,
    }
}
