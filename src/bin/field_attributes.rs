// * Field attributes in serde are used to customize
// the serialization of strut fields
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Employee {
    #[serde(rename = "empID")]
    id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(default)]
    age: u32,
}

fn main() {
    let emp = Employee {
        id: 1234,
        nickname: Some("hello".to_string()),
        age: 20,
    };

    let serialized = serde_json::to_string(&emp).unwrap();
    println!("{:?}", serialized);
}