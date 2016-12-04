extern crate read_color;

use client_api;
use town;
use messages;

extern crate ws;
use protobuf::Message;
use rustc_serialize::json;
use std::net::{SocketAddr, UdpSocket};
use std::rc::Rc;
use std::sync::mpsc;

pub struct TownController {
    arduino_socket: UdpSocket,
    arduino_addr: SocketAddr,
    town: Rc<town::Town>,
    previous_town: Rc<town::Town>,
    commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>
}

impl TownController {
    pub fn new(arduino_addr: SocketAddr,
               town: town::Town,
               commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>
              ) -> TownController {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let town = Rc::new(town);
        TownController {
            arduino_socket: socket,
            arduino_addr: arduino_addr,
            town: town.clone(),
            previous_town: town.clone(),
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

                client_api::Msg::SetLights{building_id, light_ids, color} => {
                    for light_id in light_ids {
                        let mut cmd = messages::Command::new();
                        let mut sl = messages::SetLights::new();
                        sl.set_light_group(building_id as u32);
                        sl.set_light_id_start(light_id as u32);
                        sl.set_light_id_end((light_id + 1) as u32);
                        sl.set_color(color);
                        cmd.set_set_lights(sl);
                        self.send_arduino_command(cmd);
                    }

                    let response = self.get_state();
                    let response = json::encode(&response).unwrap();
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
            let mut cmd = messages::Command::new();
            let mut sl = messages::SetLights::new();
            sl.set_light_group(building_id as u32);
            sl.set_light_id_start(0 as u32);
            sl.set_light_id_end(building.lights.len() as u32);
            cmd.set_set_lights(sl);

            self.send_arduino_command(cmd);
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




// TESTS

#[cfg(test)]
fn initial_town() -> town::Town {
    town::Town {
        buildings: vec![
            Rc::new(town::Building {
                name: "Cafe Corner".to_string(),
                lights: vec![
                    Rc::new(town::Light{ color: messages::Color::RED }),
                    Rc::new(town::Light{ color: messages::Color::RED }),
                    Rc::new(town::Light{ color: messages::Color::RED }),
                    Rc::new(town::Light{ color: messages::Color::RED })
                ]
            }),
            Rc::new(town::Building {
                name: "Green Grocer".to_string(),
                lights: vec![
                    Rc::new(town::Light{ color: messages::Color::RED }),
                    Rc::new(town::Light{ color: messages::Color::RED }),
                    Rc::new(town::Light{ color: messages::Color::RED }),
                    Rc::new(town::Light{ color: messages::Color::RED })
                ]
            })
        ]
    }
 }

#[test]
fn initialization() {
    let town = Rc::new(initial_town());
    let messages = initialization_commands(&town);
    let expected = vec![
        protobuf_init!(messages::Command::new(), {
            initialize => {
                string_lengths: vec![4, 4]
            }
        }),
        protobuf_init!(messages::Command::new(), {
            set_lights => {
                light_group: 0,
                light_id_start: 0,
                light_id_end: 4,
                color: messages::Color::RED
            }
        }),
        protobuf_init!(messages::Command::new(), {
            set_lights => {
                light_group: 1,
                light_id_start: 0,
                light_id_end: 4,
                color: messages::Color::RED
            }
        })
    ];

    for (m, e) in messages.iter().zip(&expected) {
        assert_eq!(*m, *e);
    }
    assert_eq!(messages, expected);
}

#[test]
fn set_lights() {
    let mut town = Rc::new(initial_town());
    let old_town = town.clone();

    let msg = client_api::Msg::SetLights {
        building_id: 1,
        light_ids: vec![0,2,3],
        color: messages::Color::BLUE
    };

    update_town(msg, &mut town);


    // Original is unchanged
    assert_eq!(old_town.buildings[0].lights[0].color, messages::Color::RED);
    assert_eq!(old_town.buildings[0].lights[1].color, messages::Color::RED);
    assert_eq!(old_town.buildings[0].lights[2].color, messages::Color::RED);
    assert_eq!(old_town.buildings[0].lights[3].color, messages::Color::RED);

    assert_eq!(old_town.buildings[1].lights[0].color, messages::Color::RED);
    assert_eq!(old_town.buildings[1].lights[1].color, messages::Color::RED);
    assert_eq!(old_town.buildings[1].lights[2].color, messages::Color::RED);
    assert_eq!(old_town.buildings[1].lights[3].color, messages::Color::RED);


    // New town is updated
    assert_eq!(town.buildings[0].lights[0].color, messages::Color::RED);
    assert_eq!(town.buildings[0].lights[1].color, messages::Color::RED);
    assert_eq!(town.buildings[0].lights[2].color, messages::Color::RED);
    assert_eq!(town.buildings[0].lights[3].color, messages::Color::RED);

    assert_eq!(town.buildings[1].lights[0].color, messages::Color::BLUE);
    assert_eq!(town.buildings[1].lights[1].color, messages::Color::RED);
    assert_eq!(town.buildings[1].lights[2].color, messages::Color::BLUE);
    assert_eq!(town.buildings[1].lights[3].color, messages::Color::BLUE);
}

#[test]
fn message_diff() {
    let mut town = Rc::new(initial_town());
    let old_town = town.clone();
    let msg = client_api::Msg::SetLights {
        building_id: 1,
        light_ids: vec![0,2,3],
        color: messages::Color::BLUE
    };

    update_town(msg, &mut town);

    let messages = make_diff(&old_town, &town);
    let expected = vec![
        protobuf_init!(messages::Command::new(), {
            set_lights => {
                light_group: 1,
                light_id_start: 0,
                light_id_end: 1
            }
        }),
        protobuf_init!(messages::Command::new(), {
            set_lights => {
                light_group: 1,
                light_id_start: 2,
                light_id_end: 4
            }
        })
    ];

    for (m, e) in messages.iter().zip(&expected) {
        assert_eq!(*m, *e);
    }
    assert_eq!(messages, expected);
}
