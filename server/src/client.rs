use client_api;
use town;

extern crate rustc_serialize;
extern crate ws;

use ws::{Error, Handler, Handshake, Message};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use rustc_serialize::json;
use rustc_serialize::hex::ToHex;

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

        let msg = match json::decode(msg) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Failed to parse message: {}", e);
                return Ok(())
            }
        };

        let response = match msg {
            client_api::Msg::GetState => {
                let town = self.town.lock().unwrap();
                Some(client_api::Response::State {
                    buildings: town.buildings.iter().enumerate().map(|(i,b)| client_api::Building {
                        name: b.name.clone(),
                        id: i as u8,
                        lights: b.lights.iter().enumerate().map(|(i,l)| client_api::Light {
                            id: i as u8,
                            color: format!("#{}", l.color.to_hex())
                        }).collect()
                    }).collect()
                })
            }

            client_api::Msg::SetBuilding{..} => {
                None
            }

            client_api::Msg::SetLight{..} => {
                None
            }
        };

        self.town_command.send(msg).unwrap_or_else(|e| {
            println!("Failed to send command to town controller: {}", e)
        });

        match response {
            Some(response) => {
                let response = json::encode(&response).unwrap();
                println!("Sending response: {}", response);
                self.out.send(response)
            }
            None => Ok(())
        }
    }

    fn on_error(&mut self, error: Error) {
        println!("The server encountered an error: {}", error)
    }
}
