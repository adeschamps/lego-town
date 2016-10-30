mod town;
mod client;

#[macro_use]
extern crate json;
extern crate ws;

use json::{JsonValue};
use std::fs::File;
use std::io::Read;
use std::net::UdpSocket;

fn main() {
    let init_data = load_init_data("init-data.json");

    // Some test code for sending commands to the arduino
    let arduino_addr = "127.0.0.1:12345";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let msg = "hello";
    let buf = msg.as_bytes();
    socket.send_to(buf, arduino_addr).unwrap();
    println!("Sent msg: {}", msg);

    let town = town::Town::new(socket, init_data).unwrap();

    ws::listen("0.0.0.0:1234", |out| client::Client::new(out, &town)).unwrap();
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
    let init_data = json::parse(s.as_ref()).unwrap();
    init_data
}
