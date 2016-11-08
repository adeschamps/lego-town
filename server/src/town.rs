extern crate read_color;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::hex::ToHex;

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
    pub color: [u8; 3]
}

impl Decodable for Light {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Light", 0, |d| {
            d.read_struct_field("color", 0, D::read_str).and_then(|color| {
                // TODO: Replace this with FromHex
                read_color::rgb(color[1..].chars().by_ref())
                    .ok_or(d.error("Failed to parse color"))
            }).map(|color| {
                Light{
                    color: color
                }
            })
        })
    }
}

impl Encodable for Light {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(format!("#{}", self.color.to_hex()).as_str())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn decode_town() {
        let light = format!("{{\"color\":\"#ff0000\"}}");
        let cafe_corner = format!("{{\"name\":\"Cafe Corner\", \"lights\": [{}, {}]}}", light, light);
        let init_data = format!("{{\"buildings\":[{}]}}", cafe_corner);

        let town : Town = json::decode(init_data.as_str()).unwrap();
        let expected = Town {
            buildings: vec![
                Building {
                    name: "Cafe Corner".to_string(),
                    lights: vec![
                        Light { color: [255, 0, 0] },
                        Light { color: [255, 0, 0] }
                    ]
                }
            ]
        };
        assert_eq!(town, expected);
    }
}