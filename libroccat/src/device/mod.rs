pub mod ryosmkfx;
pub mod tyon;

use std::path::Path;

use self::ryosmkfx::RyosMkFx;
use self::tyon::Tyon;
use errors::*;

pub trait HidrawData {
    fn read(path: &Path) -> Result<Self>
    where
        Self: Sized;
    fn write(path: &Path, data: &Self) -> Result<()>;
}

pub enum Device {
    RyosMkFx(RyosMkFx),
    Tyon(Tyon),
}

impl Device {
    // TODO: write macro for generating enum function calls

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
}
