use std::io;
use std::io::Write;

use vulkano as vk;
use vk::instance::{Instance, InstanceExtensions, PhysicalDevice};

/// Reads a single line after prompting the player.
pub fn read(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("unable to prompt user");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to read user input");

    return input.trim().to_string();
}

pub fn select_device(instance: &Instance) -> Option<device> {
    // get all the devices
    let devices = PhysicalDevice::enumerate(&instance).to_vec();

    // easy choice
    if devices.is_empty() {
        println!("No devices detected >:(");
        return None;
    } else if devices.len() == 1 {
        println!("Using only device present");
        return Some(devices[1]);
    }

    // leave the choice up to the user
    println!("Detected the following devices:");
    for (i, device) in devices.iter().enumerate() {
        println!("({}) {}", i, device);
    }

    // not really forgiving haha
    let number = usize::from(read("Which device would you like to use: "));
    let device = devices.get(number);
    device
}

pub fn run() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");

    let device = select_device(&instance);
}
