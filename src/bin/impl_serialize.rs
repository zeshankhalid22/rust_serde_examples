// * Serializing custom struct
use serde::ser::{Serialize, Serializer, SerializeStruct};

struct Person {
    name: String,
    age: u8,
    gpa: f32,
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut curr = serializer.serialize_struct("Person", 3)?;
        curr.serialize_field("name", &self.name)?;
        curr.serialize_field("age", &self.age)?;
        curr.serialize_field("gpa", &self.gpa)?;
        curr.end()
    }
}
fn main() {
    let p = Person {name: "Zeeshan".to_string(), age: 12, gpa: 2.3};
    let ser = serde_json::to_string(&p).unwrap();
    print!("{}", ser);
}