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
            None => return Err(format!("Message is not a JSON object."))
        };

        let msg_type = match msg.get("type") {
            Some(t) => t,
            None => return Err(format!("Message doesn't contain a \"type\" field."))
        };
        let msg_type = match msg_type.as_string() {
            Some(t) => t,
            None => return Err(format!("\"type\" field is not a string."))
        };

        let msg = match msg_type {
            "init" => {
                Msg::Init
            }

            "setBuilding" => {
                let building_id =
                    try!(msg.get("buildingId").ok_or("Missing \"buildingId\""));
                let building_id =
                    try!(building_id.as_i64().ok_or("\"buildingId\" is not an integer")) as i32;

                let color =
                    try!(msg.get("color").ok_or("Missing \"color\""));
                let color =
                    try!(color.as_string().ok_or("\"color\" is not a string")).to_string();

                Msg::SetBuilding {
                    building_id: building_id,
                    color: color
                }
            }

            "setLight" => {
                let building_id =
                    try!(msg.get("buildingId").ok_or("Missing \"buildingId\""));
                let building_id =
                    try!(building_id.as_i64().ok_or("\"buildingId\" is not an integer")) as i32;

                let light_id =
                    try!(msg.get("lightId").ok_or("Missing \"lightId\""));
                let light_id =
                    try!(light_id.as_i64().ok_or("\"lightId\" is not an integer")) as i32;

                let color =
                    try!(msg.get("color").ok_or("Missing \"color\""));
                let color =
                    try!(color.as_string().ok_or("\"color\" is not a string")).to_string();

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
