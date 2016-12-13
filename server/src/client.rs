use client_api;

extern crate serde;
extern crate ws;

use ws::{Error, Handler, Handshake, Message};
use std::sync::mpsc;
use serde_json;

pub struct Client {
    out: ws::Sender,
    town_command: mpsc::Sender<(client_api::Msg, ws::Sender)>
}

impl Client {
    pub fn new(out: ws::Sender,
               town_command: mpsc::Sender<(client_api::Msg, ws::Sender)>) -> Client {
        Client{
            out: out,
            town_command: town_command
        }
    }
}

impl Handler for Client {
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

        let msg = match serde_json::from_str(msg) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Failed to parse message: {}", e);
                return Ok(())
            }
        };

        self.town_command.send((msg, self.out.clone())).unwrap_or_else(|e| {
            println!("Failed to send command to town controller: {}", e)
        });

        Ok(())
    }

    fn on_error(&mut self, error: Error) {
        println!("The server encountered an error: {}", error)
    }
}
