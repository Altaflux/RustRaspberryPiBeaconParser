

const HCIDUMP_PREFIX:i32 = 7;
const BADDR_SIZE:i32 = 6;
const UUID_SIZE:i32 = 16;

use std::time::{Duration, SystemTime};
   
pub struct Beacon {
    scanner_id: String,
    uuid: String,
    scanner_sequence_no: i32,
    code: i32,
    manufacturer: i32,
    major: i32,
    minor: i32,
    power: i32,
    calibrated_power: i32,
    rssi: i32,
    message_type: i32,
    time: SystemTime,
}
pub fn parseHCIDump(scanner_id: &str, packet: String) {
    let size = packet.len();

    let mut index = 2 + HCIDUMP_PREFIX * 3;
    index += BADDR_SIZE * 3;
    index += 3;

    let mut length = mystoi(&packet[index as usize..2], 16);
    let mut type_ = mystoi(&packet[index as usize + 3..2], 16);

    loop {
        if type_ != 0xFF && (index as usize) < (size -1) {
            break;
        }

        index += 3 * (length + 1);
        length = mystoi(&packet[index as usize..2], 16);
        type_ = mystoi(&packet[index as usize + 3..2], 16);   
    }

    if index as usize >= size {
        panic!("Input packet has no manufacturer specific data");
    }

    index += 6;
    let mut manufacturer = 256 * mystoi(&packet[index as usize..2], 16);
    index += 3;
    manufacturer += mystoi(&packet[index as usize..2], 16);
    index += 3;

    let code0 = &packet[index as usize..2];
    index += 3;

    let code1 = &packet[index as usize..2];
    index += 3;

    let code = 256 * mystoi(code0, 16) + mystoi(code1, 16);
    let uuid: Vec<char> = packet[index as usize..index as usize + UUID_SIZE as usize]
        .chars()
        .filter(| s| *s == ' ')
        .collect();

    index += UUID_SIZE * 3;

    let major0 = &packet[index as usize..2];
    index += 3;
    let major1 = &packet[index as usize..2];
    index += 3;
    let imajor = 256 * mystoi(major0, 16) + mystoi(major1, 16);


    let minor0 = &packet[index as usize..2];
    index += 3;
    let minor1 = &packet[index as usize..2];
    index += 3;
    let iminor = 256 * mystoi(minor0, 16) + mystoi(minor1, 16);

    let power = &packet[index as usize..2];
    index += 3;
    let mut ipower = mystoi(power, 16);
    ipower -= 256;

    let rssi = &packet[index as usize..2];
    let mut irssi = mystoi(rssi, 16);
    irssi -= 256;

     let now = SystemTime::now();

    Beacon{
        scanner_id: scanner_id.to_owned(),
        uuid: uuid.into_iter().collect(), 
        scanner_sequence_no: 0, 
        code: code,
        manufacturer: manufacturer, 
        major: imajor, 
        minor: iminor,
        power: 0,
        calibrated_power: ipower,
        rssi: irssi,
        time: now,
        message_type: 0
        };
}

pub fn mystoi(packet: &str, base: u32) -> i32 {
    return i32::from_str_radix(packet, base).unwrap_or(0);
}