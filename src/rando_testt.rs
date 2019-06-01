extern crate rand;
extern crate rumble;

//use rand::{thread_rng, Rng};
use rumble::api::{Central, CentralEvent, EventHandler, Peripheral, UUID};
use rumble::bluez::manager::Manager;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();
    println!("Post adapter");
    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();
    println!("Post up/down");
    // connect to the adapter
    let central = adapter.connect().unwrap();

    let ar_central = Arc::new(Mutex::new(central));
    //    println!("Post connect: {:?}", central.clone().lock());
    // start scanning for devices

    // instead of waiting, you can use central.on_event to be notified of
    // new devices

    let central_2 = ar_central.clone();
    let central_3 = ar_central.clone();
    let handler: EventHandler = Box::new(move |ev| {
        match ev {
            CentralEvent::DeviceDiscovered(addr)
            | CentralEvent::DeviceUpdated(addr) => {
                let guard = central_3.lock().unwrap();
                let per = guard.peripheral(addr).unwrap();
                std::mem::drop(guard);
                println!("Device found: {:?}", per);

                println!("Props: {:?}", per.properties());
                // match per.connect() {
                //     Ok(_) => {
                //         for car in per.discover_characteristics().into_iter() {
                //             println!("Characteristics: {:?}", car);
                //         }
                //         println!("\n");
                //     }
                //     Err(err) => {
                //         println!("Connect failed: {:?}", err);
                //     }
                // }

            }
            _ => {}
        }
    });
    central_2.lock().unwrap().on_event(handler);
    //
    for i in 1..10 {
        println!("loop");
        thread::sleep(Duration::from_secs(5));
    }

    // find the device we're interested in
    // for light in central.peripherals().into_iter() {
    //     println!("Device: {:?}", light);
    //     println!("");
    //     let props = light.properties();
    //     println!("Props: {:?}", props);
    //     for car in light.discover_characteristics().into_iter() {
    //         println!("Characteristics: {:?}", car);
    //     }
    //     println!("");
    //     println!("\n");
    // }
}
