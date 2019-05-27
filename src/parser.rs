

use std::io::Stdin;
use std::io::BufRead;
use super::beacon::Beacon;
use super::publisher::publisher::Publisher;

pub struct ParseCommand {
    scannerID: String,
    brokerURL: String,
    clientID: String,
    destinationName: String,
    skipPublish: bool,
    asyncMode: bool
}


pub fn parse(stream: &mut BufRead, parse_command: &ParseCommand) {
    use std::result;

    let mut mqtt = crate::publisher::mqtt::MqttPublisher::new("tcp://localhost:1883", "fooBar");

    
    let mut the_line = String::new();
    while stream.read_line(&mut the_line).is_ok() {
        let mut line = the_line.as_bytes();
        let mut lenght = line.len();
        
        if line[lenght -1] == ' ' as u8 {
            lenght = lenght - 1;
            line = &line[0..lenght]
        }

        if &line[0..7] == "> 04 3E".as_bytes() && std::str::from_utf8(&line[16..lenght]).unwrap().find(" 1A FF ").is_some(){
            let mut string = std::str::from_utf8(&line).unwrap().trim().to_owned();
            string.push_str(" ");


            let mut tmp = String::new();


            stream.read_line(&mut tmp).unwrap();
            string.push_str(&tmp.trim());
            string.push_str(" ");

            stream.read_line(&mut tmp).unwrap();
            string.push_str(&tmp.trim());

            let beacon = Beacon::new_from_packet(&parse_command.scannerID, &string);
            mqtt.publish(&beacon);

        }
      
    }
}

