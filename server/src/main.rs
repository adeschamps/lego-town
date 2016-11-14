mod client;
mod client_api;
mod town;
mod town_controller;
mod messages;

extern crate protobuf;
extern crate rustc_serialize;
extern crate ws;

use rustc_serialize::json;
use std::fs::File;
use std::io::Read;
use std::net::{ToSocketAddrs};
use std::sync::mpsc;
use std::thread;
use town::Town;

fn main() {
    // Initialize town model
    let town = construct_town("init-data.json");
    println!("Initialized town: {}", json::as_pretty_json(&town));

    // Create town controller
    let (tx, rx) = mpsc::channel();
    let arduino_addr = "127.0.0.1:12345".to_string().to_socket_addrs().unwrap().next().unwrap();
    let mut town_controller = town_controller::TownController::new(arduino_addr, town, rx);
    thread::spawn(move || town_controller.run());

    // Listen for websocket connections
    println!("Listening for clients...");
    match ws::listen("0.0.0.0:1234", |out| {
        client::Client::new(out, tx.clone())
    }) {
        Ok(()) => {}
        Err(e) => panic!("Failed to create WebSocket server: {}", e)
    };
}

fn construct_town(filename: &str) -> Town {
    let mut f = match File::open(filename) {
        Err(e) => panic!("Couldn't open file: {}", e),
        Ok(file) => file
    };
    let mut init_data = String::new();
    match f.read_to_string(&mut init_data) {
        Err(e) => panic!("Failed to read file: {}", e),
        Ok(x) => x
    };
    let town = match json::decode(init_data.as_str()) {
        Err(e) => panic!("Failed to parse init data: {}", e),
        Ok(town) => town
    };
    town
}
