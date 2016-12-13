extern crate serde;

use std::net::{SocketAddr};
use town;

#[derive(PartialEq, Debug, Deserialize)]
pub enum Msg {
    GetState,

    SetBuilding {
        #[serde(rename = "buildingId")]
        building_id: u8,
        color: town::Color
    },

    SetLights {
        #[serde(rename = "buildingId")]
        building_id: u8,
        #[serde(rename = "lightIds")]
        light_ids: Vec<u8>,
        color: town::Color
    },

    SetArduinoAddress {
        address: SocketAddr
    }
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    State {
        #[serde(rename = "arduinoAddress")]
        arduino_address: String,
        buildings: Vec<Building>
    }
}

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    #[serde(rename = "buildingId")]
    pub id: u8,
    pub lights: Vec<Light>
}

#[derive(Serialize, Deserialize)]
pub struct Light {
    #[serde(rename = "lightId")]
    pub id: u8,
    pub color: town::Color
}




#[cfg(test)]
mod tests {
    use serde_json;
    use std::net::{ToSocketAddrs};
    use super::*;
    use town;

    #[test]
    fn deserialize_get_state() {
        let msg = r##"{"GetState":null}"##;
        let msg : Msg = serde_json::from_str(msg).unwrap();
        let expected = Msg::GetState;
        assert_eq!(msg, expected);
    }

    #[test]
    fn deserialize_set_lights() {
        let msg = r##"{"SetLights":{"buildingId":0,"lightIds":[1,2,4],"color":"RED"}}"##;
        let msg : Msg = serde_json::from_str(msg).unwrap();
        let expected = Msg::SetLights{
            building_id: 0,
            light_ids: vec![1,2,4],
            color: town::Color::RED
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn deserialize_color() {
        let red = serde_json::from_str::<town::Color>("\"RED\"").unwrap();
        assert_eq!(red, town::Color::RED);

        let cyan : town::Color = serde_json::from_str("\"CYAN\"").unwrap();
        assert_eq!(cyan, town::Color::CYAN);

        let off : town::Color = serde_json::from_str("\"OFF\"").unwrap();
        assert_eq!(off, town::Color::OFF);

        assert!(serde_json::from_str::<town::Color>("\"INVALID\"").is_err());
    }

    #[test]
    fn serialize_color() {
        assert_eq!(serde_json::to_string(&town::Color::OFF).unwrap(), "\"OFF\"");
        assert_eq!(serde_json::to_string(&town::Color::RED).unwrap(), "\"RED\"");
        assert_eq!(serde_json::to_string(&town::Color::MAGENTA).unwrap(), "\"MAGENTA\"");
    }

    #[test]
    fn deserialize_set_arduino_address() {
        let msg = r##"{"SetArduinoAddress":{"address":"127.0.0.1:12345"}}"##;
        let msg : Msg = serde_json::from_str(msg).unwrap();
        let expected = Msg::SetArduinoAddress {
            address: "127.0.0.1:12345".to_socket_addrs().unwrap().next().unwrap()
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn serialize_response_state() {
        let state = Response::State {
            arduino_address: "127.0.0.1:12345".to_string(),
            buildings: Vec::new()
        };
        let state = serde_json::to_string(&state).unwrap();
        let expected = r##"{"State":{"arduinoAddress":"127.0.0.1:12345","buildings":[]}}"##;
        assert_eq!(state, expected);
    }

    #[test]
    fn serialize_building() {
        let building = Building {
            name: "Cafe Corner".to_string(),
            id: 0,
            lights: Vec::new()
        };
        let building = serde_json::to_string(&building).unwrap();
        let expected = r##"{"name":"Cafe Corner","buildingId":0,"lights":[]}"##;
        assert_eq!(building, expected);
    }

    #[test]
    fn serialize_light() {
        let light = Light {
            id: 0,
            color: town::Color::RED
        };
        let light = serde_json::to_string(&light).unwrap();
        let expected = r##"{"lightId":0,"color":"RED"}"##;
        assert_eq!(light, expected);
    }

}
