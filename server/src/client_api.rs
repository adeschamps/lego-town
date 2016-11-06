extern crate rustc_serialize;
extern crate try_from;

use self::rustc_serialize::json::Json;

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

impl try_from::TryFrom<Json> for Msg {
    type Err = String;
    fn try_from(msg: Json) -> Result<Self, Self::Err> {
        let msg = match msg.as_object() {
            Some(msg) => msg,
            Nothing => return Err(format!("Message is not a JSON object."))
        };

        let msg_type = match msg.get("type") {
            Some(t) => t,
            Nothing => return Err(format!("Message doesn't contain a \"type\" field."))
        };
        let msg_type = match msg_type.as_string() {
            Some(t) => t,
            Nothing => return Err(format!("\"type\" field is not a string."))
        };

        let msg = match msg_type {
            "init" => {
                Msg::Init
            }

            "setBuilding" => {
                let building_id = match msg.get("buildingId") {
                    Some(id) => id,
                    Nothing => return Err(format!("Missing \"buildingId\" field."))
                };
                let building_id = match building_id.as_i64() {
                    Some(id) => id as i32,
                    Nothing => return Err(format!("\"buildingId\" field is not an integer."))
                };
                let color = match msg.get("color") {
                    Some(color) => color,
                    Nothing => return Err(format!("Missing \"color\" field."))
                };
                let color = match color.as_string() {
                    Some(color) => color.to_string(),
                    Nothing => return Err(format!("\"color\" field is not a string"))
                };
                Msg::SetBuilding {
                    building_id: building_id,
                    color: color
                }
            }

            "setLight" => {
                let building_id = match msg.get("buildingId") {
                    Some(id) => id,
                    Nothing => return Err(format!("Missing \"buildingId\" field."))
                };
                let building_id = match building_id.as_i64() {
                    Some(id) => id as i32,
                    Nothing => return Err(format!("\"buildingId\" field is not an integer."))
                };
                let light_id = match msg.get("lightId") {
                    Some(id) => id,
                    Nothing => return Err(format!("Missing \"lightId\" field."))
                };
                let light_id = match light_id.as_i64() {
                    Some(id) => id as i32,
                    Nothing => return Err(format!("\"lightId\" field is not an integer."))
                };
                let color = match msg.get("color") {
                    Some(color) => color,
                    Nothing => return Err(format!("Missing \"color\" field."))
                };
                let color = match color.as_string() {
                    Some(color) => color.to_string(),
                    Nothing => return Err(format!("\"color\" field is not a string"))
                };
                Msg::SetLight {
                    building_id: building_id,
                    light_id: light_id,
                    color: color
                }
            }

            _ => return Err(format!("Unknown message type: {}", msg_type))
        };

        Ok(msg)
    }
}
