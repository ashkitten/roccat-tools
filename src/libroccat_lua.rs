use errors::*;
use libroccat;
use rlua::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std;

pub fn run_script(path: &str) -> Result<()> {
    let lua = Lua::new();
    lua.globals().set("libroccat", Libroccat)?;

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let file_name = if let Some(file_name) = Path::new(path).file_name() {
        file_name.to_str()
    } else {
        None
    };

    lua.exec::<()>(&contents, file_name)?;

    Ok(())
}

struct Libroccat;

impl LuaUserData for Libroccat {
    fn add_methods(methods: &mut LuaUserDataMethods<Self>) {
        methods.add_function("find_devices", |lua, ()| {
            let table = lua.create_table();
            for (i, device) in libroccat::find_devices().unwrap().into_iter().enumerate() {
                match device {
                    libroccat::device::Device::RyosMkFx(device) => {
                        table.set(i + 1, RyosMkFx(device))?
                    }
                    libroccat::device::Device::Tyon(device) => table.set(i + 1, Tyon(device))?,
                };
            }
            Ok(table)
        });

        methods.add_function("sleep", |_, time| {
            std::thread::sleep(std::time::Duration::from_millis(time));
            Ok(())
        });
    }
}

struct RyosMkFx(libroccat::device::ryosmkfx::RyosMkFx);

impl RyosMkFx {
    fn get_event_table<'lua>(&self, lua: &'lua Lua) -> LuaResult<Option<LuaTable<'lua>>> {
        use libroccat::device::ryosmkfx::{EventKeyAction, EventLiveRecordingAction,
                                          EventRadSubtype, EventType};

        if let Some(event) = self.0.get_event() {
            let table = lua.create_table();
            if event.type_ == EventType::Rad {
                table.set(
                    "subtype",
                    match unsafe { event.subtype.rad } {
                        EventRadSubtype::W => "w",
                        EventRadSubtype::A => "a",
                        EventRadSubtype::S => "s",
                        EventRadSubtype::D => "d",
                        EventRadSubtype::Thumbster1 => "thumbster_1",
                        EventRadSubtype::Thumbster2 => "thumbster_2",
                        EventRadSubtype::Thumbster3 => "thumbster_3",
                        EventRadSubtype::Easyshift => "easyshift",
                        EventRadSubtype::Multimedia => "multimedia",
                        EventRadSubtype::M1 => "m1",
                        EventRadSubtype::M2 => "m2",
                        EventRadSubtype::M3 => "m3",
                        EventRadSubtype::M4 => "m4",
                        EventRadSubtype::M5 => "m5",
                        EventRadSubtype::MacroShortcut => "macro_shortcut",
                        EventRadSubtype::Talk => "talk",
                        EventRadSubtype::MacroLifeRec => "macro_life_rec",
                        EventRadSubtype::Backlight => "backlight",
                        EventRadSubtype::Total => "total",
                    },
                )?;
            }
            table.set(
                "type",
                match event.type_ {
                    EventType::Unknown => "unknown",
                    EventType::ProfileStart => "profile_start",
                    EventType::Profile => "profile",
                    EventType::Macro => "macro",
                    EventType::LiveRecording => "live_recording",
                    EventType::Quicklaunch => "quicklaunch",
                    EventType::Easyshift => "easyshift",
                    EventType::Multimedia => "multimedia",
                    EventType::Backlight => "backlight",
                    EventType::TimerStart => "timer_start",
                    EventType::TimerStop => "timer_stop",
                    EventType::OpenDriver => "open_driver",
                    EventType::LedMacro => "led_macro",
                    EventType::Rad => "rad",
                    EventType::Effect => "effect",
                    EventType::Layer => "layer",
                    EventType::EasyshiftSelf => "easyshift_self",
                    EventType::Talk => "talk",
                },
            )?;
            table.set(
                "data",
                match event.type_ {
                    EventType::Effect => event.sdk_index(),
                    _ => event.data,
                },
            )?;
            if event.type_ == EventType::Effect || event.type_ == EventType::LiveRecording {
                table.set(
                    "action",
                    match event.type_ {
                        EventType::Effect => match unsafe { event.action.key } {
                            EventKeyAction::Release => "release",
                            EventKeyAction::Press => "press",
                        },
                        EventType::LiveRecording => {
                            match unsafe { event.action.live_recording } {
                                EventLiveRecordingAction::Start => "start",
                                EventLiveRecordingAction::MacroKeySelected => "macro_key_selected",
                                EventLiveRecordingAction::EndSuccess => "end_success",
                                EventLiveRecordingAction::EndAbort => "end_abort",
                                EventLiveRecordingAction::InvalidKey => "invalid_key",
                            }
                        }
                        _ => unreachable!(),
                    },
                )?;
            }
            return Ok(Some(table));
        }
        Ok(None)
    }
}

impl LuaUserData for RyosMkFx {
    fn add_methods(methods: &mut LuaUserDataMethods<Self>) {
        methods.add_method("name", |_, _, ()| Ok("ryos_mk_fx"));

        methods.add_method("get_event", |lua, this, ()| loop {
            if let Some(table) = this.get_event_table(lua)? {
                return Ok(Some(table));
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        });

        methods.add_method("get_event_timed", |lua, this, timeout| {
            let start = std::time::Instant::now();
            let duration = std::time::Duration::from_millis(timeout);

            while start.elapsed() < duration {
                if let Some(table) = this.get_event_table(lua)? {
                    return Ok(Some(table));
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            Ok(None)
        });

        methods.add_method("get_event_immediate", |lua, this, ()| {
            if let Some(table) = this.get_event_table(lua)? {
                Ok(Some(table))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_profile", |_, this, ()| {
            Ok(this.0.get_profile().unwrap())
        });

        methods.add_method("set_profile", |_, this, profile| {
            this.0.set_profile(profile).unwrap();
            Ok(())
        });

        methods.add_method("get_lights", |lua, this, profile| {
            use libroccat::device::ryosmkfx::*;

            let lights = this.0.get_lights(profile).unwrap();
            let table = lua.create_table();
            table.set("brightness", lights.brightness)?;
            table.set("dimness", lights.dimness)?;
            table.set("timeout", lights.timeout)?;
            table.set(
                "mode",
                match lights.mode {
                    LightMode::Plain => "plain",
                    LightMode::Layer => "layer",
                },
            )?;
            table.set(
                "effect",
                match lights.effect {
                    LightEffect::Off => "off",
                    LightEffect::FullyLit => "fully_lit",
                    LightEffect::Blinking => "blinking",
                    LightEffect::Breathing => "breathing",
                    LightEffect::Heartbeat => "heartbeat",
                    LightEffect::Equalizer => "equalizer",
                    LightEffect::Ripple => "ripple",
                    LightEffect::Wave => "wave",
                    LightEffect::Heatmap => "heatmap",
                    LightEffect::GamePreset => "game_preset",
                    LightEffect::Fade => "fade",
                },
            )?;
            table.set("effect_speed", lights.effect_speed)?;
            table.set(
                "led_feedback",
                match lights.led_feedback {
                    LightLedFeedback::Off => "off",
                    LightLedFeedback::MacroExecution => "macro_execution",
                },
            )?;
            table.set(
                "dimness_type",
                match lights.dimness_type {
                    LightDimnessType::Off => "off",
                    LightDimnessType::StarlitSky => "starlit_sky",
                    LightDimnessType::FallAsleep => "fall_asleep",
                },
            )?;
            table.set("red", lights.red)?;
            table.set("green", lights.green)?;
            table.set("blue", lights.blue)?;

            Ok(table)
        });

        methods.add_method("set_lights", |_, this, (profile, table): (u8, LuaTable)| {
            use libroccat::device::ryosmkfx::*;

            let mut lights = Lights::default();
            lights.profile = profile;

            if let Ok(brightness) = table.get("brightness") {
                lights.brightness = brightness;
            }
            if let Ok(dimness) = table.get("dimness") {
                lights.dimness = dimness;
            }
            if let Ok(timeout) = table.get("timeout") {
                lights.timeout = timeout;
            }
            if let Ok(mode) = table.get("mode"): LuaResult<String> {
                lights.mode = match &mode as &str {
                    "plain" => LightMode::Plain,
                    "layer" => LightMode::Layer,
                    _ => {
                        return Err(LuaError::FromLuaConversionError {
                            from: "table",
                            to: "Lights",
                            message: Some("Invalid value for field 'mode'".to_string()),
                        })
                    }
                };
            }
            if let Ok(effect) = table.get("effect"): LuaResult<String> {
                lights.effect = match &effect as &str {
                    "off" => LightEffect::Off,
                    "fully_lit" => LightEffect::FullyLit,
                    "blinking" => LightEffect::Blinking,
                    "breathing" => LightEffect::Breathing,
                    "heartbeat" => LightEffect::Heartbeat,
                    "equalizer" => LightEffect::Equalizer,
                    "ripple" => LightEffect::Ripple,
                    "wave" => LightEffect::Wave,
                    "heatmap" => LightEffect::Heatmap,
                    "game_preset" => LightEffect::GamePreset,
                    "fade" => LightEffect::Fade,
                    _ => {
                        return Err(LuaError::FromLuaConversionError {
                            from: "table",
                            to: "Lights",
                            message: Some("Invalid value for field 'effect'".to_string()),
                        })
                    }
                };
            }
            if let Ok(effect_speed) = table.get("effect_speed") {
                lights.effect_speed = effect_speed;
            }
            if let Ok(led_feedback) = table.get("led_feedback"): LuaResult<String> {
                lights.led_feedback = match &led_feedback as &str {
                    "off" => LightLedFeedback::Off,
                    "macro_execution" => LightLedFeedback::MacroExecution,
                    _ => {
                        return Err(LuaError::FromLuaConversionError {
                            from: "table",
                            to: "Lights",
                            message: Some("Invalid value for field 'led_feedback'".to_string()),
                        })
                    }
                }
            }
            if let Ok(dimness_type) = table.get("dimness_type"): LuaResult<String> {
                lights.dimness_type = match &dimness_type as &str {
                    "off" => LightDimnessType::Off,
                    "starlit_sky" => LightDimnessType::StarlitSky,
                    "fall_asleep" => LightDimnessType::FallAsleep,
                    _ => {
                        return Err(LuaError::FromLuaConversionError {
                            from: "table",
                            to: "Lights",
                            message: Some("Invalid value for field 'dimness_type'".to_string()),
                        })
                    }
                }
            }
            if let Ok(red) = table.get("red") {
                lights.red = red;
            }
            if let Ok(green) = table.get("green") {
                lights.green = green;
            }
            if let Ok(blue) = table.get("blue") {
                lights.blue = blue;
            }
            this.0.set_lights(&lights).unwrap();

            Ok(())
        });

        methods.add_method("set_custom_lights_active", |_, this, active| {
            this.0.set_custom_lights_active(active).unwrap();
            Ok(())
        });

        methods.add_method("get_custom_lights", |lua, this, ()| {
            let data = this.0.get_custom_lights().unwrap().light_layer.get_data();
            let table = lua.create_table();

            for i in 0..120 {
                let key_table = lua.create_table();

                key_table.set("state", data.get_key_state(i))?;
                key_table.set("red", data.get_key_red(i))?;
                key_table.set("green", data.get_key_green(i))?;
                key_table.set("blue", data.get_key_blue(i))?;

                table.set(i, key_table)?;
            }

            Ok(table)
        });

        methods.add_method("set_custom_lights", |_, this, table: LuaTable| {
            use libroccat::device::ryosmkfx::{CustomLights, LightLayer, LightLayerData};

            let mut data = LightLayerData::default();

            for i in 0..120 {
                if let Ok(key_table) = table.get(i): LuaResult<LuaTable> {
                    data.set_key_state(i, key_table.get("state")?);
                    data.set_key_red(i, key_table.get("red")?);
                    data.set_key_green(i, key_table.get("green")?);
                    data.set_key_blue(i, key_table.get("blue")?);
                }
            }

            this.0
                .set_custom_lights(&CustomLights::new(LightLayer::from_data(&data)))
                .unwrap();

            Ok(())
        });
    }
}

struct Tyon(libroccat::device::tyon::Tyon);

impl LuaUserData for Tyon {
    fn add_methods(methods: &mut LuaUserDataMethods<Self>) {
        methods.add_method("name", |_, _, ()| Ok("tyon"));

        methods.add_method("get_profile", |_, this, ()| {
            Ok(this.0.get_profile().unwrap())
        });

        methods.add_method("set_profile", |_, this, profile| {
            this.0.set_profile(profile).unwrap();
            Ok(())
        });
    }
}
