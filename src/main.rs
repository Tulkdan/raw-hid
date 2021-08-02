extern crate hidapi;

use hidapi::{DeviceInfo, HidApi};

const VENDOR_ID: u16 = 0x4B50; // "KP"
const PRODUCT_ID: u16 = 0x3430; // "40"

fn is_my_device(device: &DeviceInfo) -> bool {
    device.vendor_id() == VENDOR_ID
        && device.product_id() == PRODUCT_ID
}

fn main() {
    let api = HidApi::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let device = api.device_list()
        .find(|device| is_my_device(device))
        .unwrap_or_else(|| {
            eprintln!("Couldn't find keyboard");
            std::process::exit(1);
        })
        .open_device(&api)
        .unwrap_or_else(|e| {
            eprintln!("Couldn't open HID device: {}", e);
            std::process::exit(1);
        });

    let res = device.write(&[1, 0]).unwrap_or_else(|e| {
        eprintln!("Couldn't write into keyboard: {}", e);
        std::process::exit(1);
    });
    println!("Wrote {} byte(s)", res);
}
