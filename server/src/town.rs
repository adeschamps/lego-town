extern crate read_color;

use messages;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Town {
    pub buildings: Vec<Building>
}

#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Building {
    pub name: String,
    pub lights: Vec<Light>
}

#[derive(Debug, PartialEq)]
pub struct Light {
    pub color: messages::Color
}

impl Decodable for Light {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Light", 0, |d| {
            d.read_struct_field("color", 0, messages::Color::decode)
        }).map(|color| {
            Light{
                color: color
            }
        })
    }
}

impl Encodable for Light {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.color.encode(s)
    }
}

#[cfg(test)]
mod tests {
    use messages;
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn decode_town() {
        let light = r##"{"color":"RED"}"##;
        let cafe_corner = format!("{{\"name\":\"Cafe Corner\", \"lights\": [{}, {}]}}", light, light);
        let init_data = format!("{{\"buildings\":[{}]}}", cafe_corner);

        let town : Town = json::decode(init_data.as_str()).unwrap();
        let expected = Town {
            buildings: vec![
                Building {
                    name: "Cafe Corner".to_string(),
                    lights: vec![
                        Light { color: messages::Color::RED },
                        Light { color: messages::Color::RED }
                    ]
                }
            ]
        };
        assert_eq!(town, expected);
    }
}