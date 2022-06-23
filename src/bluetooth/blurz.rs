use super::Beacon;
use super::EventHandler;
use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
use blurz::bluetooth_session::BluetoothSession as Session;
use simple_error::SimpleError;
use std::error::Error;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uuid::Builder;
 use anyhow::Error as AnyhowError;

const UUID_SIZE: usize = 16;
// The minimum size of manufacturer data we are interested in. This consists of:
// manufacturer(2), code(2), uuid(16), major(2), minor(2), calibrated power(1)
const MIN_MANUFACTURER_DATA_SIZE: usize = 2 + UUID_SIZE + 2 + 2 + 1;
use std::time::{SystemTime};
pub struct BlurzListener<'a> {
    should_stop: Arc<AtomicBool>,
    session: &'a Session,
    adapter: Adapter<'a>,
}


impl<'a> BlurzListener<'a> {
    pub fn new(
        session: &'a Session,
        should_stop: Arc<AtomicBool>,
    ) -> Result<BlurzListener<'a>, Box<dyn Error>> {
        let adapter = (Adapter::init(&session))?;
        Ok(BlurzListener {
            should_stop: should_stop,
            session: &session,
            adapter: adapter,
        })
    }


    pub fn work2(&self, handler: EventHandler) -> Result<(), Box<dyn Error>> {

        let discovery_session = DiscoverySession::create_session(self.session, self.adapter.get_id())?;
        discovery_session.start_discovery()?;

        while !self.should_stop.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(1100));
            
            let devices = self.adapter.get_device_list()?;
            
            for device_entry in devices {
                let device = Device::new(self.session, device_entry);

                if let Ok(beacon) = build_device(&device) {
                    self.process_beacon(beacon, &device, &handler);
                }
                if let Err(err) = self.adapter.remove_device(device.get_id()) {
                    println!("Could not remove device: {:?}, reason: {:?}",
                        device.get_id(),
                        err)
                }
            }
           
        }

        discovery_session.stop_discovery()?;
        Ok(())
    }


    fn process_beacon(&self, beacon: Beacon, device: &Device, handler: &EventHandler) {
        if beacon.major == 85 {
            println!("--------");
            println!(
                "id: {} addr: {:?} rssi: {:?} txP: {:?}",
                device.get_id(),
                device.get_address(),
                device.get_rssi(),
                device.get_tx_power()
            );
            
           // println!("ALL PROPS D: {:?}", device.get_all_properties());
            println!("Beacon Info: {:?}", beacon);
            handler(beacon);
        }
    }
}

fn build_device(device: &Device) -> Result<Beacon, Box<dyn Error>> {
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

fn parse_beacon_info(manufacturer: u16, data: &Vec<u8>, rssi: i16) -> Result<Beacon, Box<dyn Error>> {

    let code = 256 * data[1] as i32; // + data[2] as i32;

    let mut index: usize = 2;

    let uuid = match Builder::from_slice(&data[index..index + UUID_SIZE]) {
        Ok(uuid) => uuid,
        Err(e) => return Err(Box::new(e)),
    };
    index = index + UUID_SIZE;


    let m0 = data[index];
    let m1 = data[index + 1];
    index = index + 2;
    let major = 256 * m0 as u16 + m1 as u16;

    let m0 = data[index];
    let m1 = data[index + 1];
    index = index + 2;
    let minor = 256 * m0 as u16 + m1 as u16;

    let calibrated_power = data[index] as i32 - 256;

    Ok(Beacon {
        scanner_id: "sdf".to_owned(),
        scanner_sequence_no: 0,
        manufacturer: manufacturer as i32,
        uuid: format!("{:?}", uuid.as_uuid()),
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
