use messages;

use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Town {
    pub buildings: Vec<Rc<Building>>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub id: u8,
    pub name: String,
    pub lights: Vec<Rc<Light>>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Light {
    pub id: u8,
    pub color: Color
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Color {
    OFF,
    WHITE,
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    CYAN,
    BLUE,
    PURPLE,
    MAGENTA,
}

impl Into<messages::Color> for Color {
    fn into(self) -> messages::Color {
        match self {
            Color::OFF => messages::Color::OFF,
            Color::WHITE => messages::Color::WHITE,
            Color::RED => messages::Color::RED,
            Color::ORANGE => messages::Color::ORANGE,
            Color::YELLOW => messages::Color::YELLOW,
            Color::GREEN => messages::Color::GREEN,
            Color::CYAN => messages::Color::CYAN,
            Color::BLUE => messages::Color::BLUE,
            Color::PURPLE => messages::Color::PURPLE,
            Color::MAGENTA => messages::Color::MAGENTA,
        }
    }
}


#[cfg(test)]
use serde_json;

#[test]
fn decode_town() {
    let lights = r##"[{"id":0, "color":"RED"}, {"id":1, "color":"RED"}]"##;
    let cafe_corner = format!("{{\"id\":0, \"name\":\"Cafe Corner\", \"lights\": {}}}", lights);
    let init_data = format!("{{\"buildings\":[{}]}}", cafe_corner);

    let town : Town = serde_json::from_str(init_data.as_str()).unwrap();
    let expected = Town {
        buildings: vec![
            Rc::new(Building {
                id: 0,
                name: "Cafe Corner".to_string(),
                lights: vec![
                    Rc::new(Light{ id: 0, color: Color::RED }),
                    Rc::new(Light{ id: 1, color: Color::RED })
                ]
            })
        ]
    };
    assert_eq!(town, expected);
}
