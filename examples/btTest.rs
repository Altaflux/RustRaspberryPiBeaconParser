extern crate rand;
extern crate rumble;

//use rand::{thread_rng, Rng};
use rumble::api::{Central, CentralEvent, EventHandler, Peripheral, UUID};
use rumble::bluez::manager::Manager;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const UUID_SIZE:usize = 16;
// The minimum size of manufacturer data we are interested in. This consists of:
// manufacturer(2), code(2), uuid(16), major(2), minor(2), calibrated power(1)
const MIN_MANUFACTURER_DATA_SIZE: usize = 2 + 2 + UUID_SIZE + 2 + 2 + 1;

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();
    println!("Post adapter");
    // reset the adapter -- clears out any errant state
    //adapter = manager.down(&adapter).unwrap();
    //adapter = manager.up(&adapter).unwrap();
    println!("Post up/down");
    // connect to the adapter
    let central = adapter.connect().unwrap();
    central.active(false);
    central.filter_duplicates(false);
    let ar_central = Arc::new(Mutex::new(central));


    let central_2 = ar_central.clone();
    let central_3 = ar_central.clone();
    let handler: EventHandler = Box::new(move |ev| {
        println!("Event type: {:?}", ev);
        match ev {
            CentralEvent::DeviceDiscovered(addr)
            | CentralEvent::DeviceUpdated(addr) => {
                let guard = central_3.lock().unwrap();
                let per = guard.peripheral(addr).unwrap();
                std::mem::drop(guard);
                println!("");
                println!("------------------------------------------------");
                println!("Device found: {:?}", per);
                println!("");
                //println!("Props: {:?}", per.properties());

                let data = match per.properties().manufacturer_data {
                    Some(data) => data,
                    _ => return
                };
                if data.len() < MIN_MANUFACTURER_DATA_SIZE {
                    println!("Size not enough: {:?}", data.len());
                    return;
                }
                parse_beacon_info(&data);
            }
            _ => {}
        }
    });
    central_2.lock().unwrap().on_event(handler);

    for i in 1..10 {
        println!("loop");
        thread::sleep(Duration::from_secs(5));
    }


}


fn parse_beacon_info(data: &Vec<u8>) {

    let manufacturer = 256 * data[0] as i32 + data[1] as i32;
    let code = 256 * data[2] as i32 + data[3] as i32;

    let mut index:usize = 4;
    use uuid::{Builder};
    let uuid = Builder::from_slice(&data[4.. 4 + UUID_SIZE]);
    index =  index + UUID_SIZE;

    println!("manufacturer: {:?}", manufacturer);
    println!("code: {:?}", code);
    println!("uuid: {:?}", uuid);
    println!("uuid u8: {:?}", &data[4.. 4 + UUID_SIZE]);

    let m0 = data[index];
    let m1 = data[index + 1];
    index = index + 2;
    let major = 256 * m0 as u16 + m1 as u16;

    let m0 = data[index];
    let m1 = data[index + 1];
    index = index + 2;
    let minor = 256 * m0 as u16 + m1 as u16;

    println!("major: {:?}", major);
    println!("minor: {:?}", minor);

    let calibrated_power = data[index] as i32 - 256;
    println!("calibrated_power: {:?}", calibrated_power);
}
