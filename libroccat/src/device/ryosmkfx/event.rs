use super::effect_index_to_sdk_index;
use std::fmt;

pub struct Event {
    _report_id: u8,
    pub subtype: EventSubtype,
    pub type_: EventType,
    pub data: u8,
    pub action: EventAction,
}

impl Event {
    pub fn sdk_index(&self) -> u8 {
        effect_index_to_sdk_index(self.data)
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Event")
            .field("_report_id", &self._report_id)
            .field(
                "subtype",
                match self.type_ {
                    EventType::Rad => unsafe { &self.subtype.rad },
                    _ => unsafe { &self.subtype.none },
                },
            )
            .field("type_", &self.type_)
            .field("data", &self.data)
            .field(
                "action",
                match self.type_ {
                    EventType::Effect => unsafe { &self.action.key },
                    EventType::LiveRecording => unsafe { &self.action.live_recording },
                    _ => unsafe { &self.action.none },
                },
            )
            .finish()
    }
}

#[derive(Copy, Clone)]
pub union EventSubtype {
    pub none: u8,
    pub rad: EventRadSubtype,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum EventRadSubtype {
    W = 0x01,
    A = 0x02,
    S = 0x03,
    D = 0x04,
    Thumbster1 = 0x05,
    Thumbster2 = 0x06,
    Thumbster3 = 0x07,
    Easyshift = 0x08,
    Multimedia = 0x09,
    M1 = 0x0a,
    M2 = 0x0b,
    M3 = 0x0c,
    M4 = 0x0d,
    M5 = 0x0e,
    MacroShortcut = 0x0f,
    Talk = 0x10,
    MacroLifeRec = 0x11,
    Backlight = 0x12,
    Total = 0x13,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum EventType {
    Unknown = 0x00,
    ProfileStart = 0x01,
    Profile = 0x02,
    Macro = 0x03,
    LiveRecording = 0x04,
    Quicklaunch = 0x07,
    Easyshift = 0xa, // TODO: confirm
    Multimedia = 0x0b,
    Backlight = 0x0c,
    TimerStart = 0x0d,
    TimerStop = 0x0e,
    OpenDriver = 0x10,
    LedMacro = 0xbf,
    Rad = 0xfa,
    Effect = 0xfb,
    Layer = 0xfc,
    EasyshiftSelf = 0xfd,
    Talk = 0xff,
}

#[derive(Copy, Clone)]
pub union EventAction {
    pub none: u8,
    pub key: EventKeyAction,
    pub live_recording: EventLiveRecordingAction,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum EventKeyAction {
    Release = 0x00,
    Press = 0x01,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum EventLiveRecordingAction {
    Start = 0x01,
    MacroKeySelected = 0x02,
    EndSuccess = 0x03,
    EndAbort = 0x04,
    InvalidKey = 0x05,
}
