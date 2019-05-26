use stomp::connection::{Credentials, HeartBeat};
use stomp::header::SuppressedHeader;
use stomp::header::*;

use super::super::beacon::Beacon;
use stomp::session_builder::SessionBuilder;
use tokio::net::TcpStream;
use futures::*;

pub fn oli() {
    let addr = "127.0.0.1:61613".parse().unwrap();
    let f = Box::new(TcpStream::connect(&addr));
    let header = HeaderName::from_str("custom-client-id");


    let mut session = SessionBuilder::new()
        .with(Header::new(header, "hmspna4"))
        //.with(SuppressedHeader("content-length"))
        .with(HeartBeat(5_000, 2_000))
        .with(Credentials("admin", "admin"))
        .build(f);

        //session.

    

    // .with(stomp::subscription::AckMode::Auto);
    // let mut foo = stomp::session("", 5656).start().unwrap();
    //  sess.message("SampleQueue", "Simples Assim").with(SuppressedHeader("content-length")).send();

}

pub struct StompPublisher {
    session: stomp::Session<tokio::net::TcpStream>,
    destination: String
}

struct Bar {
    session: stomp::Session<tokio::net::TcpStream>,
    destination: String
}

impl futures::Future for Bar {
    type Item = StompPublisher;
    type Error = std::io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        use stomp::session::SessionEvent::*;
        let msg = match try_ready!(self.session.poll()) {
            None => {
                    return Ok(Async::NotReady);
            }
            Some(msg) => {
                 msg
            }
        };
        match msg {
            Connected => {
                // let publisher = StompPublisher {
                //     destination: self.destination.to_owned(),
                //     session: self.session
                // };
                return Ok(Async::NotReady);
            }
            _ => {
                return Ok(Async::NotReady);
            }
        }
       // Ok(futures::Async::Ready(()))
        return Ok(Async::NotReady);
    }
}

impl Bar {
    pub fn new(address: &str, port: u32, destination: &str) -> Bar {

        let addr = address.to_owned().parse().unwrap();
        let f = Box::new(TcpStream::connect(&addr));

        let session = SessionBuilder::new()
            .with(HeartBeat(5_000, 2_000))
            .with(Credentials("admin", "admin"))
            .build(f);
        Bar {
            session: session,
            destination: destination.to_owned()
        }
    }
}

impl StompPublisher {

    pub fn new(address: &str, port: u32, destination: &str) -> StompPublisher {

        let addr = address.to_owned().parse().unwrap();
        let f = Box::new(TcpStream::connect(&addr));

        let session = SessionBuilder::new()
            .with(HeartBeat(5_000, 2_000))
            .with(Credentials("admin", "admin"))
            .build(f);
        
        StompPublisher {
            destination: destination.to_owned(),
            session: session
        }
    }
       
    pub fn publish2(&mut self, message: &str) {
       // let byte_message: &[u8] =  &message.to_byte_message();
        self.session.message(&self.destination, message).send();
    }
}

impl super::publisher::Publisher for StompPublisher {

    fn publish(&mut self, message: &Beacon) {
        let byte_message: &[u8] =  &message.to_byte_message();
        self.session.message(&self.destination, byte_message).send();
    }
}
impl Drop for StompPublisher {
    fn drop(&mut self) {
        self.session.disconnect();
    }
}