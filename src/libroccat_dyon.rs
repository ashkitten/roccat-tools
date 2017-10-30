use libroccat;
use libroccat::device::Device;
use std::sync::Arc;

dyon_fn! {
    fn num_devices() -> f64 {
        libroccat::find_devices().unwrap_or_default().len() as f64
    }
}

dyon_fn! {
    fn device_type(device_index: f64) -> String {
        libroccat::find_devices().unwrap_or_default()[device_index as usize].get_common_name().to_string()
    }
}

dyon_fn! {
    fn get_profile(device_index: f64) -> f64 {
        match libroccat::find_devices().unwrap()[device_index as usize] {
            Device::RyosMkFx(ref device) => device.get_profile().unwrap() as f64,
            Device::Tyon(ref device) => device.get_profile().unwrap() as f64,
        }
    }
}

dyon_fn! {
    fn set_profile(device_index: f64, profile: f64) {
        match libroccat::find_devices().unwrap()[device_index as usize] {
            Device::RyosMkFx(ref device) => device.set_profile(profile as u8).unwrap(),
            Device::Tyon(ref device) => device.set_profile(profile as u8).unwrap(),
        }
    }
}

pub fn run_dyon(path: &str) {
    use dyon::{error, Runtime};

    let mut runtime = Runtime::new();
    let module = {
        use dyon::*;

        let mut module = Module::new();

        module.add(Arc::new("roccat_num_devices".into()), num_devices, Dfn {
            lts: vec![],
            tys: vec![],
            ret: Type::F64,
        });

        module.add(Arc::new("roccat_device_type".into()), device_type, Dfn {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::Text,
        });

        module.add(Arc::new("roccat_get_profile".into()), get_profile, Dfn {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::F64,
        });

        module.add(Arc::new("roccat_set_profile".into()), set_profile, Dfn {
            lts: vec![Lt::Default, Lt::Default],
            tys: vec![Type::F64, Type::F64],
            ret: Type::Void,
        });

        if error(load(path, &mut module)) {
            None
        } else {
            Some(module)
        }
    };

    if error(runtime.run(&Arc::new(module.unwrap()))) {
        return
    }
}
