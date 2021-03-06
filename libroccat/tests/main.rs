use libroccat::device::Device;

#[test]
fn ryosmkfx() {
    for device in libroccat::find_devices().unwrap() {
        if let Device::RyosMkFx(device) = device {
            use libroccat::device::ryosmkfx::*;

            println!("Found Ryos MK FX");

            println!("Profile index: {}", device.get_profile().unwrap());

            println!(
                "Firmware version: {}",
                device.get_info().unwrap().firmware_version
            );

            for i in 0..5 {
                let lights = device.get_lights(i).unwrap();
                println!(
                    "Light effect for profile {}: {:?}",
                    lights.profile, lights.effect
                );
            }

            let active = device.get_custom_lights_active().unwrap();
            println!("Custom lights active: {}", active);

            device.set_custom_lights_active(true).unwrap();
            let mut data = LightLayerData::default();
            data.set_all_states(true);
            for i in 0..120 {
                data.set_key_red(i, 0xff);
                data.set_key_green(i, (f32::from(i) * 255.0 / 120.0).min(255.0) as u8);
                data.set_key_blue(i, (f32::from(i) * 255.0 / 120.0).min(255.0) as u8);
                device
                    .set_custom_lights(&CustomLights::new(LightLayer::from_data(&data)))
                    .unwrap();
            }
            for i in 0..10 {
                data.set_all_states(i % 2 == 0);
                device
                    .set_custom_lights(&CustomLights::new(LightLayer::from_data(&data)))
                    .unwrap();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            device.set_custom_lights_active(active).unwrap();

            println!("Press any key to end test");
            println!("{:?}", device.get_event());
        }
    }
}

#[test]
fn tyon() {
    for device in libroccat::find_devices().unwrap() {
        if let Device::Tyon(device) = device {
            println!("Found Tyon");
            println!("Profile index: {}", device.get_profile().unwrap());
        }
    }
}

#[test]
fn hardware_color() {
    use libroccat::device::ryosmkfx::HardwareColor;

    for color in 0..255 {
        let hardware = HardwareColor::from_color(color);
        assert_eq!(hardware.to_color(), color);
    }
}
