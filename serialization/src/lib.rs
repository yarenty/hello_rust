use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn ser() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    let mut map = BTreeMap::new();
    map.insert("x".to_string(), 3.0);
    map.insert("y".to_string(), 4.0);

    // Serialize the map into a pickle stream.
    // The second argument are serialization options.
    let serialized = serde_pickle::to_vec(&map, Default::default()).unwrap();

    println!(" Input = {:?}", map);
    println!(" Serialized picke = {:?}", serialized);
    // Deserialize the pickle stream back into a map.
    // Because we compare it to the original `map` below, Rust infers
    // the type of `deserialized` and lets serde work its magic.
    // The second argument are additional deserialization options.
    let deserialized = serde_pickle::from_slice(&serialized, Default::default()).unwrap();
    assert_eq!(map, deserialized);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
