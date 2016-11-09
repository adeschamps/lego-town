extern crate read_color;

use client_api;
use town;
use messages;

extern crate ws;

use protobuf::Message;
use rustc_serialize::hex::ToHex;
use rustc_serialize::json;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::mpsc;

pub struct TownController {
    arduino_socket: UdpSocket,
    arduino_addr: SocketAddr,
    town: town::Town,
    commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>
}

impl TownController {
    pub fn new<A: ToSocketAddrs>(arduino_addr: A,
                                 town: town::Town,
                                 commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>
                                ) -> TownController {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let addr = arduino_addr.to_socket_addrs().unwrap().next().unwrap();
        TownController {
            arduino_socket: socket,
            arduino_addr: addr,
            town: town,
            commands: commands
        }
    }

    pub fn run(&self) {
        println!("Running town controller.");
        for (cmd, out) in self.commands.iter() {
            println!("Received command: {:?}", cmd);

            match cmd {
                client_api::Msg::GetState => {
                    let response = client_api::Response::State{
                        buildings: self.town.buildings.iter().enumerate().map(|(i,b)| client_api::Building {
                            name: b.name.clone(),
                            id: i as u8,
                            lights: b.lights.iter().enumerate().map(|(i,l)| client_api::Light {
                                id: i as u8,
                                color: format!("#{}", l.color.to_hex())
                            }).collect()
                        }).collect()
                    };
                    let response = json::encode(&response).unwrap();
                    out.send(response).unwrap();
                }

                client_api::Msg::SetLight{..} => {
                    let mut c = messages::Command::new();
                    let mut sl = messages::SetLight::new();
                    c.set_set_light(sl);
                    self.send(c);
                    println!("Set light")
                },
                client_api::Msg::SetBuilding{building_id, color} => {
                    let mut c = messages::Command::new();
                    let mut sg = messages::SetGroup::new();
                    let mut col = messages::Color::new();
                    let color = read_color::rgb(color.as_str()[1..].chars().by_ref()).unwrap();
                    col.set_red(color[0] as i32);
                    col.set_green(color[1] as i32);
                    col.set_blue(color[2] as i32);
                    sg.set_light_group(building_id as i32);
                    sg.set_color(col);
                    c.set_set_group(sg);
                    self.send(c);
                    println!("Set Building")
                }
            };
        }
    }

    fn send<M: Message>(&self, msg: M) {
        let msg = msg.write_to_bytes().unwrap();
        let msg = msg.as_slice();
        println!("Sending msg: {:?}", msg);
        self.arduino_socket.send_to(msg, self.arduino_addr);
    }
}
