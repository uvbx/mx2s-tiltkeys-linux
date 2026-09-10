use evdev::{ AttributeSet, EventType, InputEvent, Key, RelativeAxisType };
use evdev::uinput::VirtualDeviceBuilder;

use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use std::path::PathBuf;
use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    version: String,
    left_key: String,
    right_key: String,
}

fn config_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap()).join(".config/mx2s-tiltkeys/config.json")
}

fn check_config() -> Result<Config, Box<dyn Error>> {
    let path: PathBuf = config_path();

    if !path.exists() {
        let default_config = r#"{
    "version": "1.2",
    "left_key": "KEY_VOLUMEUP",
    "right_key": "KEY_VOLUMEDOWN"
}
"#;

        println!("no config found! generating default\n========================================\n{}========================================", &default_config);

        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, default_config)?;
    }

    let config_content: String = fs::read_to_string(&path)?;
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}



fn get_dev() -> evdev::Device {
    loop {
        let target_device: Option<evdev::Device> = evdev::enumerate().find_map(|(_, device)| {
            let name: &str = device.name()?;

            if name.contains("MX Anywhere") {
                Some(device)
            } else {
                None
            }

        });

        if let Some(dev) = target_device {
            return dev;
        }

        sleep(Duration::from_secs(3));
    }
}


fn main() -> Result <(), Box<dyn Error>> {


    let config: Config = check_config()?;

    println!("{}", config.left_key);
    let left_key: Key = config.left_key.parse().map_err(|e| format!("invalid left key set {:?}", e))?;
    let right_key: Key = config.right_key.parse().map_err(|e| format!("invalid left key set {:?}", e))?;

    println!("version: {} \nleft: {} \nright: {}", config.version, config.left_key, config.right_key);

    let mut device: evdev::Device = get_dev();

    let mut keys = AttributeSet::<Key>::new();
    keys.insert(left_key);
    keys.insert(right_key);

    let mut virtdev = VirtualDeviceBuilder::new()?
        .name("Wheel_Tilt_Keys")
        .with_keys(&keys)?
        .build()?;

    loop {
        let disconnected: bool = match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    if event.event_type() == EventType::RELATIVE && event.code() == RelativeAxisType::REL_HWHEEL.0 {
                        let key = if event.value() > 0 {
                            left_key
                        } else {
                            right_key
                        };

                        virtdev.emit(&[
                            InputEvent::new(EventType::KEY, key.code(),1),
                            InputEvent::new(EventType::KEY, key.code(), 0)
                        ])?;
                    }
                }
                false
            }
            Err(_) => true,
        };

        if disconnected {
            println!("mx2s disconnected. waiting for reconnection.");
            device = get_dev();
        }

    }
}
