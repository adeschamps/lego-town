#[macro_use]
extern crate json;
extern crate ws;

use json::{JsonValue};
use std::fs::File;
use std::io::Read;
use std::net::UdpSocket;
use ws::{listen, Handler, Handshake, Message, Sender};

struct Town {
    buildings: Vec<Building>,
    socket: UdpSocket
}

struct Building {
    name: String,
    lights: Vec<Light>
}

struct Light {
    color: [u8; 3]
}

impl Light {
    // TODO: Replace this with actual colour.
    fn color_as_hex_string(&self) -> String {
        "#ffffff".to_string()
    }
}

impl Town  {
    fn new(socket: UdpSocket, init_data: JsonValue) -> Result<Town, String> {
        let buildings = match Town::init_buildings(init_data) {
            Ok(buildings) => buildings,
            Err(e) => return Err(e)
        };
        Ok(Town{
            buildings: buildings,
            socket: socket
        })
    }

    fn init_buildings(init_data: JsonValue) -> Result<Vec<Building>, String> {
        Ok(
            init_data["buildings"].members().map(|b| Building{
                name: b["name"].to_string(),
                lights: b["lights"].members().map(|l| Light{
                    color: [0; 3]
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        )
    }

    fn get_state(&self) -> JsonValue {
        object!{
            "type" => "initialize",
            "buildings" => self.buildings.iter().enumerate().map(|(i, b)| object!{
                "buildingId" => i,
                "name" => b.name.to_string(),
                "lights" => b.lights.iter().enumerate().map(|(i, l)| object!{
                    "lightId" => i,
                    "color" => l.color_as_hex_string()
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
         }
    }
}

struct Client<'a> {
    out: Sender,
    town: &'a Town
}

impl<'a> Client<'a> {
    fn new(out: Sender, town: &'a Town) -> Client {
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

fn load_init_data(filename: &str) -> JsonValue {
    let mut f = match File::open(filename) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(file) => file
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("Failed to read file: {}", why),
        Ok(x) => x
    };
    let init_data = json::parse(s.as_ref()).unwrap();
    init_data
}

fn main() {
    let init_data = load_init_data("init-data.json");

    let arduino_addr = "127.0.0.1:12345";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let msg = "hello";
    let buf = msg.as_bytes();
    socket.send_to(buf, arduino_addr).unwrap();
    println!("Sent msg: {}", msg);

    let town = Town::new(socket, init_data).unwrap();

    listen("0.0.0.0:1234", |out| Client::new(out, &town)).unwrap();
}
