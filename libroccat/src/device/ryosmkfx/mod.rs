mod control;
mod light;

use std::path::{Path, PathBuf};

pub use self::control::*;
pub use self::light::*;
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

    pub fn get_common_name<'a>() -> &'a str {
        "Ryos MK FX"
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

    pub fn get_light(&self, profile: u8) -> Result<Light> {
        Control::write(
            &self.path,
            &Control::new(profile, ControlRequest::Light as u8),
        )?;
        Control::check_write(&self.path)?;
        Light::read(&self.path)
    }

    pub fn set_light(&self, light: &Light) -> Result<()> {
        let mut data = light.clone();
        // Bytesum is 2 bytes, we shouldn't include that
        let bytes: [u8; ::std::mem::size_of::<Light>() - 2] =
            unsafe { ::std::mem::transmute_copy(&data) };
        data.bytesum = bytes.iter().map(|b| *b as u16).sum();
        Light::write(&self.path, &data)
    }
}

pub enum Interface {
    Keyboard = 0x00,
    Mouse = 0x01,
}

impl_hidraw! {
    readwrite;
    Profile {
        @constant _report_id: u8 = 0x05,
        @constant _size: u8 = ::std::mem::size_of::<Self>() as u8,
        index: u8,
    }
}

impl_hidraw! {
    read;
    DeviceInfo {
        @constant _report_id: u8 = 0x0f,
        @constant _size: u8 = ::std::mem::size_of::<Self>() as u8,
        firmware_version: u8,
        dfu_version: u8,
        led_firmware_version: u8,
        unknown: [u8; 2],
    }
}
