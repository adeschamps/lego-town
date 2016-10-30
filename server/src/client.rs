use town;

extern crate ws;
use ws::{Handler, Handshake, Message, Sender};

use json;

pub struct Client<'a> {
    out: Sender,
    town: &'a town::Town
}

impl<'a> Client<'a> {
    pub fn new(out: Sender, town: &'a town::Town) -> Client {
        Client{
            out: out,
            town: town
        }
    }

    fn handle_init(&self) -> Result<(), ws::Error> {
        let s = format!("{}", self.town.get_state());
        self.out.send(s)
    }

    fn handle_set_building(&self) -> Result<(), ws::Error> {
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
