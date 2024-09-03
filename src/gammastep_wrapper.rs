use configparser::ini::Ini;
use expanduser::expanduser;
use std::fs;
use std::process::Command;

const CONFIG_PATH: &str = "~/.config/gammastep/config.ini";
const DEFAULT_CONFIG: &str = r###"
[general]
temp-day= 6500
temp-night=6500
brightness-day=1
brightness-night=1
location-provider=manual
fade=0

[manual]
lat=0
lon=0
"###;

pub struct Gammastep {
    pub config: Ini,
    pub changes_made: bool,
}

impl Gammastep {
    pub fn new() -> Self {
        let new_config_path = expanduser(CONFIG_PATH).expect("Error expanding the user");
        let mut slf = Self {
            config: Ini::new(),
            changes_made: true,
        };
        if !new_config_path.try_exists().unwrap() {
            fs::write(&new_config_path, DEFAULT_CONFIG).expect("Error writing to config file");
            slf.config
                .read(DEFAULT_CONFIG.to_string())
                .expect("Error loading the config file");
        } else {
            slf.config
                .load(new_config_path)
                .expect("Error loading the config file");
        }
        slf.restart_gammastep();
        slf
    }

    pub fn current_state(&self) -> String {
        format!(
            "{{\"tooltip\": \"\", \"percentage\": {}}}",
            self.get_current_brightness()
        )
    }

    pub fn update(&mut self, value: u8, add: bool) -> String {
        let current_brightness = self.get_current_brightness();
        let new_brightness = if add {
            (current_brightness + value).min(100)
        } else {
            (current_brightness as i16 - value as i16).max(10) as u8
        } as f32
            / 100.0;

        self.config.set(
            "general",
            "brightness-day",
            Some(new_brightness.to_string()),
        );
        self.config.set(
            "general",
            "brightness-night",
            Some(new_brightness.to_string()),
        );
        if current_brightness != self.get_current_brightness() {
            self.changes_made = true;
        }
        self.current_state()
    }

    pub fn restart_gammastep(&mut self) {
        if !self.changes_made {
            return;
        }
        let new_config_path = expanduser(CONFIG_PATH).expect("Error expanding the user");
        self.config
            .write(new_config_path)
            .expect("Error writing to config file");
        let process_name = "gammastep";
        Command::new("pkill").arg(process_name).status().unwrap();
        Command::new(process_name).spawn().unwrap();
        self.changes_made = false;
    }

    fn get_current_brightness(&self) -> u8 {
        (self
            .config
            .getfloat("general", "brightness-day")
            .unwrap()
            .unwrap()
            * 100.0) as u8
    }
}
