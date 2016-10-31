extern crate read_color;

use json::{JsonValue};
use std::net::UdpSocket;
use std::thread;
use std::time;
use std::rc::Rc;

pub struct Town {
    buildings: Vec<Building>
}

struct Building {
    name: String,
    lights: Vec<Light>
}

struct Light {
    color: [u8; 3]
}

impl Light {
    // TODO: Replace this with actual colour.
    fn color_as_hex_string(&self) -> String {
        "#ffffff".to_string()
    }
}

impl Town  {
    pub fn new(init_data: JsonValue) -> Result<Town, String> {
        let buildings = match Town::init_buildings(init_data) {
            Ok(buildings) => buildings,
            Err(e) => return Err(e)
        };
        Ok(Town{
            buildings: buildings
        })
    }

    fn init_buildings(init_data: JsonValue) -> Result<Vec<Building>, String> {
        Ok(
            init_data["buildings"].members().map(|b| Building{
                name: b["name"].to_string(),
                lights: b["lights"].members().map(|l| Light{
                    // TODO: Fix this horrible colour parsing
                    color: read_color::rgb(l["color"].as_str().unwrap()[1..].chars().by_ref()).unwrap()
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        )
    }

    /*
    pub fn add_client(&mut self, sender: Sender) {
        self.client_sockets.push(sender);
    }

    pub fn run(&self) {
        loop {
            println!("running...");
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    pub fn set_light(&self) {
//        for client in self.client_sockets {
//            client.send("test-message").unwrap();
//        }
    }
    */

    pub fn get_state(&self) -> JsonValue {
        object!{
            "type" => "initialize",
            "buildings" => self.buildings.iter().enumerate().map(|(i, b)| object!{
                "buildingId" => i,
                "name" => b.name.to_string(),
                "lights" => b.lights.iter().enumerate().map(|(i, l)| object!{
                    "lightId" => i,
                    "color" => l.color_as_hex_string()
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
         }
    }
}
