use town;
use town_controller;

extern crate ws;
use ws::{Error, Handler, Handshake, Message};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use json;

pub struct Client {
    out: ws::Sender,
    town: Arc<Mutex<town::Town>>,
    town_command: mpsc::Sender<town_controller::TownCommand>
}

impl Client {
    pub fn new(out: ws::Sender,
               town: Arc<Mutex<town::Town>>,
               town_command: mpsc::Sender<town_controller::TownCommand>) -> Client {
        Client{
            out: out,
            town: town,
            town_command: town_command
        }
    }

    fn handle_init(&self) -> Result<(), ws::Error> {
        let town = self.town.lock().unwrap();
        let state = format!("{}", town.get_state());
        self.out.send(state)
    }

    fn handle_set_building(&self) -> Result<(), ws::Error> {
        // self.town.set_light();
        Ok(())
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

    fn on_error(&mut self, error: Error) {
        println!("The server encountered an error: {}", error)
    }
}
