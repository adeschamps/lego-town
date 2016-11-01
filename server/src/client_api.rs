// use std::convert::{TryFrom};
extern crate try_from;
use json::{JsonValue};

pub enum Msg {
    Init,
    SetBuilding {
        building_id: i32,
        color: String
    },
    SetLight {
        building_id: i32,
        light_id: i32,
        color: String
    }
}

impl try_from::TryFrom<JsonValue> for Msg {
    type Err = String;
    fn try_from(msg: JsonValue) -> Result<Self, Self::Err> {
        let msg_type = match msg["type"].as_str() {
            Some(t) => t,
            None => return Err("Message does not contain a \"type\" field.".to_string())
        };

        match msg_type {
            "init" => {
                Ok(Msg::Init)
            },

            "setBuilding" => {
                let building_id = match msg["buildingId"].as_i32() {
                    Some(n) => n,
                    None => return Err(format!("Missing buildingId"))
                };
                let color = match msg["color"].as_str() {
                    Some(c) => c.to_string(),
                    None => return Err(format!("Missing color"))
                };
                Ok(Msg::SetBuilding{
                    building_id: building_id,
                    color: color
                })
            },

            "setLight" => {
                let building_id = match msg["buildingId"].as_i32() {
                    Some(n) => n,
                    None => return Err(format!("Missing buildingId"))
                };
                let light_id = match msg["lightId"].as_i32() {
                    Some(n) => n,
                    None => return Err(format!("Missing lightId"))
                };
                let color = match msg["color"].as_str() {
                    Some(c) => c.to_string(),
                    None => return Err(format!("Missing color"))
                };
                Ok(Msg::SetLight{
                    building_id: building_id,
                    light_id: light_id,
                    color: color
                })
            },

            t => Err(format!("Unsupported message type: {}", t))
        }
    }

}
