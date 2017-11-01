pub mod ryosmkfx;
pub mod tyon;

use std::path::Path;

use self::ryosmkfx::RyosMkFx;
use self::tyon::Tyon;
use errors::*;

pub enum Device {
    RyosMkFx(RyosMkFx),
    Tyon(Tyon),
}

impl Device {
    pub fn get_path(&self) -> &Path {
        match *self {
            Device::RyosMkFx(ref device) => device.get_path(),
            Device::Tyon(ref device) => device.get_path(),
        }
    }

    pub fn get_common_name(&self) -> &str {
        match *self {
            Device::RyosMkFx(_) => RyosMkFx::get_common_name(),
            Device::Tyon(_) => Tyon::get_common_name(),
        }
    }

    pub fn get_profile(&self) -> Result<u8> {
        match *self {
            Device::RyosMkFx(ref device) => device.get_profile(),
            Device::Tyon(ref device) => device.get_profile(),
        }
    }

    pub fn set_profile(&self, profile: u8) -> Result<()> {
        match *self {
            Device::RyosMkFx(ref device) => device.set_profile(profile),
            Device::Tyon(ref device) => device.set_profile(profile),
        }
    }
}
