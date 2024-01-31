#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    println!("Starting struct program!");

    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
    };

    println!("Person: {:?}", person);
}