use serde::{Serialize, Deserialize};

// variant attributes are applied on individual element of struct
#[derive(Serialize, Deserialize, Debug)]
enum Shape {
    #[serde(rename = "round")]
    Circle {radius: f32},
    #[serde(rename = "square")]
    Rectangle {width: f32, height: f32},
    #[serde(rename = "unknown")]
    ABC,
}

fn main () {
    let shapes: Vec<Shape> = vec![
        Shape::ABC,
        Shape::Circle {radius: 34.34},
        Shape::Rectangle {width: 12.40, height: 87.3},
    ];
    let serialized = serde_json::to_string(&shapes).unwrap();
    println!("{}", serialized);
}