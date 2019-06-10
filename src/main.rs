

mod beacon;
mod publisher;
mod parser;
mod sample;
mod rando_testt;
mod bluetooth;
extern crate stomp;
extern crate tokio_io;
extern crate tokio;
extern crate simple_error;
#[macro_use]
extern crate futures;
extern crate paho_mqtt;
use publisher::*;
use crate::publisher::*;
use crate::beacon::*;
use publisher::mqtt::*;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate rumble;
extern crate rand;

// fn main() {
//     println!("Hello, world!");
//     let fjkd = "sfds";
//
//     let mut publisher = publisher::mqtt::MqttPublisher::new("127.0.0.1:1883", "test1");
//     publisher.publish2("message: &str");
//     std::thread::sleep_ms(2000);
//     println!("message sent");
// }

fn main() {
    rando_testt::main();
}
