pub mod ryosmkfx;
pub mod tyon;

use std::path::Path;

use errors::*;

pub trait DeviceData {
    fn read(path: &Path) -> Result<Self>
    where
        Self: Sized;
    fn write(path: &Path, data: &Self) -> Result<()>;
}

pub enum Device {
    RyosMkFx(ryosmkfx::RyosMkFx),
    Tyon(tyon::Tyon),
}

impl Device {
    pub fn get_path(&self) -> &Path {
        match *self {
            Device::RyosMkFx(ref device) => device.get_path(),
            Device::Tyon(ref device) => device.get_path(),
        }
    }
}
