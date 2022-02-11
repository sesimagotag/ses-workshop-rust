use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use rusb::{Context, Device, HotplugBuilder, Registration, UsbContext};

struct HotPlugHandler;

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        let device_desc = device.device_descriptor().unwrap();
        let s = format!("{:03}_{:03}_{:04x}_{:04x}",
                        device.bus_number(),
                        device.address(),
                        device_desc.vendor_id(),
                        device_desc.product_id());
        println!("device arrived {}", s);
    }

    fn device_left(&mut self, device: Device<T>) {
        let device_desc = device.device_descriptor().unwrap();
        let s = format!("{:03}_{:03}_{:04x}_{:04x}",
                        device.bus_number(),
                        device.address(),
                        device_desc.vendor_id(),
                        device_desc.product_id());
        println!("device left {}", s);
    }
}

impl Drop for HotPlugHandler {
    fn drop(&mut self) {
        println!("HotPlugHandler dropped");
    }
}

pub fn hotplug_loop() {
    if rusb::has_hotplug() {
        let context = match Context::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("usb hotplug failed: {}", e);
                return;
            }
        };

        let _reg: Option<Registration<Context>> = Some(
            match HotplugBuilder::new()
                .enumerate(true)
                .register(&context, Box::new(HotPlugHandler {}))
            {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("usb hotplug failed: {}", e);
                    return;
                }
            }
        );

        loop {
            context.handle_events(None).unwrap();
            let sec = Duration::from_secs_f32(0.5);
            thread::sleep(sec);
        }
    } else {
        eprintln!("libusb hotplug api unsupported");
        let mut list: HashSet<String> = HashSet::new();

        loop {
            let mut current_list: Vec<String> = Vec::new();
            for device in rusb::devices().unwrap().iter() {
                let device_desc = device.device_descriptor().unwrap();
                let s = format!("{:03}_{:03}_{:04x}_{:04x}",
                                device.bus_number(),
                                device.address(),
                                device_desc.vendor_id(),
                                device_desc.product_id());
                current_list.push(s);
            }

            for item in &current_list {
                if !list.contains(item) {
                    list.insert(item.clone());
                    println!("device arrived {}", item);
                }
            }

            let mut remove_list: Vec<String> = Vec::new();
            for item in &list {
                if !current_list.contains(item) {
                    remove_list.push(item.clone());
                }
            }
            for item in remove_list {
                list.remove(&item);
                println!("device left {}", item);
            }

            let sec = Duration::from_secs_f32(1.0);
            thread::sleep(sec);
        }
    }
}

fn main() {
    hotplug_loop();
}
