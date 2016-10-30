use json::{JsonValue};
use std::net::UdpSocket;

pub struct Town {
    buildings: Vec<Building>,
    socket: UdpSocket
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
    pub fn new(socket: UdpSocket, init_data: JsonValue) -> Result<Town, String> {
        let buildings = match Town::init_buildings(init_data) {
            Ok(buildings) => buildings,
            Err(e) => return Err(e)
        };
        Ok(Town{
            buildings: buildings,
            socket: socket
        })
    }

    fn init_buildings(init_data: JsonValue) -> Result<Vec<Building>, String> {
        Ok(
            init_data["buildings"].members().map(|b| Building{
                name: b["name"].to_string(),
                lights: b["lights"].members().map(|l| Light{
                    color: [0; 3]
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        )
    }

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
