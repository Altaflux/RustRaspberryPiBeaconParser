use super::Beacon;
use super::EventHandler;
use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
use blurz::bluetooth_session::BluetoothSession as Session;
use simple_error::SimpleError;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use uuid::Builder;
use uuid::Uuid;

const UUID_SIZE: usize = 16;
// The minimum size of manufacturer data we are interested in. This consists of:
// manufacturer(2), code(2), uuid(16), major(2), minor(2), calibrated power(1)
const MIN_MANUFACTURER_DATA_SIZE: usize = 2 + UUID_SIZE + 2 + 2 + 1;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
pub struct BlurzListener<'a> {
    should_stop: Arc<AtomicBool>,
    receiver: Option<mpsc::Receiver<()>>,
    msg_sender: Option<EventHandler>,
    session: &'a Session,
    adapter: Adapter<'a>,
}

#[derive(Debug)]
struct SimpleBeacon {
    uuid: Uuid,
    minor: u16,
    major: u16,
    calibrated_power: i32,
}


impl<'a> BlurzListener<'a> {
    pub fn new(
        session: &'a Session,
        should_stop: Arc<AtomicBool>,
    ) -> Result<BlurzListener<'a>, Box<Error>> {
        let adapter = (Adapter::init(&session))?;
        Ok(BlurzListener {
            should_stop: should_stop,
            receiver: None,
            msg_sender: None,
            session: &session,
            adapter: adapter,
        })
    }

    pub fn work(&mut self) {
        let discovery_session =
            match DiscoverySession::create_session(self.session, self.adapter.get_id()) {
                Ok(discovery_session) => discovery_session,
                Err(err) => {
                    println!("Failed to get discovery session: {:}", err);
                    return;
                }
            };
        match discovery_session.start_discovery() {
            Ok(_) => {}
            Err(err) => {
                println!("Failure start discovery: {:?}", err);
                discovery_session.stop_discovery().unwrap();
                return;
            }
        };

        while !self.should_stop.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(800));
            let devices = match self.adapter.get_device_list() {
                Ok(devices) => devices,
                Err(err) => {
                    println!("Failure get device list: {:?}", err);
                    continue;
                }
            };
            println!("{} device(s) found", devices.len());
            'device_loop: for d in devices {
                let device = Device::new(self.session, d.clone());
                match build_device(&device) {
                    Ok(beacon) => {
                        self.process_beacon(beacon, &device);
                    }
                    Err(_) => {}
                }
                match self.adapter.remove_device(device.get_id()) {
                    Err(err) => println!(
                        "Could not remove device: {:?}, reason: {:?}",
                        device.get_id(),
                        err
                    ),
                    _ => {}
                };
            }
        }
        println!("Shutting down");
        match discovery_session.stop_discovery() {
            Ok(_) => {}
            Err(err) => {
                println!("Failure stop discovery");
                println!("{:?}", err);
                match discovery_session.stop_discovery() {
                    Ok(_) => {}
                    Err(err) => {
                        println!("SECOND Failure stop discovery");
                        println!("SECOND {:?}", err);
                    }
                }
            }
        };
    }

    fn process_beacon(&self, beacon: Beacon, device: &Device) {
        if beacon.major == 85 {
            println!("--------");
            println!(
                "id: {} addr: {:?} rssi: {:?} txP: {:?}",
                device.get_id(),
                device.get_address(),
                device.get_rssi(),
                device.get_tx_power()
            );

            println!("ALL PROPS D: {:?}", device.get_all_properties());
            println!("Beacon Info: {:?}", beacon);
            match &self.msg_sender {
                Some(handler) => {
                    handler(beacon);
                }
                None => {}
            };
        }
    }
}

fn build_device(device: &Device) -> Result<Beacon, Box<Error>> {
    let manufacturer_data = device.get_manufacturer_data()?;
    let rssi = device.get_rssi()?;
    let manufacturer_data = match manufacturer_data.get(&(76)) {
        Some(man_data) => {
            if man_data.len() < MIN_MANUFACTURER_DATA_SIZE {
                return Err(Box::new(SimpleError::new("Min data size")));
            }
            (76, man_data)
        }
        None => {
            return Err(Box::new(SimpleError::new("Manufacterer data 76 not found")));
        }
    };
    match parse_beacon_info(manufacturer_data.0, manufacturer_data.1, rssi) {
        Ok(beacon) => Ok(beacon),
        Err(_) => Err(Box::new(SimpleError::new("Failed to parse info"))),
    }
}

fn parse_beacon_info(manufacturer: u16, data: &Vec<u8>, rssi: i16) -> Result<Beacon, Box<Error>> {
    //let manufacturer = 256 * data[0] as i32 + data[1] as i32;
    let code = 256 * data[1] as i32; // + data[2] as i32;

    let mut index: usize = 2;

    let mut uuid = match Builder::from_slice(&data[index..index + UUID_SIZE]) {
        Ok(uuid) => uuid,
        Err(e) => return Err(Box::new(e)),
    };
    index = index + UUID_SIZE;

    // println!("manufacturer: {:?}", manufacturer);
    // println!("code: {:?}", code);
    // println!("uuid: {:?}", uuid);
    // println!("uuid u8: {:?}", &data[2..2 + UUID_SIZE]);

    let m0 = data[index];
    let m1 = data[index + 1];
    index = index + 2;
    let major = 256 * m0 as u16 + m1 as u16;

    let m0 = data[index];
    let m1 = data[index + 1];
    index = index + 2;
    let minor = 256 * m0 as u16 + m1 as u16;

    // println!("major: {:?}", major);
    // println!("minor: {:?}", minor);

    let calibrated_power = data[index] as i32 - 256;
    //    println!("calibrated_power: {:?}", calibrated_power);

    Ok(Beacon {
        scanner_id: "sdf".to_owned(),
        scanner_sequence_no: 0,
        manufacturer: manufacturer as i32,
        uuid: format!("{:?}", uuid.build()),
        code: code,
        rssi: rssi as i32,
        minor: minor as i32,
        major: major as i32,
        message_type: 1,
        time: SystemTime::now(),
        power: 0,
        calibrated_power: calibrated_power,
    })
}
