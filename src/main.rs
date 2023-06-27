use std::{sync::Arc, time::Duration, u8};

use libusb::{self, Context, Device, DeviceHandle};

enum Style {
    STATIC = 0x01,
    BREATHE = 0x02,
    WAVE = 0x03,
    FLASH = 0x12,
}

#[derive(Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

enum Brightness {
    OFF = 0x0,
    VERY_DIM = 0x08,
    DIM = 0x16,
    BRIGHT = 0x24,
    VERY_BRIGHT = 0x32,
}

enum Speed {
    VERY_SLOW = 0x0a,
    SLOW = 0x07,
    MEDIUM = 0x05,
    FAST = 0x03,
    VERY_FAST = 0x01,
}

fn main() {
    let context = Context::new().unwrap();

    let keyboard = get_keyboard(&context).unwrap();

    println!("{}", keyboard.device_descriptor().unwrap().product_id());

    let device = keyboard.open().unwrap();

    let color: Color = Color {
        red: 0,
        green: 255,
        blue: 0,
    };

    set_mono_color(&device, color, Brightness::VERY_BRIGHT);
}

fn get_keyboard(context: &Context) -> Result<Arc<Device>, String> {
    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == 0x048d && device_desc.product_id() == 0xce00 {
            let device_arc = Arc::new(device);
            return Ok(device_arc);
        }
    }

    Err("Device not found!".to_owned())
}

fn transfer_color(handle: &DeviceHandle, color: &Color, idx: u8) {
    let colors = vec![
        0x14,
        0x00,
        idx,
        color.red,
        color.green,
        color.blue,
        0x00,
        0x00,
    ];
    transfer_message(handle, colors);
}

fn transfer_message(handle: &DeviceHandle, msg: Vec<u8>) {
    let res = handle.write_control(0x21, 9, 0x0300, 1, &msg, Duration::from_secs(1));

    match res {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e.strerror()),
    }
}

fn set_static_style(handle: &DeviceHandle, colors: &Vec<Color>, brightness: Brightness) {
    set_color_palette(handle, colors);

    let msg = vec![
        0x08,
        0x02,
        Style::STATIC as u8,
        0x00,
        brightness as u8,
        0x08,
        0x00,
        0x01,
    ];
    transfer_message(handle, msg)
}

fn set_wave_style(handle: &DeviceHandle, colors: &Vec<Color>, speed: Speed, brightness: Brightness) {
    set_color_palette(handle, colors);

    transfer_message(handle, vec![0x08, 0x02, Style::WAVE as u8, speed as u8, brightness as u8, 0x08, 0x00, 0x01]);
}

fn set_mono_color(handle: &DeviceHandle, color: Color, brightness: Brightness) {
    set_static_style(handle, &vec![color, color, color, color], brightness)
}

fn set_color_palette(handle: &DeviceHandle, colors: &Vec<Color>) {
    let mut i = 1;

    for color in colors.iter() {
        transfer_color(handle, color, i);
        i+=1;
    }
}
