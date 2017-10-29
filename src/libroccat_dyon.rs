use libroccat;
use std::sync::Arc;

dyon_fn! {
    fn num_devices() -> f64 {
        libroccat::find_devices().unwrap_or_default().len() as f64
    }
}

dyon_fn! {
    fn device_type(i: f64) -> String {
        libroccat::find_devices().unwrap_or_default()[i as usize].get_common_name().to_string()
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
