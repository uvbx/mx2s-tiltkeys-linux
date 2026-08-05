use evdev::{ AttributeSet, EventType, InputEvent, Key, RelativeAxisType};
use evdev::uinput::VirtualDeviceBuilder;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

fn main() -> Result < (), Box<dyn Error> > {

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
                println!("connected to mouse");
                return dev;
            }

            sleep(Duration::from_secs(2));
        }
    }

    let mut device = get_dev();

    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_VOLUMEDOWN);
    keys.insert(Key::KEY_VOLUMEUP);

    let mut virtdev = VirtualDeviceBuilder::new()?
        .name("Wheel_Tilt_Keys")
        .with_keys(&keys)?
        .build()?;

    println!("spawned virtual keyboard");

    println!("listening for keypresses");
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
            println!("mouse slept or disconnected. waiting for reconnected");
            device = get_dev();
        }

    }
}