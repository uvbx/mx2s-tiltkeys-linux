use evdev::{ AttributeSet, EventType, InputEvent, Key, RelativeAxisType };
use evdev::uinput::VirtualDeviceBuilder;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;


fn get_dev() -> evdev::Device {
    loop {
        let target_device = evdev::enumerate().find_map(|(_, device)| {
            let name = device.name()?;

            if name.contains("MX Anywhere 2S") {
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

    
    let mut device = get_dev();

    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_VOLUMEDOWN);
    keys.insert(Key::KEY_VOLUMEUP);

    let mut virtdev = VirtualDeviceBuilder::new()?
        .name("Wheel_Tilt_Keys")
        .with_keys(&keys)?
        .build()?;
        
    loop {

        let disconnected = match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    if event.event_type() == EventType::RELATIVE && event.code() == RelativeAxisType::REL_HWHEEL.0 {
                        let key = if event.value() > 0 {
                            Key::KEY_VOLUMEUP
                        } else {
                            Key::KEY_VOLUMEDOWN
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