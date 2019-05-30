extern crate rumble;
extern crate rand;

use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};
use rumble::bluez::manager::Manager;
use rumble::api::{UUID, Central, Peripheral};

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();
    println!("Post adapter");
    // reset the adapter -- clears out any errant state
  // adapter = manager.down(&adapter).unwrap();
   //  adapter = manager.up(&adapter).unwrap();
    println!("Post up/down");
    // connect to the adapter
    let central = adapter.connect().unwrap();

    println!("Post connect: {:?}", central.adapter);
    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices
    thread::sleep(Duration::from_secs(7));

    // find the device we're interested in
    let light = central.peripherals();
    println!("{:?}", light);

    // connect to the device
   // light.connect().unwrap();

    // discover characteristics
  //  light.discover_characteristics().unwrap();

    // find the characteristic we want
  //  let chars = light.characteristics();
  //  let cmd_char = chars.iter().find(|c| c.uuid == UUID::B16(0xFFE9)).unwrap();

    // dance party
    // let mut rng = thread_rng();
    // for _ in 0..20 {
    //     let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //     light.command(&cmd_char, &color_cmd).unwrap();
    //     thread::sleep(Duration::from_millis(200));
    // }
}

// extern crate blurz;

// use std::error::Error;

// use blurz::bluetooth_event::BluetoothEvent;
// use blurz::bluetooth_session::BluetoothSession as Session;

// fn test5() -> Result<(), Box<Error>> {
//     let session = &Session::create_session(None)?;

//     //  let session = &Session::create_session(Some("/org/bluez/hci0")).unwrap();
//     loop {
//         for event in session.incoming(1000).map(BluetoothEvent::from) {
//             println!("{:?}", event);
//         }
//     }
// }

// fn main() {
//     match test5() {
//         Ok(_) => (),
//         Err(e) => println!("{:?}", e),
//     }
// }
