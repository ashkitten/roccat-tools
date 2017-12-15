mod control;
mod custom_lights;
mod hardware_color;
mod lights;
mod light_control;
mod sdk;
mod event;
mod keys;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

pub use self::control::*;
pub use self::custom_lights::*;
pub use self::hardware_color::*;
pub use self::lights::*;
pub use self::light_control::*;
pub use self::sdk::*;
pub use self::event::*;
pub use self::keys::*;
use errors::*;

pub struct RyosMkFx {
    interfaces: Arc<Mutex<Vec<File>>>,
    event_queue: Arc<Mutex<Vec<Event>>>,
}

impl RyosMkFx {
    pub fn new(paths: Vec<PathBuf>) -> Result<Self> {
        let mut interfaces = Vec::new();
        for path in paths {
            interfaces.push(File::open(path)?);
        }

        let device = Self {
            interfaces: Arc::new(Mutex::new(interfaces)),
            event_queue: Arc::new(Mutex::new(Vec::new())),
        };

        let interfaces = device.interfaces.clone();
        let event_queue = device.event_queue.clone();
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
                (*event_queue_guard)
                    .insert(0, unsafe { ::std::mem::transmute::<_, Event>(buf) });
            }
        });

        Ok(device)
    }

    pub fn get_interface(&self, interface: Interface) -> Result<File> {
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
    pub fn get_profile(&self) -> Result<u8> {
        unsafe {
            // Numbering starts from 16 for some reason in the API
            Ok(Profile::read(&self.get_interface(Interface::Primary)?)?.index - 15)
        }
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<()> {
        unsafe {
            ensure!(
                index >= 1 && index <= 5,
                "Profile {} is out of range",
                index
            );
            // Numbering starts from 16 for some reason in the API
            Profile::new(index + 15).write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn get_info(&self) -> Result<DeviceInfo> {
        unsafe { DeviceInfo::read(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_lights(&self, profile: u8) -> Result<Lights> {
        unsafe {
            Control::new(profile, ControlRequest::Light as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            Lights::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_lights(&self, lights: &Lights) -> Result<()> {
        unsafe {
            lights
                .clone()
                .write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_custom_lights_active(&self, active: bool) -> Result<()> {
        unsafe {
            let state = if active {
                LightControlState::Custom
            } else {
                LightControlState::Stored
            };
            LightControl::new(state).write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn get_custom_lights_active(&self) -> Result<bool> {
        unsafe {
            Ok(
                match LightControl::read(&self.get_interface(Interface::Primary)?)?.state {
                    LightControlState::Custom => true,
                    LightControlState::Stored => false,
                },
            )
        }
    }

    pub fn get_custom_lights(&self) -> Result<CustomLights> {
        unsafe { CustomLights::read(&self.get_interface(Interface::Primary)?) }
    }

    pub fn set_custom_lights(&self, custom_lights: &CustomLights) -> Result<()> {
        unsafe {
            custom_lights
                .clone()
                .write(&self.get_interface(Interface::Primary)?)?;
            LightControl::check_write(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn get_keys_primary(&self, profile: u8) -> Result<KeysPrimary> {
        unsafe {
            Control::new(profile, ControlRequest::KeysPrimary as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysPrimary::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_primary(&self, keys: KeysPrimary) -> Result<()> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_function(&self, profile: u8) -> Result<KeysFunction> {
        unsafe {
            Control::new(profile, ControlRequest::KeysFunction as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysFunction::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_function(&self, keys: KeysFunction) -> Result<()> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_macro(&self, profile: u8) -> Result<KeysMacro> {
        unsafe {
            Control::new(profile, ControlRequest::KeysMacro as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysMacro::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_macro(&self, keys: KeysMacro) -> Result<()> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_thumbster(&self, profile: u8) -> Result<KeysThumbster> {
        unsafe {
            Control::new(profile, ControlRequest::KeysThumbster as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysThumbster::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_thumbster(&self, keys: KeysThumbster) -> Result<()> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_extra(&self, profile: u8) -> Result<KeysExtra> {
        unsafe {
            Control::new(profile, ControlRequest::KeysExtra as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysExtra::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_extra(&self, keys: KeysExtra) -> Result<()> {
        unsafe { keys.write(&self.get_interface(Interface::Primary)?) }
    }

    pub fn get_keys_easyzone(&self, profile: u8) -> Result<KeysEasyzone> {
        unsafe {
            Control::new(profile, ControlRequest::KeysEasyzone as u8)
                .write(&self.get_interface(Interface::Primary)?)?;
            Control::check_write(&self.get_interface(Interface::Primary)?)?;

            KeysEasyzone::read(&self.get_interface(Interface::Primary)?)
        }
    }

    pub fn set_keys_easyzone(&self, keys: KeysEasyzone) -> Result<()> {
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
    #[hidraw_constant = "0x05"] _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"] _size: u8,
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
    #[hidraw_constant = "0x0f"] _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"] _size: u8,
    pub firmware_version: u8,
    pub dfu_version: u8,
    pub led_firmware_version: u8,
    pub unknown: [u8; 2],
}
