use town;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

pub struct TownCommand {}

pub struct TownController {
    arduino_socket: UdpSocket,
    town: Arc<Mutex<town::Town>>,
    commands: mpsc::Receiver<TownCommand>
}

impl TownController {
    pub fn new(arduino_socket: UdpSocket,
               town: Arc<Mutex<town::Town>>,
               commands: mpsc::Receiver<TownCommand>) -> TownController {
        TownController {
            arduino_socket: arduino_socket,
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
