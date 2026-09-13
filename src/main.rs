use evdev::{ Device, AttributeSet, EventType, InputEvent, Key, RelativeAxisType };
use evdev::uinput::VirtualDeviceBuilder;

use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;

use serde::Deserialize;

const OUTPUT_ENABLED: bool = true;

#[derive(Deserialize)]
struct Config {
    version: String,
    left_key: String,
    right_key: String,
}

fn config_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap()).join(".config/mx2s-tiltkeys/config.json")
}

fn check_config() -> Config {
    let path: PathBuf = config_path();

    if !path.exists() {
        let default_config = r#"{
    "version": "1.3",
    "left_key": "KEY_VOLUMEUP",
    "right_key": "KEY_VOLUMEDOWN"
}
"#;

        println!("no config found! generating default\n========================================\n{}========================================", &default_config);

        fs::create_dir_all(path.parent().unwrap());
        fs::write(&path, default_config);
    }

    let config_content: String = fs::read_to_string(&path).expect("unable to generate config content");
    let config: Config = serde_json::from_str(&config_content).expect("serde config issue");
    config
}

fn get_dev() -> Device {
    loop {
        let target_device: Option<Device> = evdev::enumerate().find_map(|(_, device)| {
            let name: &str = device.name().expect("");

            if name.contains("MX Anywhere 2s") {
                Some(device)
            } else {
                None
            }

        });

        if let Some(dev) = target_device {
            return dev;
        }
        sleep(Duration::from_secs(10));
    }
}

fn main() {
    let config: Config = check_config();

    let left_key: Key = config.left_key.parse().expect("unable to parse left key");
    let right_key: Key = config.right_key.parse().expect("unable to parse right key");

    let mut device: Device= get_dev();
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(left_key);
    keys.insert(right_key);

    let mut virtdev = VirtualDeviceBuilder::new().expect("unable to build virtual device")
        .name("wheel_tilt_keys")
        .with_keys(&keys).expect("unable to assign keys (virtual device builder)")
        .build().expect("unable to build virtual device");
    loop {
        let disconnected: bool = match device.fetch_events() {
            Ok(events) => {
                for e in events {
                    if e.event_type() == EventType::RELATIVE && e.code() == RelativeAxisType::REL_HWHEEL.0{
                        let key = if e.value() > 0 {
                            left_key
                        } else {
                            right_key
                        };

                        virtdev.emit(&[
                            InputEvent::new(EventType::KEY, key.code(), 1),
                            InputEvent::new(EventType::KEY, key.code(), 0)
                        ]);
                    }
                }
                false
            }
            Err(_) => true,
        };

        if disconnected {
            if OUTPUT_ENABLED {
                println!("mx2s disconnected. waiting for reconnection");
            }
            device = get_dev();

        }
    }
}
