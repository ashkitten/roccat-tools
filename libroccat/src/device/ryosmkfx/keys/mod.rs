mod defaults;

use std::fmt;
use std;

pub use self::defaults::*;
use device::button::*;
use errors::*;

#[derive(HidrawRead, HidrawWrite, Copy, Clone)]
#[repr(C, packed)]
pub struct KeysPrimary {
    #[hidraw_constant = "0x06"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub profile_index: u8,
    pub keys: [u8; 120], // Just key mappings
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl KeysPrimary {
    pub fn new(profile_index: u8, keys: [u8; 120]) -> Self {
        Self {
            profile_index,
            keys,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl Default for KeysPrimary {
    fn default() -> Self {
        Self {
            profile_index: Default::default(),
            keys: DEFAULT_KEYS_PRIMARY,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl fmt::Debug for KeysPrimary {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bytesum = self._bytesum;
        fmt.debug_struct("KeysPrimary")
            .field("_report_id", &self._report_id)
            .field("_size", &self._size)
            .field("profile_index", &self.profile_index)
            .field("keys", &&self.keys[..])
            .field("_bytesum", &bytesum)
            .finish()
    }
}

#[derive(HidrawRead, HidrawWrite)]
#[repr(C, packed)]
pub struct KeysFunction {
    #[hidraw_constant = "0x07"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub profile_index: u8,
    pub keys: [ButtonConfig; 15 * 2], // Regular and with Fn pressed
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl KeysFunction {
    pub fn new(profile_index: u8, keys: [ButtonConfig; 15 * 2]) -> Self {
        Self {
            profile_index,
            keys,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl Default for KeysFunction {
    fn default() -> Self {
        Self {
            profile_index: Default::default(),
            keys: DEFAULT_KEYS_FUNCTION,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl fmt::Debug for KeysFunction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bytesum = self._bytesum;
        fmt.debug_struct("KeysFunction")
            .field("_report_id", &self._report_id)
            .field("_size", &self._size)
            .field("profile_index", &self.profile_index)
            .field("keys", &&self.keys[..])
            .field("_bytesum", &bytesum)
            .finish()
    }
}

#[derive(HidrawRead, HidrawWrite)]
#[repr(C, packed)]
pub struct KeysMacro {
    #[hidraw_constant = "0x08"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub profile_index: u8,
    pub keys: [ButtonConfig; 5 * 2], // Regular and with EasyShift pressed
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl KeysMacro {
    pub fn new(profile_index: u8, keys: [ButtonConfig; 5 * 2]) -> Self {
        Self {
            profile_index,
            keys,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl Default for KeysMacro {
    fn default() -> Self {
        Self {
            profile_index: Default::default(),
            keys: DEFAULT_KEYS_MACRO,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl fmt::Debug for KeysMacro {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bytesum = self._bytesum;
        fmt.debug_struct("KeysMacro")
            .field("_report_id", &self._report_id)
            .field("_size", &self._size)
            .field("profile_index", &self.profile_index)
            .field("keys", &&self.keys[..])
            .field("_bytesum", &bytesum)
            .finish()
    }
}

#[derive(HidrawRead, HidrawWrite)]
#[repr(C, packed)]
pub struct KeysThumbster {
    #[hidraw_constant = "0x09"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub profile_index: u8,
    pub keys: [ButtonConfig; 3 * 2], // Regular and with EasyShift pressed
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl KeysThumbster {
    pub fn new(profile_index: u8, keys: [ButtonConfig; 3 * 2]) -> Self {
        Self {
            profile_index,
            keys,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl Default for KeysThumbster {
    fn default() -> Self {
        Self {
            profile_index: Default::default(),
            keys: DEFAULT_KEYS_THUMBSTER,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl fmt::Debug for KeysThumbster {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bytesum = self._bytesum;
        fmt.debug_struct("KeysThumbster")
            .field("_report_id", &self._report_id)
            .field("_size", &self._size)
            .field("profile_index", &self.profile_index)
            .field("keys", &&self.keys[..])
            .field("_bytesum", &bytesum)
            .finish()
    }
}

#[derive(HidrawRead, HidrawWrite, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct KeysExtra {
    #[hidraw_constant = "0x0a"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    pub profile_index: u8,
    pub capslock: u8,
    pub fn_: u8,
    pub unused: u8,
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl KeysExtra {
    pub fn new(profile_index: u8, capslock: u8, fn_: u8, unused: u8) -> Self {
        Self {
            profile_index,
            capslock,
            fn_,
            unused,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl Default for KeysExtra {
    fn default() -> Self {
        Self {
            profile_index: Default::default(),
            capslock: 0xff,
            fn_: 0xf1,
            unused: 0x00,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

#[derive(HidrawRead, HidrawWrite)]
#[repr(C, packed)]
pub struct KeysEasyzone {
    #[hidraw_constant = "0x0b"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u16"]
    _size: u16,
    pub profile_index: u8,
    pub keys: [ButtonConfig; 96], // Just key mappings
    #[hidraw_bytesum]
    _bytesum: u16,
}

impl KeysEasyzone {
    pub fn new(profile_index: u8, keys: [ButtonConfig; 96]) -> Self {
        Self {
            profile_index,
            keys,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl Default for KeysEasyzone {
    fn default() -> Self {
        Self {
            profile_index: Default::default(),
            keys: DEFAULT_KEYS_EASYZONE,
            ..unsafe { std::mem::uninitialized() }
        }
    }
}

impl fmt::Debug for KeysEasyzone {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let size = self._size;
        let bytesum = self._bytesum;
        fmt.debug_struct("KeysEasyzone")
            .field("_report_id", &self._report_id)
            .field("_size", &size)
            .field("profile_index", &self.profile_index)
            .field("keys", &&self.keys[..])
            .field("_bytesum", &bytesum)
            .finish()
    }
}
