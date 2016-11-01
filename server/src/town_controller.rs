use client_api;
use town;

use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

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
                    println!("Set light")
                },
                client_api::Msg::SetBuilding{..} => {
                    println!("Set Building")
                }
            };
        }
    }
}
