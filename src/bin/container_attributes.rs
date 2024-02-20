use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
// tag attribute add a new filed on serialized type
// with "argument given: enum field" format
#[serde(tag = "type > ")]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    ABC,
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 10.0 },
        Shape::Rectangle { width: 5.0, height: 20.0 },
        Shape::ABC,
    ];

    let serialized = serde_json::to_string(&shapes).unwrap();
    println!("Serialized: {}", serialized);

    // let deserialized: Vec<Shape> = serde_json::from_str(&serialized).unwrap();
    // println!("Deserialized: {:?}", deserialized);
}