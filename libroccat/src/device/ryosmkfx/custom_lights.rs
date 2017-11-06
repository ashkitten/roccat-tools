use bitfield::*;
use std::fmt;

use errors::*;
use super::hardware_color::*;
use super::sdk::*;

#[derive(Clone)]
#[repr(packed)]
pub struct LightLayer {
    states: [u8; 15],
    numbers_red: [u8; 60],
    numbers_green: [u8; 60],
    numbers_blue: [u8; 60],
    colors_red_pwm: [u8; 7],
    colors_green_pwm: [u8; 7],
    colors_blue_pwm: [u8; 7],
    colors_red_brightness: [u8; 4],
    colors_green_brightness: [u8; 4],
    colors_blue_brightness: [u8; 4],
}

// TODO: Waiting for goddamn RFC 2000
impl Default for LightLayer {
    fn default() -> Self {
        Self {
            states: Default::default(),
            numbers_red: [Default::default(); 60],
            numbers_green: [Default::default(); 60],
            numbers_blue: [Default::default(); 60],
            colors_red_pwm: Default::default(),
            colors_green_pwm: Default::default(),
            colors_blue_pwm: Default::default(),
            colors_red_brightness: Default::default(),
            colors_green_brightness: Default::default(),
            colors_blue_brightness: Default::default(),
        }
    }
}

impl fmt::Debug for LightLayer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LightLayer")
            .field(
                "states",
                &self.states
                    .iter()
                    .flat_map(|b| {
                        vec![
                            b.get_bit(0),
                            b.get_bit(1),
                            b.get_bit(2),
                            b.get_bit(3),
                            b.get_bit(4),
                            b.get_bit(5),
                            b.get_bit(6),
                            b.get_bit(7),
                        ]
                    })
                    .collect::<Vec<bool>>(),
            )
            .field(
                "numbers_red",
                &self.numbers_red
                    .iter()
                    .flat_map(|b| vec![b.get_nibble(0), b.get_nibble(1)])
                    .collect::<Vec<u8>>(),
            )
            .field(
                "numbers_green",
                &self.numbers_green
                    .iter()
                    .flat_map(|b| vec![b.get_nibble(0), b.get_nibble(1)])
                    .collect::<Vec<u8>>(),
            )
            .field(
                "numbers_blue",
                &self.numbers_blue
                    .iter()
                    .flat_map(|b| vec![b.get_nibble(0), b.get_nibble(1)])
                    .collect::<Vec<u8>>(),
            )
            .field("colors_red_pwm", &self.colors_red_pwm)
            .field("colors_green_pwm", &self.colors_green_pwm)
            .field("colors_blue_pwm", &self.colors_blue_pwm)
            .field(
                "colors_red_brightness",
                &self.colors_red_brightness
                    .iter()
                    .flat_map(|b| vec![b.get_nibble(0), b.get_nibble(1)])
                    .collect::<Vec<u8>>(),
            )
            .field(
                "colors_green_brightness",
                &self.colors_green_brightness
                    .iter()
                    .flat_map(|b| vec![b.get_nibble(0), b.get_nibble(1)])
                    .collect::<Vec<u8>>(),
            )
            .field(
                "colors_blue_brightness",
                &self.colors_blue_brightness
                    .iter()
                    .flat_map(|b| vec![b.get_nibble(0), b.get_nibble(1)])
                    .collect::<Vec<u8>>(),
            )
            .finish()
    }
}

impl LightLayer {
    fn get_red(&self, color_number: usize) -> u8 {
        if color_number == 0 {
            return 0;
        }

        HardwareColor {
            brightness: self.colors_red_brightness.get_nibble(color_number - 1),
            pwm: self.colors_red_pwm[color_number - 1],
        }.to_color()
    }

    fn set_red(&mut self, color_number: usize, color: u8) {
        let hardware = HardwareColor::from_color(color);
        self.colors_red_pwm[color_number - 1] = hardware.pwm;
        self.colors_red_brightness
            .set_nibble(color_number - 1, hardware.brightness & 0x0f);
    }

    fn get_green(&self, color_number: usize) -> u8 {
        if color_number == 0 {
            return 0;
        }

        HardwareColor {
            brightness: self.colors_green_brightness.get_nibble(color_number - 1),
            pwm: self.colors_green_pwm[color_number - 1],
        }.to_color()
    }

    fn set_green(&mut self, color_number: usize, color: u8) {
        let hardware = HardwareColor::from_color(color);
        self.colors_green_pwm[color_number - 1] = hardware.pwm;
        self.colors_green_brightness
            .set_nibble(color_number - 1, hardware.brightness & 0x0f);
    }

    fn get_blue(&self, color_number: usize) -> u8 {
        if color_number == 0 {
            return 0;
        }

        HardwareColor {
            brightness: self.colors_blue_brightness.get_nibble(color_number - 1),
            pwm: self.colors_blue_pwm[color_number - 1],
        }.to_color()
    }

    fn set_blue(&mut self, color_number: usize, color: u8) {
        let hardware = HardwareColor::from_color(color);
        self.colors_blue_pwm[color_number - 1] = hardware.pwm;
        self.colors_blue_brightness
            .set_nibble(color_number - 1, hardware.brightness & 0x0f);
    }

    pub fn set_data(&mut self, data: &LightLayerData) {
        let mut values_red = [0u8; 120];
        let mut values_green = [0u8; 120];
        let mut values_blue = [0u8; 120];
        let mut means_red = [0u8; 7];
        let mut means_green = [0u8; 7];
        let mut means_blue = [0u8; 7];
        let mut cluster_red = [u8::max_value(); 120];
        let mut cluster_green = [u8::max_value(); 120];
        let mut cluster_blue = [u8::max_value(); 120];

        for i in 0..120 {
            values_red[i] = data.keys[i].red;
            values_green[i] = data.keys[i].green;
            values_blue[i] = data.keys[i].blue;
        }

        init_means(values_red, &mut means_red, data);
        while set_cluster(values_red, means_red, &mut cluster_red, data) {
            update_means(values_red, &mut means_red, cluster_red, data);
        }

        init_means(values_green, &mut means_green, data);
        while set_cluster(values_green, means_green, &mut cluster_green, data) {
            update_means(values_green, &mut means_green, cluster_green, data);
        }

        init_means(values_blue, &mut means_blue, data);
        while set_cluster(values_blue, means_blue, &mut cluster_blue, data) {
            update_means(values_blue, &mut means_blue, cluster_blue, data);
        }

        for i in 0..7 {
            self.set_red(i + 1, means_red[i]);
            self.set_green(i + 1, means_green[i]);
            self.set_blue(i + 1, means_blue[i]);
        }

        for i in 0..120 {
            self.numbers_red
                .set_nibble(i, ((cluster_red[i] as usize + 1) & 0x0f) as u8);
            self.numbers_green
                .set_nibble(i, ((cluster_green[i] as usize + 1) & 0x0f) as u8);
            self.numbers_blue
                .set_nibble(i, ((cluster_blue[i] as usize + 1) & 0x0f) as u8);

            self.states.set_bit(i, data.keys[i].state);
        }
    }

    pub fn get_data(&self) -> LightLayerData {
        let mut data = LightLayerData::default();

        for i in 0..120 {
            data.keys[i].state = self.states.get_bit(i);
            data.keys[i].red = self.get_red(self.numbers_red.get_nibble(i) as usize);
            data.keys[i].green = self.get_green(self.numbers_green.get_nibble(i) as usize);
            data.keys[i].blue = self.get_blue(self.numbers_blue.get_nibble(i) as usize);
        }

        data
    }

    pub fn from_data(data: &LightLayerData) -> Self {
        let mut ret = Self::default();
        ret.set_data(data);
        ret
    }
}

fn get_unique_values(values: [u8; 120], data: &LightLayerData) -> Vec<u8> {
    use std::collections::BTreeSet;

    let mut set = BTreeSet::new();

    for i in 0..values.len() {
        if data.keys[i].state {
            set.insert(values[i]);
        }
    }

    set.iter().map(|i| *i).collect()
}

fn init_means(values: [u8; 120], means: &mut [u8; 7], data: &LightLayerData) {
    let unique_values = get_unique_values(values, data);
    for i in 0..usize::min(unique_values.len(), 7) {
        means[i] = unique_values[i]
    }
}

fn update_means(values: [u8; 120], means: &mut [u8; 7], cluster: [u8; 120], data: &LightLayerData) {
    let mut mean_sums = [0usize; 7];
    let mut counts = [0usize; 7];

    for i in 0..120 {
        if !data.keys[i].state {
            continue;
        }

        mean_sums[cluster[i] as usize] += values[i] as usize;
        counts[cluster[i] as usize] += 1;
    }

    for i in 0..7 {
        if counts[i] > 0 {
            means[i] = (mean_sums[i] / counts[i]) as u8;
        }
    }
}

fn set_cluster(
    values: [u8; 120],
    means: [u8; 7],
    cluster: &mut [u8; 120],
    data: &LightLayerData,
) -> bool {
    let mut smallest_cluster = 0u8;
    let mut changed = false;

    for i in 0..120 {
        if !data.keys[i].state {
            continue;
        }

        let mut smallest_error = isize::max_value();
        for j in 0..7 {
            let error = (values[i] as isize - means[j] as isize).abs();
            if error < smallest_error {
                smallest_error = error;
                smallest_cluster = j as u8;
            }
        }

        if cluster[i] != smallest_cluster {
            cluster[i] = smallest_cluster;
            changed = true;
        }
    }

    changed
}

#[derive(Copy, Clone, Default, Debug)]
pub struct LightLayerKey {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub state: bool,
}

pub struct LightLayerData {
    keys: [LightLayerKey; 256],
}

impl Default for LightLayerData {
    fn default() -> Self {
        Self {
            keys: [Default::default(); 256],
        }
    }
}

impl fmt::Debug for LightLayerData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LightLayerData")
            .field("keys", &&self.keys[..])
            .finish()
    }
}

impl LightLayerData {
    pub fn set_all_states(&mut self, state: bool) {
        for key in self.keys.iter_mut() {
            key.state = state;
        }
    }

    pub fn get_key_state(&self, sdk: u8) -> bool {
        self.keys[sdk_index_to_light_index(sdk) as usize].state
    }

    pub fn set_key_state(&mut self, sdk: u8, state: bool) {
        self.keys[sdk_index_to_light_index(sdk) as usize].state = state;
    }

    pub fn get_key_red(&self, sdk: u8) -> u8 {
        self.keys[sdk_index_to_light_index(sdk) as usize].red
    }

    pub fn set_key_red(&mut self, sdk: u8, red: u8) {
        self.keys[sdk_index_to_light_index(sdk) as usize].red = red;
    }

    pub fn get_key_green(&self, sdk: u8) -> u8 {
        self.keys[sdk_index_to_light_index(sdk) as usize].green
    }

    pub fn set_key_green(&mut self, sdk: u8, green: u8) {
        self.keys[sdk_index_to_light_index(sdk) as usize].green = green;
    }

    pub fn get_key_blue(&self, sdk: u8) -> u8 {
        self.keys[sdk_index_to_light_index(sdk) as usize].blue
    }

    pub fn set_key_blue(&mut self, sdk: u8, blue: u8) {
        self.keys[sdk_index_to_light_index(sdk) as usize].blue = blue;
    }
}

#[derive(HidrawRead, HidrawWrite, Clone)]
#[repr(C, packed)]
pub struct CustomLights {
    #[hidraw_constant = "0x18"]
    _report_id: u8,
    #[hidraw_constant = "::std::mem::size_of::<Self>() as u8"]
    _size: u8,
    light_layer: LightLayer,
    pub bytesum: u16,
}
