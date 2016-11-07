extern crate rustc_serialize;
extern crate try_from;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder, EncoderHelpers};
use self::rustc_serialize::json::Json;

pub enum Msg {
    Init,
    SetBuilding {
        building_id: u8,
        color: String
    },
    SetLight {
        building_id: u8,
        light_id: u8,
        color: String
    }
}


impl Decodable for Msg {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Msg", 0, |d| {
            d.read_struct_field("type", 0, D::read_str).and_then(|msg_type| {
                Ok(match msg_type.as_str() {
                    "init" => {
                        Msg::Init
                    }

                    "setBuilding" => {
                        let building_id =
                            try!(d.read_struct_field("buildingId", 0, D::read_u8));
                        let color =
                            try!(d.read_struct_field("color", 1, D::read_str));
                        Msg::SetBuilding {
                            building_id: building_id,
                            color: color
                        }
                    }

                    "setLight" => {
                        let building_id =
                            try!(d.read_struct_field("buildingId", 0, D::read_u8));
                        let light_id =
                            try!(d.read_struct_field("lightId", 1, D::read_u8));
                        let color =
                            try!(d.read_struct_field("color", 2, D::read_str));
                        Msg::SetLight {
                            building_id: building_id,
                            light_id: light_id,
                            color: color
                        }
                    }

                    _ => return Err(d.error(format!("Unknown message type: {}", msg_type).as_str()))
                })
            })
        })
    }
}



pub enum Response {
    State {
        buildings: Vec<Building>
    }
}

pub struct Building {
    pub name: String,
    pub id: u8,
    pub lights: Vec<Light>
}

pub struct Light {
    id: u8,
    color: String
}

impl Encodable for Response {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match *self {
            Response::State{ref buildings} => {
                s.emit_struct("State", 1, |s| {
                    try!(s.emit_struct_field("type", 0, |s| {
                        s.emit_str("state")
                    }));
                    try!(s.emit_struct_field("buildings", 1, |s| {
                        s.emit_from_vec(&buildings, |s, b| {
                            b.encode(s)
                        })
                    }));
                    Ok(())
                })
            }
        }
    }
}

impl Encodable for Building {
    fn encode<S:Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Building", 3, |s| {
            try!(s.emit_struct_field("name", 0, |s| {
                s.emit_str(self.name.as_str())
            }));
            try!(s.emit_struct_field("buildingId", 1, |s| {
                s.emit_u8(self.id)
            }));
            try!(s.emit_struct_field("lights", 2, |s| {
                s.emit_from_vec(&self.lights, |s, l| {
                    l.encode(s)
                })
            }));
            Ok(())
        })
    }
}

impl Encodable for Light {
    fn encode<S:Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Light", 2, |s| {
            try!(s.emit_struct_field("lightId", 0, |s| {
                s.emit_u8(self.id)
            }));
            try!(s.emit_struct_field("color", 1, |s| {
                s.emit_str(self.color.as_str())
            }));
            Ok(())
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn encode_response_state() {
        let state = Response::State {
            buildings: Vec::new()
        };
        let state = json::encode(&state).unwrap();
        let expected = r##"{"type":"state","buildings":[]}"##;
        assert_eq!(state, expected);
    }

    #[test]
    fn encode_building() {
        let building = Building {
            name: "Cafe Corner".to_string(),
            id: 0,
            lights: Vec::new()
        };
        let building = json::encode(&building).unwrap();
        let expected = r##"{"name":"Cafe Corner","buildingId":0,"lights":[]}"##;
        assert_eq!(building, expected);
    }

    #[test]
    fn encode_light() {
        let light = Light {
            id: 0,
            color: "#ff0000".to_string()
        };
        let light = json::encode(&light).unwrap();
        let expected = r##"{"lightId":0,"color":"#ff0000"}"##;
        assert_eq!(light, expected);
    }

}
