extern crate blurz;
extern crate simple_error;

use simple_error::SimpleError;
use std::error::Error;
use std::thread;
use std::time::Duration;
use uuid::Builder;
use uuid::Uuid;
use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
use blurz::bluetooth_discovery_session::DiscoveryTransport;
use blurz::bluetooth_session::BluetoothSession as Session;

const UUID_SIZE: usize = 16;
// The minimum size of manufacturer data we are interested in. This consists of:
// manufacturer(2), code(2), uuid(16), major(2), minor(2), calibrated power(1)
const MIN_MANUFACTURER_DATA_SIZE: usize = 2 + UUID_SIZE + 2 + 2 + 1;

fn test3() -> Result<(), Box<Error>> {
    let bt_session = &Session::create_session(None)?;

    let adapter: Adapter = (Adapter::init(bt_session))?;
    (adapter.set_powered(true))?;

    let session = match DiscoverySession::create_session(&bt_session, adapter.get_id()) {
        Ok(session) => session,
        Err(err) => {
            println!("Failure create session");
            println!("{:?}", err);
            return Err(err);
        }
    };
    match session.set_discovery_filter(vec![], None, None, Some(DiscoveryTransport::LE)) {
        Ok(_) => {}
        Err(err) => {
            println!("Failure set_discovery_filter");
            println!("{:?}", err);
            return Err(err);
        }
    }

    println!("filter: {:?}", session.get_discovery_filters());

    match session.start_discovery() {
        Ok(_) => {}
        Err(err) => {
            println!("Failure start discovery");
            println!("{:?}", err);
            session.stop_discovery()?;
            return Err(err);
        }
    };
    loop {

        thread::sleep(Duration::from_millis(800));
        let devices = match adapter.get_device_list() {
            Ok(devices) => devices,
            Err(err) => {
                println!("Failure get device list");
                println!("{:?}", err);
                continue;
            }
        };

        println!("{} device(s) found", devices.len());
        'device_loop: for d in devices {
            let device = Device::new(bt_session, d.clone());
            match build_device(&device) {
                Ok(beacon) => { process_beacon(&beacon, &device);},
                Err(e) => {}
            }
            (adapter.remove_device(device.get_id()))?;
        }
    };

    match session.stop_discovery() {
        Ok(_) => {}
        Err(err) => {
            println!("Failure stop discovery");
            println!("{:?}", err);
            match session.stop_discovery() {
                Ok(_) => {}
                Err(err) => {

                    println!("SECOND Failure stop discovery");
                    println!("SECOND {:?}", err);
                }
            }
        }
    };
}

fn process_beacon(beacon: &SimpleBeacon, device: &Device) {

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
        //println!("manufacturer D: {:?}", manufacturer_data.1);
    }
}

fn build_device(device: &Device) -> Result<SimpleBeacon, Box<Error>>{
    let tmp = device.get_manufacturer_data();
    let manufacturer_data = match tmp {
        Ok(ref data) => {
            match data.get(&(76)) {
                Some(man_data) => {
                    if man_data.len() < MIN_MANUFACTURER_DATA_SIZE {
                        return Err(Box::new(SimpleError::new("Min data size")));
                    }
                    (76, man_data)
                }
                None => {return Err(Box::new(SimpleError::new("Manufacterer data 76 not found")));},
            }
        }
        Err(_) => {return Err(Box::new(SimpleError::new("Manufacterer data not found")));},
    };
    match parse_beacon_info(manufacturer_data.0, manufacturer_data.1) {
        Ok(beacon) => Ok(beacon),
        Err(_) => Err(Box::new(SimpleError::new("Failed to parse info")))
    }
}

fn main() {
    match test3() {
        Ok(_) => (),
        Err(e) => {
            println!("IN THE ERROR");
            println!("{:?}", e);
        }
    }
}
#[derive(Debug)]
struct SimpleBeacon {
    uuid: Uuid,
    minor: u16,
    major: u16,
    calibrated_power: i32
}

fn parse_beacon_info(manufacturer: u16, data: &Vec<u8>) -> Result<SimpleBeacon, Box<Error>> {
    //let manufacturer = 256 * data[0] as i32 + data[1] as i32;
    let code = 256 * data[1] as i32; // + data[2] as i32;

    let mut index: usize = 2;

    let mut uuid = match Builder::from_slice(&data[index..index + UUID_SIZE]) {
        Ok(uuid) => uuid,
        Err(e) => return Err(Box::new(e))
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

    Ok(SimpleBeacon {
        uuid:  uuid.build(),
        minor: minor,
        major: major,
        calibrated_power: calibrated_power
    })
}
