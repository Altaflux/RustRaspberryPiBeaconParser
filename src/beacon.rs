

const HCIDUMP_PREFIX:i32 = 7;
const BADDR_SIZE:i32 = 6;
const UUID_SIZE:i32 = 16;

use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};
use bytebuffer::ByteBuffer;

const VERSION:i32 = 5;

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

pub fn fromByteMessage(msg: &[u8], length: usize) -> Result<Beacon, ()> {
    let mut dis = ByteBuffer::from_bytes(&msg[0..length]);

    let version = dis.read_i32();
    let scanner_id = dis.read_string();

    let uuid = dis.read_string();
    let code = dis.read_i32();
    let manufacturer = dis.read_i32();
    let major = dis.read_i32();
    let minor = dis.read_i32();
    let power = dis.read_i32();
    let calibrated_power = dis.read_i32();
    let rssi = dis.read_i32();
    let message_type = dis.read_i32();
    let heartbeat  = dis.read_i32();
    let time = dis.read_u64();


    Ok(Beacon{
        scanner_id: scanner_id,
        uuid: uuid, 
        scanner_sequence_no: 0, 
        code: code,
        manufacturer: manufacturer, 
        major: major, 
        minor: minor,
        power: power,
        calibrated_power: calibrated_power,
        rssi: rssi,
        time: match UNIX_EPOCH.checked_add(Duration::from_millis(time as u64)) {
            Some(name) => name,
            None => UNIX_EPOCH,
        },
        message_type: message_type
    })

}

impl Beacon {
    pub fn toByteMessage(&self) -> Vec<u8> {
        let mut dos = ByteBuffer::new();
        dos.write_i32(VERSION);
        dos.write_bytes(self.scanner_id.as_bytes());
        dos.write_bytes(self.uuid.as_bytes());
        dos.write_i32(self.code);
        dos.write_i32(self.manufacturer);
        dos.write_i32(self.major);
        dos.write_i32(self.minor);
        dos.write_i32(self.power);
        dos.write_i32(self.calibrated_power);
        dos.write_i32(self.rssi);
        dos.write_u64(match self.time.duration_since(UNIX_EPOCH) {
            Ok(name) => name.as_millis() as u64,
            Err(_) => 0 as u64,
        });
        dos.write_i32(self.message_type);
        return dos.read_bytes(dos.len());
    }
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