use std::{env, process};
use futures::Future;
use paho_mqtt as mqtt;
use super::super::beacon::Beacon;

const QOS: i32 = 1;


pub struct MqttPublisher {
    destination: String,
    connection: mqtt::AsyncClient,
}

impl MqttPublisher {
    pub fn new(address: &str,  destination: &str) -> MqttPublisher {
        let host = env::args().skip(1).next().unwrap_or(
               address.to_string()
        );

        let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
		    println!("Error creating the client: {}", err);
		    process::exit(1);
	    });

        let conn_opts = mqtt::ConnectOptions::new();

        if let Err(e) = cli.connect(conn_opts).wait() {
            println!("Unable to connect: {:?}", e);
            process::exit(1);
	    }
        MqttPublisher {
            connection: cli,
            destination: destination.to_owned()
        }
    }

    pub fn publish2(&self, message: &str) {
        let topic = mqtt::Topic::new(&self.connection,
        self.destination.to_owned(), QOS);

        let tok = topic.publish(message);
        if let Err(e) = tok.wait() {
			println!("Error sending message: {:?}", e);

		}
    }
}


 impl super::publisher::Publisher for MqttPublisher {
    fn publish(&mut self, message: &Beacon) {
      //  let destination = self.destination;
        let topic = mqtt::Topic::new(&self.connection,
            self.destination.to_owned(), QOS);

        let tok = topic.publish(message.to_byte_message());
        if let Err(e) = tok.wait() {
			println!("Error sending message: {:?}", e);

		}
    }
}

fn main() {
	// Initialize the logger from the environment
	env_logger::init();

    let host = env::args().skip(1).next().unwrap_or(
        "tcp://localhost:1883".to_string()
    );

	// Create a client & define connect options
	let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
		println!("Error creating the client: {}", err);
		process::exit(1);
	});

	let conn_opts = mqtt::ConnectOptions::new();

	// Connect and wait for it to complete or fail
	if let Err(e) = cli.connect(conn_opts).wait() {
		println!("Unable to connect: {:?}", e);
		process::exit(1);
	}

	// Create a topic and publish to it
	println!("Publishing messages on the 'test' topic");
	let topic = mqtt::Topic::new(&cli, "test", QOS);
	for _ in 0..5 {
		let tok = topic.publish("Hello there");

		if let Err(e) = tok.wait() {
			println!("Error sending message: {:?}", e);
			break;
		}
	}

	// Disconnect from the broker
	let tok = cli.disconnect(None);
	tok.wait().unwrap();
}
