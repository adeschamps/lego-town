#[macro_use]
extern crate json;
extern crate ws;

use json::{JsonValue};
use std::fs::File;
use std::io::Read;
use ws::{listen, Handler, Handshake, Message, Sender};

struct Client<'a> {
    out: Sender,
    init_data: &'a JsonValue
}

impl<'a> Client<'a> {
    fn new(out: Sender, init_data: &'a JsonValue) -> Client {
        Client{
            out: out,
            init_data: init_data
        }
    }

    fn handle_init(&mut self) -> Result<(), ws::Error> {
        let s = format!("{}", self.init_data);
        self.out.send(s)
    }

    fn handle_set_building(&mut self) -> Result<(), ws::Error> {
        Ok(())
    }
}

impl<'a> Handler for Client<'a> {
    fn on_open(&mut self, handshake: Handshake) -> Result<(), ws::Error> {
        println!("Client connected from {}", handshake.peer_addr.unwrap());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        let msg = match msg.as_text() {
            Ok(msg) => msg,
            Err(e) => {
                println!("Message is not text: {}", e);
                return Ok(())
            }
        };

        println!("Received message: {}", msg);
        let msg = match json::parse(&msg) {
            Ok(msg) => msg,
            Err(_) => {
                println!("Failed to parse json.");
                return Ok(())
            }
        };

        let msg_type = match msg["type"].as_str() {
            Some(t) => t,
            None => {
                println!("type is not a string.");
                return Ok(())
            }
        };

        println!("message type: {}", msg_type);
        match msg_type {
            "init" => self.handle_init(),
            "setBuilding" => self.handle_set_building(),
            _ => Ok(())
        }
    }
}

fn main() {
    let mut f = match File::open("init-data.json") {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(file) => file
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("Failed to read file: {}", why),
        Ok(x) => x
    };
    let init_data = json::parse(s.as_ref()).unwrap();

    listen("0.0.0.0:1234", |out| Client::new(out, &init_data)).unwrap();
}
