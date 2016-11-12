extern crate rustc_serialize;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder, EncoderHelpers};
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(PartialEq, Debug)]
pub enum Msg {
    GetState,

    SetBuilding {
        building_id: u8,
        color: String
    },

    SetLight {
        building_id: u8,
        light_id: u8,
        color: String
    },

    SetArduinoAddress {
        address: SocketAddr
    }
}


impl Decodable for Msg {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Msg", 0, |d| {
            d.read_struct_field("type", 0, D::read_str).and_then(|msg_type| {
                Ok(match msg_type.as_str() {
                    "getState" => {
                        Msg::GetState
                    }

                    "setBuilding" => {
                        Msg::SetBuilding {
                            building_id:
                                d.read_struct_field("buildingId", 0, D::read_u8)?,
                            color:
                                d.read_struct_field("color", 1, D::read_str)?
                        }
                    }

                    "setLight" => {
                        Msg::SetLight {
                            building_id:
                                d.read_struct_field("buildingId", 0, D::read_u8)?,
                            light_id:
                                d.read_struct_field("lightId", 1, D::read_u8)?,
                            color:
                                d.read_struct_field("color", 2, D::read_str)?
                        }
                    }

                    "setArduinoAddress" => {
                        Msg::SetArduinoAddress {
                            address:
                                d.read_struct_field("address", 0, D::read_str)?
                                .to_socket_addrs()
                                .map_err(|e| d.error(format!("Failed to parse address: {}", e).as_str()))?
                                .next().ok_or(d.error("No address was parsed"))?
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
        arduino_address: String,
        buildings: Vec<Building>
    }
}

pub struct Building {
    pub name: String,
    pub id: u8,
    pub lights: Vec<Light>
}

pub struct Light {
    pub id: u8,
    pub color: String
}

impl Encodable for Response {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match *self {
            Response::State{ref arduino_address, ref buildings} => {
                s.emit_struct("State", 1, |s| {
                    s.emit_struct_field("type", 0, |s| {
                        s.emit_str("state")
                    })?;
                    s.emit_struct_field("arduinoAddress", 1, |s| {
                        s.emit_str(arduino_address)
                    })?;
                    s.emit_struct_field("buildings", 2, |s| {
                        s.emit_from_vec(&buildings, |s, b| {
                            b.encode(s)
                        })
                    })?;
                    Ok(())
                })
            }
        }
    }
}

impl Encodable for Building {
    fn encode<S:Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Building", 3, |s| {
            s.emit_struct_field("name", 0, |s| {
                s.emit_str(self.name.as_str())
            })?;
            s.emit_struct_field("buildingId", 1, |s| {
                s.emit_u8(self.id)
            })?;
            s.emit_struct_field("lights", 2, |s| {
                s.emit_from_vec(&self.lights, |s, l| {
                    l.encode(s)
                })
            })?;
            Ok(())
        })
    }
}

impl Encodable for Light {
    fn encode<S:Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Light", 2, |s| {
            s.emit_struct_field("lightId", 0, |s| {
                s.emit_u8(self.id)
            })?;
            s.emit_struct_field("color", 1, |s| {
                s.emit_str(self.color.as_str())
            })?;
            Ok(())
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rustc_serialize::json;
    use std::net::{ToSocketAddrs};

    #[test]
    fn decode_init() {
        let msg = r##"{"type":"getState"}"##;
        let msg : Msg = json::decode(msg).unwrap();
        let expected = Msg::GetState;
        assert_eq!(msg, expected);
    }

    #[test]
    fn decode_set_building() {
        let msg = r##"{"type":"setBuilding","buildingId":0,"color":"#ff0000"}"##;
        let msg : Msg = json::decode(msg).unwrap();
        let expected = Msg::SetBuilding{
            building_id: 0,
            color: "#ff0000".to_string()
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn decode_set_light() {
        let msg = r##"{"type":"setLight","buildingId":0,"lightId":1,"color":"#ff0000"}"##;
        let msg : Msg = json::decode(msg).unwrap();
        let expected = Msg::SetLight{
            building_id: 0,
            light_id: 1,
            color: "#ff0000".to_string()
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn decode_set_arduino_address() {
        let msg = r##"{"type":"setArduinoAddress","address":"127.0.0.1:12345"}"##;
        let msg : Msg = json::decode(msg).unwrap();
        let expected = Msg::SetArduinoAddress {
            address: "127.0.0.1:12345".to_socket_addrs().unwrap().next().unwrap()
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn encode_response_state() {
        let state = Response::State {
            arduino_address: "127.0.0.1:12345".to_string(),
            buildings: Vec::new()
        };
        let state = json::encode(&state).unwrap();
        let expected = r##"{"type":"state","arduinoAddress":"127.0.0.1:12345","buildings":[]}"##;
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
