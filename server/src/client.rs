use client_api;
use town;

extern crate rustc_serialize;
extern crate try_from;
extern crate ws;

use client::try_from::TryFrom;
use ws::{Error, Handler, Handshake, Message};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use self::rustc_serialize::json;
use self::rustc_serialize::json::Json;

pub struct Client {
    out: ws::Sender,
    town: Arc<Mutex<town::Town>>,
    town_command: mpsc::Sender<client_api::Msg>
}

impl Client {
    pub fn new(out: ws::Sender,
               town: Arc<Mutex<town::Town>>,
               town_command: mpsc::Sender<client_api::Msg>) -> Client {
        Client{
            out: out,
            town: town,
            town_command: town_command
        }
    }

    fn handle_init(&self) -> Result<(), ws::Error> {
        let town = self.town.lock().unwrap();
        let state = client_api::Response::State {
            buildings: town.buildings.iter().enumerate().map(|(i, b)| client_api::Building {
                name: b.name.clone(),
                id: i as u8,
                lights: Vec::new()
            }).collect()
        };
        let msg = json::encode(&state).unwrap();
        println!("state: {}", json::encode(&state).unwrap());
        self.out.send(msg)
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

        println!("Received message: {}", msg);
        let msg = match Json::from_str(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Failed to parse json: {}", e);
                return Ok(())
            }
        };

        let msg = match client_api::Msg::try_from(msg) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Failed to convert json to message: {}", e);
                return Ok(())
            }
        };

        match msg {
            client_api::Msg::Init => {
                self.handle_init()
            }

            client_api::Msg::SetBuilding{..} => {
                self.town_command.send(msg);
                Ok(())
            }

            client_api::Msg::SetLight{..} => {
                self.town_command.send(msg);
                Ok(())
            }
        }
    }

    fn on_error(&mut self, error: Error) {
        println!("The server encountered an error: {}", error)
    }
}
