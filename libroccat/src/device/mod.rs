pub mod ryosmkfx;
pub mod tyon;
pub mod button;

use std::convert::TryInto;

use self::ryosmkfx::RyosMkFx;
use self::tyon::Tyon;
use errors::*;

pub enum Device {
    RyosMkFx(RyosMkFx),
    Tyon(Tyon),
}

impl Device {
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

pub enum Interface {
    Primary,
    Events,
}

impl TryInto<ryosmkfx::Interface> for Interface {
    type Error = Error;

    fn try_into(self) -> Result<ryosmkfx::Interface> {
        match self {
            Interface::Primary => Ok(ryosmkfx::Interface::Primary),
            Interface::Events => Ok(ryosmkfx::Interface::Events),
        }
    }
}

impl TryInto<tyon::Interface> for Interface {
    type Error = Error;

    fn try_into(self) -> Result<tyon::Interface> {
        match self {
            Interface::Primary => Ok(tyon::Interface::Primary),
            Interface::Events => Ok(tyon::Interface::Events),
        }
    }
}
