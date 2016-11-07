extern crate read_color;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::hex::ToHex;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Town {
    buildings: Vec<Building>
}

#[derive(RustcDecodable, RustcEncodable)]
struct Building {
    name: String,
    lights: Vec<Light>
}

struct Light {
    color: [u8; 3]
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