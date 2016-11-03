use client_api;
use town;
use messages;

use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use protobuf::Message;

pub struct TownController {
    arduino_socket: UdpSocket,
    arduino_addr: SocketAddr,
    town: Arc<Mutex<town::Town>>,
    commands: mpsc::Receiver<client_api::Msg>
}

impl TownController {
    pub fn new<A: ToSocketAddrs>(arduino_addr: A,
                                 town: Arc<Mutex<town::Town>>,
                                 commands: mpsc::Receiver<client_api::Msg>
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
        for cmd in self.commands.iter() {
            println!("Received command");

            match cmd {
                client_api::Msg::Init => {}

                client_api::Msg::SetLight{..} => {
                    let mut c = messages::Command::new();
                    let mut sl = messages::SetLight::new();
                    c.set_set_light(sl);
                    self.send(c);
                    println!("Set light")
                },
                    client_api::Msg::SetBuilding{..} => {
                    let mut c = messages::Command::new();
                    let mut sg = messages::SetGroup::new();
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
