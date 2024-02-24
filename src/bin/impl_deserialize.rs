use std::fmt;
use serde::de::{self, Visitor};

use serde::Deserializer;

struct MyVisitor;

impl<'de> Visitor<'de> for MyVisitor {
    type Value = i32;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer b/w -2^31 and 2^31.")
    }
    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(i32::from(value))
    }
}

fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
{
    deserializer.deserialize_i32(MyVisitor)
}

fn main() {
    let json = r#"42"#;
    let des = &mut serde_json::Deserializer::from_str(json);
    let result: Result<i32, _> = des.deserialize_any(MyVisitor);
    match result {
        Ok(value) => println!("Deserialized value is -> {}", value),
        Err(e) => println!("Error {}", e),
    }
}