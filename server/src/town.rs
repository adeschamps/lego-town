use messages;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Town {
    pub buildings: Vec<Rc<Building>>
}

#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Building {
    pub id: u8,
    pub name: String,
    pub lights: Vec<Rc<Light>>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Light {
    pub id: u8,
    pub color: messages::Color
}

impl Decodable for Light {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Light", 0, |d| Ok(Light{
            id:
                d.read_struct_field("id", 0, D::read_u8)?,
            color:
                d.read_struct_field("color", 1, messages::Color::decode)?
        }))
    }
}

impl Encodable for Light {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.color.encode(s)
    }
}

#[cfg(test)]
use rustc_serialize::json;

#[test]
fn decode_town() {
    let lights = r##"[{"id":0, "color":"RED"}, {"id":1, "color":"RED"}]"##;
    let cafe_corner = format!("{{\"id\":0, \"name\":\"Cafe Corner\", \"lights\": {}}}", lights);
    let init_data = format!("{{\"buildings\":[{}]}}", cafe_corner);

    let town : Town = json::decode(init_data.as_str()).unwrap();
    let expected = Town {
        buildings: vec![
            Rc::new(Building {
                id: 0,
                name: "Cafe Corner".to_string(),
                lights: vec![
                    Rc::new(Light{ id: 0, color: messages::Color::RED }),
                    Rc::new(Light{ id: 1, color: messages::Color::RED })
                ]
            })
        ]
    };
    assert_eq!(town, expected);
}
