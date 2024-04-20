use std::collections::HashMap;

#[macro_use]
extern crate serde_json;

fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Tuvalu", "Funafuti");

    println!("The capital of Tonga is: {}", capitals["Tonga"]);

    let capitals_0 = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Nuku'alofa",
        "Tuvalu": "Funafuti",
    });

    println!("The capital of Tonga is: {}", capitals_0["Tonga"]);
}
