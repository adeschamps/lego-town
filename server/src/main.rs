#![feature(plugin)]
#![feature(proc_macro)]
#![plugin(protobuf_macros)]

mod client;
mod client_api;
mod town;
mod town_controller;
include!(concat!(env!("OUT_DIR"), "/messages.rs"));

#[macro_use]
extern crate clap;
extern crate itertools;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use town::Town;
use std::str::FromStr;

fn main() {
    let matches = App::new("LEGO Town Server")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Controls the lights in LEGO Town")
        .arg(Arg::with_name("arduino address")
             .short("a")
             .long("addr")
             .help("Sets the ip address of the arduino")
             .default_value("127.0.0.1:12345")
             .validator(|s| SocketAddr::from_str(&s).map(|_| ()).map_err(|e| e.to_string() )))
        .arg(Arg::with_name("config data")
             .short("c")
             .long("config")
             .help("Configuration data for the town")
             .default_value("config-data.json"))
        .get_matches();


    // Initialize town model
    let config_data = matches.value_of("config data").unwrap();
    let town = construct_town(config_data);
    println!("Initialized town: {}", serde_json::to_string_pretty(&town).unwrap());

    let (tx, rx) = mpsc::channel();
    // Listen for websocket connections
    thread::spawn(move || match ws::listen("0.0.0.0:1234", |out| {
        client::Client::new(out, tx.clone())
    }) {
        Ok(()) => {}
        Err(e) => panic!("Failed to create WebSocket server: {}", e)
    });
    println!("Listening for clients...");

    // Create town controller
    let arduino_addr = value_t!(matches, "arduino address", SocketAddr).unwrap();
    let mut town_controller = town_controller::TownController::new(arduino_addr, town);
    town_controller.run(rx);
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
    let town = match serde_json::from_str(&init_data) {
        Err(e) => panic!("Failed to parse init data: {}", e),
        Ok(town) => town
    };
    town
}
