use town;

use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::io;
pub struct TownCommand {}

pub struct TownController {
    arduino_socket: UdpSocket,
    arduino_addr: SocketAddr,
    town: Arc<Mutex<town::Town>>,
    commands: mpsc::Receiver<TownCommand>
}

impl TownController {
    pub fn new<A: ToSocketAddrs>(arduino_addr: A,
                                 town: Arc<Mutex<town::Town>>,
                                 commands: mpsc::Receiver<TownCommand>
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
        let cmd = self.commands.recv().unwrap();
        println!("Received command (but not handled)")
    }
}
