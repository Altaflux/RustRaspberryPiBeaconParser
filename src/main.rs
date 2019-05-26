

mod beacon;
mod publisher;
//mod bytebuffer::;
extern crate stomp;
extern crate tokio_io;
extern crate tokio;
#[macro_use]
extern crate futures;
extern crate paho_mqtt;
use publisher::*;
use crate::publisher::*;
use crate::beacon::*;
use publisher::mqtt::*;
#[macro_use] extern crate log;
extern crate env_logger;

fn main() {
    println!("Hello, world!");

    let mut publisher = publisher::stomp::StompPublisher::new("127.0.0.1:61613",61613,  "test1");
    publisher.publish2("message: &str");
    std::thread::sleep_ms(2000);
    println!("message sent");
}
