use errors::*;
use libroccat::device::ryosmkfx::EventKeyAction as RyosMkFxEventKeyAction;
use libroccat::device::ryosmkfx::EventType as RyosMkFxEventType;
use libroccat;
use rlua::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

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
        methods.add_function("find_devices", |lua, ()| -> LuaResult<LuaTable> {
            let table = lua.create_table();
            for (i, device) in libroccat::find_devices().unwrap().into_iter().enumerate() {
                match device {
                    libroccat::Device::RyosMkFx(device) => table.set(i + 1, RyosMkFx(device))?,
                    libroccat::Device::Tyon(device) => table.set(i + 1, Tyon(device))?,
                };
            }
            Ok(table)
        });
    }
}

struct RyosMkFx(libroccat::device::RyosMkFx);

impl LuaUserData for RyosMkFx {
    fn add_methods(methods: &mut LuaUserDataMethods<Self>) {
        methods.add_method("name", |_, _, ()| Ok("ryos_mk_fx"));

        methods.add_method("get_event", |_, this, ()| loop {

            loop {
                if let Some(event) = this.0.get_event() {
                    if event.type_ == RyosMkFxEventType::Effect {
                        return Ok((
                            event.sdk_index(),
                            unsafe { event.action.key } == RyosMkFxEventKeyAction::Press,
                        ));
                    }
                }
            }
        });

        methods.add_method("get_event_timed", |_, this, timeout| {
            let start = ::std::time::Instant::now();
            let duration = ::std::time::Duration::from_millis(timeout);

            while start.elapsed() < duration {
                if let Some(event) = this.0.get_event() {
                    if event.type_ == RyosMkFxEventType::Effect {
                        return Ok((
                            Some(event.sdk_index()),
                            Some(unsafe { event.action.key } == RyosMkFxEventKeyAction::Press),
                        ));
                    }
                }
            }

            return Ok((None, None));
        });

        methods.add_method(
            "get_profile",
            |_, this, ()| Ok(this.0.get_profile().unwrap()),
        );

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

            let mut lights = this.0.get_lights(profile).unwrap();
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
            use libroccat::device::ryosmkfx::{CustomLights, LightLayer};

            let mut data = this.0.get_custom_lights().unwrap().light_layer.get_data();

            for i in 0..120 {
                if let Ok(key_table) = table.get(i): LuaResult<LuaTable> {
                    data.set_key_state(i, key_table.get("state")?);
                    data.set_key_red(i, key_table.get("red")?);
                    data.set_key_green(i, key_table.get("green")?);
                    data.set_key_blue(i, key_table.get("blue")?);
                }
            }

            this.0
                .set_custom_lights(&CustomLights::new(LightLayer::from_data(&data), 0))
                .unwrap();

            Ok(())
        });
    }
}

struct Tyon(libroccat::device::Tyon);

impl LuaUserData for Tyon {
    fn add_methods(methods: &mut LuaUserDataMethods<Self>) {
        methods.add_method("name", |_, _, ()| Ok("tyon"));
    }
}
