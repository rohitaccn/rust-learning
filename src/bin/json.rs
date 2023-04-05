use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    addresses: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

fn main() {
    let json_str = r#"{
        "name": "John Smith",
        "age": 30,
        "addresses": [
            {
                "street": "123 Main St",
                "city": "Anytown",
                "state": "CA",
                "zip": "12345"
            },
            {
                "street": "456 Park Ave",
                "city": "Anytown",
                "state": "CA",
                "zip": "67890"
            }
        ]
    }"#;

    let person: Person = serde_json::from_str(json_str).unwrap();

    for address in &person.addresses {
        println!("{} {}", address.city, address.zip);
    }
}
