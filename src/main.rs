mod beacon;
mod publisher;
mod bluetooth;

use std::error::Error;
use blurz::bluetooth_session::BluetoothSession as Session;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;


fn main() -> Result<(), Box<dyn Error>>{

    let bt_session = &Session::create_session(None)?;
    let should_stop = Arc::new(AtomicBool::new(false));
    
    let r = should_stop.clone();
    ctrlc::set_handler(move || {
            r.store(true, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");

    let mut listener = bluetooth::BlurzListener::new(bt_session, should_stop)?;

    ///let publisher = publisher::mqtt::MqttPublisher::new("127.0.0.1:1883", "test1");

    listener.work2(Box::new(move |ds| {
       // publisher.publish2(&format!("{:?}", ds));
        println!("\n{:?}\n", ds);
    }))?;
    Ok(())
}
