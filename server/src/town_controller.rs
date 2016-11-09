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
                    let response = self.get_state();
                    let response = json::encode(&response).unwrap();
                    out.send(response).unwrap();
                }

                client_api::Msg::SetLight{building_id, light_id, color} => {
                    let mut cmd = messages::Command::new();
                    let mut sl = messages::SetLight::new();
                    let mut col = messages::Color::new();
                    let color = read_color::rgb(color.as_str()[1..].chars().by_ref()).unwrap();
                    col.set_red(color[0] as i32);
                    col.set_green(color[1] as i32);
                    col.set_blue(color[2] as i32);
                    sl.set_light_group(building_id as i32);
                    sl.set_light_id(light_id as i32);
                    sl.set_color(col);
                    cmd.set_set_light(sl);
                    self.send_arduino_command(cmd);

                    let response = self.get_state();
                    let response = json::encode(&response).unwrap();
                    out.broadcast(response).unwrap();
                },
                client_api::Msg::SetBuilding{building_id, color} => {
                    let mut cmd = messages::Command::new();
                    let mut sg = messages::SetGroup::new();
                    let mut col = messages::Color::new();
                    let color = read_color::rgb(color.as_str()[1..].chars().by_ref()).unwrap();
                    col.set_red(color[0] as i32);
                    col.set_green(color[1] as i32);
                    col.set_blue(color[2] as i32);
                    sg.set_light_group(building_id as i32);
                    sg.set_color(col);
                    cmd.set_set_group(sg);
                    self.send_arduino_command(cmd);

                    let response = self.get_state();
                    let response = json::encode(&response).unwrap();
                    out.broadcast(response).unwrap();
                }
            };
        }
    }

    fn send_arduino_command<M: Message>(&self, msg: M) {
        let msg = msg.write_to_bytes().unwrap();
        let msg = msg.as_slice();
        println!("Sending msg: {:?}", msg);
        match self.arduino_socket.send_to(msg, self.arduino_addr) {
            Ok(_) => {}
            Err(e) => println!("Failed to send arduino message: {}", e)
        }
    }

    fn get_state(&self) -> client_api::Response {
        client_api::Response::State{
            buildings: self.town.buildings.iter().enumerate().map(|(i,b)| client_api::Building {
                name: b.name.clone(),
                id: i as u8,
                lights: b.lights.iter().enumerate().map(|(i,l)| client_api::Light {
                    id: i as u8,
                    color: format!("#{}", l.color.to_hex())
                }).collect()
            }).collect()
        }
    }
}
