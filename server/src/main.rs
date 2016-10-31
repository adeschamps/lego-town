mod client;
mod town;
mod town_controller;

#[macro_use]
extern crate json;
extern crate ws;

use json::{JsonValue};
use std::fs::File;
use std::io::Read;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::sync::mpsc;

fn main() {
    let init_data = load_init_data("init-data.json");

    let arduino_addr = "127.0.0.1:12345";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let (tx, rx) = mpsc::channel();
    let town = town::Town::new(init_data).unwrap();
    let town = Arc::new(Mutex::new(town));

    let town_controller = town_controller::TownController::new(socket, town.clone(), rx);

    thread::spawn(move || town_controller.run());

    println!("Listening for clients...");
    ws::listen("0.0.0.0:1234", |out| {
        client::Client::new(out, town.clone(), tx.clone())
    }).unwrap();
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
    json::parse(s.as_ref()).unwrap()
}
