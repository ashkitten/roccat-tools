pub mod ryosmkfx;
pub mod tyon;

use std::convert::TryInto;

pub use self::ryosmkfx::RyosMkFx;
pub use self::tyon::Tyon;
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
    Keyboard,
    Mouse,
    Joystick,
    Misc,
}

impl TryInto<ryosmkfx::Interface> for Interface {
    type Error = Error;

    fn try_into(self) -> Result<ryosmkfx::Interface> {
        match self {
            Interface::Keyboard => Ok(ryosmkfx::Interface::Keyboard),
            Interface::Mouse => Ok(ryosmkfx::Interface::Mouse),
            _ => bail!("No such interface on Ryos MK FX"),
        }
    }
}

impl TryInto<tyon::Interface> for Interface {
    type Error = Error;

    fn try_into(self) -> Result<tyon::Interface> {
        match self {
            Interface::Keyboard => Ok(tyon::Interface::Keyboard),
            Interface::Mouse => Ok(tyon::Interface::Mouse),
            Interface::Joystick => Ok(tyon::Interface::Joystick),
            Interface::Misc => Ok(tyon::Interface::Mouse),
        }
    }
}
