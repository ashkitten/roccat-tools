mod control;
mod custom_lights;
mod event;
mod hardware_color;
mod keys;
mod light_control;
mod lights;
mod sdk;

use bitfield::NibbleField;
use failure::{ensure, Error};
use hidraw_derive::{HidrawRead, HidrawWrite};
use std::{
    fs::File,
    io::prelude::*,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

pub use self::{
    control::*, custom_lights::*, event::*, hardware_color::*, keys::*, light_control::*,
    lights::*, sdk::*,
};

pub struct RyosMkFx {
    interfaces: Arc<Mutex<Vec<File>>>,
    event_queue: Arc<Mutex<Vec<Event>>>,
}

impl RyosMkFx {
    pub fn new(paths: Vec<PathBuf>) -> Result<Self, Error> {
        let mut interfaces = Vec::new();
        for path in paths {
            interfaces.push(File::open(path)?);
        }

        let device = Self {
            interfaces: Arc::new(Mutex::new(interfaces)),
            event_queue: Arc::new(Mutex::new(Vec::new())),
        };

        let interfaces = Arc::clone(&device.interfaces);
        let event_queue = Arc::clone(&device.event_queue);
        thread::spawn(move || {
            let mut file;
            {
                let interfaces_guard = interfaces.lock().unwrap();
                file = (*interfaces_guard)[Interface::Events as usize]
                    .try_clone()
                    .unwrap();
            }

            loop {
                let mut buf = [0u8; ::std::mem::size_of::<Event>()];
                file.read_exact(&mut buf).unwrap();

                let mut event_queue_guard = event_queue.lock().unwrap();
                (*event_queue_guard).insert(0, unsafe { ::std::mem::transmute::<_, Event>(buf) });
            }
        });

        Ok(device)
    }

    pub fn get_interface(&self, interface: Interface) -> Result<File, Error> {
        let guard = self.interfaces.lock().unwrap();
        Ok((*guard)[interface as usize].try_clone()?)
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Ryos MK FX"
    }

    pub fn get_event(&self) -> Option<Event> {
        let mut guard = self.event_queue.lock().unwrap();
        (*guard).pop()
    }

    /// Gets the current profile
    pub fn get_profile(&self) -> Result<u8, Error> {
        unsafe {
            Ok(Profile::read(&self.get_interface(Interface::Primary)?)?
                .index
                .get_nibble(0)
                + 1)
        }
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<(), Error> {
        unsafe {
            ensure!(
                index >= 1 && index <= 5,
                "Profile {} is out of range",
                index
            );
            let mut profile = Profile::read(&self.get_interface(Interface::Primary)?)?.index;
            profile.set_nibble(0, index - 1);
            Profile::new(profile).write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn get_info(&self) -> Result<DeviceInfo, Error> {
        unsafe { DeviceInfo::read(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_lights(&self, profile: u8) -> Result<Lights, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::Light as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            Lights::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_lights(&self, lights: &Lights) -> Result<(), Error> {
        unsafe {
            let mut lights = lights.clone();
            lights.profile -= 1;
            lights.write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_custom_lights_active(&self, active: bool) -> Result<(), Error> {
        unsafe {
            let state = if active {
                LightControlState::Custom
            } else {
                LightControlState::Stored
            };
            LightControl::new(state).write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn get_custom_lights_active(&self) -> Result<bool, Error> {
        unsafe {
            Ok(
                match LightControl::read(&self.get_interface(Interface::Primary)?)?.state {
                    LightControlState::Custom => true,
                    LightControlState::Stored => false,
                },
            )
        }
    }

    pub fn get_custom_lights(&self) -> Result<CustomLights, Error> {
        unsafe { CustomLights::read(&self.get_interface(Interface::Primary)?) }
    }

    pub fn set_custom_lights(&self, custom_lights: &CustomLights) -> Result<(), Error> {
        unsafe {
            custom_lights
                .clone()
                .write(&self.get_interface(Interface::Primary)?)?;
            LightControl::check_write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn get_keys_primary(&self, profile: u8) -> Result<KeysPrimary, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::KeysPrimary as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysPrimary::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_primary(&self, keys: KeysPrimary) -> Result<(), Error> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_function(&self, profile: u8) -> Result<KeysFunction, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::KeysFunction as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysFunction::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_function(&self, keys: KeysFunction) -> Result<(), Error> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_macro(&self, profile: u8) -> Result<KeysMacro, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::KeysMacro as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysMacro::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_macro(&self, keys: KeysMacro) -> Result<(), Error> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_thumbster(&self, profile: u8) -> Result<KeysThumbster, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::KeysThumbster as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysThumbster::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_thumbster(&self, keys: KeysThumbster) -> Result<(), Error> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_extra(&self, profile: u8) -> Result<KeysExtra, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::KeysExtra as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysExtra::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_extra(&self, keys: KeysExtra) -> Result<(), Error> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_easyzone(&self, profile: u8) -> Result<KeysEasyzone, Error> {
        unsafe {
            Control::new(profile - 1, ControlRequest::KeysEasyzone as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysEasyzone::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_easyzone(&self, keys: KeysEasyzone) -> Result<(), Error> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }
}

pub enum Interface {
    Primary = 0,
    Events = 1,
}

#[derive(HidrawRead, HidrawWrite, Debug)]
#[repr(C, packed)]
pub struct Profile {
    #[hidraw_constant = "0x05"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    // first nibble: number of profiles enabled
    // second nibble: index of current profile
    pub index: u8,
}

impl Profile {
    fn new(index: u8) -> Self {
        Profile {
            index: index,
            ..unsafe { ::std::mem::uninitialized() }
        }
    }
}

#[derive(HidrawRead, Debug)]
#[repr(C, packed)]
pub struct DeviceInfo {
    #[hidraw_constant = "0x0f"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub firmware_version: u8,
    pub dfu_version: u8,
    pub led_firmware_version: u8,
    pub unknown: [u8; 2],
}
