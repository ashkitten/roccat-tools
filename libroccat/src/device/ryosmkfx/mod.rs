mod control;
mod custom_lights;
mod hardware_color;
mod lights;
mod light_control;
mod sdk;
mod event;

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
                file = (*interfaces_guard)[Interface::Mouse as usize].try_clone().unwrap();
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

    pub fn get_interface(&self, interface: Interface) -> Result<File> {
        let guard = self.interfaces.lock().unwrap();
        Ok((*guard)[interface as usize].try_clone()?)
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Ryos MK FX"
    }

    /// Gets the current profile
    pub fn get_profile(&self) -> Result<u8> {
        // Numbering starts from 32 for some reason in the API
        Ok(Profile::read(&self.get_interface(Interface::Keyboard)?)?.index - 31)
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<()> {
        ensure!(
            index > 31 && index <= 36,
            "Profile {} is out of range",
            index
        );
        // Numbering starts from 32 for some reason in the API
        Profile::write(
            &self.get_interface(Interface::Keyboard)?,
            &Profile::new(index + 31),
        )
    }

    pub fn get_info(&self) -> Result<DeviceInfo> {
        DeviceInfo::read(&self.get_interface(Interface::Keyboard)?)
    }

    pub fn get_lights(&self, profile: u8) -> Result<Lights> {
        Control::write(
            &self.get_interface(Interface::Keyboard)?,
            &Control::new(profile, ControlRequest::Light as u8),
        )?;
        Control::check_write(&self.get_interface(Interface::Keyboard)?)?;
        Lights::read(&self.get_interface(Interface::Keyboard)?)
    }

    pub fn set_lights(&self, lights: &Lights) -> Result<()> {
        let mut data = lights.clone();
        // Bytesum is 2 bytes, we shouldn't include that
        let bytes: [u8; ::std::mem::size_of::<Lights>() - 2] =
            unsafe { ::std::mem::transmute_copy(&data) };
        data.bytesum = bytes.iter().map(|b| *b as u16).sum();
        Lights::write(&self.get_interface(Interface::Keyboard)?, &data)
    }

    pub fn set_custom_lights_active(&self, active: bool) -> Result<()> {
        let state = if active {
            LightControlState::Custom
        } else {
            LightControlState::Stored
        };
        LightControl::write(
            &self.get_interface(Interface::Keyboard)?,
            &LightControl::new(
                state,
                Default::default(),
                Default::default(),
                Default::default(),
            ),
        )
    }

    pub fn get_custom_lights_active(&self) -> Result<bool> {
        Ok(
            match LightControl::read(&self.get_interface(Interface::Keyboard)?)?.state {
                LightControlState::Custom => true,
                LightControlState::Stored => false,
            },
        )
    }

    pub fn get_custom_lights(&self) -> Result<CustomLights> {
        CustomLights::read(&self.get_interface(Interface::Keyboard)?)
    }

    pub fn set_custom_lights(&self, custom_lights: &CustomLights) -> Result<()> {
        let mut data = custom_lights.clone();
        // Bytesum is 2 bytes, we shouldn't include that
        let bytes: [u8; ::std::mem::size_of::<CustomLights>() - 2] =
            unsafe { ::std::mem::transmute_copy(&data) };
        data.bytesum = bytes.iter().map(|b| *b as u16).sum();
        CustomLights::write(&self.get_interface(Interface::Keyboard)?, &data)?;
        LightControl::check_write(&self.get_interface(Interface::Keyboard)?)
    }

    pub fn get_event(&self) -> Option<Event> {
        let mut guard = self.event_queue.lock().unwrap();
        (*guard).pop()
    }
}

pub enum Interface {
    Keyboard = 0,
    Mouse = 1,
}

impl_hidraw! {
    readwrite;
    #[derive(Debug)]
    Profile {
        @constant _report_id: u8 = 0x05,
        @constant _size: u8 = ::std::mem::size_of::<Self>() as u8,
        index: u8,
    }
}

impl_hidraw! {
    read;
    #[derive(Debug)]
    DeviceInfo {
        @constant _report_id: u8 = 0x0f,
        @constant _size: u8 = ::std::mem::size_of::<Self>() as u8,
        firmware_version: u8,
        dfu_version: u8,
        led_firmware_version: u8,
        unknown: [u8; 2],
    }
}
