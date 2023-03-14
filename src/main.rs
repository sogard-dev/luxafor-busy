use rusb::{DeviceHandle, GlobalContext};
use std::env;

const VID: u16 = 0x04D8;
const PID: u16 = 0xF372;

const USB_OUT: u8 = 0x01;
const USB_IN: u8 = 0x81;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("1 argument expected, one of: red, green, yellow or blue");
        return;
    }

    let query = &args[1][..];

    let mut device = rusb::open_device_with_vid_pid(VID, PID).unwrap();
    let iface: u8 = 0;

    device.claim_interface(iface).unwrap();

    match query {
        "red" => switch_to_red(&mut device),
        "green" => switch_to_green(&mut device),
        "yellow" => switch_to_yellow(&mut device),
        "blue" => switch_to_blue(&mut device),
        _ => {
            println!("Argument '{}' not supported", query);
        }
    }

    device.release_interface(iface).unwrap();
}

fn switch_to_red(device: &mut DeviceHandle<GlobalContext>) {
    switch_to(device, get_red_message());
}

fn switch_to_yellow(device: &mut DeviceHandle<GlobalContext>) {
    switch_to(device, get_yellow_message());
}

fn switch_to_green(device: &mut DeviceHandle<GlobalContext>) {
    switch_to(device, get_green_message());
}

fn switch_to_blue(device: &mut DeviceHandle<GlobalContext>) {
    switch_to(device, get_blue_message());
}

fn switch_to(device: &mut DeviceHandle<GlobalContext>, buf: [u8; 8]) {
    write_to_device(device, get_first_message());
    read_from_device(device);

    write_to_device(device, buf);
    read_from_device(device);
}

fn write_to_device(device: &mut DeviceHandle<GlobalContext>, buf: [u8; 8]) {
    device
        .write_interrupt(USB_OUT, &buf, std::time::Duration::from_millis(500))
        .unwrap();
}

fn read_from_device(device: &mut DeviceHandle<GlobalContext>) {
    let mut rbuf = [0u8; 8];
    device
        .read_interrupt(USB_IN, &mut rbuf, std::time::Duration::from_millis(500))
        .unwrap();
}

fn get_green_message() -> [u8; 8] {
    let mut buf = get_color_message();
    buf[3] = 0x80;

    buf
}

fn get_red_message() -> [u8; 8] {
    let mut buf = get_color_message();
    buf[2] = 0x80;

    buf
}

fn get_blue_message() -> [u8; 8] {
    let mut buf = get_color_message();
    buf[4] = 0x80;

    buf
}

fn get_yellow_message() -> [u8; 8] {
    let mut buf = get_color_message();
    buf[2] = 0x80;
    buf[3] = 0x80;

    buf
}

fn get_color_message() -> [u8; 8] {
    let mut buf = [0u8; 8];
    buf[0] = 0x01;
    buf[1] = 0xff;

    buf
}

fn get_first_message() -> [u8; 8] {
    let mut buf = [0u8; 8];
    buf[0] = 0x80;

    buf
}

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x} ", byte))?;
        }
        Ok(())
    }
}
