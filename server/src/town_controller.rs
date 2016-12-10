use client_api;
use town;
use messages;

extern crate ws;
use itertools::Itertools;
use protobuf::Message;
use serde_json;
use std::net::{SocketAddr, UdpSocket};
use std::rc::Rc;
use std::sync::mpsc;

pub struct TownController {
    arduino_socket: UdpSocket,
    arduino_addr: SocketAddr,
    town: Rc<town::Town>,
    previous_town: Rc<town::Town>
}

impl TownController {
    pub fn new(arduino_addr: SocketAddr,
               town: town::Town
              ) -> TownController {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let town = Rc::new(town);
        TownController {
            arduino_socket: socket,
            arduino_addr: arduino_addr,
            town: town.clone(),
            previous_town: town.clone()
        }
    }

    pub fn run(&mut self, commands: mpsc::Receiver<(client_api::Msg, ws::Sender)>) {
        println!("Running town controller.");
        self.initialize_arduino();

        for (msg, out) in commands.iter() {
            println!("Received message: {:?}", msg);

            update_town(&msg, &mut self.town);
            self.handle_message(&msg);

            for message in make_diff(&self.previous_town, &self.town) {
                self.send_arduino_command(message);
            }
            self.previous_town = self.town.clone();

            let state = self.get_state();
            let msg = serde_json::to_string(&state).unwrap();
            out.broadcast(msg).unwrap();
        }
    }

    fn handle_message(&mut self, msg: &client_api::Msg) {
        match *msg {
            client_api::Msg::GetState => {
                self.initialize_arduino();
            }

            client_api::Msg::SetBuilding{..} => {
            }

            client_api::Msg::SetLights{..} => {
            }

            client_api::Msg::SetArduinoAddress{address} => {
                self.arduino_addr = address;
            }
        };
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

    fn initialize_arduino(&self) {
        for cmd in initialization_commands(&self.town) {
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
                    color: l.color.clone()
                }).collect()
            }).collect()
        }
    }
}


fn update_town(msg: &client_api::Msg, mut town: &mut Rc<town::Town>) {
    match msg {
        &client_api::Msg::GetState => {
        }

        &client_api::Msg::SetBuilding{building_id, ref color} => {
            let mut town = Rc::make_mut(&mut town);
            let building_id = building_id as usize;
            if let Some(building) = town.buildings.get_mut(building_id).map(Rc::make_mut) {
                for light in building.lights.iter_mut().map(Rc::make_mut) {
                    light.color = color.clone();
                }
            }
        }

        &client_api::Msg::SetLights{building_id, ref light_ids, ref color} => {
            let mut town = Rc::make_mut(&mut town);
            let building_id = building_id as usize;
            if let Some(building) = town.buildings.get_mut(building_id).map(Rc::make_mut) {
                for &light_id in light_ids {
                    if let Some(light) = building.lights.get_mut(light_id as usize).map(Rc::make_mut) {
                        light.color = color.clone();
                    }
                }
            }
        }

        &client_api::Msg::SetArduinoAddress{..} => {
        }
    }
}

trait Coalescable
    where Self: Sized {
    fn coalesce(Self, Self) -> Result<Self, (Self, Self)>;
}

impl Coalescable for messages::SetLights {
    fn coalesce(mut sl_a: Self, sl_b: Self) -> Result<Self, (Self, Self)> {
        protobuf_bind!(sl_a, {
            light_group: group_a,
            // light_id_start: start_a,
            light_id_end: end_a,
            color: color_a
        });
        protobuf_bind!(sl_b, {
            light_group: group_b,
            light_id_start: start_b,
            light_id_end: end_b,
            color: color_b
        });

        let adjacent =
            group_a == group_b
            && color_a == color_b
            && end_a == start_b;

        match adjacent {
            true => {
                sl_a.set_light_id_end(end_b);
                Ok(sl_a)
            }
            false => {
                Err((sl_a, sl_b))
            }
        }
    }
}

fn initialization_commands(town: &town::Town) -> Vec<messages::Command> {
    town.buildings.iter().flat_map(|building| {
        building.lights.iter()
            .map(|light| {
                protobuf_init!(messages::SetLights::new(), {
                    light_group: building.id as u32,
                    light_id_start: light.id as u32,
                    light_id_end: (light.id + 1) as u32,
                    color: light.color.clone().into()
                })
            })
            .coalesce(Coalescable::coalesce)
            .collect::<Vec<_>>().into_iter()
        }).map(|set_lights| protobuf_init!(messages::Command::new(), {
            set_lights: set_lights
        })).collect()
}

fn make_diff(old_town: &town::Town, new_town: &town::Town) -> Vec<messages::Command> {
    old_town.buildings.iter()
        .zip(&new_town.buildings)
        .filter(|&(old_building, building)| Rc::eq(old_building, building) == false)
        .flat_map(|(old_building, building)| {
            old_building.lights.iter()
                .zip(&building.lights)
                .filter(|&(old_light, light)| Rc::eq(old_light, light) == false)
                .map(|(_, light)| light)
                .map(|light| protobuf_init!(messages::SetLights::new(), {
                    light_group: building.id as u32,
                    light_id_start: light.id as u32,
                    light_id_end: (light.id + 1) as u32,
                    color: light.color.clone().into()
                }))
                .coalesce(Coalescable::coalesce)
                .collect::<Vec<_>>().into_iter()
        })
        .map(|sl| protobuf_init!(messages::Command::new(), { set_lights: sl }))
        .collect()
}





// TESTS

#[cfg(test)]
use std::str::FromStr;


#[cfg(test)]
fn initial_town() -> town::Town {
    town::Town {
        buildings: vec![
            Rc::new(town::Building {
                id: 0,
                name: "Cafe Corner".to_string(),
                lights: vec![
                    Rc::new(town::Light{ id: 0, color: town::Color::RED }),
                    Rc::new(town::Light{ id: 1, color: town::Color::RED }),
                    Rc::new(town::Light{ id: 2, color: town::Color::RED }),
                    Rc::new(town::Light{ id: 3, color: town::Color::RED })
                ]
            }),
            Rc::new(town::Building {
                id: 1,
                name: "Green Grocer".to_string(),
                lights: vec![
                    Rc::new(town::Light{ id: 0, color: town::Color::RED }),
                    Rc::new(town::Light{ id: 1, color: town::Color::RED }),
                    Rc::new(town::Light{ id: 2, color: town::Color::RED }),
                    Rc::new(town::Light{ id: 3, color: town::Color::RED })
                ]
            })
        ]
    }
}

#[cfg(test)]
fn initial_controller() -> TownController {
    let addr = SocketAddr::from_str("127.0.0.1:12345").unwrap();
    TownController::new(addr, initial_town())
}

#[test]
fn set_arduino_address() {
    let mut controller = initial_controller();
    let new_addr = SocketAddr::from_str("192.168.0.100:5555").unwrap();
    let msg = client_api::Msg::SetArduinoAddress {
        address: new_addr
    };
    controller.handle_message(&msg);
    assert_eq!(controller.arduino_addr, new_addr);
}

#[test]
fn initialization() {
    let town = Rc::new(initial_town());
    let messages = initialization_commands(&town);
    let expected = vec![
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
fn test_set_building() {
    let mut town = Rc::new(initial_town());

    let msg = client_api::Msg::SetBuilding {
        building_id: 1,
        color: town::Color::BLUE
    };

    update_town(&msg, &mut town);

    assert_eq!(town.buildings[1].lights[0].color, town::Color::BLUE);
    assert_eq!(town.buildings[1].lights[1].color, town::Color::BLUE);
    assert_eq!(town.buildings[1].lights[2].color, town::Color::BLUE);
    assert_eq!(town.buildings[1].lights[3].color, town::Color::BLUE);
}

#[test]
fn test_set_lights() {
    let mut town = Rc::new(initial_town());
    let old_town = town.clone();

    let msg = client_api::Msg::SetLights {
        building_id: 1,
        light_ids: vec![0,2,3],
        color: town::Color::BLUE
    };

    update_town(&msg, &mut town);


    // Original is unchanged
    assert_eq!(old_town.buildings[0].lights[0].color, town::Color::RED);
    assert_eq!(old_town.buildings[0].lights[1].color, town::Color::RED);
    assert_eq!(old_town.buildings[0].lights[2].color, town::Color::RED);
    assert_eq!(old_town.buildings[0].lights[3].color, town::Color::RED);

    assert_eq!(old_town.buildings[1].lights[0].color, town::Color::RED);
    assert_eq!(old_town.buildings[1].lights[1].color, town::Color::RED);
    assert_eq!(old_town.buildings[1].lights[2].color, town::Color::RED);
    assert_eq!(old_town.buildings[1].lights[3].color, town::Color::RED);


    // New town is updated
    assert_eq!(town.buildings[0].lights[0].color, town::Color::RED);
    assert_eq!(town.buildings[0].lights[1].color, town::Color::RED);
    assert_eq!(town.buildings[0].lights[2].color, town::Color::RED);
    assert_eq!(town.buildings[0].lights[3].color, town::Color::RED);

    assert_eq!(town.buildings[1].lights[0].color, town::Color::BLUE);
    assert_eq!(town.buildings[1].lights[1].color, town::Color::RED);
    assert_eq!(town.buildings[1].lights[2].color, town::Color::BLUE);
    assert_eq!(town.buildings[1].lights[3].color, town::Color::BLUE);
}

#[test]
fn message_diff() {
    let mut town = Rc::new(initial_town());
    let old_town = town.clone();
    let msg = client_api::Msg::SetLights {
        building_id: 1,
        light_ids: vec![0,2,3],
        color: town::Color::BLUE
    };

    update_town(&msg, &mut town);

    let messages = make_diff(&old_town, &town);
    let expected = vec![
        protobuf_init!(messages::Command::new(), {
            set_lights => {
                light_group: 1,
                light_id_start: 0,
                light_id_end: 1,
                color: messages::Color::BLUE
            }
        }),
        protobuf_init!(messages::Command::new(), {
            set_lights => {
                light_group: 1,
                light_id_start: 2,
                light_id_end: 4,
                color: messages::Color::BLUE
            }
        })
    ];

    for (m, e) in messages.iter().zip(&expected) {
        assert_eq!(*m, *e);
    }
    assert_eq!(messages, expected);
}
