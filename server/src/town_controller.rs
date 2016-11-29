extern crate read_color;

use client_api;
use town;
use messages;

extern crate ws;
use protobuf::Message;
use rustc_serialize::json;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;

pub struct TownController {
    arduino_socket: UdpSocket,
    arduino_addr: SocketAddr,
    town: town::Town,
    commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>
}

impl TownController {
    pub fn new(arduino_addr: SocketAddr,
               town: town::Town,
               commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>
              ) -> TownController {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        TownController {
            arduino_socket: socket,
            arduino_addr: arduino_addr,
            town: town,
            commands: commands
        }
    }

    pub fn run(&mut self) {
        println!("Running town controller.");
        self.initialize_arduino();

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
                    sl.set_light_group(building_id as u32);
                    sl.set_light_id(light_id as u32);
                    sl.set_color(color);
                    cmd.set_set_light(sl);
                    self.send_arduino_command(cmd);

                    let response = self.get_state();
                    let response = json::encode(&response).unwrap();
                    out.broadcast(response).unwrap();
                }

                client_api::Msg::SetBuilding{building_id, color} => {
                    let mut cmd = messages::Command::new();
                    let mut sg = messages::SetGroup::new();
                    sg.set_light_group(building_id as u32);
                    sg.set_color(color);
                    cmd.set_set_group(sg);
                    self.send_arduino_command(cmd);

                    for light in &mut self.town.buildings[building_id as usize].lights {
                        light.color = color;
                    }

                    let response = self.get_state();
                    let response = json::encode(&response).unwrap();
                    println!("responding: {}", response);
                    out.broadcast(response).unwrap();
                }

                client_api::Msg::SetArduinoAddress{address} => {
                    self.arduino_addr = address;
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

    /// Sets each light individually to match the current model.
    /// This sends one packet per light.
    fn initialize_arduino(&self) {
        let mut cmd = messages::Command::new();
        let mut init = messages::Initialize::new();
        for length in self.town.buildings.iter().map(|b| b.lights.len()) {
            init.mut_string_lengths().push(length as u32);
        }
        cmd.set_initialize(init);
        self.send_arduino_command(cmd);

        for (building_id, building) in self.town.buildings.iter().enumerate() {
            for (light_id, light) in building.lights.iter().enumerate() {

                let mut cmd = messages::Command::new();
                let mut sl = messages::SetLight::new();
                sl.set_light_group(building_id as u32);
                sl.set_light_id(light_id as u32);
                sl.set_color(light.color);
                cmd.set_set_light(sl);

                self.send_arduino_command(cmd);
            }
        }
    }

    fn get_state(&self) -> client_api::Response {
        client_api::Response::State{
            arduino_address: self.arduino_addr.to_string(),
            buildings: self.town.buildings.iter().enumerate().map(|(i,b)| client_api::Building {
                name: b.name.clone(),
                id: i as u8,
                lights: b.lights.iter().enumerate().map(|(i,l)| client_api::Light {
                    id: i as u8,
                    color: l.color
                }).collect()
            }).collect()
        }
    }
}
